# Decisões da v1

> **O que é:** as decisões de produto e de tecnologia que já foram tomadas, com a consequência de cada uma.
> **Quando usar:** antes de propor mudança de rumo. Decisão registrada aqui não se reabre sem pedido explícito de quem mantém o projeto.
> **Origem:** alinhamento por questionário de 19 perguntas, em 05/11/2026, antes de qualquer código. Para o recorte, veja [ESCOPO-E-ROADMAP.md](ESCOPO-E-ROADMAP.md).

| # | Tema | Decisão | Consequência |
| --- | --- | --- | --- |
| 1 | Ações no disco | O launcher pode mover e apagar, com confirmação dupla | É a parte de maior risco; ganha travas próprias ([ARQUITETURA.md](ARQUITETURA.md)) |
| 2 | Contas, rede e dados | Open source, roda local, sem login em loja e sem rede obrigatória; distribuído em GitHub Releases | Nada sai do computador; capas vêm de arte local. Chave de API opcional fica como evolução |
| 3 | Abrir um jogo | Híbrido: pelo launcher original e, se não houver, pelo executável | Mantém login e anti-cheat quando possível; exige dois caminhos testados |
| 4 | Plataformas | Windows completo; Linux e macOS com limitações documentadas | Launchers só do Windows (Riot, Battle.net, Rockstar) ficam nele; CI roda nos três sistemas |
| 5 | Apagar | Lixeira quando possível; apagar de forma permanente só com aviso extra | Onde a Lixeira não existe (disco removível) ou não comporta o tamanho, há uma terceira confirmação |
| 6 | Detecção | Varredura completa em segundo plano; obrigatória na primeira vez e opcional nas seguintes | Primeira execução demora; precisa de progresso visível e de cuidado com discos lentos ou com defeito |
| 7 | Atualização | Ao abrir, pelo botão Atualizar e por observador de pastas em segundo plano | Há escrita concorrente na biblioteca, o que pesou na escolha do SQLite |
| 8 | Fontes da v1 | Steam, Epic, GOG; Riot, Ubisoft, Battle.net, Rockstar, EA; avulsos e emuladores; Minecraft, Roblox, osu! e Microsoft Store | Escopo grande; por isso as [fases](ESCOPO-E-ROADMAP.md) |
| 9 | Tecnologia | Tauri (Rust com interface web) | Executável leve e varredura rápida; duas linguagens |
| 10 | Persistência | SQLite local | Consultas, ordenação por tamanho e escrita concorrente segura |
| 11 | Entregas da v1 | Gestão de espaço, organização, histórico de uso, ícones e capas locais | Veja o escopo |
| 12 | Fora da v1 | Instalar e atualizar jogos, backup de saves, modo TV, mods e patches | O launcher organiza o que já está instalado |
| 13 | Validação | Uso manual no computador do mantenedor mais testes automatizados | Apagar e mover são testados só em pastas descartáveis |
| 14 | CI | Windows, Linux e macOS desde o primeiro commit | Gratuito em repositório público; também gera os binários da Release |
| 15 | Licença | MIT | Permissiva, a mesma dos demais repositórios do autor |
| 16 | Nome | Felixo-Game-Launcher | Segue o padrão `Felixo-<descrição>` |
| 17 | Idioma | Interface e documentação em português e inglês | Dobra a manutenção dos textos |
| 18 | Evolução | Adaptadores da comunidade para novas lojas | Contrato de fonte estável e isolado desde a v1 |
| 19 | Complemento | Nenhum item adicional | — |
