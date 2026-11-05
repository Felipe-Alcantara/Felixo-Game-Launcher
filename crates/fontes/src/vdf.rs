//! Leitor mínimo do formato de texto KeyValues (VDF) usado pela Steam.
//!
//! Cobre o que `libraryfolders.vdf` e `appmanifest_*.acf` usam: chaves e valores entre
//! aspas (com escapes `\\`, `\"`, `\n` e `\t`), blocos `{ }` e comentários `//`.

use std::fmt;

/// Limite de blocos aninhados, para que um arquivo corrompido não estoure a pilha.
const PROFUNDIDADE_MAXIMA: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Valor {
    Texto(String),
    Mapa(Vec<(String, Valor)>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErroVdf {
    /// Posição, em bytes, do problema no texto.
    pub posicao: usize,
    pub mensagem: &'static str,
}

impl fmt::Display for ErroVdf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (posição {})", self.mensagem, self.posicao)
    }
}

impl std::error::Error for ErroVdf {}

impl Valor {
    pub fn texto(&self) -> Option<&str> {
        match self {
            Valor::Texto(texto) => Some(texto),
            Valor::Mapa(_) => None,
        }
    }

    pub fn filhos(&self) -> &[(String, Valor)] {
        match self {
            Valor::Mapa(filhos) => filhos,
            Valor::Texto(_) => &[],
        }
    }

    /// Primeiro filho com a chave dada, sem diferenciar maiúsculas de minúsculas
    /// (a Steam mistura `AppState`, `appid` e `StateFlags`).
    pub fn filho(&self, chave: &str) -> Option<&Valor> {
        self.filhos()
            .iter()
            .find(|(nome, _)| nome.eq_ignore_ascii_case(chave))
            .map(|(_, valor)| valor)
    }
}

/// Lê um documento VDF. O resultado é sempre um [`Valor::Mapa`] com as chaves de topo.
pub fn analisar(texto: &str) -> Result<Valor, ErroVdf> {
    let mut leitor = Leitor { texto, pos: 0 };
    Ok(Valor::Mapa(leitor.ler_mapa(0, false)?))
}

struct Leitor<'a> {
    texto: &'a str,
    pos: usize,
}

