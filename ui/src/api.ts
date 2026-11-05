import { traduzir, type Idioma } from "./i18n";

export type Ordem = "nome" | "tamanho_decrescente";

/** Espelha `felixo_biblioteca::Jogo`. */
export interface Jogo {
  id: number;
  fonte: string;
  id_externo: string;
  nome: string;
  pasta: string | null;
  tamanho_bytes: number | null;
  disponivel: boolean;
  visto_em: number;
}

/** Espelha o resultado de `atualizar_biblioteca` no shell Tauri. */
export interface Atualizacao {
  detectados: number;
  novos: number;
  atualizados: number;
  indisponiveis: number;
  avisos: string[];
}

function dentroDoTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

async function chamar<T>(comando: string, args?: Record<string, unknown>): Promise<T> {
  if (dentroDoTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return invoke<T>(comando, args);
  }
  if (import.meta.env.DEV) {
    // Só em desenvolvimento no navegador; o build de produção descarta este ramo.
    const { simular } = await import("./simulacao");
    return simular<T>(comando, args);
  }
  throw { codigo: "fora_do_tauri" };
}

export function listarJogos(busca: string, ordem: Ordem): Promise<Jogo[]> {
  return chamar<Jogo[]>("listar_jogos", { busca, ordem });
}

export function atualizarBiblioteca(): Promise<Atualizacao> {
  return chamar<Atualizacao>("atualizar_biblioteca");
}

export function abrirJogo(id: number): Promise<void> {
  return chamar<void>("abrir_jogo", { id });
}

function lerErro(erro: unknown): { codigo: string; detalhe: string } {
  if (typeof erro === "object" && erro !== null) {
    const { codigo, detalhe } = erro as { codigo?: unknown; detalhe?: unknown };
    if (typeof codigo === "string") {
      return { codigo, detalhe: typeof detalhe === "string" ? detalhe : "" };
    }
    if (erro instanceof Error) return { codigo: "", detalhe: erro.message };
  }
  return { codigo: "", detalhe: String(erro) };
}

/** Traduz um erro do shell (`{ codigo, detalhe }`) para uma frase no idioma da tela. */
export function mensagemDeErro(erro: unknown, idioma: Idioma): string {
  const { codigo, detalhe } = lerErro(erro);
  switch (codigo) {
    case "jogo_indisponivel":
      return traduzir(idioma, "erro_jogo_indisponivel");
    case "jogo_nao_encontrado":
      return traduzir(idioma, "erro_jogo_nao_encontrado");
    case "fora_do_tauri":
      return traduzir(idioma, "erro_fora_do_tauri");
    default:
      return traduzir(idioma, "erro_generico", { detalhe: detalhe || codigo || "?" });
  }
}
