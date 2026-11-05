//! Biblioteca local de jogos em SQLite.
//!
//! Regra central: um jogo que some da detecção (disco desconectado, loja desinstalada)
//! fica **indisponível**, mas nunca é removido sozinho. Só o usuário decide apagar.

use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::Duration;

use felixo_fontes::{ErroFonte, Fonte, JogoDetectado};
use rusqlite::{params, Connection, OptionalExtension, Row};
use serde::{Deserialize, Serialize};

/// Versão do esquema que este código sabe usar. Bancos mais novos são recusados.
const VERSAO_ATUAL: i64 = 1;

const COLUNAS: &str = "id, fonte, id_externo, nome, pasta, tamanho_bytes, disponivel, visto_em";

#[derive(Debug, thiserror::Error)]
pub enum ErroBiblioteca {
    #[error("erro no banco de dados: {0}")]
    Banco(#[from] rusqlite::Error),

    #[error("não foi possível preparar a pasta da biblioteca: {0}")]
    Pasta(#[from] std::io::Error),

    #[error(
        "a biblioteca foi criada por uma versão mais nova do aplicativo (esquema {0}); \
         atualize o aplicativo para abri-la"
    )]
    VersaoFutura(i64),

    #[error("o jogo {jogo:?} é da fonte {encontrada:?}, não de {esperada:?}")]
    FonteDiferente {
        esperada: String,
        encontrada: String,
        jogo: String,
    },

    #[error("jogo não encontrado na biblioteca")]
    JogoNaoEncontrado,

    #[error("o jogo está indisponível agora (disco desconectado ou jogo removido)")]
    JogoIndisponivel,

    #[error("nenhuma fonte conhece {0:?}")]
    FonteDesconhecida(String),

    #[error(transparent)]
    Fonte(#[from] ErroFonte),
}

/// Um jogo já guardado na biblioteca.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Jogo {
    pub id: i64,
    pub fonte: String,
    pub id_externo: String,
    pub nome: String,
    pub pasta: Option<String>,
    pub tamanho_bytes: Option<u64>,
    pub disponivel: bool,
    /// Instante (segundos Unix) da última vez em que a detecção o encontrou.
    pub visto_em: i64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Ordem {
    #[default]
    Nome,
    TamanhoDecrescente,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Filtro {
    pub busca: Option<String>,
    pub ordem: Ordem,
}

/// O que uma sincronização mudou.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Resumo {
    pub novos: usize,
    pub atualizados: usize,
    /// Jogos da fonte que estão indisponíveis depois da sincronização (inclui os que sumiram).
    pub indisponiveis: usize,
}

pub struct Biblioteca {
    conexao: Connection,
}

impl Biblioteca {
    /// Abre (ou cria) a biblioteca no arquivo indicado.
    pub fn abrir(caminho: &Path) -> Result<Self, ErroBiblioteca> {
        if let Some(pasta) = caminho.parent() {
            fs::create_dir_all(pasta)?;
        }
        Self::preparar(Connection::open(caminho)?, true)
    }

    /// Biblioteca descartável, para testes.
    pub fn em_memoria() -> Result<Self, ErroBiblioteca> {
        Self::preparar(Connection::open_in_memory()?, false)
    }

    fn preparar(conexao: Connection, em_arquivo: bool) -> Result<Self, ErroBiblioteca> {
        // A varredura e o observador gravam ao mesmo tempo: WAL e espera em vez de "database is locked".
        conexao.busy_timeout(Duration::from_secs(5))?;
        if em_arquivo {
            conexao.query_row("PRAGMA journal_mode = WAL", [], |_| Ok(()))?;
        }
        migrar(&conexao)?;
        Ok(Self { conexao })
    }