impl Leitor<'_> {
    fn erro(&self, mensagem: &'static str) -> ErroVdf {
        ErroVdf {
            posicao: self.pos,
            mensagem,
        }
    }

    fn espiar(&self) -> Option<char> {
        self.texto[self.pos..].chars().next()
    }

    fn pular_espacos(&mut self) {
        loop {
            let resto = &self.texto[self.pos..];
            let sem_espacos = resto.trim_start();
            self.pos += resto.len() - sem_espacos.len();
            if sem_espacos.starts_with("//") {
                self.pos += sem_espacos.find('\n').unwrap_or(sem_espacos.len());
            } else {
                return;
            }
        }
    }

    fn ler_texto(&mut self) -> Result<String, ErroVdf> {
        self.pos += 1; // aspas de abertura
        let mut saida = String::new();
        let mut caracteres = self.texto[self.pos..].char_indices();
        while let Some((i, c)) = caracteres.next() {
            match c {
                '"' => {
                    self.pos += i + 1;
                    return Ok(saida);
                }
                '\\' => match caracteres.next() {
                    Some((_, 'n')) => saida.push('\n'),
                    Some((_, 't')) => saida.push('\t'),
                    Some((_, outro)) => saida.push(outro),
                    None => break,
                },
                _ => saida.push(c),
            }
        }
        self.pos = self.texto.len();
        Err(self.erro("texto sem aspas de fechamento"))
    }

    fn ler_mapa(
        &mut self,
        profundidade: usize,
        aninhado: bool,
    ) -> Result<Vec<(String, Valor)>, ErroVdf> {
        if profundidade > PROFUNDIDADE_MAXIMA {
            return Err(self.erro("blocos aninhados demais"));
        }
        let mut itens = Vec::new();
        loop {
            self.pular_espacos();
            match self.espiar() {
                None if aninhado => return Err(self.erro("bloco sem chave de fechamento")),
                None => return Ok(itens),
                Some('}') if aninhado => {
                    self.pos += 1;
                    return Ok(itens);
                }
                Some('}') => return Err(self.erro("chave de fechamento sobrando")),
                Some('"') => {
                    let chave = self.ler_texto()?;
                    self.pular_espacos();
                    let valor = match self.espiar() {
                        Some('"') => Valor::Texto(self.ler_texto()?),
                        Some('{') => {
                            self.pos += 1;
                            Valor::Mapa(self.ler_mapa(profundidade + 1, true)?)
                        }
                        _ => return Err(self.erro("valor ausente depois da chave")),
                    };
                    itens.push((chave, valor));
                }
                Some(_) => return Err(self.erro("símbolo inesperado")),
            }
        }
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn le_pares_e_blocos_aninhados() {
        let valor =
            analisar("\"raiz\"\n{\n\t\"a\"\t\"1\"\n\t\"bloco\"\n\t{\n\t\t\"b\" \"2\"\n\t}\n}")
                .unwrap();
        let raiz = valor.filho("raiz").unwrap();
        assert_eq!(raiz.filho("a").and_then(Valor::texto), Some("1"));
        let b = raiz.filho("bloco").unwrap().filho("b");
        assert_eq!(b.and_then(Valor::texto), Some("2"));
    }

    #[test]
    fn interpreta_escapes_de_caminho_do_windows() {
        let valor =
            analisar(r#""path" "C:\\Program Files (x86)\\Steam" "aspas" "diz \"oi\"""#).unwrap();
        assert_eq!(
            valor.filho("path").and_then(Valor::texto),
            Some(r"C:\Program Files (x86)\Steam")
        );
        assert_eq!(
            valor.filho("aspas").and_then(Valor::texto),
            Some("diz \"oi\"")
        );
    }

    #[test]
    fn ignora_comentarios_e_espacos() {
        let valor = analisar("// cabeçalho\n\"a\" \"1\" // fim da linha\n\n\"b\" \"2\"\n").unwrap();
        assert_eq!(valor.filhos().len(), 2);
    }

    #[test]
    fn busca_de_chave_ignora_maiusculas() {
        let valor = analisar("\"AppState\" { \"AppID\" \"10\" }").unwrap();
        let estado = valor.filho("appstate").unwrap();
        assert_eq!(estado.filho("appid").and_then(Valor::texto), Some("10"));
    }

    #[test]
    fn documento_vazio_vira_mapa_vazio() {
        assert_eq!(analisar("  \n ").unwrap(), Valor::Mapa(Vec::new()));
    }

    #[test]
    fn rejeita_texto_sem_fechamento() {
        let erro = analisar("\"a\" \"sem fim").unwrap_err();
        assert_eq!(erro.mensagem, "texto sem aspas de fechamento");
    }

    #[test]
    fn rejeita_bloco_sem_fechamento() {
        let erro = analisar("\"a\" { \"b\" \"1\"").unwrap_err();
        assert_eq!(erro.mensagem, "bloco sem chave de fechamento");
    }

    #[test]
    fn rejeita_chave_de_fechamento_sobrando() {
        let erro = analisar("\"a\" \"1\" }").unwrap_err();
        assert_eq!(erro.mensagem, "chave de fechamento sobrando");
    }

    #[test]
    fn rejeita_chave_sem_valor() {
        let erro = analisar("\"a\"").unwrap_err();
        assert_eq!(erro.mensagem, "valor ausente depois da chave");
    }

    #[test]
    fn rejeita_simbolo_fora_de_aspas() {
        let erro = analisar("solto").unwrap_err();
        assert_eq!(erro.mensagem, "símbolo inesperado");
    }

    #[test]
    fn limita_a_profundidade_dos_blocos() {
        let fundo = "\"a\" {".repeat(PROFUNDIDADE_MAXIMA + 5);
        let erro = analisar(&fundo).unwrap_err();
        assert_eq!(erro.mensagem, "blocos aninhados demais");
    }

    #[test]
    fn aceita_acentos_e_emojis_no_texto() {
        let valor = analisar("\"name\" \"Pokémon: Edição Ação 🎮\"").unwrap();
        assert_eq!(
            valor.filho("name").and_then(Valor::texto),
            Some("Pokémon: Edição Ação 🎮")
        );
    }
}
