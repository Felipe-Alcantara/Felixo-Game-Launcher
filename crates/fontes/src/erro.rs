use std::io;

/// Falhas ao agir sobre uma fonte.
#[derive(Debug, thiserror::Error)]
pub enum ErroFonte {
    #[error("identificador de jogo inválido: {0:?}")]
    IdInvalido(String),

    #[error("não foi possível abrir o jogo: {0}")]
    Abrir(#[source] io::Error),
}
