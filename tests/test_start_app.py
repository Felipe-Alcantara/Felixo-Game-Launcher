from pathlib import Path

import pytest

import start_app


def entradas(*valores: str):
    """Entrada simulada: devolve os valores em ordem e, no fim, simula Ctrl+D."""
    restantes = iter(valores)

    def _entrada(_prompt: str = "") -> str:
        try:
            return next(restantes)
        except StopIteration:
            raise EOFError from None

    return _entrada


def rodar(*valores: str, pasta: Path):
    linhas: list[str] = []
    codigo = start_app.executar_menu(entradas(*valores), linhas.append, pasta)
    return codigo, "\n".join(linhas)


def test_idiomas_tem_as_mesmas_chaves():
    assert set(start_app.TEXTOS["pt-BR"]) == set(start_app.TEXTOS["en"])


def test_config_padrao_quando_o_arquivo_nao_existe(tmp_path):
    assert start_app.carregar_config(tmp_path) == {"idioma": "pt-BR"}


def test_config_ignora_arquivo_corrompido(tmp_path):
    (tmp_path / "config.json").write_text("{isto nao e json", encoding="utf-8")
    assert start_app.carregar_config(tmp_path) == {"idioma": "pt-BR"}


def test_config_ignora_idioma_desconhecido(tmp_path):
    (tmp_path / "config.json").write_text('{"idioma": "klingon"}', encoding="utf-8")
    assert start_app.carregar_config(tmp_path) == {"idioma": "pt-BR"}


def test_salvar_e_carregar_config_cria_a_pasta(tmp_path):
    pasta = tmp_path / "ainda" / "nao" / "existe"
    start_app.salvar_config(pasta, {"idioma": "en"})
    assert start_app.carregar_config(pasta) == {"idioma": "en"}
    assert not (pasta / "config.json.tmp").exists()


def test_pasta_config_no_windows(monkeypatch, tmp_path):
    monkeypatch.setattr(start_app.sys, "platform", "win32")
    monkeypatch.setenv("APPDATA", str(tmp_path))
    assert start_app.pasta_config() == tmp_path / start_app.NOME_APP


def test_pasta_config_no_macos(monkeypatch, tmp_path):
    monkeypatch.setattr(start_app.sys, "platform", "darwin")
    monkeypatch.setattr(Path, "home", lambda: tmp_path)
    esperado = tmp_path / "Library" / "Application Support" / start_app.NOME_APP
    assert start_app.pasta_config() == esperado


def test_pasta_config_no_linux(monkeypatch, tmp_path):
    monkeypatch.setattr(start_app.sys, "platform", "linux")
    monkeypatch.setenv("XDG_CONFIG_HOME", str(tmp_path))
    assert start_app.pasta_config() == tmp_path / start_app.NOME_APP


def test_ferramentas_ausentes_usa_o_path(monkeypatch):
    monkeypatch.setattr(start_app.shutil, "which", lambda exe: None if exe == "rustc" else exe)
    assert start_app.ferramentas_ausentes() == ["Rust"]


def test_menu_sai_com_zero(tmp_path):
    codigo, texto = rodar("0", pasta=tmp_path)
    assert codigo == 0
    assert "Felixo Game Launcher" in texto


def test_menu_encerra_sem_erro_no_fim_da_entrada(tmp_path):
    codigo, _ = rodar(pasta=tmp_path)
    assert codigo == 0


def test_menu_opcao_invalida_avisa_e_continua(tmp_path):
    codigo, texto = rodar("x", "0", pasta=tmp_path)
    assert codigo == 0
    assert "Opção inválida: 'x'" in texto


def test_menu_ignora_o_bom_do_pipe_do_powershell(tmp_path):
    _, texto = rodar("﻿4", "0", pasta=tmp_path)
    assert "Opção inválida" not in texto
    assert "Sistema:" in texto


def test_iniciar_avisa_que_nao_ha_aplicativo(tmp_path):
    _, texto = rodar("1", "0", pasta=tmp_path)
    assert "Fase 1" in texto


def test_instalar_lista_o_que_falta(monkeypatch, tmp_path):
    monkeypatch.setattr(start_app.shutil, "which", lambda exe: None)
    _, texto = rodar("2", "0", pasta=tmp_path)
    assert "Faltam ferramentas" in texto
    assert "Rust" in texto


def test_instalar_confirma_quando_tudo_existe(monkeypatch, tmp_path):
    monkeypatch.setattr(start_app.shutil, "which", lambda exe: exe)
    _, texto = rodar("2", "0", pasta=tmp_path)
    assert "estão instaladas" in texto


def test_configurar_troca_o_idioma_e_persiste(tmp_path):
    _, texto = rodar("3", "en", "0", pasta=tmp_path)
    assert start_app.carregar_config(tmp_path) == {"idioma": "en"}
    assert "Quit" in texto  # o menu seguinte já aparece em inglês


def test_configurar_rejeita_idioma_invalido(tmp_path):
    _, texto = rodar("3", "xx", "0", pasta=tmp_path)
    assert "Nada foi alterado" in texto
    assert start_app.carregar_config(tmp_path) == {"idioma": "pt-BR"}


def test_configurar_avisa_quando_nao_consegue_salvar(tmp_path):
    ocupada = tmp_path / "arquivo"
    ocupada.write_text("x", encoding="utf-8")  # um arquivo no lugar da pasta de configuração
    _, texto = rodar("3", "en", "0", pasta=ocupada)
    assert "Could not save" in texto


def test_status_mostra_sistema_idioma_e_ferramentas(monkeypatch, tmp_path):
    monkeypatch.setattr(start_app.shutil, "which", lambda exe: None if exe == "git" else exe)
    _, texto = rodar("4", "0", pasta=tmp_path)
    assert "Sistema:" in texto
    assert "Git: falta" in texto
    assert "Rust: ok" in texto
    assert "Português (Brasil)" in texto


@pytest.mark.parametrize("idioma", ["pt-BR", "en"])
def test_todo_texto_do_menu_formata_sem_erro(idioma, tmp_path):
    ctx = start_app.Contexto(entradas(), lambda _: None, tmp_path, {"idioma": idioma})
    for chave in ("subtitulo", "op_1", "op_2", "op_3", "op_4", "op_0", "escolha", "tudo_ok"):
        assert ctx.t(chave)
