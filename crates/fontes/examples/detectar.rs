//! Mostra o que a fonte Steam detecta neste computador.
//!
//! Uso: `cargo run -p felixo-fontes --example detectar`

use felixo_fontes::steam::Steam;
use felixo_fontes::Fonte;

fn main() {
    let steam = Steam::do_sistema();
    let resultado = steam.detectar();

    let mut jogos = resultado.jogos;
    jogos.sort_by_key(|jogo| std::cmp::Reverse(jogo.tamanho_bytes));
    for jogo in &jogos {
        let tamanho = jogo
            .tamanho_bytes
            .map_or_else(|| "?".to_owned(), |b| format!("{:.1}", b as f64 / 1e9));
        let situacao = if jogo.disponivel {
            ""
        } else {
            " [indisponível]"
        };
        println!(
            "{tamanho:>7} GB | {:<9} | {}{situacao}",
            jogo.id_externo, jogo.nome
        );
    }
    println!("\n{} jogo(s) na {}.", jogos.len(), steam.nome());
    for aviso in &resultado.avisos {
        println!("aviso: {aviso}");
    }
}