    /// Reconcilia a biblioteca com o que uma fonte acabou de detectar, numa transação só.
    ///
    /// Jogos detectados são criados ou atualizados e ficam com a disponibilidade informada
    /// pela fonte; jogos da fonte que não vieram na detecção ficam indisponíveis.
    pub fn sincronizar(
        &mut self,
        fonte: &str,
        detectados: &[JogoDetectado],
        agora: i64,
    ) -> Result<Resumo, ErroBiblioteca> {
        if let Some(estranho) = detectados.iter().find(|jogo| jogo.fonte != fonte) {
            return Err(ErroBiblioteca::FonteDiferente {
                esperada: fonte.to_owned(),
                encontrada: estranho.fonte.clone(),
                jogo: estranho.nome.clone(),
            });
        }

        let transacao = self.conexao.transaction()?;
        let existentes: HashSet<String> = transacao
            .prepare("SELECT id_externo FROM jogos WHERE fonte = ?1")?
            .query_map([fonte], |linha| linha.get(0))?
            .collect::<Result<_, _>>()?;
        transacao.execute("UPDATE jogos SET disponivel = 0 WHERE fonte = ?1", [fonte])?;

        let mut resumo = Resumo::default();
        {
            let mut gravar = transacao.prepare(
                "INSERT INTO jogos (fonte, id_externo, nome, pasta, tamanho_bytes, disponivel, visto_em)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                 ON CONFLICT (fonte, id_externo) DO UPDATE SET
                   nome = excluded.nome, pasta = excluded.pasta,
                   tamanho_bytes = excluded.tamanho_bytes,
                   disponivel = excluded.disponivel, visto_em = excluded.visto_em",
            )?;
            for jogo in detectados {
                // shortcut: caminho não-UTF-8 é gravado com perda; tratar quando surgir um caso real.
                let pasta = jogo
                    .pasta
                    .as_ref()
                    .map(|p| p.to_string_lossy().into_owned());
                let tamanho = jogo.tamanho_bytes.and_then(|b| i64::try_from(b).ok());
                gravar.execute(params![
                    fonte,
                    jogo.id_externo,
                    jogo.nome,
                    pasta,
                    tamanho,
                    jogo.disponivel,
                    agora
                ])?;
                if existentes.contains(&jogo.id_externo) {
                    resumo.atualizados += 1;
                } else {
                    resumo.novos += 1;
                }
            }
        }
        let indisponiveis: i64 = transacao.query_row(
            "SELECT COUNT(*) FROM jogos WHERE fonte = ?1 AND disponivel = 0",
            [fonte],
            |linha| linha.get(0),
        )?;
        resumo.indisponiveis = usize::try_from(indisponiveis).unwrap_or(0);
        transacao.commit()?;
        Ok(resumo)
    }

    pub fn listar(&self, filtro: &Filtro) -> Result<Vec<Jogo>, ErroBiblioteca> {
        let padrao = filtro
            .busca
            .as_deref()
            .map(str::trim)
            .filter(|busca| !busca.is_empty())
            .map(padrao_like);
        // O ORDER BY vem de um enum, nunca de texto do usuário.
        let ordem = match filtro.ordem {
            Ordem::Nome => "nome COLLATE NOCASE, id",
            Ordem::TamanhoDecrescente => {
                "tamanho_bytes IS NULL, tamanho_bytes DESC, nome COLLATE NOCASE, id"
            }
        };
        let sql = format!(
            "SELECT {COLUNAS} FROM jogos \
             WHERE (?1 IS NULL OR nome LIKE ?1 ESCAPE '\\') ORDER BY {ordem}"
        );
        let mut consulta = self.conexao.prepare(&sql)?;
        let jogos = consulta
            .query_map([padrao], ler_jogo)?
            .collect::<Result<_, _>>()?;
        Ok(jogos)
    }

    pub fn obter(&self, id: i64) -> Result<Option<Jogo>, ErroBiblioteca> {
        let sql = format!("SELECT {COLUNAS} FROM jogos WHERE id = ?1");
        Ok(self.conexao.query_row(&sql, [id], ler_jogo).optional()?)
    }

