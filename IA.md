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

---

## 05/11/2026 — Verificação de links internos

- **Mudança:** `scripts/qualidade/verificar_links.py` e `tests/test_verificar_links.py`.
- **Motivo:** o critério de pronto do padrão inclui "links internos válidos". Com tanta documentação ligada entre si, conferir à mão não escala.
- **Decisão:** ignora links externos, âncoras puras e qualquer coisa dentro de código; todo o resto que parece link precisa apontar para um arquivo que exista.
- **Validação:** testes cobrindo link bom, quebrado (com arquivo e linha), âncora, código e pastas ignoradas; rodado sobre o próprio repositório.
- **Limitação:** não confere se a âncora (`#secao`) existe dentro do arquivo.

---

## 05/11/2026 — CI em Windows, Linux e macOS

- **Mudança:** `.github/workflows/ci.yml`.
- **Motivo:** o padrão exige multiplataforma, e há histórico de testes que só passavam no Windows (decisão 14). É gratuito em repositório público.
- **O que roda:** `ruff`, `pytest` e a verificação de links, em Ubuntu, Windows e macOS, com Python 3.11 e 3.13. Ações fixadas por versão principal, `persist-credentials: false` e permissão mínima (`contents: read`).
- **Limitação:** o CI ainda não gera binários nem Release; isso entra com o aplicativo (Fases 1 e 2). O resultado da primeira execução será registrado em entrada própria, depois do push.

---

## 05/11/2026 — Primeira execução do CI

