# Felixo Game Launcher

> An open-source launcher that gathers, in one place, the games installed on your computer — from any store or launcher.

[Leia em português](README.md)

**Status:** Phase 1 delivered as a pre-release (v0.1.0). The application (Tauri) lists the installed Steam games, with search, size sorting and a refresh on startup. The Play button exists but has not been tested end to end yet. See the [roadmap](docs/ESCOPO-E-ROADMAP.md). The detailed docs are written in Portuguese.

## What it is

Today games are scattered: one library per store, others installed by hand, some on different drives. Felixo Game Launcher **finds everything by itself**, shows it in a single library and opens a game with one click.

It **does not install or update** games: that stays with each store. The focus is organizing what is already on your computer and helping you manage disk space.

## What v1 will have

- Automatic detection of Steam, Epic, GOG, Riot, Ubisoft, Battle.net, Rockstar, EA, Microsoft Store, Minecraft, Roblox, osu!, standalone games and emulators.
- Hybrid launching: through the original launcher and, when there is none, through the executable.
- Space management: size per game, duplicates, move and delete with double confirmation and the Recycle Bin.
- Favorites, tags, collections, search and play history.
- Fully local: no store login and nothing is sent to the internet.
- Portuguese and English, with binaries on GitHub Releases. Full support on Windows; Linux and macOS with documented limitations.

## Download

[Release v0.1.0](https://github.com/Felipe-Alcantara/Felixo-Game-Launcher/releases/tag/v0.1.0) is a **pre-release without code signing**.

| System | File | Status |
| --- | --- | --- |
| Windows | `Felixo.Game.Launcher_0.1.0_x64-setup.exe` or `..._x64_en-US.msi` | The `.msi` executable was opened and listed the Steam games. |
| Linux | `.AppImage`, `.deb` or `.rpm` | The package is built by CI, but nobody has opened it yet. |
| macOS (Apple Silicon) | `.dmg` | The package is built by CI, but nobody has opened it yet. There is no Intel Mac build. |

Without a signature, Windows may show the SmartScreen warning and macOS may ask you to open it from the context menu (right click, Open). Signing is under evaluation.

## Using it today

Requirement: Python 3.11 or newer.

```bash
python start_app.py
```

The menu checks the tools needed to build the application (Git, Node.js and Rust) and shows what is missing. CI tests the menu on Windows, Linux and macOS.

## Documentation

- [Scope and roadmap](docs/ESCOPO-E-ROADMAP.md) (Portuguese)
- [v1 decisions](docs/DECISOES.md) (Portuguese)
- [Game sources](docs/FONTES-DE-JOGOS.md) (Portuguese)
- [Architecture](docs/ARQUITETURA.md) (Portuguese)
- [Development: build, test and debug](docs/DESENVOLVIMENTO.md) (Portuguese)
- [IA.md](IA.md) — decision timeline

## Contributing

See the [contributing guide](CONTRIBUTING.md). Adapters for new stores are welcome.

## License

[MIT](LICENSE).
