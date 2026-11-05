# Arquitetura-alvo

> **O que é:** a organização em camadas proposta para o aplicativo.
> **Status:** em implementação na Fase 1. Já existem `crates/fontes`, `crates/biblioteca`, `crates/ipc`, `src-tauri` e `ui/`; `varredura` e `acoes` ainda não. O contrato pode mudar quando a segunda fonte for escrita; qualquer mudança vira uma entrada nova no `IA.md`.
> **Quando usar:** antes de criar um módulo, uma fonte nova ou qualquer ação que altere arquivos. As decisões de base estão em [DECISOES.md](DECISOES.md) e as fontes em [FONTES-DE-JOGOS.md](FONTES-DE-JOGOS.md).

## Camadas

| Camada | Responsabilidade | Não faz |
| --- | --- | --- |
| `crates/fontes` | Um módulo por loja ou launcher, cada um implementando o contrato `Fonte` | Não conhece interface, banco nem ações destrutivas |
| `crates/biblioteca` | Persistência em SQLite: jogos, tamanhos, tags, histórico; e a regra de só abrir jogo disponível | Não lê arquivos de lojas |
| `crates/varredura` | Varredura em segundo plano e observador de pastas | Não grava direto: entrega resultados à biblioteca |
| `crates/acoes` | Mover e apagar arquivos, isolado e com as travas abaixo | Não decide o que apagar: recebe um pedido já confirmado |
| `crates/ipc` | Contrato com a interface: os erros e o resultado de atualizar que atravessam a ponte do Tauri | Não conhece o Tauri, para ser testável em qualquer sistema |
| `src-tauri` | Borda: comandos entre a interface e o núcleo | **Sem regra de negócio** e sem testes de unidade (o executável de teste de um crate Tauri não abre no Windows; medido com o toolchain GNU) |
| `ui/` | Interface web em TypeScript, com pt-BR e en | Não acessa o disco |

Só `fontes` lê arquivos de lojas. Só `acoes` altera arquivos. Só a borda fala com a interface.

## Contrato de uma fonte

```rust
pub trait Fonte {
    fn id(&self) -> &'static str;
    fn nome(&self) -> &'static str;
    fn detectar(&self) -> Deteccao;
    fn abrir(&self, id_externo: &str) -> Result<(), ErroFonte>;
}
```

Uma fonte nova — inclusive de quem contribui — implementa só isto. `detectar` nunca falha: devolve `Deteccao { jogos, avisos }`. Loja não instalada dá lista vazia, e problemas parciais (disco ausente, manifesto corrompido) vão em `avisos`, sem derrubar os demais jogos. A lista de plataformas por fonte entra quando houver a segunda fonte.

## Travas das ações no disco

Mover e apagar são a parte de maior risco do projeto.

**Decididas** (veja [DECISOES.md](DECISOES.md), itens 1 e 5):

1. Confirmação dupla na interface, mostrando o caminho completo e o tamanho.
2. Lixeira por padrão. Onde ela não existe ou não comporta o tamanho, o aplicativo avisa e pede uma confirmação extra antes de apagar de forma permanente.

**Adotadas** (propostas do projeto, aprovadas em 05/11/2026; veja [DECISOES.md](DECISOES.md)):

3. Só agir dentro da pasta de instalação conhecida do jogo, resolvendo links simbólicos; nunca na raiz de um disco nem em pasta do sistema.
4. Bloquear a ação em disco ausente ou com alerta de saúde.
5. Testar sempre em pastas temporárias, nunca em jogos reais.