- **Agente:** Buscar por jogos (Claude) · **Repositório:** Felixo-Game-Launcher
- **Contexto:** primeiro envio da `main` (10 commits). O GitHub recusou a primeira tentativa porque o token do `gh` não tinha o escopo `workflow`, exigido para enviar arquivos de `.github/workflows/`. O mantenedor concedeu o escopo e o envio foi refeito.
- **Resultado medido:** execução [42055149702](https://github.com/Felipe-Alcantara/Felixo-Game-Launcher/actions/runs/42055149702) no commit `c5415f4`, com 6 de 6 jobs verdes (Ubuntu, Windows e macOS, com Python 3.11 e 3.13), em cerca de 10 minutos.
- **O que isso prova:** `ruff`, os 28 testes e a verificação de links passam nos três sistemas. Os testes de caminho de configuração, que simulam cada sistema, passaram também em cada sistema real.
- **Limitação:** o CI valida o menu e a documentação; o aplicativo (Fase 1) ainda não existe, então ainda não há o que testar além disso.

---

## 05/11/2026 — Premissa e travas adotadas; início da Fase 1

- **Agente:** Buscar por jogos (Claude) · **Repositório:** Felixo-Game-Launcher
- **Contexto:** com o CI verde, o mantenedor respondeu "Pode ir fazendo tudo" ao resumo de pendências (Fase 1, premissa da decisão 2, travas 3 a 5 das ações no disco e registro no Notion).
- **Decisão:** a premissa da decisão 2 e as travas 3 a 5 viram regras do projeto, e o início da Fase 1 está autorizado.
- **Motivo:** as travas só restringem (agir dentro da pasta de instalação, bloquear em disco ausente, testar em pastas temporárias), então adotá-las é o caminho conservador.
- **Limitação:** a aprovação veio em bloco, não item a item. Para afrouxar uma trava, o mantenedor precisa dizer isso explicitamente.

---

## 05/11/2026 — Núcleo Rust: fonte Steam

- **Agente:** Buscar por jogos (Claude) · **Repositório:** Felixo-Game-Launcher
- **Mudança:** workspace Cargo e `crates/fontes` (contrato `Fonte`, leitor VDF, fonte Steam e o exemplo `detectar`). O `Cargo.lock` passa a ser versionado, por ser um aplicativo.
- **Decisões:** `detectar` devolve `Deteccao { jogos, avisos }` e nunca falha, para que um disco ausente ou um manifesto corrompido não derrube os demais jogos. `abrir` recebe só o identificador dentro da fonte; a proposta da arquitetura usava o `Jogo` inteiro e uma lista de plataformas, que ficam para quando houver a segunda fonte. A arquitetura foi atualizada no mesmo passo.
- **Validação de fronteira:** o `appid` só aceita dígitos (ele vira URL) e o `installdir` não pode ter separador nem `..` (será a base das ações no disco).
- **Medido:** 24 testes verdes e `clippy -D warnings` limpo. Contra uma instalação real da Steam, o exemplo achou 30 jogos, os mesmos de um levantamento feito à mão, e transformou a biblioteca de um disco ausente em aviso.
- **Achado de ambiente:** o toolchain `gnu` do Rust com o gcc do MSYS2 falha ao linkar quando o diretório de build tem acento no caminho: o `ld.exe` não encontra os objetos. Definir `CARGO_TARGET_DIR` num caminho ASCII resolve.
- **Limitação:** a abertura (`steam://rungameid/...`) só foi testada com identificadores inválidos; a abertura de verdade depende do aplicativo.

---

## 05/11/2026 — Biblioteca local em SQLite

- **Agente:** Buscar por jogos (Claude) · **Repositório:** Felixo-Game-Launcher
- **Mudança:** `crates/biblioteca`, a camada que guarda os jogos detectados.
- **Decisões:** o esquema é versionado por `PRAGMA user_version`, e um banco criado por versão mais nova é recusado sem tocar no arquivo. A sincronização roda numa transação só. O banco usa WAL e espera até 5 s por trava, porque a varredura e o observador gravam ao mesmo tempo. A busca trata `%` e `_` digitados como texto, e o `ORDER BY` vem de um enum, nunca de texto do usuário.
- **Regra central, testada:** jogo que some da detecção fica indisponível e guarda a última vez em que foi visto; nunca é removido. Sincronizar uma fonte não mexe nas outras.
- **Medido:** 14 testes verdes (incluindo reabrir o arquivo, duas conexões gravando e versão futura) e `clippy -D warnings` limpo no workspace.
- **Limitações:** o `LIKE` do SQLite ignora maiúsculas só em ASCII, então a busca sem acento fica para depois. Caminhos que não são UTF-8 são gravados com perda.

---

## 05/11/2026 — Interface web em TypeScript

- **Agente:** Buscar por jogos (Claude) · **Repositório:** Felixo-Game-Launcher
- **Mudança:** `ui/` (Vite e TypeScript), `package.json` com versões exatas e `package-lock.json`. A tela lista os jogos, com busca, ordenação por nome ou tamanho, botões Atualizar e Jogar, em português e inglês.
- **Decisões:** o nome do jogo vem do disco, então entra sempre como texto e nunca como HTML. A camada `api.ts` fala com o shell Tauri e, só em desenvolvimento no navegador, com dados simulados que o build de produção descarta. A paridade dos textos entre idiomas é garantida pelo compilador (`Record<Chave, string>`) e por um teste dos `{marcadores}`.
- **Bug achado por teste:** `ehChave("toString")` devolvia verdadeiro porque o operador `in` enxerga propriedades herdadas; corrigido com `Object.hasOwn`.
- **Achado de acessibilidade:** a borda dos campos e botões tinha 1,4:1 a 1,6:1 de contraste, abaixo dos 3:1 exigidos para elementos de interface. Ganhou um token próprio (`--borda-controle`, acima de 4,4:1). Os textos ficaram entre 7,3:1 e 18,4:1. As razões foram calculadas, não estimadas.
- **Medido:** checagem de tipos limpa, 18 testes, build de produção (JS de cerca de 8 kB) e `npm audit` sem vulnerabilidades. No navegador, com a simulação: busca, ordenação (tamanho desconhecido por último), troca de idioma, 0 campos sem rótulo, 0 botões sem nome acessível, foco visível e nenhum erro no console. Um nome de jogo com `<img onerror=…>` apareceu como texto e a página ficou com 0 imagens.
- **Limitações:** os avisos da detecção chegam em português do núcleo (marcado como `shortcut` no código). A tela ainda não rodou dentro do Tauri. O leiaute estreito (celular) foi escrito em CSS mas não foi testado em tela pequena.

---

## 05/11/2026 — Abrir jogo pela biblioteca

- **Agente:** Buscar por jogos (Claude) · **Repositório:** Felixo-Game-Launcher
- **Mudança:** `Biblioteca::abrir_jogo` e os erros `JogoNaoEncontrado`, `JogoIndisponivel`, `FonteDesconhecida` e `Fonte`.
- **Decisão:** a regra "jogo indisponível não abre" mora na biblioteca, não no shell do Tauri. A arquitetura diz que a borda não tem regra de negócio, e assim a regra é testável com uma fonte falsa, sem abrir janela. Abrir um jogo de disco ausente faria o launcher original tentar reinstalá-lo ou falhar de forma confusa.
- **Medido:** 19 testes na biblioteca (5 novos, incluindo o que garante que a fonte não é chamada quando o jogo está indisponível) e `clippy -D warnings` limpo.

---

## 05/11/2026 — Ícone do aplicativo gerado por script

- **Agente:** Buscar por jogos (Claude) · **Repositório:** Felixo-Game-Launcher
- **Mudança:** `scripts/icones/gerar_icone.py` (só biblioteca padrão), o ícone-fonte em `assets/` e os formatos gerados pelo Tauri CLI em `src-tauri/icons/` (o Tauri exige o `.ico` para compilar no Windows).
- **Motivo:** um ícone próprio e reproduzível, sem baixar arte de terceiros nem depender de licença alheia. Os formatos de Android, iOS e da Microsoft Store foram descartados: o aplicativo é só para desktop.
- **Medido:** 3 testes (PNG válido com CRC correto, cantos transparentes, play branco sobre fundo em degradê); o ícone de 1024 pixels sai em cerca de 4 s.

---

## 05/11/2026 — Detecção da Steam em paralelo e com uma sondagem por disco

- **Agente:** Buscar por jogos (Claude) · **Repositório:** Felixo-Game-Launcher
- **O que aconteceu:** ao rodar o aplicativo de verdade, a tela ficou em "Atualizando…" e o banco com 0 jogos. Os testes de unidade não pegavam isso. Cronometrada sozinha, a detecção levava **28,9 s**; cada acesso ao leitor de cartão vazio custava cerca de 4,7 s, e o código sondava a unidade ausente várias vezes (verificar, canonicalizar e ler).
- **Mudança:** cada biblioteca é sondada uma única vez (`read_dir`) e todas são lidas ao mesmo tempo (`std::thread::scope`), mesclando o resultado na ordem do VDF para continuar determinístico. A comparação de pastas deixou de usar `canonicalize` (que consulta o disco) e passou a normalizar o texto.
- **Medido:** a detecção contra a mesma instalação real caiu de 28,9 s para **2,2 s**, com o mesmo resultado (30 jogos e o mesmo aviso do disco ausente). 29 testes nas fontes, 4 novos (grafias do registro e do VDF, várias bibliotecas e ordem dos avisos).
- **Limitação:** links simbólicos não são resolvidos na comparação de pastas; dois links para a mesma biblioteca só repetem avisos (marcado como `shortcut` no código).

---

## 05/11/2026 — Aplicativo Tauri e contrato com a interface

- **Agente:** Buscar por jogos (Claude) · **Repositório:** Felixo-Game-Launcher
- **Mudança:** `src-tauri` (Tauri 2, três comandos, CSP restritiva e permissões mínimas) e `crates/ipc` (`ErroApp` e `Atualizacao`). A biblioteca ganhou testes do formato JSON de `Jogo` e de `Ordem`.
- **Decisão:** os tipos do contrato ficam fora do shell. Motivo medido: o executável de teste de um crate Tauri não abre no Windows (`0xC0000139`, falta o manifesto do Common Controls v6), o que vale também para o toolchain MSVC. O shell não tem testes de unidade (`test = false`, com o motivo escrito no manifesto) e o que dá para testar mora em `ipc`, `biblioteca` e `fontes`.
- **Verificado rodando o aplicativo de verdade** (janela do WebView2 inspecionada por depuração remota, banco limpo antes): a tela chegou a "Biblioteca atualizada: 30 detectados, 30 novos", com 30 linhas, o aviso do disco ausente e 30 jogos no SQLite. A captura da janela real mostrou os tamanhos corretos.
- **Medido:** `clippy -D warnings` limpo no workspace; 21 testes na biblioteca, 29 nas fontes e 5 no contrato.
- **Achados de ambiente:** com o toolchain `gnu` do Rust e o MSYS2, o `windres` não abre o ícone quando o caminho **do código** tem acento (antes, o `ld.exe` já falhava com acento no caminho do build). O `tauri-build` canonicaliza o caminho, então nem uma junção resolve; compilei por um espelho em caminho ASCII. Isso não afeta o CI do GitHub.
- **Limitações:** o botão Jogar foi testado só com identificadores inválidos e pelas regras da biblioteca; abrir um jogo de verdade não foi feito, porque lançaria um jogo no computador de quem testou. Linux e macOS ficam por conta do CI.

---

## 05/11/2026 — Guia de desenvolvimento

- **Agente:** Buscar por jogos (Claude) · **Repositório:** Felixo-Game-Launcher
- **Mudança:** `docs/DESENVOLVIMENTO.md` (requisitos por sistema, comandos do dia a dia e as armadilhas medidas), ligado do README, do `CONTRIBUTING.md` e do `AGENTS.md`, que também ganharam o gate do Rust e da interface.
- **Correção de uma afirmação minha:** o guia dizia, no começo do dia, que o acento só atrapalhava no diretório de build. A medição seguinte mostrou que o `windres` também falha quando a pasta do código tem acento, e o guia foi corrigido antes de ser commitado.
- **Medido x suposto:** as duas falhas por acento foram observadas; a alternativa com o Build Tools da Microsoft (MSVC) é o caminho oficial do Tauri, mas não foi exercitada neste computador, que não tem o Build Tools.

---

## 05/11/2026 — Correção de uma afirmação sobre o executável de teste do Tauri

- **Agente:** Buscar por jogos (Claude) · **Repositório:** Felixo-Game-Launcher
- **Correção:** a entrada "Aplicativo Tauri e contrato com a interface" diz que o executável de teste de um crate Tauri não abre no Windows "também com o toolchain MSVC". Isso **não foi medido**: o erro `0xC0000139` foi observado só com o toolchain GNU. Com o MSVC é um problema conhecido do Tauri, mas não foi reproduzido aqui. Os documentos de desenvolvimento e de arquitetura foram ajustados; esta entrada preserva o registro anterior, como manda a regra de só acrescentar.
- **Efeito prático:** nenhum. O shell tem `test = false`, então o CI do Windows não tenta abrir esse executável de qualquer forma.

---

## 05/11/2026 — CI com Rust e interface nos três sistemas

- **Agente:** Buscar por jogos (Claude) · **Repositório:** Felixo-Game-Launcher
- **Mudança:** o CI ganha o job `interface` (tipos, testes, build e `npm audit` com Node 22) e o job `rust` em Ubuntu, Windows e macOS: `cargo fmt --check`, `clippy -D warnings` e testes em todo o workspace, com `--locked`. O Tauri embute a interface já compilada, então o job compila a interface antes do `cargo`. No Linux instala os pacotes do WebKitGTK.
- **Motivo:** o shell Tauri só compila de verdade nos três sistemas, e a promessa do README de testar nos três precisa ser verdade para o Rust também.
- **Limitação:** este job nunca rodou fora do computador do autor, que usou o toolchain GNU; o MSVC, o Linux e o macOS são exercitados pela primeira vez no GitHub. O resultado será registrado em entrada própria.

---

## 05/11/2026 — Release por tag

- **Agente:** Buscar por jogos (Claude) · **Repositório:** Felixo-Game-Launcher
- **Mudança:** `.github/workflows/release.yml`: ao enviar uma tag `v*`, compila o aplicativo em Windows, Ubuntu 22.04 e macOS com `tauri-apps/tauri-action` e publica os instaladores numa Release marcada como pré-lançamento. Só o job de publicação tem permissão de escrita.
- **Decisões:** pré-lançamento porque os binários **não têm assinatura de código**: o Windows pode mostrar o SmartScreen e o macOS pode pedir para abrir pelo menu de contexto. O macOS é o `macos-latest`, que hoje é Apple Silicon: não há build para Mac com Intel ainda.
- **Limitação:** o workflow nunca rodou. Ele só será exercitado quando a primeira tag for enviada; o resultado entra em entrada própria.

---

## 05/11/2026 — Versão 0.1.0

- **Agente:** Buscar por jogos (Claude) · **Repositório:** Felixo-Game-Launcher
- **Mudança:** a versão do aplicativo (`tauri.conf.json`, `src-tauri/Cargo.toml`, `package.json` e os lockfiles) passa de 0.0.0 para 0.1.0. Os crates internos continuam em 0.0.0, porque não são publicados.
- **Motivo:** os instaladores da Release herdam a versão do `tauri.conf.json`; uma tag `v0.1.0` com binários chamados 0.0.0 confundiria quem baixa.

---

## 05/11/2026 — Estado da Fase 1 no README e no roadmap

- **Agente:** Buscar por jogos (Claude) · **Repositório:** Felixo-Game-Launcher
- **Mudança:** o README (em português e em inglês) passa a dizer o que o aplicativo já faz, sem prometer instalador, e o roadmap ganha a coluna "Estado": Fase 0 feita, Fase 1 em andamento, as demais por fazer.
- **Cuidado:** o texto diz com todas as letras que o botão Jogar ainda não foi testado de ponta a ponta e que não há instalador para baixar. Isso muda quando a primeira Release existir e for conferida.

---

## 05/11/2026 — Resultado do CI do Rust e da interface

- **Agente:** Buscar por jogos (Claude) · **Repositório:** Felixo-Game-Launcher
- **Resultado medido:** execução [42063499358](https://github.com/Felipe-Alcantara/Felixo-Game-Launcher/actions/runs/42063499358) no commit `a54f754`: **10 de 10 jobs verdes**. São os 6 de qualidade (Python 3.11 e 3.13), a interface (tipos, testes, build e `npm audit`) e o Rust em Ubuntu, Windows e macOS (`fmt`, `clippy -D warnings` e testes, com `--locked`).
- **O que isso prova:** o aplicativo Tauri compila e passa no `clippy` nos três sistemas, e o Windows do GitHub usa o toolchain MSVC, que não existe no computador do autor. Até aqui o Rust só tinha rodado com o toolchain GNU.
- **O que não prova:** abrir um jogo de verdade, nem que os instaladores sejam gerados; o `cargo` do CI não empacota. O shell tem `test = false`, então a suposição de que o executável de teste do Tauri também quebra no MSVC continua não medida.
- **Próximo:** a tag `v0.1.0` foi enviada para exercitar o workflow de Release pela primeira vez; o resultado entra em entrada própria.
