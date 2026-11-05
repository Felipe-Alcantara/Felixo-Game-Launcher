//! Fonte Steam: lê `libraryfolders.vdf` e os `appmanifest_*.acf` de cada biblioteca.

use std::collections::HashSet;
use std::fs;
use std::io::ErrorKind;
use std::path::{Component, Path, PathBuf};
use std::thread;

use crate::vdf::{self, Valor};
use crate::{Deteccao, ErroFonte, Fonte, JogoDetectado};

const ID_FONTE: &str = "steam";

/// Bit de `StateFlags` que indica instalação completa. Os demais bits (atualização
/// pendente, pausada etc.) podem vir ligados junto sem que o jogo deixe de estar instalado.
const INSTALADO: u64 = 4;

/// Itens que a Steam instala como "apps", mas que não são jogos.
const APPIDS_IGNORADOS: &[&str] = &["228980"]; // Steamworks Common Redistributables
const PREFIXOS_IGNORADOS: &[&str] = &[
    "Steamworks Common Redistributables",
    "Proton ",
    "Steam Linux Runtime",
];

pub struct Steam {
    raizes: Vec<PathBuf>,
}

impl Steam {
    /// Procura a Steam onde o sistema costuma guardá-la (registro no Windows, pastas
    /// padrão no Linux e no macOS).
    pub fn do_sistema() -> Self {
        Self {
            raizes: raizes_candidatas(),
        }
    }

    /// Usa pastas de instalação já conhecidas; serve a testes e a pastas indicadas pelo usuário.
    pub fn com_raizes(raizes: Vec<PathBuf>) -> Self {
        Self { raizes }
    }
}

impl Fonte for Steam {
    fn id(&self) -> &'static str {
        ID_FONTE
    }

    fn nome(&self) -> &'static str {
        "Steam"
    }

    fn detectar(&self) -> Deteccao {
        let mut resultado = Deteccao::default();
        let Some(raiz) = self.raizes.iter().find(|r| r.join("steamapps").is_dir()) else {
            return resultado; // Steam não instalada: lista vazia, não é erro
        };
        let bibliotecas = bibliotecas(raiz, &mut resultado.avisos);
        // Cada biblioteca costuma estar num disco diferente, e um disco ausente leva segundos
        // para responder. Ler uma de cada vez somaria as esperas; lê todas ao mesmo tempo.
        let leituras: Vec<Deteccao> = thread::scope(|escopo| {
            let tarefas: Vec<_> = bibliotecas
                .iter()
                .map(|biblioteca| escopo.spawn(move || ler_biblioteca(biblioteca)))
                .collect();
            tarefas
                .into_iter()
                .map(|tarefa| {
                    tarefa.join().unwrap_or_else(|_| Deteccao {
                        avisos: vec!["falha inesperada ao ler uma biblioteca".to_owned()],
                        ..Deteccao::default()
                    })
                })
                .collect()
        });
        let mut vistos = HashSet::new();
        for leitura in leituras {
            resultado.avisos.extend(leitura.avisos);
            for jogo in leitura.jogos {
                if vistos.insert(jogo.id_externo.clone()) {
                    resultado.jogos.push(jogo);
                }
            }
        }
        resultado.jogos.sort_by_key(|jogo| jogo.nome.to_lowercase());
        resultado
    }

    fn abrir(&self, id_externo: &str) -> Result<(), ErroFonte> {
        // O id vira parte de uma URL: só dígitos, para ninguém injetar outro destino.
        if !id_numerico(id_externo) {
            return Err(ErroFonte::IdInvalido(id_externo.to_owned()));
        }
        open::that_detached(format!("steam://rungameid/{id_externo}")).map_err(ErroFonte::Abrir)
    }
}

fn id_numerico(id: &str) -> bool {
    !id.is_empty() && id.bytes().all(|b| b.is_ascii_digit())
}

/// A pasta da Steam e as demais bibliotecas listadas em `libraryfolders.vdf`, sem repetir.
fn bibliotecas(raiz: &Path, avisos: &mut Vec<String>) -> Vec<PathBuf> {
    let mut lista = vec![raiz.to_path_buf()];
    let mut chaves = HashSet::from([chave_da_pasta(raiz)]);
    let arquivo = raiz.join("steamapps").join("libraryfolders.vdf");
    match fs::read_to_string(&arquivo) {
        Ok(texto) => match vdf::analisar(&texto) {
            Ok(valor) => {
                for caminho in caminhos_das_bibliotecas(&valor) {
                    if chaves.insert(chave_da_pasta(&caminho)) {
                        lista.push(caminho);
                    }
                }
            }
            Err(erro) => avisos.push(format!("libraryfolders.vdf ilegível: {erro}")),
        },
        Err(erro) if erro.kind() == ErrorKind::NotFound => {}
        Err(erro) => avisos.push(format!("não consegui ler libraryfolders.vdf: {erro}")),
    }
    lista
}

