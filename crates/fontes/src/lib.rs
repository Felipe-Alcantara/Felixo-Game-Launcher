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

/// Contrato de uma loja ou launcher. `Send + Sync` porque a detecção roda fora da thread da tela.
pub trait Fonte: Send + Sync {
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

/// As fontes que o aplicativo consulta neste computador. Adicionar uma loja nova é acrescentar
/// uma linha aqui; o resto do aplicativo não precisa conhecê-la.
pub fn fontes_do_sistema() -> Vec<Box<dyn Fonte>> {
    vec![Box::new(steam::Steam::do_sistema())]
}

#[cfg(test)]
mod testes {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn cada_fonte_do_sistema_tem_id_e_nome_unicos_e_nao_vazios() {
        let fontes = fontes_do_sistema();
        let ids: HashSet<_> = fontes.iter().map(|f| f.id()).collect();
        assert_eq!(ids.len(), fontes.len(), "dois ids iguais");
        assert!(fontes
            .iter()
            .all(|f| !f.id().is_empty() && !f.nome().is_empty()));
    }
}
