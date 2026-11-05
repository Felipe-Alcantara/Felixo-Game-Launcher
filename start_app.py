#!/usr/bin/env python3
"""Menu de entrada do Felixo Game Launcher.

Porta de entrada única do projeto: ``python start_app.py``.
Usa só a biblioteca padrão e roda em Windows, Linux e macOS.
"""

from __future__ import annotations

import json
import os
import platform
import shutil
import sys
from collections.abc import Callable
from dataclasses import dataclass, field
from pathlib import Path

RAIZ = Path(__file__).resolve().parent
NOME_APP = "felixo-game-launcher"
IDIOMA_PADRAO = "pt-BR"
IDIOMAS = {"pt-BR": "Português (Brasil)", "en": "English"}
URL_PRE_REQUISITOS = "https://tauri.app/start/prerequisites/"
# Ferramentas para construir o aplicativo (Tauri): executável -> nome legível.
FERRAMENTAS = {"git": "Git", "node": "Node.js", "npm": "npm", "rustc": "Rust", "cargo": "Cargo"}

TEXTOS = {
    "pt-BR": {
        "subtitulo": "Todos os seus jogos, de qualquer loja, num só lugar.",
        "op_1": "Iniciar         — abre o launcher",
        "op_2": "Instalar/Setup  — confere o que falta para rodar",
        "op_3": "Configurar      — idioma e preferências",
        "op_4": "Status          — sistema, ferramentas e configuração",
        "op_0": "Sair",
        "escolha": "Escolha uma opção: ",
        "invalida": "Opção inválida: {valor!r}. Digite um dos números do menu.",
        "sem_app": (
            "O aplicativo ainda não foi implementado (Fase 1 do roadmap). "
            "Veja docs/ESCOPO-E-ROADMAP.md."
        ),
        "tudo_ok": "Todas as ferramentas para construir o aplicativo estão instaladas.",
        "faltam": "Faltam ferramentas: {lista}.\nGuia de instalação: {url}",
        "idioma_prompt": "Idioma ({opcoes}): ",
        "idioma_salvo": "Idioma salvo: {nome}.",
        "idioma_invalido": "Idioma não reconhecido: {valor!r}. Nada foi alterado.",
        "erro_salvar": "Não consegui salvar a configuração em {pasta}: {erro}",
        "st_sistema": "Sistema",
        "st_python": "Python",
        "st_ferramentas": "Ferramentas",
        "st_app": "Aplicativo",
        "st_app_sim": "presente (src-tauri)",
        "st_app_nao": "ainda não implementado",
        "st_config": "Configuração",
        "st_idioma": "Idioma",
        "ok": "ok",
        "falta": "falta",
    },
    "en": {
        "subtitulo": "All your games, from any store, in one place.",
        "op_1": "Start           — opens the launcher",
        "op_2": "Install/Setup   — checks what is missing to run",
        "op_3": "Configure       — language and preferences",
        "op_4": "Status          — system, tools and configuration",
        "op_0": "Quit",
        "escolha": "Choose an option: ",
        "invalida": "Invalid option: {valor!r}. Type one of the menu numbers.",
        "sem_app": (
            "The application is not implemented yet (roadmap Phase 1). "
            "See docs/ESCOPO-E-ROADMAP.md."
        ),
        "tudo_ok": "All the tools needed to build the application are installed.",
        "faltam": "Missing tools: {lista}.\nInstallation guide: {url}",
        "idioma_prompt": "Language ({opcoes}): ",
        "idioma_salvo": "Language saved: {nome}.",
        "idioma_invalido": "Unknown language: {valor!r}. Nothing was changed.",
        "erro_salvar": "Could not save the configuration in {pasta}: {erro}",
        "st_sistema": "System",
        "st_python": "Python",
        "st_ferramentas": "Tools",
        "st_app": "Application",
        "st_app_sim": "present (src-tauri)",
        "st_app_nao": "not implemented yet",
        "st_config": "Configuration",
        "st_idioma": "Language",
        "ok": "ok",
        "falta": "missing",
    },
}


def pasta_config() -> Path:
    """Pasta de configuração do usuário, no padrão de cada sistema."""
    if sys.platform == "win32":
        base = Path(os.environ.get("APPDATA") or Path.home() / "AppData" / "Roaming")
    elif sys.platform == "darwin":
        base = Path.home() / "Library" / "Application Support"
    else:
        base = Path(os.environ.get("XDG_CONFIG_HOME") or Path.home() / ".config")
    return base / NOME_APP


def carregar_config(pasta: Path) -> dict[str, str]:
    """Lê a configuração; arquivo ausente, corrompido ou com idioma desconhecido vira o padrão."""
    try:
        dados = json.loads((pasta / "config.json").read_text(encoding="utf-8"))
    except (OSError, ValueError):
        return {"idioma": IDIOMA_PADRAO}
    idioma = dados.get("idioma") if isinstance(dados, dict) else None
    return {"idioma": idioma if idioma in IDIOMAS else IDIOMA_PADRAO}


