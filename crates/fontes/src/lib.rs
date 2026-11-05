//! Detecção de jogos instalados em lojas e launchers.
//!
//! Cada loja é uma [`Fonte`]. Só este crate lê os arquivos das lojas; o restante do
//! aplicativo recebe [`JogoDetectado`] já normalizado (veja `docs/ARQUITETURA.md`).

mod erro;
mod modelo;
pub mod steam;
pub mod vdf;

pub use erro::ErroFonte;
pub use modelo::{Deteccao, JogoDetectado};

/// Contrato de uma loja ou launcher.
pub trait Fonte {
    /// Identificador estável, gravado na biblioteca (por exemplo, `steam`).
    fn id(&self) -> &'static str;

    /// Nome para mostrar ao usuário.
    fn nome(&self) -> &'static str;

    /// Procura os jogos instalados. Loja não instalada devolve lista vazia, não erro;
    /// problemas parciais (disco ausente, arquivo corrompido) vão em [`Deteccao::avisos`].
    fn detectar(&self) -> Deteccao;

    /// Abre o jogo pelo launcher original, a partir do identificador dentro da fonte.
    fn abrir(&self, id_externo: &str) -> Result<(), ErroFonte>;
}
