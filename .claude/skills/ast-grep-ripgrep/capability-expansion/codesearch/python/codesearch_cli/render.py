"""Rendering catalog rows, safely.

Probe P01 measured what happens when catalog content meets Rich's markup parser: of 24 bracketed
forms this tool actually prints, **11 were silently deleted**. Among them ``[a-z]``, ``[a-zA-Z_]``
and every one of the six ast-grep rule field names -- ``[pattern] [kind] [inside] [has] [stopBy]
[regex]``. No error, no warning, the text simply vanishes.

The bug is invisible to spot-checking because one character flips it: ``[A-Z]`` and ``[0-9]``
survive while ``[a-z]`` does not, and ``[^a-z]`` survives because the caret saves it.

So there are exactly two safe ways to put catalog text on a terminal, and this module is the only
place either is used:

1. :func:`safe` -- ``rich.markup.escape``, which P01 measured as rescuing all 11 altered forms.
2. ``markup=False``, which preserved all 24.

Probe PB08 adds the one renderable that needs neither: ``Syntax`` does **not** markup-parse its
content (0 of 8 lossy forms altered, with both controls clean), so patterns go through
:func:`pattern` instead. There is no regex lexer among pygments' 602 -- ``Syntax(code, "regex")``
resolves to ``None`` *identically to a nonsense lexer name* -- so the lexer is named ``text``
explicitly rather than asking for one that silently is not there.
"""

from __future__ import annotations

from typing import Any

from rich.console import Console
from rich.markup import escape
from rich.syntax import Syntax
from rich.table import Table

# Columns whose values are catalog content rather than our own labels. Everything here is
# untrusted for markup purposes, because all of it can contain brackets.
_CONTENT_COLUMNS = frozenset(
    {
        "entity_key",
        "natural_key",
        "invocation_form",
        "summary",
        "predicate",
        "command",
        "control_command",
        "value_or_target",
        "syntax",
    }
)


def safe(value: object) -> str:
    """Escape a value so Rich renders it literally.

    Everything that is not an authored literal goes through here. ``escape`` is P01's measured
    remedy, not a precaution.
    """
    if value is None:
        return ""
    return escape(str(value))


def pattern(text: str) -> Syntax:
    """A regex or rule fragment, rendered without markup parsing.

    ``Syntax`` is markup-safe by construction (PB08), which makes it the right carrier for exactly
    the content most at risk. It will not highlight -- pygments has no regex lexer -- so this asks
    for ``text`` rather than for a lexer that would silently resolve to ``None``.
    """
    return Syntax(text, "text", theme="ansi_light", background_color="default", word_wrap=True)


def rows_table(rows: list[dict[str, Any]], title: str | None = None) -> Table:
    """Render catalog rows as a table, escaping every cell.

    Escaping is applied to *every* cell rather than only to the columns known to be risky: a new
    column added upstream would otherwise arrive unescaped, and the failure would be silent.

    Cells are looked up **by name**, and the columns are the union of every row's keys rather than
    the first row's. Arrow's JSON writer omits a null field entirely instead of emitting ``null``,
    so a row with a null in the middle is shorter than its neighbours -- and a positional render
    then shifts every later value one column left, which reads as data rather than as an error.
    ``compare`` is where this first showed: an added entity has no ``before_id``, and its
    ``after_id`` was rendered under ``before_id``. Taking the union likewise keeps a column that
    only later rows populate, which a first-row header would have dropped in silence.
    """
    table = Table(title=safe(title) if title else None, header_style="bold", expand=False)
    if not rows:
        return table
    columns: list[str] = []
    for row in rows:
        for column in row:
            if column not in columns:
                columns.append(column)
    for column in columns:
        table.add_column(safe(column), overflow="fold")
    for row in rows:
        table.add_row(*(safe(row.get(column)) for column in columns))
    return table


def console(*, force_terminal: bool | None = None, width: int | None = None) -> Console:
    """The one Console constructor.

    Width and colour are parameters so capture tests can pin them; P01 showed that an unpinned
    construction makes rendered output non-reproducible, which is how its own first draft came out
    inconclusive.
    """
    return Console(force_terminal=force_terminal, width=width, soft_wrap=False)
