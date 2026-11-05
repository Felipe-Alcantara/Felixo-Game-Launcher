// Dados de mentira para desenvolver a tela no navegador (`npm run dev`), sem o shell Tauri.
// Nada aqui entra no build de produção: `api.ts` só importa este arquivo em modo DEV.

import type { Atualizacao, Jogo } from "./api";

const GB = 1024 ** 3;

function jogo(id: number, nome: string, gb: number | null, disponivel = true): Jogo {
  return {
    id,
    fonte: "steam",
    id_externo: String(1000 + id),
    nome,
    pasta: null,
    tamanho_bytes: gb === null ? null : Math.round(gb * GB),
    disponivel,
    visto_em: 1_800_000_000,
  };
}

const JOGOS: Jogo[] = [
  jogo(1, "Aventura Estelar", 112.3),
  jogo(2, "Corrida dos Reinos", 64.1),
  jogo(3, "Pôr do Sol: Edição Épica", 24.6),
  jogo(4, "Puzzle Minúsculo", 0.3),
  jogo(5, "Jogo de um nome absurdamente comprido para testar a quebra de linha na tabela e no celular", 6.5),
  jogo(6, "Tamanho Desconhecido", null),
  jogo(7, "Saudade do Disco Externo", 35.1, false),
  // Prova visual de que o nome é tratado como texto, nunca como HTML:
  jogo(8, '<b>Negrito?</b> & "aspas" <img src=x onerror=alert(1)>', 1.2),
  jogo(9, "100% Orange Juice", 1.1),
  jogo(10, "Snake_Game", 0.2),
];

const espera = (ms: number): Promise<void> => new Promise((resolver) => setTimeout(resolver, ms));

export async function simular<T>(comando: string, args: Record<string, unknown> = {}): Promise<T> {
  await espera(comando === "atualizar_biblioteca" ? 700 : 60);
  switch (comando) {
    case "listar_jogos": {
      const busca = String(args["busca"] ?? "").trim().toLowerCase();
      const lista = JOGOS.filter((j) => j.nome.toLowerCase().includes(busca));
      if (args["ordem"] === "tamanho_decrescente") {
        lista.sort((a, b) => (b.tamanho_bytes ?? -1) - (a.tamanho_bytes ?? -1));
      } else {
        lista.sort((a, b) => a.nome.localeCompare(b.nome));
      }
      return lista as T;
    }
    case "atualizar_biblioteca": {
      const resultado: Atualizacao = {
        detectados: JOGOS.length - 1,
        novos: 0,
        atualizados: JOGOS.length - 1,
        indisponiveis: 1,
        avisos: ["Biblioteca indisponível: X:\\Jogos\\Steam (disco desconectado?)"],
      };
      return resultado as T;
    }
    case "abrir_jogo": {
      const alvo = JOGOS.find((j) => j.id === args["id"]);
      if (!alvo) throw { codigo: "jogo_nao_encontrado" };
      if (!alvo.disponivel) throw { codigo: "jogo_indisponivel" };
      return undefined as T;
    }
    default:
      throw { codigo: "comando_desconhecido", detalhe: comando };
  }
}
