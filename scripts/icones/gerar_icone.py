"""Gera o ícone-fonte do aplicativo: um quadrado arredondado em degradê com um "play" branco.

Só biblioteca padrão. Uso::

    python scripts/icones/gerar_icone.py              # grava assets/icone-fonte.png (1024x1024)
    npm run tauri -- icon assets/icone-fonte.png -o src-tauri/icons   # gera os demais formatos
"""

from __future__ import annotations

import math
import struct
import sys
import zlib
from pathlib import Path

COR_INICIO = (49, 80, 201)
COR_FIM = (124, 156, 255)
RAIO_DO_CANTO = 0.22
# Vértices do triângulo "play", em coordenadas de 0 a 1.
TRIANGULO = ((0.38, 0.28), (0.38, 0.72), (0.74, 0.50))


def _limitar(valor: float) -> float:
    return max(0.0, min(1.0, valor))


def _distancia_ao_quadrado_arredondado(u: float, v: float) -> float:
    """Distância com sinal (negativa dentro) até o contorno do quadrado arredondado."""
    dx = max(abs(u - 0.5) - (0.5 - RAIO_DO_CANTO), 0.0)
    dy = max(abs(v - 0.5) - (0.5 - RAIO_DO_CANTO), 0.0)
    return math.hypot(dx, dy) - RAIO_DO_CANTO


def _distancia_ao_triangulo(u: float, v: float) -> float:
    """Distância com sinal (positiva dentro) até a borda mais próxima do triângulo."""
    (ax, ay), (bx, by), (cx, cy) = TRIANGULO
    orientacao = 1.0 if (bx - ax) * (cy - ay) - (by - ay) * (cx - ax) > 0 else -1.0
    menor = math.inf
    for (x1, y1), (x2, y2) in (((ax, ay), (bx, by)), ((bx, by), (cx, cy)), ((cx, cy), (ax, ay))):
        comprimento = math.hypot(x2 - x1, y2 - y1)
        lado = ((x2 - x1) * (v - y1) - (y2 - y1) * (u - x1)) / comprimento
        menor = min(menor, orientacao * lado)
    return menor


def _pixel(coluna: int, linha: int, tamanho: int) -> bytes:
    u, v = (coluna + 0.5) / tamanho, (linha + 0.5) / tamanho
    # A distância vira pixels para a borda ter um pixel de suavização.
    opacidade = _limitar(0.5 - _distancia_ao_quadrado_arredondado(u, v) * tamanho)
    mistura = (u + v) / 2
    fundo = [COR_INICIO[i] + (COR_FIM[i] - COR_INICIO[i]) * mistura for i in range(3)]
    play = _limitar(0.5 + _distancia_ao_triangulo(u, v) * tamanho)
    rgb = [round(fundo[i] + (255 - fundo[i]) * play) for i in range(3)]
    return bytes((*rgb, round(opacidade * 255)))


def _bloco(tipo: bytes, dados: bytes) -> bytes:
    corpo = tipo + dados
    return struct.pack(">I", len(dados)) + corpo + struct.pack(">I", zlib.crc32(corpo))


def gerar_png(tamanho: int = 1024) -> bytes:
    """PNG RGBA quadrado com o ícone."""
    linhas = bytearray()
    for linha in range(tamanho):
        linhas.append(0)  # filtro "nenhum"
        for coluna in range(tamanho):
            linhas += _pixel(coluna, linha, tamanho)
    cabecalho = struct.pack(">IIBBBBB", tamanho, tamanho, 8, 6, 0, 0, 0)
    return (
        b"\x89PNG\r\n\x1a\n"
        + _bloco(b"IHDR", cabecalho)
        + _bloco(b"IDAT", zlib.compress(bytes(linhas), 9))
        + _bloco(b"IEND", b"")
    )


def main() -> int:
    raiz = Path(__file__).resolve().parents[2]
    destino = Path(sys.argv[1]) if len(sys.argv) > 1 else raiz / "assets" / "icone-fonte.png"
    destino.parent.mkdir(parents=True, exist_ok=True)
    destino.write_bytes(gerar_png())
    print(f"Ícone gravado em {destino}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
