import struct
import zlib

from scripts.icones.gerar_icone import gerar_png


def _decodificar(png: bytes) -> tuple[int, int, bytes]:
    """Devolve (largura, altura, bytes das linhas já sem compressão)."""
    assert png[:8] == b"\x89PNG\r\n\x1a\n"
    largura, altura = struct.unpack(">II", png[16:24])
    posicao, dados = 8, b""
    while posicao < len(png):
        comprimento, tipo = struct.unpack(">I4s", png[posicao : posicao + 8])
        corpo = png[posicao + 8 : posicao + 8 + comprimento]
        crc = struct.unpack(">I", png[posicao + 8 + comprimento : posicao + 12 + comprimento])[0]
        assert crc == zlib.crc32(tipo + corpo), f"CRC errado em {tipo!r}"
        if tipo == b"IDAT":
            dados += corpo
        posicao += 12 + comprimento
    return largura, altura, zlib.decompress(dados)


def _alfa(linhas: bytes, largura: int, coluna: int, linha: int) -> int:
    passo = 1 + 4 * largura
    return linhas[linha * passo + 1 + coluna * 4 + 3]


def test_png_valido_com_o_tamanho_pedido():
    largura, altura, linhas = _decodificar(gerar_png(64))
    assert (largura, altura) == (64, 64)
    assert len(linhas) == 64 * (1 + 4 * 64)


def test_cantos_transparentes_e_centro_opaco():
    largura, _, linhas = _decodificar(gerar_png(64))
    assert _alfa(linhas, largura, 0, 0) == 0
    assert _alfa(linhas, largura, 63, 63) == 0
    assert _alfa(linhas, largura, 32, 32) == 255


def test_o_play_e_branco_e_o_fundo_nao():
    largura, _, linhas = _decodificar(gerar_png(64))
    passo = 1 + 4 * largura

    def rgb(coluna: int, linha: int) -> tuple[int, int, int]:
        inicio = linha * passo + 1 + coluna * 4
        return tuple(linhas[inicio : inicio + 3])  # type: ignore[return-value]

    assert rgb(32, 32) == (255, 255, 255)  # dentro do triângulo
    assert rgb(8, 32) != (255, 255, 255)  # fundo em degradê, fora do triângulo
