#!/usr/bin/env python3
"""Validate docs/reports/acceptance.json.

This is the mechanical half of "truthful reporting". It enforces two things that prose cannot:

  1. A gate marked `passed` must carry a command that ran and a log that was written.
  2. Every gate ID in the frozen tests/ACCEPTANCE_PLAN.md appears exactly once, with no
     invented IDs and none quietly dropped.

Run by `just acceptance-check`, by CI, and by the Stop hook.
"""
from __future__ import annotations

import json, re, sys, tomllib
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
VALID = {"passed", "failed", "blocked", "not_run"}


def main() -> int:
    report_path = ROOT / "docs/reports/acceptance.json"
    if not report_path.exists():
        print(f"acceptance-check: {report_path.relative_to(ROOT)} does not exist. "
              f"Run `just acceptance-report` first.", file=sys.stderr)
        return 1

    report = json.loads(report_path.read_text())
    gates = report.get("gates", [])
    problems: list[str] = []

    # The frozen plan is the authority on which IDs exist.
    plan = (ROOT / "tests/ACCEPTANCE_PLAN.md").read_text()
    plan_ids = set(re.findall(r'^\|\s*([RPCA]\d{2})\s*\|', plan, re.M))
    registry_ids = set(tomllib.load(open(ROOT / "tests/gates.toml", "rb"))["gates"])
    report_ids = [g["gate_id"] for g in gates]

    if missing := plan_ids - registry_ids:
        problems.append(f"gates.toml is missing IDs from ACCEPTANCE_PLAN.md: {sorted(missing)}")
    if invented := registry_ids - plan_ids:
        problems.append(f"gates.toml has IDs absent from ACCEPTANCE_PLAN.md: {sorted(invented)}")
    if dupes := [i for i, n in Counter(report_ids).items() if n > 1]:
        problems.append(f"acceptance.json repeats gate IDs: {sorted(dupes)}")
    if missing := plan_ids - set(report_ids):
        problems.append(f"acceptance.json omits gates: {sorted(missing)}")

    for g in gates:
        gid = g.get("gate_id", "?")
        status = g.get("status")
        if status not in VALID:
            problems.append(f"{gid}: status {status!r} is not one of {sorted(VALID)}")
            continue
        if status == "passed":
            if not g.get("command"):
                problems.append(f"{gid}: marked 'passed' with no recorded command. "
                                f"A gate passes only when a command actually ran.")
            if not g.get("log_path"):
                problems.append(f"{gid}: marked 'passed' with no log_path. "
                                f"An unrecorded result is not evidence.")
        if status == "blocked" and not g.get("limitation"):
            problems.append(f"{gid}: marked 'blocked' without naming the missing prerequisite. "
                            f"'blocked' must say what was unavailable.")

    if problems:
        print("acceptance-check FAILED:", file=sys.stderr)
        for p in problems:
            print(f"  - {p}", file=sys.stderr)
        return 1

    c = Counter(g["status"] for g in gates)
    print(f"acceptance-check: OK -- {len(gates)} gates accounted for "
          f"({c['passed']} passed / {c['failed']} failed / {c['blocked']} blocked / "
          f"{c['not_run']} not_run)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
