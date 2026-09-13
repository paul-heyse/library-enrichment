#!/usr/bin/env python3
"""Generate docs/reports/acceptance.json by joining real test output against tests/gates.toml.

The join is one-directional on purpose: a gate becomes `passed` only when a test named in its
`tests` list actually executed and reported success. Every gate not matched by executed output
defaults to `not_run`. Nothing in this script can promote a gate on the strength of a claim.

Inputs (all optional -- absent input means `not_run`, never a failure):
  --nextest  JSON produced by `cargo nextest run --message-format libtest-json`
  --pytest   JSON produced by `pytest --json-report`
"""
from __future__ import annotations

import argparse, json, subprocess, sys, tomllib
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


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
    if not path or not path.exists():
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
            results[ev.get("name", "")] = (ev["event"] == "ok", str(path))
    return results


def load_pytest(path: Path | None) -> dict[str, tuple[bool, str]]:
    results: dict[str, tuple[bool, str]] = {}
    if not path or not path.exists():
        return results
    try:
        data = json.loads(path.read_text())
    except json.JSONDecodeError:
        return results
    for t in data.get("tests", []):
        results[t.get("nodeid", "")] = (t.get("outcome") == "passed", str(path))
    return results


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--nextest", type=Path)
    ap.add_argument("--pytest", type=Path)
    ap.add_argument("--out", type=Path, default=ROOT / "docs/reports/acceptance.json")
    args = ap.parse_args()

    registry = tomllib.load(open(ROOT / "tests/gates.toml", "rb"))["gates"]
    executed = {**load_nextest(args.nextest), **load_pytest(args.pytest)}

    gates = []
    for gid in sorted(registry):
        spec = registry[gid]
        named = spec.get("tests", [])
        matched = {t: executed[t] for t in named if t in executed}

        if spec.get("superseded"):
            # A retired gate is never passed and never failed: its premise no longer holds.
            status, command, log, limitation = "not_run", "", "", f"superseded -- {spec['superseded']}"
        elif not named:
            status, command, log = "not_run", "", ""
            limitation = "No test is registered for this gate yet."
        elif not matched:
            status, command, log = "not_run", "", ""
            limitation = f"Registered tests did not execute in this run: {', '.join(named)}"
        elif all(ok for ok, _ in matched.values()):
            status = "passed"
            command = f"tests: {', '.join(sorted(matched))}"
            log = next(iter(matched.values()))[1]
            limitation = ""
            if len(matched) < len(named):
                missing = sorted(set(named) - set(matched))
                status, limitation = "not_run", f"Only some registered tests ran; missing: {', '.join(missing)}"
                command, log = "", ""
        else:
            status = "failed"
            command = f"tests: {', '.join(sorted(matched))}"
            log = next(iter(matched.values()))[1]
            limitation = ""

        gates.append({
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
        })

    report = {
        "schema_version": "1.0",
        "generated_at": datetime.now(timezone.utc).isoformat(timespec="seconds"),
        "tool_versions": tool_versions(),
        "gates": gates,
    }
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(report, indent=2) + "\n")

    from collections import Counter
    c = Counter(g["status"] for g in gates)
    print(f"acceptance: {c['passed']} passed / {c['failed']} failed / "
          f"{c['blocked']} blocked / {c['not_run']} not_run  (of {len(gates)})")
    print(f"  -> {args.out.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
