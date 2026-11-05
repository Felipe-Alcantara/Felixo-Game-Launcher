# Como contribuir / Contributing

Obrigado por querer ajudar! / Thanks for helping!

## Português

1. Leia o [README](README.md) e as [decisões](docs/DECISOES.md); elas já respondem a maior parte das dúvidas de escopo.
2. Para **adicionar uma loja ou launcher**, comece por [`docs/FONTES-DE-JOGOS.md`](docs/FONTES-DE-JOGOS.md) e pelo contrato em [`docs/ARQUITETURA.md`](docs/ARQUITETURA.md). Conte o que você verificou de fato e o que ainda é suposição.
3. Abra uma *issue* antes de uma mudança grande. Correções pequenas e de documentação podem ir direto em Pull Request.
4. Mensagens de commit: Conventional Commits em português, um tema por commit, explicando o que mudou **e por quê**.
5. Antes de enviar, rode `uv run ruff check .`, `uv run pytest -q` e `uv run python -m scripts.qualidade.verificar_links`; no Rust e na interface, os comandos de [`docs/DESENVOLVIMENTO.md`](docs/DESENVOLVIMENTO.md).
6. Não inclua segredos, caminhos pessoais nem dados de contas.

## English

1. Read the [README](README.en.md) and the [decisions](docs/DECISOES.md) (in Portuguese).
2. To **add a store or launcher**, start with [`docs/FONTES-DE-JOGOS.md`](docs/FONTES-DE-JOGOS.md) and the contract in [`docs/ARQUITETURA.md`](docs/ARQUITETURA.md). Say what you verified and what is still an assumption.
3. Open an issue before a large change. Small fixes and documentation go straight to a Pull Request.
4. Commits: Conventional Commits, one topic per commit, explaining what changed **and why**.
5. Before submitting, run `uv run ruff check .`, `uv run pytest -q` and `uv run python -m scripts.qualidade.verificar_links`; for Rust and the UI, the commands in [`docs/DESENVOLVIMENTO.md`](docs/DESENVOLVIMENTO.md) (Portuguese).
6. Never include secrets, personal paths or account data.
