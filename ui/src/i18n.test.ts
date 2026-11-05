import { describe, expect, it } from "vitest";
import { CHAVES, IDIOMAS, contagem, ehChave, escolherIdioma, traduzir } from "./i18n";

const marcadores = (texto: string): string[] =>
  [...texto.matchAll(/\{(\w+)\}/g)].map((achado) => achado[1] ?? "").sort();

describe("textos", () => {
  it("têm o mesmo conjunto de {marcadores} em todos os idiomas", () => {
    for (const chave of CHAVES) {
      const [primeiro, ...demais] = IDIOMAS.map((idioma) =>
        marcadores(traduzir(idioma, chave)),
      );
      for (const outro of demais) {
        expect(outro, `marcadores diferentes em "${chave}"`).toEqual(primeiro);
      }
    }
  });

  it("nenhum texto fica vazio", () => {
    for (const idioma of IDIOMAS) {
      for (const chave of CHAVES) {
        expect(traduzir(idioma, chave).trim(), `${idioma}/${chave}`).not.toBe("");
      }
    }
  });
});

describe("traduzir", () => {
  it("troca os marcadores pelos valores", () => {
    expect(traduzir("pt-BR", "jogar_rotulo", { nome: "Alfa" })).toBe("Jogar Alfa");
    expect(traduzir("en", "jogar_rotulo", { nome: "Alfa" })).toBe("Play Alfa");
  });

  it("deixa o marcador visível se faltar o valor, em vez de escrever 'undefined'", () => {
    expect(traduzir("en", "jogar_rotulo")).toBe("Play {nome}");
  });

  it("não interpreta o valor como marcador nem como HTML", () => {
    const nome = "{n} <b>x</b>";
    expect(traduzir("en", "jogar_rotulo", { nome })).toBe("Play {n} <b>x</b>");
  });
});

describe("contagem", () => {
  it("usa singular para 1 e plural nos demais", () => {
    expect(contagem("pt-BR", 1)).toBe("1 jogo");
    expect(contagem("pt-BR", 2)).toBe("2 jogos");
    expect(contagem("pt-BR", 0)).toBe("0 jogos");
    expect(contagem("en", 1)).toBe("1 game");
    expect(contagem("en", 30)).toBe("30 games");
  });
});

describe("ehChave", () => {
  it("reconhece só chaves existentes", () => {
    expect(ehChave("atualizar")).toBe(true);
    expect(ehChave("toString")).toBe(false);
    expect(ehChave("nao_existe")).toBe(false);
  });
});

describe("escolherIdioma", () => {
  it("respeita a escolha salva", () => {
    expect(escolherIdioma("en", "pt-BR")).toBe("en");
    expect(escolherIdioma("pt-BR", "en-US")).toBe("pt-BR");
  });

  it("sem escolha salva, segue o idioma do sistema", () => {
    expect(escolherIdioma(null, "pt-BR")).toBe("pt-BR");
    expect(escolherIdioma(null, "pt-PT")).toBe("pt-BR");
    expect(escolherIdioma(null, "en-GB")).toBe("en");
    expect(escolherIdioma(null, "ja-JP")).toBe("en");
  });

  it("ignora um valor salvo que não é um idioma conhecido", () => {
    expect(escolherIdioma("klingon", "pt-BR")).toBe("pt-BR");
  });
});