def salvar_config(pasta: Path, config: dict[str, str]) -> None:
    """Grava a configuração por arquivo temporário, para nunca deixar um JSON pela metade."""
    pasta.mkdir(parents=True, exist_ok=True)
    temporario = pasta / "config.json.tmp"
    temporario.write_text(json.dumps(config, ensure_ascii=False, indent=2), encoding="utf-8")
    os.replace(temporario, pasta / "config.json")


def ferramentas_ausentes() -> list[str]:
    """Nomes das ferramentas de build que não estão no PATH."""
    return [nome for exe, nome in FERRAMENTAS.items() if shutil.which(exe) is None]


@dataclass
class Contexto:
    entrada: Callable[[str], str]
    saida: Callable[[str], None]
    pasta: Path
    config: dict[str, str] = field(default_factory=dict)
    cor: bool = False

    def t(self, chave: str, **valores: object) -> str:
        return TEXTOS[self.config["idioma"]][chave].format(**valores)

    def perguntar(self, chave: str, **valores: object) -> str:
        # O PowerShell 5.1 antepõe um BOM à primeira linha enviada por pipe a um programa nativo.
        return self.entrada(self.t(chave, **valores)).lstrip("﻿").strip()


def pintar(texto: str, codigo: str, ativo: bool) -> str:
    return f"\033[{codigo}m{texto}\033[0m" if ativo else texto


def iniciar(ctx: Contexto) -> None:
    # shortcut: ainda não há aplicativo para abrir; ligar ao `npm run tauri dev` na Fase 1.
    ctx.saida(ctx.t("sem_app"))


def instalar(ctx: Contexto) -> None:
    ausentes = ferramentas_ausentes()
    if ausentes:
        ctx.saida(ctx.t("faltam", lista=", ".join(ausentes), url=URL_PRE_REQUISITOS))
    else:
        ctx.saida(ctx.t("tudo_ok"))


def configurar(ctx: Contexto) -> None:
    valor = ctx.perguntar("idioma_prompt", opcoes=", ".join(IDIOMAS))
    if valor not in IDIOMAS:
        ctx.saida(ctx.t("idioma_invalido", valor=valor))
        return
    ctx.config["idioma"] = valor
    try:
        salvar_config(ctx.pasta, ctx.config)
    except OSError as erro:
        ctx.saida(ctx.t("erro_salvar", pasta=ctx.pasta, erro=erro))
        return
    ctx.saida(ctx.t("idioma_salvo", nome=IDIOMAS[valor]))


def status(ctx: Contexto) -> None:
    ausentes = set(ferramentas_ausentes())
    ctx.saida(f"{ctx.t('st_sistema')}: {platform.platform()}")
    ctx.saida(f"{ctx.t('st_python')}: {platform.python_version()}")
    ctx.saida(f"{ctx.t('st_ferramentas')}:")
    for nome in FERRAMENTAS.values():
        ctx.saida(f"  {nome}: {ctx.t('falta') if nome in ausentes else ctx.t('ok')}")
    app = "st_app_sim" if (RAIZ / "src-tauri").is_dir() else "st_app_nao"
    ctx.saida(f"{ctx.t('st_app')}: {ctx.t(app)}")
    ctx.saida(f"{ctx.t('st_config')}: {ctx.pasta}")
    ctx.saida(f"{ctx.t('st_idioma')}: {IDIOMAS[ctx.config['idioma']]}")


ACOES = {"1": iniciar, "2": instalar, "3": configurar, "4": status}


def executar_menu(
    entrada: Callable[[str], str] = input,
    saida: Callable[[str], None] = print,
    pasta: Path | None = None,
    cor: bool = False,
) -> int:
    """Laço do menu. Devolve o código de saída; fim da entrada (Ctrl+D/Ctrl+C) encerra sem erro."""
    pasta = pasta or pasta_config()
    ctx = Contexto(entrada, saida, pasta, carregar_config(pasta), cor)
    while True:
        saida("")
        saida(pintar("Felixo Game Launcher", "1;36", cor))
        saida(ctx.t("subtitulo"))
        for chave in (*ACOES, "0"):
            saida(f"  {pintar(chave, '1;33', cor)}) {ctx.t('op_' + chave)}")
        try:
            escolha = ctx.perguntar("escolha")
        except (EOFError, KeyboardInterrupt):
            saida("")
            return 0
        if escolha == "0":
            return 0
        acao = ACOES.get(escolha)
        if acao is None:
            saida(ctx.t("invalida", valor=escolha))
            continue
        acao(ctx)


def main() -> int:
    for fluxo in (sys.stdout, sys.stderr):
        if hasattr(fluxo, "reconfigure"):
            fluxo.reconfigure(encoding="utf-8", errors="replace")
    if os.name == "nt":
        os.system("")  # habilita sequências ANSI no console do Windows
    cor = sys.stdout.isatty() and "NO_COLOR" not in os.environ
    return executar_menu(cor=cor)


if __name__ == "__main__":
    sys.exit(main())