/// Chave para reconhecer a mesma pasta escrita de jeitos diferentes (o registro do Windows
/// guarda `c:/steam` e o VDF guarda `C:\Steam`). É só texto, de propósito: perguntar ao disco
/// (`canonicalize`) custa segundos por consulta quando a unidade está ausente.
// shortcut: não resolve links simbólicos; dois links para a mesma pasta só repetem avisos.
fn chave_da_pasta(pasta: &Path) -> String {
    let texto = pasta.to_string_lossy();
    if cfg!(windows) {
        texto
            .replace('\\', "/")
            .trim_end_matches('/')
            .to_lowercase()
    } else {
        texto.trim_end_matches('/').to_owned()
    }
}

fn caminhos_das_bibliotecas(documento: &Valor) -> Vec<PathBuf> {
    let Some(raiz) = documento.filho("libraryfolders") else {
        return Vec::new();
    };
    raiz.filhos()
        .iter()
        .filter(|(chave, _)| !chave.is_empty() && chave.bytes().all(|b| b.is_ascii_digit()))
        .filter_map(|(_, valor)| match valor {
            Valor::Mapa(_) => valor.filho("path").and_then(Valor::texto),
            Valor::Texto(caminho) => Some(caminho.as_str()), // formato antigo: "1" "D:\\Jogos"
        })
        .map(PathBuf::from)
        .collect()
}

/// Lê os manifestos de uma biblioteca. Uma única sondagem do disco: se `steamapps` não abre,
/// a biblioteca vira aviso e nada mais é consultado nela.
fn ler_biblioteca(biblioteca: &Path) -> Deteccao {
    let mut resultado = Deteccao::default();
    let apps = biblioteca.join("steamapps");
    let Ok(entradas) = fs::read_dir(&apps) else {
        resultado.avisos.push(format!(
            "Biblioteca indisponível: {} (disco desconectado?)",
            biblioteca.display()
        ));
        return resultado;
    };
    for entrada in entradas.flatten() {
        let nome = entrada.file_name();
        let Some(nome) = nome.to_str() else { continue };
        if !(nome.starts_with("appmanifest_") && nome.ends_with(".acf")) {
            continue;
        }
        match ler_manifesto(&entrada.path(), &apps) {
            Ok(Some(jogo)) => resultado.jogos.push(jogo),
            Ok(None) => {}
            Err(aviso) => resultado.avisos.push(aviso),
        }
    }
    resultado
}

/// `Ok(None)` quando o app existe mas não é um jogo instalado (em download, redistribuível).
fn ler_manifesto(arquivo: &Path, apps: &Path) -> Result<Option<JogoDetectado>, String> {
    let rotulo = arquivo
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let texto = fs::read_to_string(arquivo).map_err(|e| format!("{rotulo}: {e}"))?;
    let documento = vdf::analisar(&texto).map_err(|e| format!("{rotulo}: {e}"))?;
    let estado = documento
        .filho("AppState")
        .ok_or_else(|| format!("{rotulo}: sem o bloco AppState"))?;
    let campo = |chave: &str| estado.filho(chave).and_then(Valor::texto);

    let (Some(appid), Some(nome), Some(instalacao)) =
        (campo("appid"), campo("name"), campo("installdir"))
    else {
        return Err(format!("{rotulo}: faltam appid, name ou installdir"));
    };
    if !id_numerico(appid) {
        return Err(format!("{rotulo}: appid inválido"));
    }
    if !pasta_simples(instalacao) {
        return Err(format!("{rotulo}: installdir fora do padrão"));
    }

    let flags = campo("StateFlags")
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);
    if flags & INSTALADO == 0 || e_ignorado(appid, nome) {
        return Ok(None);
    }

    let pasta = apps.join("common").join(instalacao);
    Ok(Some(JogoDetectado {
        fonte: ID_FONTE.to_owned(),
        id_externo: appid.to_owned(),
        nome: nome.to_owned(),
        tamanho_bytes: campo("SizeOnDisk")
            .and_then(|v| v.parse::<u64>().ok())
            .filter(|&bytes| bytes > 0),
        disponivel: pasta.is_dir(),
        pasta: Some(pasta),
    }))
}

