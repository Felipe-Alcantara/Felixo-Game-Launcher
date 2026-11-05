# Arquitetura-alvo

> **O que é:** a organização em camadas proposta para o aplicativo.
> **Status:** proposta para a Fase 1. **Nada disto está implementado.** O contrato pode mudar quando o primeiro adaptador for escrito; qualquer mudança vira uma entrada nova no `IA.md`.
> **Quando usar:** antes de criar um módulo, uma fonte nova ou qualquer ação que altere arquivos. As decisões de base estão em [DECISOES.md](DECISOES.md) e as fontes em [FONTES-DE-JOGOS.md](FONTES-DE-JOGOS.md).

## Camadas

| Camada | Responsabilidade | Não faz |
| --- | --- | --- |
| `crates/fontes` | Um módulo por loja ou launcher, cada um implementando o contrato `Fonte` | Não conhece interface, banco nem ações destrutivas |
| `crates/biblioteca` | Persistência em SQLite: jogos, tamanhos, tags, histórico | Não lê arquivos de lojas |
| `crates/varredura` | Varredura em segundo plano e observador de pastas | Não grava direto: entrega resultados à biblioteca |
| `crates/acoes` | Mover e apagar arquivos, isolado e com as travas abaixo | Não decide o que apagar: recebe um pedido já confirmado |
| `src-tauri` | Borda: comandos entre a interface e o núcleo | **Sem regra de negócio** |
| `ui/` | Interface web em TypeScript, com pt-BR e en | Não acessa o disco |

Só `fontes` lê arquivos de lojas. Só `acoes` altera arquivos. Só a borda fala com a interface.

## Contrato de uma fonte (ilustrativo)

```rust
pub trait Fonte {
    fn id(&self) -> &'static str;
    fn plataformas(&self) -> &'static [Plataforma];
    fn detectar(&self) -> Result<Vec<JogoDetectado>, ErroFonte>;
    fn abrir(&self, jogo: &Jogo) -> Result<(), ErroFonte>;
}
```

Uma fonte nova — inclusive de quem contribui — implementa só isto. Se o launcher da loja não estiver instalado, `detectar` devolve lista vazia, não erro.

## Travas das ações no disco

Mover e apagar são a parte de maior risco do projeto.

**Decididas** (veja [DECISOES.md](DECISOES.md), itens 1 e 5):

1. Confirmação dupla na interface, mostrando o caminho completo e o tamanho.
2. Lixeira por padrão. Onde ela não existe ou não comporta o tamanho, o aplicativo avisa e pede uma confirmação extra antes de apagar de forma permanente.

**Adotadas** (propostas do projeto, aprovadas em 05/11/2026; veja [DECISOES.md](DECISOES.md)):

3. Só agir dentro da pasta de instalação conhecida do jogo, resolvendo links simbólicos; nunca na raiz de um disco nem em pasta do sistema.
4. Bloquear a ação em disco ausente ou com alerta de saúde.
5. Testar sempre em pastas temporárias, nunca em jogos reais.
