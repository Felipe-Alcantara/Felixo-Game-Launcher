//! Contrato entre o núcleo e a interface: os tipos que atravessam a ponte do Tauri.
//!
//! Vivem fora do shell porque o executável de teste de um crate Tauri não abre no Windows
//! (falta o manifesto do Common Controls v6), e este contrato precisa de teste em todo sistema.
//! Os nomes aqui espelham `ui/src/api.ts`: mudar um lado exige mudar o outro.

use felixo_biblioteca::{ErroBiblioteca, Resumo};
use felixo_fontes::Deteccao;
use serde::Serialize;

/// Erro enviado à interface como `{ "codigo": "...", "detalhe": "..." }`.
#[derive(Debug, Serialize)]
#[serde(tag = "codigo", content = "detalhe", rename_all = "snake_case")]
pub enum ErroApp {
    Biblioteca(String),
    JogoNaoEncontrado,
    JogoIndisponivel,
    FonteDesconhecida(String),
    Abrir(String),
    Interno(String),
}

impl From<ErroBiblioteca> for ErroApp {
    fn from(erro: ErroBiblioteca) -> Self {
        match erro {
            ErroBiblioteca::JogoNaoEncontrado => Self::JogoNaoEncontrado,
            ErroBiblioteca::JogoIndisponivel => Self::JogoIndisponivel,
            ErroBiblioteca::FonteDesconhecida(fonte) => Self::FonteDesconhecida(fonte),
            ErroBiblioteca::Fonte(erro) => Self::Abrir(erro.to_string()),
            outro => Self::Biblioteca(outro.to_string()),
        }
    }
}

/// Resultado de atualizar a biblioteca, somado sobre todas as fontes.
#[derive(Debug, Default, PartialEq, Eq, Serialize)]
pub struct Atualizacao {
    pub detectados: usize,
    pub novos: usize,
    pub atualizados: usize,
    pub indisponiveis: usize,
    pub avisos: Vec<String>,
}

impl Atualizacao {
    /// Soma o resultado de uma fonte. Os avisos ganham o nome da fonte na frente, para a
    /// interface mostrar de onde veio cada um.
    pub fn somar(&mut self, nome_da_fonte: &str, deteccao: &Deteccao, resumo: &Resumo) {
        self.detectados += deteccao.jogos.len();
        self.novos += resumo.novos;
        self.atualizados += resumo.atualizados;
        self.indisponiveis += resumo.indisponiveis;
        self.avisos.extend(
            deteccao
                .avisos
                .iter()
                .map(|aviso| format!("{nome_da_fonte}: {aviso}")),
        );
    }
}

#[cfg(test)]
mod testes {
    use felixo_fontes::{ErroFonte, JogoDetectado};
    use serde_json::json;

    use super::*;

    #[test]
    fn erros_sem_detalhe_viram_so_o_codigo() {
        let json = serde_json::to_value(ErroApp::JogoIndisponivel).unwrap();
        assert_eq!(json, json!({ "codigo": "jogo_indisponivel" }));
        let json = serde_json::to_value(ErroApp::JogoNaoEncontrado).unwrap();
        assert_eq!(json, json!({ "codigo": "jogo_nao_encontrado" }));
    }

    #[test]
    fn erros_com_detalhe_levam_codigo_e_detalhe() {
        let json = serde_json::to_value(ErroApp::Biblioteca("disco cheio".into())).unwrap();
        assert_eq!(
            json,
            json!({ "codigo": "biblioteca", "detalhe": "disco cheio" })
        );
    }

    #[test]
    fn erros_da_biblioteca_sao_traduzidos_para_os_codigos_da_tela() {
        let de = |erro: ErroBiblioteca| serde_json::to_value(ErroApp::from(erro)).unwrap();
        assert_eq!(
            de(ErroBiblioteca::JogoIndisponivel),
            json!({ "codigo": "jogo_indisponivel" })
        );
        assert_eq!(
            de(ErroBiblioteca::JogoNaoEncontrado),
            json!({ "codigo": "jogo_nao_encontrado" })
        );
        assert_eq!(
            de(ErroBiblioteca::FonteDesconhecida("x".into())),
            json!({ "codigo": "fonte_desconhecida", "detalhe": "x" })
        );
        let da_fonte = de(ErroBiblioteca::Fonte(ErroFonte::IdInvalido("1;2".into())));
        assert_eq!(da_fonte["codigo"], "abrir");
        assert_eq!(de(ErroBiblioteca::VersaoFutura(9))["codigo"], "biblioteca");
    }

    #[test]
    fn atualizacao_tem_os_campos_que_a_tela_le() {
        let json = serde_json::to_value(Atualizacao::default()).unwrap();
        assert_eq!(
            json,
            json!({
                "detectados": 0, "novos": 0, "atualizados": 0,
                "indisponiveis": 0, "avisos": []
            })
        );
    }

    fn deteccao(jogos: usize, avisos: &[&str]) -> Deteccao {
        Deteccao {
            jogos: (0..jogos)
                .map(|i| JogoDetectado {
                    fonte: "x".into(),
                    id_externo: i.to_string(),
                    nome: format!("Jogo {i}"),
                    pasta: None,
                    tamanho_bytes: None,
                    disponivel: true,
                })
                .collect(),
            avisos: avisos.iter().map(|a| (*a).to_owned()).collect(),
        }
    }

    #[test]
    fn somar_acumula_as_fontes_e_prefixa_os_avisos_com_o_nome_delas() {
        let mut total = Atualizacao::default();

        let resumo = Resumo {
            novos: 2,
            atualizados: 1,
            indisponiveis: 1,
        };
        total.somar("Steam", &deteccao(3, &["disco ausente"]), &resumo);
        let resumo = Resumo {
            novos: 0,
            atualizados: 4,
            indisponiveis: 0,
        };
        total.somar("Epic", &deteccao(4, &["manifesto ruim", "outro"]), &resumo);

        assert_eq!(total.detectados, 7);
        assert_eq!(
            (total.novos, total.atualizados, total.indisponiveis),
            (2, 5, 1)
        );
        assert_eq!(
            total.avisos,
            [
                "Steam: disco ausente",
                "Epic: manifesto ruim",
                "Epic: outro"
            ]
        );
    }
}
