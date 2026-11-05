from scripts.qualidade.verificar_links import verificar


def test_aceita_link_para_arquivo_existente(tmp_path):
    (tmp_path / "a.md").write_text("[b](b.md)", encoding="utf-8")
    (tmp_path / "b.md").write_text("ok", encoding="utf-8")
    assert verificar(tmp_path) == []


def test_aceita_ancora_em_arquivo_existente(tmp_path):
    (tmp_path / "a.md").write_text("[b](b.md#secao)", encoding="utf-8")
    (tmp_path / "b.md").write_text("ok", encoding="utf-8")
    assert verificar(tmp_path) == []


def test_aponta_link_quebrado_com_arquivo_e_linha(tmp_path):
    (tmp_path / "a.md").write_text("texto\n[x](nao-existe.md)", encoding="utf-8")
    assert verificar(tmp_path) == ["a.md:2: nao-existe.md"]


def test_resolve_o_link_a_partir_da_pasta_do_arquivo(tmp_path):
    (tmp_path / "docs").mkdir()
    (tmp_path / "docs" / "a.md").write_text("[raiz](../LEIAME.md)", encoding="utf-8")
    assert verificar(tmp_path) == ["docs/a.md:1: ../LEIAME.md"]


def test_ignora_externos_ancoras_e_codigo(tmp_path):
    texto = (
        "[site](https://exemplo.org) [ancora](#topo) [mail](mailto:a@b.c)\n"
        "```\n[x](falso.md)\n```\n"
        "`[y](falso2.md)`\n"
    )
    (tmp_path / "a.md").write_text(texto, encoding="utf-8")
    assert verificar(tmp_path) == []


def test_ignora_pastas_de_dependencias(tmp_path):
    (tmp_path / "node_modules").mkdir()
    (tmp_path / "node_modules" / "x.md").write_text("[z](nao-existe.md)", encoding="utf-8")
    assert verificar(tmp_path) == []