/// Aceita só um nome de pasta, sem separadores nem `..`: esse caminho será a base de
/// qualquer ação futura no disco, então um manifesto não pode apontar para fora de `common`.
fn pasta_simples(nome: &str) -> bool {
    let mut partes = Path::new(nome).components();
    matches!(
        (partes.next(), partes.next()),
        (Some(Component::Normal(_)), None)
    )
}

fn e_ignorado(appid: &str, nome: &str) -> bool {
    APPIDS_IGNORADOS.contains(&appid)
        || PREFIXOS_IGNORADOS
            .iter()
            .any(|prefixo| nome.starts_with(prefixo))
}

#[cfg(windows)]
fn raizes_candidatas() -> Vec<PathBuf> {
    use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
    use winreg::RegKey;

    let mut raizes = Vec::new();
    let usuario = RegKey::predef(HKEY_CURRENT_USER).open_subkey(r"Software\Valve\Steam");
    if let Ok(chave) = usuario {
        if let Ok(caminho) = chave.get_value::<String, _>("SteamPath") {
            raizes.push(PathBuf::from(caminho));
        }
    }
    let maquina =
        RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(r"SOFTWARE\WOW6432Node\Valve\Steam");
    if let Ok(chave) = maquina {
        if let Ok(caminho) = chave.get_value::<String, _>("InstallPath") {
            raizes.push(PathBuf::from(caminho));
        }
    }
    raizes
}

#[cfg(target_os = "macos")]
fn raizes_candidatas() -> Vec<PathBuf> {
    std::env::var_os("HOME")
        .map(|casa| vec![PathBuf::from(casa).join("Library/Application Support/Steam")])
        .unwrap_or_default()
}

#[cfg(all(unix, not(target_os = "macos")))]
fn raizes_candidatas() -> Vec<PathBuf> {
    let Some(casa) = std::env::var_os("HOME").map(PathBuf::from) else {
        return Vec::new();
    };
    vec![
        casa.join(".steam/steam"),
        casa.join(".local/share/Steam"),
        casa.join(".var/app/com.valvesoftware.Steam/.local/share/Steam"),
    ]
}

#[cfg(test)]
mod testes {
    use super::*;
    use tempfile::TempDir;

    fn manifesto(appid: &str, nome: &str, instalacao: &str, flags: u32, tamanho: u64) -> String {
        format!(
            "\"AppState\"\n{{\n\t\"appid\"\t\t\"{appid}\"\n\t\"name\"\t\t\"{nome}\"\n\
             \t\"StateFlags\"\t\t\"{flags}\"\n\t\"installdir\"\t\t\"{instalacao}\"\n\
             \t\"SizeOnDisk\"\t\t\"{tamanho}\"\n}}\n"
        )
    }

    fn escapar(caminho: &Path) -> String {
        caminho.to_string_lossy().replace('\\', "\\\\")
    }

    /// Cria `<base>/steamapps` e, para cada jogo, o manifesto e (opcionalmente) a pasta.
    fn biblioteca(base: &Path, jogos: &[(&str, &str, &str, u32, u64, bool)]) {
        let apps = base.join("steamapps");
        fs::create_dir_all(apps.join("common")).unwrap();
        for (appid, nome, dir, flags, tamanho, com_pasta) in jogos {
            let texto = manifesto(appid, nome, dir, *flags, *tamanho);
            fs::write(apps.join(format!("appmanifest_{appid}.acf")), texto).unwrap();
            if *com_pasta {
                fs::create_dir_all(apps.join("common").join(dir)).unwrap();
            }
        }
    }

    fn vdf_das_bibliotecas(caminhos: &[&Path]) -> String {
        let mut texto = String::from("\"libraryfolders\"\n{\n");
        for (i, caminho) in caminhos.iter().enumerate() {
            texto += &format!(
                "\t\"{i}\"\n\t{{\n\t\t\"path\"\t\t\"{}\"\n\t\t\"label\"\t\t\"\"\n\t}}\n",
                escapar(caminho)
            );
        }
        texto + "}\n"
    }

    fn escrever_vdf(raiz: &Path, conteudo: &str) {
        fs::write(raiz.join("steamapps").join("libraryfolders.vdf"), conteudo).unwrap();
    }

