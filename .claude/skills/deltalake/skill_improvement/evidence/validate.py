"""Validate the planning deliverable and its receipts without changing the active skill."""

from __future__ import annotations

import csv
import hashlib
import json
import re
import subprocess
import sys
from datetime import UTC, datetime
from pathlib import Path

HERE = Path(__file__).resolve().parent
PLAN = HERE.parent
SKILL = PLAN.parent


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    checks = []

    def check(name: str, failures: list[str], **details: object) -> None:
        checks.append(
            {
                "id": name,
                "status": "failed" if failures else "passed",
                "failures": failures,
                **details,
            }
        )

    baseline = json.loads((HERE / "baseline-files.json").read_text())
    current = {
        str(p.relative_to(SKILL)): sha(p)
        for p in SKILL.rglob("*")
        if p.is_file()
        and p.relative_to(SKILL).parts[0] != "skill_improvement"
        and not any(part in {".cache", "__pycache__", ".git"} for part in p.parts)
    }
    check(
        "active-skill-preserved",
        [p for p in sorted(set(baseline) | set(current)) if baseline.get(p) != current.get(p)],
        files=len(baseline),
    )
    failures = []
    links = 0
    docs = sorted(PLAN.glob("*.md")) + sorted((PLAN / "examples").glob("*.md"))
    docs.append(HERE / "README.md")
    for path in docs:
        text = re.sub(r"```.*?```", "", path.read_text(), flags=re.DOTALL)
        for target in re.findall(r"\[[^\]]+\]\(([^)]+)\)", text):
            if target.startswith(("https://", "http://", "#")):
                continue
            links += 1
            if not (path.parent / target.split("#", 1)[0]).exists():
                failures.append(f"{path.relative_to(PLAN)}: {target}")
    check("document-links", failures, documents=len(docs), links=links)
    source = json.loads((HERE / "source-manifest.json").read_text())
    check(
        "retained-source-bytes",
        [r["path"] for r in source["source_files"] if sha(HERE / r["path"]) != r["sha256"]],
        files=len(source["source_files"]),
    )
    check(
        "acquisition-lock",
        []
        if sha(HERE / "acquisition.Cargo.lock") == source["acquisition_lock_sha256"]
        else ["lock digest"],
    )
    runtime = json.loads((HERE / "runtime-receipt.json").read_text())
    failures = [p for p, expected in runtime["source_hashes"].items() if sha(HERE / p) != expected]
    if runtime["runner_sha256"] != sha(HERE / "run_probes.py"):
        failures.append("run_probes.py")
    if runtime["status"] != "passed" or len(runtime["passed_tests"]) != 4:
        failures.append("runtime test status/tally")
    check("runtime-receipt-current", failures, tests=runtime["passed_tests"])
    queries = json.loads((HERE / "query-runs.json").read_text())
    check(
        "query-receipts",
        [r["id"] for r in queries if r["exit_code"] != 0 or sha(HERE / r["output"]) != r["sha256"]],
        queries=len(queries),
    )
    roles = list(csv.DictReader((PLAN / "examples/crate-roles.tsv").open(), delimiter="\t"))
    manifest = json.loads((SKILL / "build/manifests/deltalake.json").read_text())
    expected = {c["package"] for c in manifest["crate_sets"][0]["crates"]}
    actual = {r["package"] for r in roles}
    check("crate-role-coverage", sorted(expected ^ actual), packages=len(actual))
    sample = json.loads((PLAN / "examples/capability-record.json").read_text())
    check(
        "sample-record-evidence",
        [path for path in sample["evidence"].values() if not ((PLAN / "examples") / path).exists()],
    )
    scripts = [str(p) for p in sorted(HERE.glob("*.py"))]
    commands = [
        ("ruff", ["uv", "run", "--no-project", "ruff", "check", *scripts]),
        ("python-format", ["uv", "run", "--no-project", "ruff", "format", "--check", *scripts]),
        (
            "rust-format",
            [
                "cargo",
                "+1.98.1",
                "fmt",
                "--manifest-path",
                str(HERE / "probes/Cargo.toml"),
                "--check",
            ],
        ),
        (
            "clippy",
            [
                "cargo",
                "+1.98.1",
                "clippy",
                "--locked",
                "--offline",
                "--manifest-path",
                str(HERE / "probes/Cargo.toml"),
                "--target-dir",
                str(HERE / ".build-target"),
                "--all-targets",
                "-j",
                "4",
                "--",
                "-D",
                "warnings",
            ],
        ),
        ("tool-versions", ["ast-grep", "--version"]),
    ]
    for name, command in commands:
        done = subprocess.run(command, cwd=HERE, capture_output=True, text=True, check=False)
        log = HERE / "logs" / f"{name}.log"
        log.write_text(done.stdout + done.stderr)
        check(
            name,
            [] if done.returncode == 0 else [done.stderr[-1600:]],
            argv=[a.replace(str(SKILL), "<skill>") for a in command],
            returncode=done.returncode,
            log=str(log.relative_to(HERE)),
            log_sha256=sha(log),
        )
    result = {
        "verified_at": datetime.now(UTC).isoformat(),
        "checks": checks,
        "status": "passed" if all(c["status"] == "passed" for c in checks) else "failed",
        "scope": (
            "Planning artifacts and local probes; implementation and comparative evaluation not_run"
        ),
    }
    (HERE / "validation.json").write_text(json.dumps(result, indent=2) + "\n")
    sys.stdout.write(json.dumps(result, indent=2) + "\n")
    raise SystemExit(0 if result["status"] == "passed" else 1)


if __name__ == "__main__":
    main()
