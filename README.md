# Felixo Game Launcher

> Um launcher open source que reúne, num só lugar, os jogos instalados no seu computador — de qualquer loja ou launcher.

[Read in English](README.en.md)

**Estado:** Fase 1 entregue como pré-lançamento (v0.1.0). O aplicativo (Tauri) lista os jogos instalados da Steam, com busca, ordenação por tamanho e atualização ao abrir. O botão Jogar existe, mas ainda não foi testado de ponta a ponta. Veja o [roadmap](docs/ESCOPO-E-ROADMAP.md).

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

## Baixar

A [Release v0.1.0](https://github.com/Felipe-Alcantara/Felixo-Game-Launcher/releases/tag/v0.1.0) é um **pré-lançamento sem assinatura de código**.

| Sistema | Arquivo | Situação |
| --- | --- | --- |
| Windows | `Felixo.Game.Launcher_0.1.0_x64-setup.exe` ou `..._x64_en-US.msi` | O executável do `.msi` foi aberto e listou os jogos da Steam. |
| Linux | `.AppImage`, `.deb` ou `.rpm` | O pacote é gerado pelo CI, mas ninguém o abriu ainda. |
| macOS (Apple Silicon) | `.dmg` | O pacote é gerado pelo CI, mas ninguém o abriu ainda. Não há build para Mac com Intel. |

Como não há assinatura, o Windows pode mostrar o aviso do SmartScreen e o macOS pode pedir para abrir pelo menu de contexto (botão direito, Abrir). A assinatura está em avaliação.

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
- [Arquitetura](docs/ARQUITETURA.md)
- [Desenvolvimento: compilar, testar e depurar](docs/DESENVOLVIMENTO.md)
- [IA.md](IA.md) — linha do tempo de decisões

## Contribuir

Veja o [guia de contribuição](CONTRIBUTING.md). Adaptadores para novas lojas são bem-vindos.

## Licença

[MIT](LICENSE).