    /// Abre um jogo da biblioteca pelo launcher da fonte de onde ele veio.
    ///
    /// Jogo indisponível não abre: o launcher original tentaria reinstalá-lo ou falharia de
    /// forma confusa, e quem decide o que fazer com um disco ausente é o usuário.
    pub fn abrir_jogo(&self, id: i64, fontes: &[Box<dyn Fonte>]) -> Result<(), ErroBiblioteca> {
        let jogo = self.obter(id)?.ok_or(ErroBiblioteca::JogoNaoEncontrado)?;
        if !jogo.disponivel {
            return Err(ErroBiblioteca::JogoIndisponivel);
        }
        let fonte = fontes
            .iter()
            .find(|fonte| fonte.id() == jogo.fonte)
            .ok_or_else(|| ErroBiblioteca::FonteDesconhecida(jogo.fonte.clone()))?;
        fonte.abrir(&jogo.id_externo)?;
        Ok(())
    }
}

fn migrar(conexao: &Connection) -> Result<(), ErroBiblioteca> {
    let versao: i64 = conexao.query_row("PRAGMA user_version", [], |linha| linha.get(0))?;
    if versao > VERSAO_ATUAL {
        return Err(ErroBiblioteca::VersaoFutura(versao));
    }
    if versao < 1 {
        conexao.execute_batch(
            "BEGIN;
             CREATE TABLE jogos (
               id INTEGER PRIMARY KEY,
               fonte TEXT NOT NULL,
               id_externo TEXT NOT NULL,
               nome TEXT NOT NULL,
               pasta TEXT,
               tamanho_bytes INTEGER,
               disponivel INTEGER NOT NULL DEFAULT 1,
               visto_em INTEGER NOT NULL,
               UNIQUE (fonte, id_externo)
             );
             PRAGMA user_version = 1;
             COMMIT;",
        )?;
    }
    Ok(())
}

fn ler_jogo(linha: &Row<'_>) -> rusqlite::Result<Jogo> {
    Ok(Jogo {
        id: linha.get(0)?,
        fonte: linha.get(1)?,
        id_externo: linha.get(2)?,
        nome: linha.get(3)?,
        pasta: linha.get(4)?,
        tamanho_bytes: linha
            .get::<_, Option<i64>>(5)?
            .and_then(|bytes| u64::try_from(bytes).ok()),
        disponivel: linha.get(6)?,
        visto_em: linha.get(7)?,
    })
}

/// Transforma o texto do usuário num padrão `LIKE` que procura o texto literalmente:
/// `%` e `_` digitados não viram curinga.
// shortcut: o LIKE do SQLite ignora maiúsculas só em ASCII; busca sem acento fica para depois.
fn padrao_like(busca: &str) -> String {
    let escapado = busca
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_");
    format!("%{escapado}%")
}

#[cfg(test)]
mod testes {
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};

    use felixo_fontes::Deteccao;
    use rusqlite::Connection;
    use tempfile::TempDir;

    use super::*;

    fn detectado(
        fonte: &str,
        id: &str,
        nome: &str,
        tamanho: Option<u64>,
        disponivel: bool,
    ) -> JogoDetectado {
        JogoDetectado {
            fonte: fonte.to_owned(),
            id_externo: id.to_owned(),
            nome: nome.to_owned(),
            pasta: Some(PathBuf::from(format!("/jogos/{nome}"))),
            tamanho_bytes: tamanho,
            disponivel,
        }
    }

    fn nomes(biblioteca: &Biblioteca, filtro: &Filtro) -> Vec<String> {
        biblioteca
            .listar(filtro)
            .unwrap()
            .into_iter()
            .map(|jogo| jogo.nome)
            .collect()
    }

    #[test]
    fn primeira_sincronizacao_cria_os_jogos() {
        let mut b = Biblioteca::em_memoria().unwrap();
        let resumo = b
            .sincronizar(
                "steam",
                &[
                    detectado("steam", "1", "Alfa", Some(100), true),
                    detectado("steam", "2", "Beta", None, true),
                ],
                1_000,
            )
            .unwrap();

        assert_eq!(
            resumo,
            Resumo {
                novos: 2,
                atualizados: 0,
                indisponiveis: 0
            }
        );
        let jogos = b.listar(&Filtro::default()).unwrap();
        assert_eq!(jogos.len(), 2);
        assert_eq!(jogos[0].nome, "Alfa");
        assert_eq!(jogos[0].tamanho_bytes, Some(100));
        assert_eq!(jogos[0].pasta.as_deref(), Some("/jogos/Alfa"));
        assert_eq!(jogos[0].visto_em, 1_000);
    }

