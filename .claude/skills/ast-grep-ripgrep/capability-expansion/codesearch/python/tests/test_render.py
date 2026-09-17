"""Output-fidelity tests for the rendering layer.

These exist because probe P01 measured that the failure is **silent**: Rich's markup parser
deletes ``[a-z]`` and every ast-grep rule field name with no error. A rendering layer that loses
catalog content produces confidently wrong documentation, so the safety is asserted here rather
than left to review.

Console width and colour are pinned in every test. P01 recorded that its own first draft came out
inconclusive because the environment was not isolated, so these fix the construction.
"""

from __future__ import annotations

import io

import pytest
from rich.console import Console

from codesearch_cli import render

# The exact forms P01 measured as deleted. Not a sample -- the whole altered set that this tool
# actually prints.
LOSSY_UNDER_MARKUP = [
    "[a-z]",
    "[a-zA-Z_]",
    "[x]",
    "[pattern]",
    "[kind]",
    "[inside]",
    "[has]",
    "[stopBy]",
    "[regex]",
    "[path]",
    "[glob]",
]

# Forms P01 measured as surviving. Included so the tests cannot pass merely by escaping
# everything into unrecognisable mush.
SURVIVES_ANYWAY = ["[A-Z]", "[0-9]", "[^a-z]", "[[:alpha:]]", "[]]"]


def capture(renderable: object, *, width: int = 200) -> str:
    buffer = io.StringIO()
    Console(
        file=buffer,
        width=width,
        force_terminal=False,
        color_system=None,
        legacy_windows=False,
        _environ={},
    ).print(renderable)
    return buffer.getvalue()


@pytest.mark.parametrize("subject", LOSSY_UNDER_MARKUP)
def test_escaped_content_survives_rendering(subject: str) -> None:
    assert subject in capture(render.safe(subject))


@pytest.mark.parametrize("subject", LOSSY_UNDER_MARKUP)
def test_the_control_confirms_these_forms_really_are_lossy(subject: str) -> None:
    """Without this, the test above could pass for the wrong reason.

    If Rich stopped deleting bracketed text, the escaping would be untested rather than
    unnecessary -- and the difference matters, because the protection would then be silently
    load-bearing nowhere.
    """
    buffer = io.StringIO()
    Console(
        file=buffer,
        width=200,
        force_terminal=False,
        color_system=None,
        legacy_windows=False,
        _environ={},
    ).print(subject)
    assert subject not in buffer.getvalue(), (
        "Rich no longer deletes this form; P01's finding has changed and the rendering rule "
        "should be re-derived rather than assumed"
    )


@pytest.mark.parametrize("subject", SURVIVES_ANYWAY)
def test_forms_that_survive_are_not_mangled_by_escaping(subject: str) -> None:
    assert subject in capture(render.safe(subject))


@pytest.mark.parametrize("subject", [*LOSSY_UNDER_MARKUP, *SURVIVES_ANYWAY])
def test_syntax_is_markup_safe_without_escaping(subject: str) -> None:
    """PB08: ``Syntax`` does not markup-parse, so patterns need no escaping."""
    assert subject in capture(render.pattern(subject))


def test_every_table_cell_is_escaped_including_unknown_columns() -> None:
    """A column added upstream must not arrive unescaped."""
    rows = [{"entity_key": "mech:sg/rule/rule.has", "a_new_column": "[a-z]+ and [stopBy]"}]
    out = capture(render.rows_table(rows, title="t"), width=300)
    assert "[a-z]+" in out
    assert "[stopBy]" in out


def test_a_pattern_renders_unhighlighted_rather_than_asking_for_a_missing_lexer() -> None:
    """PB08: there is no regex lexer, and asking for one resolves to ``None`` silently.

    Naming ``text`` keeps the absence explicit instead of depending on a fallback.
    """
    syntax = render.pattern(r"\b[a-z][a-zA-Z0-9_]*\b")
    assert syntax.lexer is not None, "the named lexer must actually resolve"


def test_a_row_missing_a_key_does_not_shift_later_columns() -> None:
    """Arrow's JSON writer omits a null field rather than emitting ``null``.

    So a row with a null in the middle arrives shorter than its neighbours, and a positional
    render moves every later value one column left -- which reads as data, not as an error. This is
    how `compare` first rendered an added entity's `after_id` under `before_id`.
    """
    rows = [
        {"key": "a", "before": "1", "after": "2"},
        {"key": "b", "after": "3"},
    ]
    table = render.rows_table(rows)
    rendered = capture(table)
    assert [c.header for c in table.columns] == ["key", "before", "after"]
    # `3` is the `after` value and must not appear under `before`.
    assert list(table.columns[1].cells) == ["1", ""]
    assert list(table.columns[2].cells) == ["2", "3"]
    assert "3" in rendered


def test_a_column_only_later_rows_carry_is_still_shown() -> None:
    """The columns are the union of every row's keys, not the first row's.

    A header taken from the first row would drop the column in silence, which is the same class of
    failure: an answer that looks complete and is not.
    """
    rows = [{"key": "a"}, {"key": "b", "note": "only here"}]
    table = render.rows_table(rows)
    assert [c.header for c in table.columns] == ["key", "note"]
    assert list(table.columns[1].cells) == ["", "only here"]
