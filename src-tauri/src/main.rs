// Em produção no Windows, não abre uma janela de console junto com o aplicativo.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    felixo_game_launcher_lib::run();
}
