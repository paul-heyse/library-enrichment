#!/usr/bin/env python3
"""Generate docs/reports/acceptance.json by joining real test output against tests/gates.toml.

The join is one-directional on purpose: a gate becomes `passed` only when a test named in its
`tests` list actually executed and reported success. Every gate not matched by executed output
defaults to `not_run`. Nothing in this script can promote a gate on the strength of a claim.

Inputs (all optional -- absent input means `not_run`, never a failure):
  --nextest  JSON produced by `cargo nextest run --message-format libtest-json`
  --pytest   JSON produced by `pytest --json-report`
  --live     the same, from the opt-in live tier
  --client   the same, from the real-client tier
"""

from __future__ import annotations

import argparse
import json
import shlex
import subprocess
import sys
import tomllib
from datetime import UTC, datetime
from pathlib import Path

from cli import say
from evidence_run import read_receipt, source_digest, valid_receipt

ROOT = Path(__file__).resolve().parent.parent
SOURCE_DIGEST = source_digest()

#: node id -> (skip reason, log path), filled by `load_pytest`.
SKIPPED: dict[str, tuple[str, str]] = {}


def load_registry() -> dict:
    """The gate registry, read through a context manager so the handle is always closed."""
    with (ROOT / "tests/gates.toml").open("rb") as handle:
        return tomllib.load(handle)["gates"]


def recorded_command(matched: dict) -> str:
    """The exact command(s) that produced this gate's evidence, deduplicated and ordered."""
    logs = sorted({path for _, path in matched.values()})
    return " ; ".join(shlex.join(read_receipt(Path(path))["argv"]) for path in logs)


def tool_versions() -> dict[str, str]:
    out = {}
    for name, cmd in [
        ("rustc", ["rustc", "--version"]),
        ("cargo", ["cargo", "--version"]),
        ("python", [sys.executable, "--version"]),
        ("ty", ["ty", "--version"]),
        ("ruff", ["ruff", "--version"]),
    ]:
        try:
            r = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
            out[name] = r.stdout.strip() or r.stderr.strip() or "unknown"
        except Exception:
            out[name] = "not installed"
    return out


def load_nextest(path: Path | None) -> dict[str, tuple[bool, str]]:
    """Map test name -> (passed, log line). nextest emits newline-delimited JSON events."""
    results: dict[str, tuple[bool, str]] = {}
    if not path or not path.exists() or not valid_receipt(path, SOURCE_DIGEST):
        return results
    for line in path.read_text().splitlines():
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            ev = json.loads(line)
        except json.JSONDecodeError:
            continue
        if ev.get("type") == "test" and ev.get("event") in {"ok", "failed", "ignored"}:
            name = ev.get("name", "")
            if ev["event"] == "ignored":
                # Same rule as a pytest skip: an unexecuted test is `blocked` with its
                # prerequisite named, never `failed`.
                #
                # This branch does not fire under nextest 0.9.143's libtest-json-plus, which
                # emits `started` for an ignored test and then no terminal event at all. The
                # real-container tier is `#[ignore]`d, so a gate registering one of those tests
                # reports `not_run` ("registered tests did not execute") rather than `blocked`.
                # That is the safe direction -- never a false pass -- but it is the wrong
                # limitation text, and it is why containment is evidenced by the qualification
                # receipt rather than by a registered gate.
                SKIPPED[name] = (
                    str(ev.get("reason") or "marked #[ignore]"),
                    str(path),
                )
                continue
            SKIPPED.pop(name, None)
            passed = ev["event"] == "ok" and read_receipt(path)["exit_code"] == 0
            results[name] = (passed, str(path))
    return results


def load_pytest(path: Path | None) -> dict[str, tuple[bool, str]]:
    """Map node id -> (passed, log line). A skipped test is reported separately.

    A skip is NOT a failure: an unsupported platform or an unbuilt prerequisite is `blocked`
    with the prerequisite named (AGENTS.md's four states). Collapsing it into `failed` would
    claim a regression where there is only a missing tool.
    """
    results: dict[str, tuple[bool, str]] = {}
    if not path or not path.exists() or not valid_receipt(path, SOURCE_DIGEST):
        return results
    try:
        data = json.loads(path.read_text())
    except json.JSONDecodeError:
        return results
    for t in data.get("tests", []):
        node = t.get("nodeid", "")
        if t.get("outcome") == "skipped":
            SKIPPED[node] = (_skip_reason(t), str(path))
            continue
        SKIPPED.pop(node, None)
        passed = t.get("outcome") == "passed" and read_receipt(path)["exit_code"] == 0
        results[node] = (passed, str(path))
    return results