    #[test]
    fn detecta_jogos_de_todas_as_bibliotecas_em_ordem_de_nome() {
        let tmp = TempDir::new().unwrap();
        let (raiz, outra) = (tmp.path().join("steam"), tmp.path().join("outra"));
        biblioteca(&raiz, &[("20", "zebra", "Zebra", 4, 2_000, true)]);
        biblioteca(&outra, &[("10", "Alfa", "Alfa", 6, 5_000, true)]); // 6 = instalado + atualização
        escrever_vdf(&raiz, &vdf_das_bibliotecas(&[&raiz, &outra]));

        let resultado = Steam::com_raizes(vec![raiz]).detectar();

        let nomes: Vec<_> = resultado.jogos.iter().map(|j| j.nome.as_str()).collect();
        assert_eq!(nomes, ["Alfa", "zebra"]);
        assert_eq!(resultado.jogos[0].id_externo, "10");
        assert_eq!(resultado.jogos[0].tamanho_bytes, Some(5_000));
        assert!(resultado.jogos[0].disponivel);
        assert!(resultado.avisos.is_empty(), "{:?}", resultado.avisos);
    }

    #[test]
    fn jogo_sem_pasta_de_instalacao_fica_indisponivel() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path().join("steam");
        biblioteca(&raiz, &[("10", "Fantasma", "Fantasma", 4, 1, false)]);

        let resultado = Steam::com_raizes(vec![raiz]).detectar();

