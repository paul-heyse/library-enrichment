"""Explicit stdout/stderr for operator command-line scripts.

`T20` (flake8-print) is on for the whole repository and deliberately so: stdout is the MCP
protocol channel, and a stray `print()` in the adapter corrupts the session. An operator script
is the one place where writing to stdout *is* the job, so it says so here, once, instead of
scattering per-line suppressions -- which this repository does not use at all.

Routing every line through these two functions also means a script's output is one grep away,
and that a future `--json` mode has a single place to change.
"""

from __future__ import annotations

import sys


def say(message: str = "") -> None:
    """Write one line of operator-facing output to stdout."""
    sys.stdout.write(f"{message}\n")


def warn(message: str) -> None:
    """Write one line of diagnostic output to stderr, where it cannot be mistaken for a result."""
    sys.stderr.write(f"{message}\n")
