"""Confere se todo link relativo dos arquivos Markdown aponta para um arquivo que existe.

Uso: ``python -m scripts.qualidade.verificar_links``. Sai com código 1 se houver link quebrado.
Links externos (http, https, mailto) e âncoras puras (#secao) são ignorados; código em bloco ou
em linha também, para que exemplos de sintaxe não contem como link de verdade.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path
from urllib.parse import unquote

PASTAS_IGNORADAS = {".git", ".venv", "node_modules", "target", "dist"}
LINK = re.compile(r"\[[^\]]*\]\(([^)\s]+)(?:\s+\"[^\"]*\")?\)")
BLOCO_DE_CODIGO = re.compile(r"```.*?```", re.DOTALL)
CODIGO_EM_LINHA = re.compile(r"`[^`\n]*`")
EXTERNOS = ("http://", "https://", "mailto:", "#")


def _sem_codigo(texto: str) -> str:
    """Remove o código, mas mantém as quebras de linha para os números de linha baterem."""
    texto = BLOCO_DE_CODIGO.sub(lambda m: "\n" * m.group(0).count("\n"), texto)
    return CODIGO_EM_LINHA.sub("", texto)


def verificar(raiz: Path) -> list[str]:
    """Devolve uma mensagem `arquivo:linha: alvo` para cada link relativo quebrado."""
    erros: list[str] = []
    for arquivo in sorted(raiz.rglob("*.md")):
        relativo = arquivo.relative_to(raiz)
        if PASTAS_IGNORADAS & set(relativo.parts):
            continue
        texto = _sem_codigo(arquivo.read_text(encoding="utf-8"))
        for numero, linha in enumerate(texto.splitlines(), start=1):
            for alvo in LINK.findall(linha):
                if alvo.startswith(EXTERNOS):
                    continue
                caminho = unquote(alvo.split("#", 1)[0].split("?", 1)[0])
                if caminho and not (arquivo.parent / caminho).exists():
                    erros.append(f"{relativo.as_posix()}:{numero}: {alvo}")
    return erros


def main() -> int:
    erros = verificar(Path(__file__).resolve().parents[2])
    for erro in erros:
        print(f"link quebrado: {erro}")
    print(f"{len(erros)} link(s) quebrado(s).")
    return 1 if erros else 0


if __name__ == "__main__":
    sys.exit(main())