    #[test]
    fn jogo_que_some_fica_indisponivel_e_nunca_e_removido() {
        let mut b = Biblioteca::em_memoria().unwrap();
        b.sincronizar(
            "steam",
            &[
                detectado("steam", "1", "Alfa", Some(1), true),
                detectado("steam", "2", "Beta", Some(2), true),
            ],
            1_000,
        )
        .unwrap();

        let resumo = b
            .sincronizar(
                "steam",
                &[detectado("steam", "1", "Alfa", Some(1), true)],
                2_000,
            )
            .unwrap();

        assert_eq!(resumo.indisponiveis, 1);
        let jogos = b.listar(&Filtro::default()).unwrap();
        assert_eq!(jogos.len(), 2, "o jogo ausente não pode ser apagado");
        let beta = jogos.iter().find(|j| j.nome == "Beta").unwrap();
        assert!(!beta.disponivel);
        assert_eq!(beta.visto_em, 1_000, "guarda a última vez em que foi visto");
    }

    #[test]
    fn jogo_que_volta_fica_disponivel_de_novo() {
        let mut b = Biblioteca::em_memoria().unwrap();
        b.sincronizar(
            "steam",
            &[detectado("steam", "1", "Alfa", Some(1), true)],
            1,
        )
        .unwrap();
        b.sincronizar("steam", &[], 2).unwrap();
        let resumo = b
            .sincronizar(
                "steam",
                &[detectado("steam", "1", "Alfa", Some(1), true)],
                3,
            )
            .unwrap();

        assert_eq!(resumo.atualizados, 1);
        assert_eq!(resumo.indisponiveis, 0);
        assert!(b.listar(&Filtro::default()).unwrap()[0].disponivel);
    }

    #[test]
    fn jogo_detectado_sem_pasta_entra_como_indisponivel() {
        let mut b = Biblioteca::em_memoria().unwrap();
        let resumo = b
            .sincronizar("steam", &[detectado("steam", "1", "Alfa", None, false)], 1)
            .unwrap();

        assert_eq!(resumo.novos, 1);
        assert_eq!(resumo.indisponiveis, 1);
    }

    #[test]
    fn ressincronizar_atualiza_nome_e_tamanho() {
        let mut b = Biblioteca::em_memoria().unwrap();
        b.sincronizar(
            "steam",
            &[detectado("steam", "1", "Velho", Some(1), true)],
            1,
        )
        .unwrap();
        b.sincronizar(
            "steam",
            &[detectado("steam", "1", "Novo", Some(9), true)],
            2,
        )
        .unwrap();

        let jogos = b.listar(&Filtro::default()).unwrap();
        assert_eq!(jogos.len(), 1);
        assert_eq!(
            (jogos[0].nome.as_str(), jogos[0].tamanho_bytes),
            ("Novo", Some(9))
        );
    }

    #[test]
    fn fontes_sao_independentes() {
        let mut b = Biblioteca::em_memoria().unwrap();
        b.sincronizar(
            "steam",
            &[detectado("steam", "1", "DaSteam", Some(1), true)],
            1,
        )
        .unwrap();
        b.sincronizar(
            "epic",
            &[detectado("epic", "1", "DaEpic", Some(1), true)],
            1,
        )
        .unwrap();

        b.sincronizar("epic", &[], 2).unwrap();

        let jogos = b.listar(&Filtro::default()).unwrap();
        let da_steam = jogos.iter().find(|j| j.fonte == "steam").unwrap();
        assert!(
            da_steam.disponivel,
            "sincronizar a Epic não pode mexer na Steam"
        );
    }

    #[test]
    fn recusa_jogo_de_outra_fonte() {
        let mut b = Biblioteca::em_memoria().unwrap();
        let erro = b
            .sincronizar("steam", &[detectado("epic", "1", "Intruso", None, true)], 1)
            .unwrap_err();

        assert!(matches!(erro, ErroBiblioteca::FonteDiferente { .. }));
        assert!(b.listar(&Filtro::default()).unwrap().is_empty());
    }

