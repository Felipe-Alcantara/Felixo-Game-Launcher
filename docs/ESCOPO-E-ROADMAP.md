# Escopo e roadmap

> **O que é:** o recorte da v1 do Felixo Game Launcher e a ordem proposta de entrega.
> **Quando usar:** antes de propor uma funcionalidade ou de decidir se algo entra agora.
> As escolhas que sustentam este recorte estão em [DECISOES.md](DECISOES.md).

## O que o projeto é

Um launcher open source e local que **encontra sozinho** os jogos já instalados no computador — de qualquer loja ou launcher — e os mostra numa biblioteca única, com busca, organização e gestão de espaço em disco.

Ele **não substitui** as lojas: quem instala e atualiza os jogos continua sendo cada loja.

## Dentro da v1

- Detectar jogos de Steam, Epic e GOG; Riot, Ubisoft, Battle.net, Rockstar e EA; Minecraft, Roblox, osu! e Microsoft Store; jogos avulsos e emuladores.
- Varredura completa em segundo plano (obrigatória na primeira vez, opcional depois), atualização ao abrir, botão Atualizar e observador de pastas.
- Abrir o jogo de forma híbrida: pelo launcher original e, quando não houver, pelo executável.
- Gerenciar espaço em disco: tamanho por jogo, lista ordenada, duplicados, mover e apagar com travas de segurança.
- Organização (favoritos, tags, coleções, busca), histórico de uso (último jogado e tempo jogado) e ícones e capas locais.
- Interface e documentação em português e inglês.
- Distribuição por GitHub Releases, com binários por sistema.

## Fora da v1

- Instalar, baixar e atualizar jogos.
- Backup e sincronização de saves.
- Modo TV / controle (Big Picture).
- Mods, traduções e patches.
- Login em lojas, sincronização entre computadores e metadados online. Ficam como evolução.

## Fases

A ordem é uma proposta e pode ser ajustada. Cada fase termina com algo que roda e foi testado.

| Fase | Entrega | Utilizável? | Estado |
| --- | --- | --- | --- |
| 0 | Fundação: repositório, documentação, menu `start_app.py` e CI nos três sistemas | Não, só a base | Feita |
| 1 | Esqueleto Tauri e núcleo, com a Steam: lista os jogos instalados, busca, ordena por tamanho e abre pela Steam | Sim — primeira entrega | Feita no código e publicada como v0.1.0 (pré-lançamento); falta testar o Jogar de ponta a ponta |
| 2 | Epic e GOG, abertura híbrida, jogos avulsos por pasta, idioma pt-BR e en | Sim | A fazer |
| 3 | Riot, Ubisoft, Battle.net, Rockstar e EA; Minecraft, Roblox, osu! e Microsoft Store; emuladores | Sim | A fazer |
| 4 | Varredura completa em segundo plano e observador de pastas | Sim | A fazer |
| 5 | Gestão de espaço (duplicados, mover e apagar com travas), organização e histórico de uso | Sim | A fazer |
| 6 | Linux e macOS: fontes multiplataforma e empacotamento por sistema | Sim | A fazer |

## Evolução esperada

O próximo passo provável depois da v1 são **adaptadores da comunidade** para novas lojas. Por isso o contrato de cada fonte nasce estável e isolado na Fase 1 — veja [ARQUITETURA.md](ARQUITETURA.md).
