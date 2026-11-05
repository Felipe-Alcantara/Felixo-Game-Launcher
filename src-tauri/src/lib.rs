//! Shell do aplicativo: liga a interface (webview) ao núcleo.
//!
//! Só cola: cada comando chama `felixo-fontes` ou `felixo-biblioteca` e devolve os tipos de
//! `felixo-ipc`. Não tem regra de negócio nem testes de unidade (o executável de teste de um
//! crate Tauri não abre no Windows); o que dá para testar mora nos outros crates.

use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};

use felixo_biblioteca::{Biblioteca, Filtro, Jogo, Ordem};
use felixo_fontes::fontes_do_sistema;
use felixo_ipc::{Atualizacao, ErroApp};
use tauri::{Manager, State};

struct Estado {
    biblioteca: Mutex<Biblioteca>,
}

impl Estado {
    fn biblioteca(&self) -> MutexGuard<'_, Biblioteca> {
        // Um pânico em outra thread não corrompe o banco; segue com o que há.
        self.biblioteca
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }
}

fn agora() -> i64 {
    let segundos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duracao| duracao.as_secs());
    i64::try_from(segundos).unwrap_or(i64::MAX)
}

#[tauri::command]
async fn atualizar_biblioteca(estado: State<'_, Estado>) -> Result<Atualizacao, ErroApp> {
    let mut total = Atualizacao::default();
    for fonte in fontes_do_sistema() {
        let (id, nome) = (fonte.id(), fonte.nome());
        // A detecção lê muitos arquivos: roda fora da thread da tela.
        let deteccao = tauri::async_runtime::spawn_blocking(move || fonte.detectar())
            .await
            .map_err(|erro| ErroApp::Interno(erro.to_string()))?;
        let resumo = estado
            .biblioteca()
            .sincronizar(id, &deteccao.jogos, agora())?;
        total.somar(nome, &deteccao, &resumo);
    }
    Ok(total)
}

#[tauri::command]
async fn listar_jogos(
    estado: State<'_, Estado>,
    busca: Option<String>,
    ordem: Option<Ordem>,
) -> Result<Vec<Jogo>, ErroApp> {
    let filtro = Filtro {
        busca,
        ordem: ordem.unwrap_or_default(),
    };
    Ok(estado.biblioteca().listar(&filtro)?)
}

#[tauri::command]
async fn abrir_jogo(estado: State<'_, Estado>, id: i64) -> Result<(), ErroApp> {
    Ok(estado.biblioteca().abrir_jogo(id, &fontes_do_sistema())?)
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let pasta = app.path().app_data_dir()?;
            let biblioteca = Biblioteca::abrir(&pasta.join("biblioteca.db"))?;
            app.manage(Estado {
                biblioteca: Mutex::new(biblioteca),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            atualizar_biblioteca,
            listar_jogos,
            abrir_jogo
        ])
        .run(tauri::generate_context!())
        .expect("não foi possível iniciar o aplicativo");
}
