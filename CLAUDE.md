# CLAUDE.md

Leia [`AGENTS.md`](AGENTS.md) primeiro: ele é o mapa de leitura e traz as regras do repositório.

O que não se negocia:

- Commits pequenos, um tema cada, direto na `main`, em Conventional Commits em português.
- `IA.md` só por acréscimo; documentação atualizada no mesmo commit da mudança.
- Rodar o gate (`ruff`, `pytest` e verificação de links) e conferir o CI nos três sistemas antes de dar algo como pronto.
- Apagar e mover arquivos é a parte de maior risco: leia as travas em [`docs/ARQUITETURA.md`](docs/ARQUITETURA.md) e teste só em pastas temporárias.
- Nos registros, separe o que foi **medido** do que foi **suposto** e declare o que não foi validado.
