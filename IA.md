# IA.md — Linha do tempo de decisões

> **O que é:** o registro cronológico das decisões, dos motivos e das validações do projeto, escrito para quem (pessoa ou agente) chegar depois.
> **Regra:** este arquivo é **append-only**. Acrescente entradas novas no fim, datadas, com contexto, motivo e validação. Nunca reescreva nem apague uma entrada antiga; se uma decisão mudar, registre a mudança em uma entrada nova que explique o porquê.

---

## 05/11/2026 — Fundação do projeto e decisões da v1

- **Agente:** Buscar por jogos (Claude) · **Repositório:** Felixo-Game-Launcher
- **Contexto:** pedido de um repositório open source, no padrão de qualidade Felixo, para um launcher que reúne jogos de qualquer loja ou launcher instalados no computador. Antes de qualquer código houve um questionário de 19 perguntas.
- **Mudança:** documentos de [escopo e roadmap](docs/ESCOPO-E-ROADMAP.md) e de [decisões da v1](docs/DECISOES.md).
- **Motivo:** fixar o que já foi decidido para que o próximo agente ou contribuidor não reabra o assunto, e separar a v1 em fases que terminam em algo que roda.
- **Premissa a confirmar:** a resposta sobre contas e rede foi aberta ("open source, rodando local, baixado em release, busca e adiciona automaticamente em qualquer sistema"). Foi interpretada como *sem login em loja e sem rede obrigatória*; capas online e chave de API ficaram como evolução.
- **Validação:** só documentação; os links são conferidos pelo verificador descrito na entrada "Verificação de links internos".
- **Limitação:** o escopo da v1 é grande (todas as fontes, apagar arquivos, varredura e observador). A ordem das fases é uma proposta.
