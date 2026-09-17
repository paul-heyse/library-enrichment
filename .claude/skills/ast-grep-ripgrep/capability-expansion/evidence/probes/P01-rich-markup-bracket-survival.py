"""Round-1 probe: which bracketed forms THIS tool prints survive Rich markup parsing?

Extends probe M002 from the typer-rich skill, which tested eight forms and covered none of the
regex syntax a code-search capability catalog actually renders.

Construction is pinned the way the skill pins its own probes: explicit width, color_system,
force_terminal and legacy_windows, under an environment that inherits nothing. Change any of
those and the bytes change.
"""

import io
import sys

from rich.console import Console
from rich.markup import escape

# The forms this tool will print, drawn from the ast-grep/ripgrep/PCRE2 catalog surface.
SUBJECTS = [
    # character classes
    "[x]",
    "[a-z]",
    "[A-Z]",
    "[0-9]",
    "[^a-z]",
    "[a-zA-Z_]",
    # POSIX and PCRE2 class syntax
    "[[:alpha:]]",
    "[[:^digit:]]",
    "[\\d-[1]]",
    # ast-grep / rule surface
    "[pattern]",
    "[kind]",
    "[inside]",
    "[has]",
    "[stopBy]",
    "[regex]",
    # option and placeholder forms
    "[path]",
    "[PATH]",
    "[--flag]",
    "[-n]",
    "[glob]",
    "[FILE]",
    # bracket-adjacent regex
    "[]]",
    "[^]]",
    "x[0]y",
]


def render(text: str, *, markup: bool = True) -> str:
    buffer = io.StringIO()
    console = Console(
        file=buffer,
        width=80,
        force_terminal=False,
        color_system=None,
        legacy_windows=False,
        _environ={},
    )
    console.print("v " + text + " w", markup=markup)
    return buffer.getvalue().strip()


def main() -> int:
    out = sys.stdout
    eaten, kept, errored = [], [], []

    out.write(f"rich version under test: {__import__('rich').__file__}\n\n")
    out.write(f"{'subject':<16} {'markup=True result':<28} verdict\n")
    out.write("-" * 66 + "\n")
    for subject in SUBJECTS:
        expected = "v " + subject + " w"
        try:
            got = render(subject)
        except Exception as exc:  # a markup error is itself a finding
            errored.append((subject, f"{type(exc).__name__}: {exc}"))
            out.write(f"{subject:<16} {'<raised>':<28} ERROR {type(exc).__name__}\n")
            continue
        if got == expected:
            kept.append(subject)
            verdict = "survives"
        else:
            eaten.append((subject, got))
            verdict = "ALTERED"
        out.write(f"{subject:<16} {got!r:<28} {verdict}\n")

    out.write("\n--- control: does escape() rescue every altered form? ---\n")
    rescued, unrescued = [], []
    for subject, _ in eaten:
        got = render(escape(subject))
        (rescued if got == "v " + subject + " w" else unrescued).append(subject)
    out.write(f"escape() rescued : {rescued}\n")
    out.write(f"escape() FAILED  : {unrescued}\n")

    out.write("\n--- control: does markup=False preserve every form? ---\n")
    literal_failures = [s for s in SUBJECTS if render(s, markup=False) != "v " + s + " w"]
    out.write(f"markup=False failures: {literal_failures}\n")

    out.write(f"\nsummary: {len(kept)} survive, {len(eaten)} altered, {len(errored)} raised\n")
    for subject, message in errored:
        out.write(f"  raised {subject}: {message}\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
