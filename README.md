# Felixo Game Launcher

> Um launcher open source que reúne, num só lugar, os jogos instalados no seu computador — de qualquer loja ou launcher.

[Read in English](README.en.md)

**Estado:** em fundação. Já existem o repositório, a documentação, o menu de entrada e o CI nos três sistemas. O aplicativo em si (Tauri) começa na Fase 1 do [roadmap](docs/ESCOPO-E-ROADMAP.md) — **ainda não há nada para baixar**.

## O que é

Hoje os jogos ficam espalhados: uma biblioteca em cada loja, outros instalados à mão, alguns em discos diferentes. O Felixo Game Launcher **encontra tudo sozinho**, mostra numa biblioteca única e abre o jogo com um clique.

Ele **não instala nem atualiza** jogos: isso continua sendo da loja. O foco é organizar o que já está no seu computador e ajudar a gerenciar o espaço em disco.

## O que a v1 vai ter

- Detecção automática de Steam, Epic, GOG, Riot, Ubisoft, Battle.net, Rockstar, EA, Microsoft Store, Minecraft, Roblox, osu!, jogos avulsos e emuladores.
- Abertura híbrida: pelo launcher original e, quando não houver, pelo executável.
- Gestão de espaço: tamanho por jogo, duplicados, mover e apagar com confirmação dupla e Lixeira.
- Favoritos, tags, coleções, busca e histórico de uso.
- Tudo local: sem login em loja e sem enviar nada para a internet.
- Português e inglês, com binários pelas Releases do GitHub. Windows completo; Linux e macOS com limitações documentadas.

## Como usar hoje

Requisito: Python 3.11 ou mais novo.

```bash
python start_app.py
```

O menu confere as ferramentas necessárias para construir o aplicativo (Git, Node.js e Rust) e mostra o que falta. O CI testa o menu em Windows, Linux e macOS.

## Documentação

- [Escopo e roadmap](docs/ESCOPO-E-ROADMAP.md)
- [Decisões da v1](docs/DECISOES.md)
- [Fontes de jogos](docs/FONTES-DE-JOGOS.md)
- [Arquitetura-alvo](docs/ARQUITETURA.md)
- [IA.md](IA.md) — linha do tempo de decisões

## Contribuir

Veja `CONTRIBUTING.md`. Adaptadores para novas lojas são bem-vindos.

## Licença

[MIT](LICENSE).
