# Desenvolvimento

> **O que é:** como preparar o computador, compilar, testar e rodar o projeto.
> **Quando usar:** antes de contribuir com código. Para escolher o que fazer, veja [ESCOPO-E-ROADMAP.md](ESCOPO-E-ROADMAP.md); para entender as camadas, [ARQUITETURA.md](ARQUITETURA.md).

## O que instalar

| Ferramenta | Versão | Para quê |
| --- | --- | --- |
| Rust | estável (mínimo 1.85) | núcleo e shell Tauri |
| Node.js | 22 ou mais novo | interface web |
| Python e [uv](https://docs.astral.sh/uv/) | 3.11 ou mais novo | menu `start_app.py` e verificações de qualidade |
| Git | recente | — |

O menu `python start_app.py` confere o que está faltando.

**Por sistema:**

- **Windows:** o caminho recomendado é o toolchain `msvc` do Rust, que exige o *Visual Studio Build Tools* com a carga "Desenvolvimento para desktop com C++". O WebView2 já vem no Windows 11.
- **Linux:** pacotes do WebKitGTK e afins. Em Ubuntu e Debian: `libwebkit2gtk-4.1-dev libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev`.
- **macOS:** as ferramentas de linha de comando do Xcode (`xcode-select --install`).

### Windows sem o Build Tools (alternativa GNU)

Se não dá para instalar o Build Tools, o toolchain `gnu` com o `gcc` do MSYS2 (UCRT64) compila e testa o projeto:

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu --profile minimal
rustup component add clippy rustfmt --toolchain stable-x86_64-pc-windows-gnu
$env:PATH = "C:\msys64\ucrt64\bin;" + $env:PATH
$env:RUSTUP_TOOLCHAIN = "stable-x86_64-pc-windows-gnu"
$env:CARGO_TARGET_DIR = "C:\tmp\fgl-target"   # precisa ser um caminho sem acento
```

Sem acento porque as ferramentas do MSYS2 não lidam com caracteres como `ç` ou `ã` no caminho, e isso aparece em **dois** lugares, ambos medidos:

- **Pasta de build** (`CARGO_TARGET_DIR`): o `ld.exe` não encontra os objetos que o `rustc` acabou de criar e a ligação falha com "No such file or directory".
- **Pasta do código**: o `windres` não abre o `icon.ico` ("Invalid argument") ao compilar o aplicativo Tauri. O `tauri-build` canonicaliza o caminho, então uma junção (link de diretório) não resolve.

Se a sua cópia do repositório estiver num caminho com acento, compile a partir de uma cópia em caminho ASCII:

```powershell
robocopy . C:\tmp\fgl-src /MIR /XD .git node_modules .venv target /NFL /NDL /NJH /NJS /NP
cd C:\tmp\fgl-src; cargo build -p felixo-game-launcher
```

Nada disso afeta o toolchain MSVC nem o CI do GitHub.

## Comandos do dia a dia

```bash
npm ci                          # dependências da interface (uma vez)
npm run dev                     # só a interface, no navegador, com dados simulados
npm run tauri -- dev            # o aplicativo completo, com recarga ao vivo
cargo test --workspace          # testes do núcleo e do shell
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all
npm run typecheck && npm test   # interface
uv run ruff check . && uv run pytest -q && uv run python -m scripts.qualidade.verificar_links
```

O Tauri embute a interface já compilada. Se `cargo` reclamar de `../dist`, rode `npm run build` antes.

Para ver o que a fonte da Steam detecta neste computador, sem abrir o aplicativo:

```bash
cargo run -p felixo-fontes --example detectar
```

## Armadilhas já medidas

- **PowerShell 5.1 e pipes.** Ele antepõe um BOM (`\ufeff`) à primeira linha enviada por pipe a um programa nativo. O menu `start_app.py` já descarta esse caractere.
- **PowerShell 5.1 e arquivos.** `Set-Content -Encoding utf8` grava UTF-8 *com* BOM e quebras CRLF. O repositório usa UTF-8 sem BOM e LF (`.editorconfig` e `.gitattributes`); grave com `[IO.File]::WriteAllText` e `UTF8Encoding($false)`.
- **Testes do shell Tauri.** O executável de teste de um crate Tauri não abre no Windows (`0xC0000139`, falta o manifesto do Common Controls v6). Foi medido com o toolchain GNU; é um problema conhecido do Tauri também com o MSVC, mas não foi reproduzido aqui. Por isso o shell não tem testes de unidade e o contrato com a interface vive em `crates/ipc`.
- **Disco ausente é lento.** Uma unidade ausente leva vários segundos para falhar em cada acesso; veja [FONTES-DE-JOGOS.md](FONTES-DE-JOGOS.md).
- **Mover e apagar arquivos.** Os testes dessas ações usam só pastas temporárias, nunca jogos reais. Veja as travas em [ARQUITETURA.md](ARQUITETURA.md).