    #[test]
    fn busca_ignora_maiusculas_e_trata_curingas_como_texto() {
        let mut b = Biblioteca::em_memoria().unwrap();
        b.sincronizar(
            "steam",
            &[
                detectado("steam", "1", "Half-Life", None, true),
                detectado("steam", "2", "100% Orange Juice", None, true),
                detectado("steam", "3", "Snake_Game", None, true),
                detectado("steam", "4", "Snake Pit", None, true),
            ],
            1,
        )
        .unwrap();
        let com = |busca: &str| {
            nomes(
                &b,
                &Filtro {
                    busca: Some(busca.to_owned()),
                    ..Filtro::default()
                },
            )
        };

        assert_eq!(com("half"), ["Half-Life"]);
        assert_eq!(com("100%"), ["100% Orange Juice"]);
        assert_eq!(com("e_g"), ["Snake_Game"], "o _ digitado não é curinga");
        assert_eq!(com("%").len(), 1, "o % digitado não é curinga");
        assert_eq!(com("   ").len(), 4, "busca em branco lista tudo");
    }

    #[test]
    fn ordena_por_tamanho_com_desconhecidos_no_fim() {
        let mut b = Biblioteca::em_memoria().unwrap();
        b.sincronizar(
            "steam",
            &[
                detectado("steam", "1", "Pequeno", Some(10), true),
                detectado("steam", "2", "Sem tamanho", None, true),
                detectado("steam", "3", "Grande", Some(500), true),
            ],
            1,
        )
        .unwrap();

        let filtro = Filtro {
            ordem: Ordem::TamanhoDecrescente,
            ..Filtro::default()
        };
        assert_eq!(nomes(&b, &filtro), ["Grande", "Pequeno", "Sem tamanho"]);
    }

    #[test]
    fn ordena_por_nome_sem_diferenciar_maiusculas() {
        let mut b = Biblioteca::em_memoria().unwrap();
        b.sincronizar(
            "steam",
            &[
                detectado("steam", "1", "banana", None, true),
                detectado("steam", "2", "Abacaxi", None, true),
                detectado("steam", "3", "Cereja", None, true),
            ],
            1,
        )
        .unwrap();

        assert_eq!(
            nomes(&b, &Filtro::default()),
            ["Abacaxi", "banana", "Cereja"]
        );
    }

    #[test]
    fn obter_devolve_o_jogo_ou_nada() {
        let mut b = Biblioteca::em_memoria().unwrap();
        b.sincronizar("steam", &[detectado("steam", "1", "Alfa", None, true)], 1)
            .unwrap();
        let id = b.listar(&Filtro::default()).unwrap()[0].id;

        assert_eq!(b.obter(id).unwrap().unwrap().nome, "Alfa");
        assert!(b.obter(id + 100).unwrap().is_none());
    }

    #[test]
    fn dados_sobrevivem_a_fechar_e_reabrir_o_arquivo() {
        let tmp = TempDir::new().unwrap();
        let caminho = tmp.path().join("pasta/nova/biblioteca.db");
        {
            let mut b = Biblioteca::abrir(&caminho).unwrap();
            b.sincronizar(
                "steam",
                &[detectado("steam", "1", "Alfa", Some(7), true)],
                1,
            )
            .unwrap();
        }

        let b = Biblioteca::abrir(&caminho).unwrap();
        let jogos = b.listar(&Filtro::default()).unwrap();
        assert_eq!(jogos.len(), 1);
        assert_eq!(jogos[0].tamanho_bytes, Some(7));
    }

    #[test]
    fn recusa_biblioteca_de_versao_futura_sem_alterar_o_arquivo() {
        let tmp = TempDir::new().unwrap();
        let caminho = tmp.path().join("biblioteca.db");
        Connection::open(&caminho)
            .unwrap()
            .execute_batch("PRAGMA user_version = 99")
            .unwrap();

        let erro = Biblioteca::abrir(&caminho).err().unwrap();

        assert!(matches!(erro, ErroBiblioteca::VersaoFutura(99)));
        let versao: i64 = Connection::open(&caminho)
            .unwrap()
            .query_row("PRAGMA user_version", [], |l| l.get(0))
            .unwrap();
        assert_eq!(versao, 99);
    }

