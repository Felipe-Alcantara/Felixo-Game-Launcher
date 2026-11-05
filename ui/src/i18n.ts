export type Idioma = "pt-BR" | "en";

export const IDIOMAS: readonly Idioma[] = ["pt-BR", "en"];

const ptBR = {
  titulo: "Felixo Game Launcher",
  idioma_rotulo: "Idioma",
  atualizar: "Atualizar",
  atualizando: "Atualizando a biblioteca…",
  busca_rotulo: "Buscar jogo",
  ordenar_rotulo: "Ordenar por",
  ordem_nome: "Nome",
  ordem_tamanho: "Tamanho (maior primeiro)",
  tabela_legenda: "Jogos instalados",
  col_jogo: "Jogo",
  col_fonte: "Loja",
  col_tamanho: "Tamanho",
  col_acao: "Ação",
  jogar: "Jogar",
  jogar_rotulo: "Jogar {nome}",
  indisponivel: "Indisponível",
  contagem_um: "1 jogo",
  contagem_varios: "{n} jogos",
  vazio_inicial: "Nenhum jogo na biblioteca ainda. Clique em Atualizar para procurar os jogos instalados.",
  vazio_busca: "Nenhum jogo encontrado para essa busca.",
  resumo_atualizacao:
    "Biblioteca atualizada: {detectados} detectados, {novos} novos, {indisponiveis} indisponíveis.",
  avisos_titulo: "Avisos da última atualização",
  abrindo: "Abrindo {nome}…",
  erro_generico: "Algo deu errado: {detalhe}",
  erro_jogo_indisponivel: "Este jogo está indisponível agora (disco desconectado ou jogo removido).",
  erro_jogo_nao_encontrado: "Jogo não encontrado na biblioteca.",
  erro_fora_do_tauri: "Esta tela só funciona dentro do aplicativo.",
} as const;

export type Chave = keyof typeof ptBR;

export const CHAVES = Object.keys(ptBR) as Chave[];

// `Record<Chave, string>` faz o compilador exigir toda chave nos dois idiomas.
const en: Record<Chave, string> = {
  titulo: "Felixo Game Launcher",
  idioma_rotulo: "Language",
  atualizar: "Refresh",
  atualizando: "Refreshing the library…",
  busca_rotulo: "Search games",
  ordenar_rotulo: "Sort by",
  ordem_nome: "Name",
  ordem_tamanho: "Size (largest first)",
  tabela_legenda: "Installed games",
  col_jogo: "Game",
  col_fonte: "Store",
  col_tamanho: "Size",
  col_acao: "Action",
  jogar: "Play",
  jogar_rotulo: "Play {nome}",
  indisponivel: "Unavailable",
  contagem_um: "1 game",
  contagem_varios: "{n} games",
  vazio_inicial: "No games in the library yet. Click Refresh to look for installed games.",
  vazio_busca: "No games match this search.",
  resumo_atualizacao:
    "Library refreshed: {detectados} detected, {novos} new, {indisponiveis} unavailable.",
  avisos_titulo: "Warnings from the last refresh",
  abrindo: "Opening {nome}…",
  erro_generico: "Something went wrong: {detalhe}",
  erro_jogo_indisponivel: "This game is unavailable right now (disconnected drive or removed game).",
  erro_jogo_nao_encontrado: "Game not found in the library.",
  erro_fora_do_tauri: "This screen only works inside the application.",
};

const textos: Record<Idioma, Record<Chave, string>> = { "pt-BR": ptBR, en };

export function ehChave(valor: string): valor is Chave {
  return Object.hasOwn(ptBR, valor); // `in` aceitaria "toString" e outras propriedades herdadas
}

export function traduzir(
  idioma: Idioma,
  chave: Chave,
  valores: Record<string, string | number> = {},
): string {
  return textos[idioma][chave].replace(/\{(\w+)\}/g, (_inteiro, nome: string) =>
    String(valores[nome] ?? `{${nome}}`),
  );
}

export function contagem(idioma: Idioma, quantidade: number): string {
  return traduzir(idioma, quantidade === 1 ? "contagem_um" : "contagem_varios", {
    n: quantidade,
  });
}

/** Escolha salva > idioma do sistema (português ou inglês) > inglês. */
export function escolherIdioma(salvo: string | null, doSistema: string): Idioma {
  if (salvo === "pt-BR" || salvo === "en") return salvo;
  return doSistema.toLowerCase().startsWith("pt") ? "pt-BR" : "en";
}
