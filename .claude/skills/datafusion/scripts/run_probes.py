#!/usr/bin/env python3
"""Run the isolated, locked reference probes and record provenance; requires Rust 1.98.1."""

from __future__ import annotations

import hashlib
import json
import os
import re
import subprocess
import sys
from datetime import UTC, datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE = ROOT / "skill_improvement/evidence/implementation"
PROBES = EVIDENCE / "probes"


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> int:
    target = EVIDENCE.parent / ".build-target-implementation"
    cargo = ["cargo", "+1.98.1"]
    commands = [
        [
            *cargo,
            "test",
            "--locked",
            "--offline",
            "--target-dir",
            str(target),
            "-j",
            "4",
            "--",
            "--show-output",
            "--test-threads=1",
        ],
        [*cargo, "run", "--locked", "--offline", "--target-dir", str(target), "--bin", "registry"],
        [*cargo, "metadata", "--locked", "--offline", "--format-version", "1"],
        ["rustc", "+1.98.1", "-Vv"],
        [
            *cargo,
            "run",
            "--locked",
            "--offline",
            "--target-dir",
            str(target),
            "--bin",
            "filter_cost",
        ],
    ]
    start = datetime.now(UTC).isoformat()
    runs = []
    outputs = []
    for command in commands:
        done = subprocess.run(command, cwd=PROBES, capture_output=True, text=True, check=False)
        outputs.append(done.stdout)
        runs.append(
            {
                "command": [part.replace(str(ROOT), "<skill>") for part in command],
                "returncode": done.returncode,
            }
        )
        log = done.stdout + done.stderr
        (EVIDENCE / f"probe-command-{len(runs)}.log").write_text(log.replace(str(ROOT), "<skill>"))
        if done.returncode:
            break
    state = (
        "passed"
        if len(runs) == len(commands) and all(r["returncode"] == 0 for r in runs)
        else "failed"
    )
    passed = sorted(set(re.findall(r"^test (\w+) \.\.\. ok$", outputs[0], flags=re.MULTILINE)))
    if state == "passed":
        (EVIDENCE / "runtime-registry.json").write_text(outputs[1])
        (EVIDENCE / "filter-cost.json").write_text(outputs[4])
        metadata = json.loads(outputs[2])
        packages = {p["id"]: p for p in metadata["packages"]}
        profile = {
            "toolchain": outputs[3],
            "target_directory": "<skill>/skill_improvement/evidence/.build-target-implementation",
            "packages": [
                {
                    "name": packages[n["id"]]["name"],
                    "version": packages[n["id"]]["version"],
                    "features": n["features"],
                    "source": packages[n["id"]]["source"],
                }
                for n in metadata["resolve"]["nodes"]
            ],
            "manifest_sha256": sha(PROBES / "Cargo.toml"),
            "lock_sha256": sha(PROBES / "Cargo.lock"),
            "flags": {
                k: os.environ.get(k)
                for k in ["RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTC_WRAPPER"]
            },
        }
        (EVIDENCE / "probe-profile.json").write_text(json.dumps(profile, indent=2) + "\n")
    receipt = {
        "state": state,
        "started": start,
        "finished": datetime.now(UTC).isoformat(),
        "commands": runs,
        "passed_tests": passed,
        "sources": {
            str(p.relative_to(ROOT)): sha(p)
            for p in sorted(PROBES.rglob("*"))
            if p.is_file() and (p.suffix == ".rs" or p.name in {"Cargo.toml", "Cargo.lock"})
        },
        "scope": (
            "Named assertion tests only. "
            "No process-RSS, network-I/O or broad performance certification."
        ),
    }
    (EVIDENCE / "probe-results.json").write_text(json.dumps(receipt, indent=2) + "\n")
    sys.stdout.write(json.dumps({"state": state, "passed_tests": len(passed)}) + "\n")
    return 0 if state == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
