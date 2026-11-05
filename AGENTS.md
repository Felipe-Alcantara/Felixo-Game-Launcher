# AGENTS.md — Roteamento para agentes

> **O que é:** o mapa de leitura para agentes de IA que trabalham neste repositório. Leia **antes** de alterar qualquer coisa.
> Para pessoas, o ponto de entrada é o [README](README.md).

## Leia sempre

1. [`README.md`](README.md) — o que o projeto é e em que estado está.
2. [`docs/DECISOES.md`](docs/DECISOES.md) — decisões já tomadas. Não as reabra sem pedido explícito.
3. [`IA.md`](IA.md) — linha do tempo do projeto. Só se acrescenta; nunca se reescreve.

## Depois, conforme o pedido

| Se o pedido é… | Abra |
| --- | --- |
| Escopo, prioridade, "isso entra na v1?" | [`docs/ESCOPO-E-ROADMAP.md`](docs/ESCOPO-E-ROADMAP.md) |
| Suportar uma loja ou launcher novo | [`docs/FONTES-DE-JOGOS.md`](docs/FONTES-DE-JOGOS.md) e [`docs/ARQUITETURA.md`](docs/ARQUITETURA.md) |
| Mover ou apagar arquivos | [`docs/ARQUITETURA.md`](docs/ARQUITETURA.md), seção das travas. **Alto risco.** |
| Rodar, instalar ou configurar | `python start_app.py` |
| Contribuir | [`CONTRIBUTING.md`](CONTRIBUTING.md) |

## Regras do repositório

- **Padrão de qualidade:** [Felixo System Design](https://github.com/Felipe-Alcantara/Felixo-System-Design).
- **Git:** commite direto na `main`, em commits pequenos, um tema por commit, no formato Conventional Commits em português (`feat:`, `fix:`, `docs:`, `refactor:`, `chore:`, `ci:`). Branch só para feature grande, refatoração significativa ou alto risco, com a justificativa escrita.
- **Documentação viva:** mudou comportamento, estrutura ou comando? Atualize README, `docs/` e `IA.md` no mesmo commit. No `IA.md`, acrescente uma entrada datada.
- **Gate antes de enviar:** `uv run ruff check .`, `uv run pytest -q` e `uv run python -m scripts.qualidade.verificar_links`. Depois do push, acompanhe o CI até o fim.
- **Multiplataforma:** Windows, Linux e macOS por padrão. Nada de caminho, separador ou comando preso a um sistema; limitação por sistema vai documentada.
- **Segurança:** nenhum segredo, token ou caminho pessoal em commit. Testes de mover e apagar usam só pastas temporárias.
- **Linguagem:** português nos textos do projeto, em linguagem de open source, sem valores locais fixos.
- **Multiagente:** outros agentes podem estar neste repositório. Confira `git status` e `git log` antes de editar e não reescreva o que outro agente está alterando.
