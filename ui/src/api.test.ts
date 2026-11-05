import { describe, expect, it } from "vitest";
import { mensagemDeErro } from "./api";

describe("mensagemDeErro", () => {
  it("traduz os códigos conhecidos do shell", () => {
    expect(mensagemDeErro({ codigo: "jogo_indisponivel" }, "pt-BR")).toContain("indisponível");
    expect(mensagemDeErro({ codigo: "jogo_indisponivel" }, "en")).toContain("unavailable");
    expect(mensagemDeErro({ codigo: "jogo_nao_encontrado" }, "en")).toContain("not found");
    expect(mensagemDeErro({ codigo: "fora_do_tauri" }, "pt-BR")).toContain("aplicativo");
  });

  it("mostra o detalhe quando o código é desconhecido", () => {
    const mensagem = mensagemDeErro({ codigo: "biblioteca", detalhe: "disco cheio" }, "pt-BR");
    expect(mensagem).toContain("disco cheio");
  });

  it("aceita um Error comum, uma string e valores estranhos sem quebrar", () => {
    expect(mensagemDeErro(new Error("falhou"), "en")).toContain("falhou");
    expect(mensagemDeErro("texto solto", "en")).toContain("texto solto");
    expect(mensagemDeErro(null, "en")).toContain("null");
    expect(mensagemDeErro({}, "en")).not.toContain("undefined");
  });
});
