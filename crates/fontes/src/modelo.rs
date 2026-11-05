use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Um jogo encontrado numa fonte, ainda sem identidade na biblioteca.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JogoDetectado {
    /// Identificador da fonte (por exemplo, `steam`).
    pub fonte: String,
    /// Identificador do jogo dentro da fonte (na Steam, o appid).
    pub id_externo: String,
    pub nome: String,
    /// Pasta de instalação, quando a fonte a informa.
    pub pasta: Option<PathBuf>,
    pub tamanho_bytes: Option<u64>,
    /// `false` quando a pasta não existe agora (disco desconectado, por exemplo).
    pub disponivel: bool,
}

/// Resultado de uma detecção: o que foi achado e o que deu errado pelo caminho.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Deteccao {
    pub jogos: Vec<JogoDetectado>,
    /// Problemas que não impedem a detecção (biblioteca ausente, manifesto corrompido).
    pub avisos: Vec<String>,
}
