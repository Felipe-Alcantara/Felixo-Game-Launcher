# Felixo Game Launcher

> An open-source launcher that gathers, in one place, the games installed on your computer — from any store or launcher.

[Leia em português](README.md)

**Status:** foundation phase. The repository, documentation, entry menu and CI on all three systems exist. The application itself (Tauri) starts in Phase 1 of the [roadmap](docs/ESCOPO-E-ROADMAP.md) — **there is nothing to download yet**. The detailed docs are written in Portuguese.

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