def _skip_reason(test: dict) -> str:
    """Pull pytest's own skip reason out of the json-report entry."""
    for phase in ("setup", "call", "teardown"):
        info = test.get(phase) or {}
        reason = info.get("longrepr") or ""
        if reason:
            # json-report renders a skip as "('path', lineno, 'Skipped: <reason>')".
            marker = "Skipped: "
            if marker in reason:
                return reason.split(marker, 1)[1].strip("')\" ")
            return str(reason)
    return "skipped without a recorded reason"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--nextest", type=Path, action="append", default=[])
    ap.add_argument("--pytest", type=Path)
    ap.add_argument("--live", type=Path)
    # The client tier is its own suite with its own log, for the same reason `live` is: it needs
    # things the deterministic run does not have, and a failure there must never be mistaken for
    # a regression in code that is fine.
    ap.add_argument("--client", type=Path)
    ap.add_argument("--out", type=Path, default=ROOT / "docs/reports/acceptance.json")
    args = ap.parse_args()

    registry = load_registry()
    executed = {}
    for path in args.nextest:
        executed.update(load_nextest(path))
    executed.update(
        {
            **load_pytest(args.pytest),
            **load_pytest(args.live),
            **load_pytest(args.client),
        }
    )

    gates = []
    for gid in sorted(registry):
        spec = registry[gid]
        named = spec.get("tests", [])
        matched = {t: executed[t] for t in named if t in executed}

        if spec.get("superseded"):
            # A retired gate is never passed and never failed: its premise no longer holds.
            limitation = f"superseded -- {spec['superseded']}"
            status, command, log = "not_run", "", ""
        elif not named:
            status, command, log = "not_run", "", ""
            limitation = "No test is registered for this gate yet."
        elif any(not ok for ok, _ in matched.values()):
            status = "failed"
            command = recorded_command(matched)
            log = ", ".join(sorted({path for _, path in matched.values()}))
            limitation = (
                "A registered test or its enclosing command failed; skips cannot conceal "
                "that failure."
            )
        elif skipped := [t for t in named if t in SKIPPED]:
            # A prerequisite was missing, which is `blocked`, not `failed` and not a pass.
            reasons = sorted({SKIPPED[t][0] for t in skipped})
            status = "blocked"
            command, log = "", SKIPPED[skipped[0]][1]
            limitation = (
                f"{len(skipped)} of {len(named)} registered tests were skipped. "
                f"Prerequisite: {'; '.join(reasons)}"
            )
        elif not matched:
            status, command, log = "not_run", "", ""
            limitation = f"Registered tests did not execute in this run: {', '.join(named)}"
        elif all(ok for ok, _ in matched.values()):
            status = "passed"
            # `caveat` is carried below: a gate can pass and still have a limitation worth
            # reading, and a limitation that lives only in a test docstring is invisible to
            # anyone reading the report.
            command = recorded_command(matched)
            # Every distinct log, not just the first: a gate spanning both suites has half its
            # evidence in nextest.json and half in pytest.json, and naming one of them points an
            # auditor at an incomplete record.
            log = ", ".join(sorted({path for _, path in matched.values()}))
            limitation = ""
            if len(matched) < len(named):
                missing = sorted(set(named) - set(matched))
                status = "not_run"
                limitation = f"Only some registered tests ran; missing: {', '.join(missing)}"
                command, log = "", ""
        else:
            status = "failed"
            command = recorded_command(matched)
            log = ", ".join(sorted({path for _, path in matched.values()}))
            limitation = ""

        # A declared caveat survives into the report even on a pass.
        if caveat := spec.get("caveat"):
            limitation = f"{limitation} {caveat}".strip() if limitation else caveat

        gates.append(
            {
                "gate_id": gid,
                "phase": spec["phase"],
                "kind": spec["kind"],
                "scenario": spec["scenario"],
                "assertion": spec["assertion"],
                "status": status,
                "command": command,
                "log_path": log,
                "limitation": limitation,
                **({"superseded": spec["superseded"]} if spec.get("superseded") else {}),
                **({"supersedes": spec["supersedes"]} if spec.get("supersedes") else {}),
            }
        )

    report = {
        "schema_version": "1.0",
        "generated_at": datetime.now(UTC).isoformat(timespec="seconds"),
        "tool_versions": tool_versions(),
        "source_digest": SOURCE_DIGEST,
        "gates": gates,
    }
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(report, indent=2) + "\n")

    from collections import Counter

    c = Counter(g["status"] for g in gates)
    say(
        f"acceptance: {c['passed']} passed / {c['failed']} failed / "
        f"{c['blocked']} blocked / {c['not_run']} not_run  (of {len(gates)})"
    )
    try:
        shown = args.out.relative_to(ROOT)
    except ValueError:
        shown = args.out  # an --out outside the repo is legitimate for an ad-hoc check
    say(f"  -> {shown}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
