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

---

## 05/11/2026 — Levantamento das fontes de jogos e arquitetura-alvo

- **Mudança:** [fontes de jogos](docs/FONTES-DE-JOGOS.md) e [arquitetura-alvo](docs/ARQUITETURA.md).
- **Motivo:** dar à Fase 1 um contrato de fonte estável (a evolução prevista são adaptadores da comunidade) e registrar as armadilhas já medidas.
- **O que foi medido:** em um computador com Windows, a leitura de manifestos da Steam e da Epic, as entradas de desinstalação do registro, os atalhos do Menu Iniciar e a busca por marcadores de arquivo acharam jogos instalados fora de qualquer launcher. A varredura de vários discos levou minutos; bibliotecas apontavam para unidades ausentes; havia pastas de jogo sem manifesto.
- **O que é suposição:** os argumentos de abertura de Riot, Ubisoft, Battle.net, EA e Microsoft Store, e as fontes de Linux e macOS. Estão marcados **a verificar** na tabela.
- **Limitação:** a arquitetura é uma proposta; as travas 3 a 5 das ações no disco ainda não foram aprovadas por quem mantém o projeto.

---

## 05/11/2026 — README em português e inglês

- **Mudança:** [README.md](README.md) e [README.en.md](README.en.md).
- **Motivo:** a decisão 17 pede português e inglês; o README é a porta de entrada para quem não conhece o projeto.
- **Cuidado:** o texto diz com todas as letras que **ainda não há aplicativo para baixar**; o que existe é a base do projeto.
- **Limitação:** a documentação detalhada (`docs/`) está só em português; o README em inglês aponta para ela e avisa.

---

## 05/11/2026 — Guias para agentes e contribuição

- **Mudança:** [AGENTS.md](AGENTS.md), [CLAUDE.md](CLAUDE.md) e [CONTRIBUTING.md](CONTRIBUTING.md); o README passa a ligar para o guia de contribuição.
- **Motivo:** agentes e pessoas precisam do mesmo mapa de leitura e das mesmas regras (git, gate, segurança), sem reler o repositório inteiro. Segue o padrão Felixo.
- **Validação:** links conferidos pelo verificador de links.

---

## 05/11/2026 — Ambiente Python com ruff, pytest e lockfile

- **Mudança:** `pyproject.toml` e `uv.lock`.
- **Motivo:** o padrão pede dependências fixadas, com lockfile. O menu `start_app.py` não tem dependências; só o desenvolvimento usa `ruff` e `pytest`, com versões exatas.
- **Decisão pequena:** `uv` (já usado nos outros projetos do autor), com `package = false` porque não há pacote Python a publicar.
- **Validação:** `uv sync --locked` reproduz o ambiente a partir do lockfile.
- **Limitação:** não há dependência de execução para auditar. A auditoria de dependências entra quando o aplicativo (Fase 1) trouxer pacotes de Rust e de Node.

---

## 05/11/2026 — Menu de entrada start_app.py

- **Mudança:** `start_app.py` e `tests/test_start_app.py`.
- **Motivo:** o padrão Felixo exige um menu interativo como porta de entrada de todo programa. Menu mínimo: Iniciar, Instalar/Setup, Configurar e Status/Sair.
- **Decisões:** usa só a biblioteca padrão, para rodar antes de qualquer instalação; textos em pt-BR e en (decisão 17); configuração em JSON na pasta de configuração de cada sistema, gravada por arquivo temporário para nunca ficar pela metade; fim da entrada (Ctrl+D ou Ctrl+C) encerra sem erro.
- **O que a execução real achou:** rodando o menu por pipe no PowerShell 5.1, a primeira linha chegou com um BOM (`﻿`) e a opção válida foi recusada. Os testes unitários não pegavam isso. A leitura de entrada foi centralizada em `Contexto.perguntar`, que descarta o BOM, e ganhou teste de regressão.
- **Limitação declarada (shortcut):** "Iniciar" só avisa que o aplicativo não existe; deve ser ligado ao `npm run tauri dev` na Fase 1.
- **Validação:** testes automatizados do menu, da configuração e dos caminhos por sistema (Windows, macOS e Linux simulados). O CI nos três sistemas entra na entrada "CI em Windows, Linux e macOS".
