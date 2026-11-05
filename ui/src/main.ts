import {
  abrirJogo,
  atualizarBiblioteca,
  listarJogos,
  mensagemDeErro,
  type Jogo,
  type Ordem,
} from "./api";
import { formatarTamanho } from "./format";
import { contagem, ehChave, escolherIdioma, traduzir, type Chave, type Idioma } from "./i18n";

const NOMES_DAS_FONTES: Record<string, string> = { steam: "Steam" };
const CHAVE_IDIOMA = "idioma";
const ATRASO_DA_BUSCA_MS = 150;

let idioma: Idioma = escolherIdioma(localStorage.getItem(CHAVE_IDIOMA), navigator.language);
let ordem: Ordem = "nome";
let busca = "";
let jogos: Jogo[] = [];
let ocupado = false;
let avisos: string[] = [];
let temporizadorDaBusca: number | undefined;

function elemento<T extends HTMLElement>(id: string): T {
  const achado = document.getElementById(id);
  if (!achado) throw new Error(`O elemento #${id} não existe no HTML.`);
  return achado as T;
}

const t = (chave: Chave, valores?: Record<string, string | number>): string =>
  traduzir(idioma, chave, valores);

function mostrarStatus(mensagem: string): void {
  elemento("status").textContent = mensagem;
}

function aplicarTextos(): void {
  document.documentElement.lang = idioma;
  for (const alvo of document.querySelectorAll<HTMLElement>("[data-i18n]")) {
    const chave = alvo.dataset["i18n"];
    if (chave !== undefined && ehChave(chave)) alvo.textContent = t(chave);
  }
  elemento<HTMLSelectElement>("idioma").value = idioma;
}

function linhaDoJogo(jogo: Jogo): HTMLTableRowElement {
  const linha = document.createElement("tr");

  // Nome, loja e tamanho entram como texto (append de string), nunca como HTML: o nome vem do disco.
  const nome = document.createElement("td");
  nome.className = "nome";
  nome.append(jogo.nome);
  if (!jogo.disponivel) {
    const selo = document.createElement("span");
    selo.className = "selo";
    selo.textContent = t("indisponivel");
    nome.append(selo);
  }

  const fonte = document.createElement("td");
  fonte.className = "col-fonte";
  fonte.append(NOMES_DAS_FONTES[jogo.fonte] ?? jogo.fonte);

  const tamanho = document.createElement("td");
  tamanho.className = "num";
  tamanho.append(formatarTamanho(jogo.tamanho_bytes, idioma));

  const acao = document.createElement("td");
  const botao = document.createElement("button");
  botao.type = "button";
  botao.textContent = t("jogar");
  botao.setAttribute("aria-label", t("jogar_rotulo", { nome: jogo.nome }));
  botao.disabled = !jogo.disponivel;
  botao.addEventListener("click", () => void abrir(jogo));
  acao.append(botao);

  linha.append(nome, fonte, tamanho, acao);
  return linha;
}

function renderizarJogos(): void {
  elemento("corpo").replaceChildren(...jogos.map(linhaDoJogo));
  const vazio = elemento("vazio");
  vazio.hidden = jogos.length > 0;
  vazio.textContent = busca.trim() ? t("vazio_busca") : t("vazio_inicial");
  elemento("tabela").hidden = jogos.length === 0;
  elemento("contagem").textContent = jogos.length > 0 ? contagem(idioma, jogos.length) : "";
}

function renderizarAvisos(): void {
  const secao = elemento("avisos");
  secao.hidden = avisos.length === 0;
  // shortcut: os avisos da detecção chegam em português do núcleo; traduzir quando virarem códigos.
  elemento("avisos-lista").replaceChildren(
    ...avisos.map((aviso) => {
      const item = document.createElement("li");
      item.append(aviso);
      return item;
    }),
  );
}

async function carregar(): Promise<void> {
  try {
    jogos = await listarJogos(busca, ordem);
    renderizarJogos();
  } catch (erro) {
    mostrarStatus(mensagemDeErro(erro, idioma));
  }
}

async function atualizar(): Promise<void> {
  if (ocupado) return;
  ocupado = true;
  elemento<HTMLButtonElement>("atualizar").disabled = true;
  mostrarStatus(t("atualizando"));
  try {
    const resultado = await atualizarBiblioteca();
    avisos = resultado.avisos;
    renderizarAvisos();
    await carregar();
    mostrarStatus(
      t("resumo_atualizacao", {
        detectados: resultado.detectados,
        novos: resultado.novos,
        indisponiveis: resultado.indisponiveis,
      }),
    );
  } catch (erro) {
    mostrarStatus(mensagemDeErro(erro, idioma));
  } finally {
    ocupado = false;
    elemento<HTMLButtonElement>("atualizar").disabled = false;
  }
}

async function abrir(jogo: Jogo): Promise<void> {
  mostrarStatus(t("abrindo", { nome: jogo.nome }));
  try {
    await abrirJogo(jogo.id);
  } catch (erro) {
    mostrarStatus(mensagemDeErro(erro, idioma));
  }
}

function ligarEventos(): void {
  elemento("atualizar").addEventListener("click", () => void atualizar());
  elemento("filtros").addEventListener("submit", (evento) => evento.preventDefault());
  elemento<HTMLInputElement>("busca").addEventListener("input", (evento) => {
    busca = (evento.target as HTMLInputElement).value;
    window.clearTimeout(temporizadorDaBusca);
    temporizadorDaBusca = window.setTimeout(() => void carregar(), ATRASO_DA_BUSCA_MS);
  });
  elemento<HTMLSelectElement>("ordem").addEventListener("change", (evento) => {
    ordem = (evento.target as HTMLSelectElement).value as Ordem;
    void carregar();
  });
  elemento<HTMLSelectElement>("idioma").addEventListener("change", (evento) => {
    idioma = (evento.target as HTMLSelectElement).value as Idioma;
    localStorage.setItem(CHAVE_IDIOMA, idioma);
    aplicarTextos();
    renderizarJogos();
    mostrarStatus("");
  });
}

async function iniciar(): Promise<void> {
  aplicarTextos();
  ligarEventos();
  await carregar(); // mostra a biblioteca guardada na hora
  await atualizar(); // e atualiza ao abrir, como decidido (decisão 7)
}

void iniciar();