    /// Fonte de mentira que só anota o que lhe pediram para abrir.
    struct FonteFalsa {
        id: &'static str,
        falha: bool,
        abertos: Arc<Mutex<Vec<String>>>,
    }

    impl Fonte for FonteFalsa {
        fn id(&self) -> &'static str {
            self.id
        }
        fn nome(&self) -> &'static str {
            "Falsa"
        }
        fn detectar(&self) -> Deteccao {
            Deteccao::default()
        }
        fn abrir(&self, id_externo: &str) -> Result<(), ErroFonte> {
            if self.falha {
                return Err(ErroFonte::IdInvalido(id_externo.to_owned()));
            }
            self.abertos.lock().unwrap().push(id_externo.to_owned());
            Ok(())
        }
    }

    fn biblioteca_com_um_jogo(disponivel: bool) -> (Biblioteca, i64) {
        let mut b = Biblioteca::em_memoria().unwrap();
        b.sincronizar(
            "falsa",
            &[detectado("falsa", "777", "Alfa", None, disponivel)],
            1,
        )
        .unwrap();
        let id = b.listar(&Filtro::default()).unwrap()[0].id;
        (b, id)
    }

    type Abertos = Arc<Mutex<Vec<String>>>;

    fn fontes(falha: bool) -> (Vec<Box<dyn Fonte>>, Abertos) {
        let abertos = Arc::new(Mutex::new(Vec::new()));
        let fonte = FonteFalsa {
            id: "falsa",
            falha,
            abertos: Arc::clone(&abertos),
        };
        (vec![Box::new(fonte)], abertos)
    }

    #[test]
    fn abrir_jogo_chama_a_fonte_com_o_id_externo() {
        let (b, id) = biblioteca_com_um_jogo(true);
        let (fontes, abertos) = fontes(false);

        b.abrir_jogo(id, &fontes).unwrap();

        assert_eq!(*abertos.lock().unwrap(), ["777"]);
    }

    #[test]
    fn abrir_jogo_indisponivel_e_recusado_sem_chamar_a_fonte() {
        let (b, id) = biblioteca_com_um_jogo(false);
        let (fontes, abertos) = fontes(false);

        let erro = b.abrir_jogo(id, &fontes).unwrap_err();

        assert!(matches!(erro, ErroBiblioteca::JogoIndisponivel));
        assert!(abertos.lock().unwrap().is_empty());
    }

    #[test]
    fn abrir_jogo_que_nao_existe_e_recusado() {
        let (b, id) = biblioteca_com_um_jogo(true);
        let (fontes, _) = fontes(false);

        let erro = b.abrir_jogo(id + 1, &fontes).unwrap_err();

        assert!(matches!(erro, ErroBiblioteca::JogoNaoEncontrado));
    }

    #[test]
    fn abrir_jogo_de_fonte_que_nao_existe_mais_e_recusado() {
        let (b, id) = biblioteca_com_um_jogo(true);

        let erro = b.abrir_jogo(id, &[]).unwrap_err();

        assert!(matches!(erro, ErroBiblioteca::FonteDesconhecida(f) if f == "falsa"));
    }

    #[test]
    fn abrir_jogo_propaga_o_erro_da_fonte() {
        let (b, id) = biblioteca_com_um_jogo(true);
        let (fontes, _) = fontes(true);

        let erro = b.abrir_jogo(id, &fontes).unwrap_err();

        assert!(matches!(
            erro,
            ErroBiblioteca::Fonte(ErroFonte::IdInvalido(_))
        ));
    }

    #[test]
    fn duas_conexoes_gravam_sem_travar() {
        let tmp = TempDir::new().unwrap();
        let caminho = tmp.path().join("biblioteca.db");
        let mut a = Biblioteca::abrir(&caminho).unwrap();
        let mut b = Biblioteca::abrir(&caminho).unwrap();

        a.sincronizar("steam", &[detectado("steam", "1", "Alfa", None, true)], 1)
            .unwrap();
        b.sincronizar("epic", &[detectado("epic", "1", "Beta", None, true)], 1)
            .unwrap();

        assert_eq!(a.listar(&Filtro::default()).unwrap().len(), 2);
    }
}