        assert_eq!(resultado.jogos.len(), 1);
        assert!(!resultado.jogos[0].disponivel);
    }

    #[test]
    fn ignora_instalacao_incompleta_e_redistribuiveis() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path().join("steam");
        biblioteca(
            &raiz,
            &[
                ("1", "Baixando", "Baixando", 1026, 10, true),
                (
                    "228980",
                    "Steamworks Common Redistributables",
                    "Redist",
                    4,
                    10,
                    true,
                ),
                ("2", "Proton 9.0", "Proton", 4, 10, true),
                ("3", "Jogo de verdade", "Jogo", 4, 10, true),
            ],
        );

        let resultado = Steam::com_raizes(vec![raiz]).detectar();

        let nomes: Vec<_> = resultado.jogos.iter().map(|j| j.nome.as_str()).collect();
        assert_eq!(nomes, ["Jogo de verdade"]);
    }

    #[test]
    fn biblioteca_em_disco_ausente_vira_aviso_e_nao_derruba_o_resto() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path().join("steam");
        let ausente = tmp.path().join("disco-desconectado");
        biblioteca(&raiz, &[("10", "Presente", "Presente", 4, 1, true)]);
        escrever_vdf(&raiz, &vdf_das_bibliotecas(&[&raiz, &ausente]));

        let resultado = Steam::com_raizes(vec![raiz]).detectar();

        assert_eq!(resultado.jogos.len(), 1);
        assert_eq!(resultado.avisos.len(), 1);
        assert!(resultado.avisos[0].contains("Biblioteca indisponível"));
    }

    #[test]
    fn manifesto_corrompido_vira_aviso_e_os_demais_continuam() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path().join("steam");
        biblioteca(&raiz, &[("10", "Bom", "Bom", 4, 1, true)]);
        fs::write(raiz.join("steamapps/appmanifest_99.acf"), "\"AppState\" {").unwrap();

        let resultado = Steam::com_raizes(vec![raiz]).detectar();

        assert_eq!(resultado.jogos.len(), 1);
        assert_eq!(resultado.avisos.len(), 1);
        assert!(resultado.avisos[0].starts_with("appmanifest_99.acf"));
    }

    #[test]
    fn rejeita_installdir_que_escapa_de_common() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path().join("steam");
        biblioteca(
            &raiz,
            &[
                ("1", "Sobe", "../fora", 4, 1, false),
                ("2", "Subpasta", "a/b", 4, 1, false),
                ("3", "Absoluto", "/etc", 4, 1, false),
            ],
        );

        let resultado = Steam::com_raizes(vec![raiz]).detectar();

        assert!(resultado.jogos.is_empty());
        assert_eq!(resultado.avisos.len(), 3);
    }

    #[test]
    fn rejeita_appid_nao_numerico() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path().join("steam");
        biblioteca(&raiz, &[("12x", "Estranho", "Estranho", 4, 1, true)]);

        let resultado = Steam::com_raizes(vec![raiz]).detectar();

        assert!(resultado.jogos.is_empty());
        assert!(resultado.avisos[0].contains("appid inválido"));
    }

    #[test]
    fn entende_o_formato_antigo_do_libraryfolders() {
        let tmp = TempDir::new().unwrap();
        let (raiz, outra) = (tmp.path().join("steam"), tmp.path().join("outra"));
        biblioteca(&raiz, &[]);
        biblioteca(&outra, &[("10", "Antigo", "Antigo", 4, 1, true)]);
        let vdf = format!(
            "\"LibraryFolders\"\n{{\n\t\"TimeNextStatsReport\"\t\"1\"\n\t\"1\"\t\"{}\"\n}}\n",
            escapar(&outra)
        );
        escrever_vdf(&raiz, &vdf);

        let resultado = Steam::com_raizes(vec![raiz]).detectar();

        assert_eq!(resultado.jogos.len(), 1);
        assert_eq!(resultado.jogos[0].nome, "Antigo");
    }

    #[test]
    fn biblioteca_repetida_nao_duplica_jogos_nem_avisos() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path().join("steam");
        biblioteca(&raiz, &[("10", "Unico", "Unico", 4, 1, true)]);
        escrever_vdf(&raiz, &vdf_das_bibliotecas(&[&raiz, &raiz]));

        let resultado = Steam::com_raizes(vec![raiz]).detectar();

        assert_eq!(resultado.jogos.len(), 1);
        assert!(resultado.avisos.is_empty());
    }

    #[test]
    fn chave_da_pasta_ignora_barra_final() {
        assert_eq!(
            chave_da_pasta(Path::new("/jogos/steam/")),
            chave_da_pasta(Path::new("/jogos/steam"))
        );
    }

    #[cfg(windows)]
    #[test]
    fn chave_da_pasta_junta_as_grafias_do_registro_e_do_vdf_no_windows() {
        assert_eq!(
            chave_da_pasta(Path::new("c:/program files (x86)/steam")),
            chave_da_pasta(Path::new(r"C:\Program Files (x86)\Steam\"))
        );
    }

    #[cfg(windows)]
    #[test]
    fn raiz_do_registro_com_barras_normais_nao_duplica_a_biblioteca_do_vdf() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path().join("steam");
        biblioteca(&raiz, &[("10", "Unico", "Unico", 4, 1, true)]);
        escrever_vdf(&raiz, &vdf_das_bibliotecas(&[&raiz]));
        let como_no_registro =
            PathBuf::from(raiz.to_string_lossy().replace('\\', "/").to_lowercase());

        let resultado = Steam::com_raizes(vec![como_no_registro]).detectar();

        assert_eq!(resultado.jogos.len(), 1);
        assert!(resultado.avisos.is_empty(), "{:?}", resultado.avisos);
    }

    #[test]
    fn varias_bibliotecas_sao_lidas_e_os_avisos_seguem_a_ordem_do_vdf() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path().join("steam");
        let (b, c) = (tmp.path().join("b"), tmp.path().join("c"));
        let (ausente1, ausente2) = (tmp.path().join("ausente1"), tmp.path().join("ausente2"));
        biblioteca(&raiz, &[("1", "Raiz", "Raiz", 4, 1, true)]);
        biblioteca(&b, &[("2", "Be", "Be", 4, 1, true)]);
        biblioteca(&c, &[("3", "Ce", "Ce", 4, 1, true)]);
        escrever_vdf(
            &raiz,
            &vdf_das_bibliotecas(&[&raiz, &ausente1, &b, &ausente2, &c]),
        );

        let resultado = Steam::com_raizes(vec![raiz]).detectar();

        let nomes: Vec<_> = resultado.jogos.iter().map(|j| j.nome.as_str()).collect();
        assert_eq!(nomes, ["Be", "Ce", "Raiz"]);
        assert_eq!(resultado.avisos.len(), 2);
        assert!(resultado.avisos[0].contains("ausente1"));
        assert!(resultado.avisos[1].contains("ausente2"));
    }

    #[test]
    fn steam_nao_instalada_devolve_lista_vazia_sem_erro() {
        let tmp = TempDir::new().unwrap();
        let resultado = Steam::com_raizes(vec![tmp.path().join("nao-existe")]).detectar();
        assert_eq!(resultado, Deteccao::default());
    }

    #[test]
    fn tamanho_zero_vira_desconhecido() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path().join("steam");
        biblioteca(&raiz, &[("10", "Recem", "Recem", 4, 0, true)]);

        let resultado = Steam::com_raizes(vec![raiz]).detectar();

        assert_eq!(resultado.jogos[0].tamanho_bytes, None);
    }

    #[test]
    fn abrir_recusa_identificador_que_nao_e_numerico() {
        let steam = Steam::com_raizes(Vec::new());
        for ruim in ["", "10; calc", "1 2", "../1", "steam://x", "-1"] {
            assert!(
                matches!(steam.abrir(ruim), Err(ErroFonte::IdInvalido(_))),
                "aceitou {ruim:?}"
            );
        }
    }
}
