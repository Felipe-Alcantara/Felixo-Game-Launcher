import { describe, expect, it } from "vitest";
import { formatarTamanho } from "./format";

const GB = 1024 ** 3;

describe("formatarTamanho", () => {
  it("mostra traço quando o tamanho é desconhecido", () => {
    expect(formatarTamanho(null, "pt-BR")).toBe("—");
  });

  it("não usa casas decimais em bytes", () => {
    expect(formatarTamanho(0, "pt-BR")).toBe("0 B");
    expect(formatarTamanho(512, "pt-BR")).toBe("512 B");
  });

  it("usa vírgula em português e ponto em inglês", () => {
    expect(formatarTamanho(1536, "pt-BR")).toBe("1,5 KB");
    expect(formatarTamanho(1536, "en")).toBe("1.5 KB");
  });

  it("usa uma casa decimal abaixo de 100 e nenhuma a partir de 100", () => {
    expect(formatarTamanho(5 * GB, "pt-BR")).toBe("5,0 GB");
    expect(formatarTamanho(120 * GB, "pt-BR")).toBe("120 GB");
  });

  it("para na maior unidade conhecida", () => {
    expect(formatarTamanho(3 * 1024 ** 5, "en")).toMatch(/TB$/);
  });
});
