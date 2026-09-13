#!/usr/bin/env python3
"""Deferred-decision register checker (standard library only, Python >= 3.11).

    python3 scripts/check_register.py --lint   validate every row
    python3 scripts/check_register.py --due    report the rows that are due

`docs/adr/register.md` carries one row per item that was deliberately deferred
with a stated trigger. `--lint` runs inside `just adr-lint`; `--due` is
`just register-check`, run at a phase gate.

A `check` cell that starts with `$ ` is a shell command. `--due` runs it and
reports what it said. A command that fails is reported, never fatal: the point
of the pass is to surface what changed, not to fail a gate on a network blip.

Adapted from pse-arrow's scripts/check_register.py. This repository has no
remote and no monthly cron, so `--due` is invoked from a phase gate rather than
from a workflow that opens an issue.
"""

from __future__ import annotations

import argparse
import datetime
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ADR_DIR = ROOT / "docs" / "adr"
REGISTER = ADR_DIR / "register.md"

ROW_RE = re.compile(r"^\|\s*(R-\d{2})\s*\|")
CELL_SPLIT_RE = re.compile(r"(?<!\\)\|")
DATE_RE = re.compile(r"^\d{4}-\d{2}-\d{2}$")
ADR_REF_RE = re.compile(r"ADR-(\d{4})")
STATUSES = {"open", "watch", "closed"}
COLUMNS = (
    "id",
    "item",
    "adr",
    "trigger",
    "check",
    "owner",
    "last-checked",
    "next-check",
    "status",
)


def out(message: str = "") -> None:
    """stdout, without tripping the linter's ban on stray `print`."""
    sys.stdout.write(f"{message}\n")


def err(message: str) -> None:
    sys.stderr.write(f"{message}\n")


def rows() -> list[dict[str, str]]:
    parsed: list[dict[str, str]] = []
    lines = REGISTER.read_text(encoding="utf-8").splitlines()
    for lineno, line in enumerate(lines, 1):
        if not ROW_RE.match(line):
            continue
        # Split on unescaped pipes only: a cell may contain a markdown-escaped
        # `\|`, which is how a shell pipeline is written inside a check cell.
        body = line.strip().strip("|")
        cells = [c.strip().replace("\\|", "|") for c in CELL_SPLIT_RE.split(body)]
        if len(cells) != len(COLUMNS):
            parsed.append(
                {
                    "id": cells[0],
                    "_lineno": str(lineno),
                    "_error": f"{len(cells)} cells, expected {len(COLUMNS)}",
                }
            )
            continue
        row = dict(zip(COLUMNS, cells, strict=True))
        row["_lineno"] = str(lineno)
        parsed.append(row)
    return parsed


def known_adrs() -> set[str]:
    return {f"ADR-{p.name[:4]}" for p in ADR_DIR.glob("[0-9][0-9][0-9][0-9]-*.md")}


def lint() -> int:
    today = datetime.date.today()
    errors: list[str] = []
    seen: set[str] = set()
    adrs = known_adrs()
    parsed = rows()
    if not parsed:
        err("error: docs/adr/register.md has no R-NN rows")
        return 1

    for row in parsed:
        where = f"register.md:{row['_lineno']} {row['id']}"
        if "_error" in row:
            errors.append(f"{where}: {row['_error']}")
            continue
        if row["id"] in seen:
            errors.append(f"{where}: duplicate row id")
        seen.add(row["id"])
        if row["status"] not in STATUSES:
            errors.append(f"{where}: status {row['status']!r} not in {sorted(STATUSES)}")
        for key in ("last-checked", "next-check"):
            if not DATE_RE.match(row[key]):
                errors.append(f"{where}: {key} {row[key]!r} must be YYYY-MM-DD")
        if not row["owner"]:
            errors.append(f"{where}: owner is empty")
        if not row["trigger"]:
            errors.append(f"{where}: trigger is empty")
        if not row["check"]:
            errors.append(f"{where}: check is empty; a trigger with no check is prose")
        for ref in ADR_REF_RE.findall(row["adr"]):
            if f"ADR-{ref}" not in adrs:
                errors.append(f"{where}: references unknown ADR-{ref}")
        if (
            DATE_RE.match(row["next-check"])
            and row["status"] != "closed"
            and datetime.date.fromisoformat(row["next-check"]) < today
        ):
            errors.append(
                f"{where}: next-check {row['next-check']} is in the past and the row is not closed"
            )
        if (
            DATE_RE.match(row["last-checked"])
            and DATE_RE.match(row["next-check"])
            and datetime.date.fromisoformat(row["next-check"])
            < datetime.date.fromisoformat(row["last-checked"])
        ):
            errors.append(f"{where}: next-check precedes last-checked")

    for problem in errors:
        err(f"error: {problem}")
    if errors:
        err(f"register lint: {len(errors)} problem(s) in {len(parsed)} row(s)")
        return 1
    out(f"register lint: {len(parsed)} row(s) OK")
    return 0


def run_check(command: str) -> str:
    try:
        proc = subprocess.run(
            command,
            shell=True,
            cwd=ROOT,
            capture_output=True,
            text=True,
            timeout=300,
            check=False,
        )
    except (OSError, subprocess.SubprocessError) as exc:
        return f"    check could not be run: {exc}"
    text = (proc.stdout + proc.stderr).strip() or "(no output)"
    lines = ["    " + line for line in text.splitlines()[:20]]
    if proc.returncode != 0:
        lines.append(f"    (command exited {proc.returncode}; reported, not fatal)")
    return "\n".join(lines)


def due() -> int:
    today = datetime.date.today()
    pending = [
        row
        for row in rows()
        if "_error" not in row
        and DATE_RE.match(row["next-check"])
        and datetime.date.fromisoformat(row["next-check"]) <= today
        and row["status"] != "closed"
    ]
    out(f"# Register review {today:%Y-%m}")
    out()
    if not pending:
        out(f"No register rows are due on {today.isoformat()}.")
        return 0
    out(f"{len(pending)} row(s) due on {today.isoformat()}:")
    out()
    for row in pending:
        out(f"## {row['id']} — {row['item']}")
        out(f"- Decision record: {row['adr'] or '—'}")
        out(f"- Trigger: {row['trigger']}")
        out(
            f"- Owner: {row['owner']}  ·  last checked {row['last-checked']}"
            f"  ·  due {row['next-check']}"
        )
        command = row["check"]
        if command.startswith("$ "):
            shell = command[2:].strip().strip("`")
            out(f"- Check: `{shell}`")
            out(run_check(shell))
        else:
            out(f"- Check (manual): {command}")
        out()
    out(
        "Closing a row means writing the ADR its trigger called for, or moving "
        "next-check forward with a reason — not deleting the row."
    )
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        prog="check_register.py",
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("--lint", action="store_true", help="validate every row")
    parser.add_argument("--due", action="store_true", help="report and run the rows that are due")
    args = parser.parse_args(argv)
    if not (args.lint or args.due):
        parser.error("choose --lint or --due")
    status = 0
    if args.lint:
        status |= lint()
    if args.due:
        status |= due()
    return status


if __name__ == "__main__":
    sys.exit(main())
