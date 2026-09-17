"""PB08: is `rich.syntax.Syntax` a safe renderable for pattern display?

Two questions, because they fail in opposite directions and the plan (§8.3) has to assign a
renderable to every kind of catalog content:

1. Does a `regex` lexer exist, so a pattern can be displayed as a pattern rather than as prose?
2. Does `Syntax` markup-parse its content? P01 established that `Console.print` DELETES
   `[a-z]`, `[pattern]` and every ast-grep rule field name. If `Syntax` inherits that, it is the
   single most dangerous renderable in the tool, because every one of its arguments is exactly
   the sort of string P01 showed disappearing.

Construction is pinned exactly as P01 pins it -- width, color_system, force_terminal,
legacy_windows and an environment that inherits nothing. Change any and the bytes change.

CONTROLS
  - `Console.print(raw)` with markup on, which P01 measured as lossy. If the Syntax arm and the
    raw arm both lose content, the cause is the Console, not Syntax.
  - `Console.print(raw, markup=False)`, which P01 measured as lossless. If THIS arm loses
    content the probe's own harness is broken and no verdict may be read from it.
  - a lexer name that certainly does not exist, so "found" means found rather than "silently
    fell back to something".
"""

import io
import sys
from importlib.metadata import version

from rich.console import Console
from rich.syntax import Syntax

# The forms P01 measured as deleted by markup parsing -- the exact strings this tool prints.
LOSSY_UNDER_MARKUP = [
    "[a-z]",
    "[a-zA-Z_]",
    "[pattern]",
    "[kind]",
    "[inside]",
    "[has]",
    "[stopBy]",
    "[regex]",
]

# Realistic catalog content: what `codesearch describe` would show for a mechanism.
PATTERNS = [
    r"\b[a-z][a-zA-Z0-9_]*\b",
    r"(?P<name>[[:alpha:]]+)",
    r"^\s*fn\s+([a-z_][a-z0-9_]*)",
]

LEXER_CANDIDATES = ["regex", "yaml", "rust", "text", "definitely-not-a-lexer-9f3a"]


def console(buffer: io.StringIO) -> Console:
    return Console(
        file=buffer,
        width=80,
        force_terminal=False,
        color_system=None,
        legacy_windows=False,
        _environ={},
    )


def render_syntax(code: str, lexer: str) -> str:
    buffer = io.StringIO()
    console(buffer).print(
        Syntax(code, lexer, theme="ansi_light", background_color="default", word_wrap=False)
    )
    return buffer.getvalue()


def render_print(text: str, *, markup: bool) -> str:
    buffer = io.StringIO()
    console(buffer).print(text, markup=markup)
    return buffer.getvalue()


def main() -> int:
    out = sys.stdout.write
    out("PB08 -- does `Syntax` have a regex lexer, and does it markup-parse its content?\n")
    out(f"  rich version: {version('rich')}\n\n")

    # ---- Q1: does a regex lexer exist? --------------------------------------------------
    out("== Q1 - lexer resolution ==\n\n")
    for name in LEXER_CANDIDATES:
        syntax = Syntax("x", name)
        lexer = syntax.lexer
        resolved = type(lexer).__name__ if lexer is not None else None
        aliases = getattr(lexer, "aliases", None) if lexer is not None else None
        out(f"  {name:<28} -> {resolved!s:<24} aliases={aliases}\n")
    out("\n")

    # ---- Q2: does Syntax delete what Console.print deletes? -----------------------------
    out("== Q2 - does Syntax markup-parse its content? ==\n\n")
    out("  Each subject is a string P01 measured as DELETED by `Console.print` with markup on.\n\n")
    syntax_lost = 0
    control_markup_lost = 0
    control_nomarkup_lost = 0
    for subject in LOSSY_UNDER_MARKUP:
        via_syntax = render_syntax(subject, "text")
        via_markup = render_print(subject, markup=True)
        via_plain = render_print(subject, markup=False)

        in_syntax = subject in via_syntax
        in_markup = subject in via_markup
        in_plain = subject in via_plain
        syntax_lost += not in_syntax
        control_markup_lost += not in_markup
        control_nomarkup_lost += not in_plain

        out(
            f"  {subject:<14} Syntax={'kept' if in_syntax else 'LOST'}   "
            f"print(markup)={'kept' if in_markup else 'LOST'}   "
            f"print(no markup)={'kept' if in_plain else 'LOST'}\n"
        )
    total = len(LOSSY_UNDER_MARKUP)
    out(f"\n  Syntax lost {syntax_lost}/{total}\n")
    out(f"  CONTROL print(markup=True)  lost {control_markup_lost}/{total} (P01: expect all)\n")
    out(f"  CONTROL print(markup=False) lost {control_nomarkup_lost}/{total} (expect 0)\n")
    if control_nomarkup_lost:
        out("  !! HARNESS BROKEN: the lossless control lost content. No verdict may be read.\n")
    if not control_markup_lost:
        out("  !! HARNESS SUSPECT: the lossy control lost nothing; P01 is not reproducing.\n")
    out("\n")

    # ---- Q3: realistic patterns through the regex lexer ---------------------------------
    out("== Q3 - real patterns through the best available lexer ==\n\n")
    for pattern in PATTERNS:
        rendered = render_syntax(pattern, "regex")
        kept = pattern in rendered
        out(f"  {'kept' if kept else 'LOST'}  {pattern}\n")
        out(f"        rendered: {rendered.rstrip()!r}\n")
    out("\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
