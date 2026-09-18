#!/usr/bin/env python3
"""Compile negative caller-access controls in an isolated, retained-lock consumer."""

from __future__ import annotations

import hashlib
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE = ROOT / "skill_improvement/evidence/implementation"


def main() -> None:
    project = EVIDENCE / ".compile-fail"
    project.mkdir(exist_ok=True)
    (project / "src").mkdir(exist_ok=True)
    for name in ["Cargo.toml", "Cargo.lock"]:
        shutil.copyfile(EVIDENCE / "probes" / name, project / name)
    cases = {
        "private_import": (
            (
                "use deltalake::operations::load::LoadBuilder;\npub fn check(_"
                ": Option<LoadBuilder>) {}\n"
            ),
            "E0603",
        ),
        "internal_trait_method": (
            (
                "pub fn check(table: deltalake::DeltaTable) {\n    let builder"
                " = table.scan_table();\n    let _ = builder.get_custom_execut"
                "e_handler();\n}\n"
            ),
            "E0599",
        ),
    }
    out = EVIDENCE / "compile-fail"
    out.mkdir(exist_ok=True)
    env = dict(os.environ)
    env["CARGO_TARGET_DIR"] = str(EVIDENCE.parent / ".build-target")
    env["CARGO_INCREMENTAL"] = "0"
    for key in ["RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS"]:
        env.pop(key, None)
    results = []
    for name, (source, expected) in cases.items():
        retained = out / f"{name}.rs"
        retained.write_text(source)
        (project / "src/lib.rs").write_text(source)
        command = ["cargo", "+1.98.1", "test", "--no-run", "--locked", "--offline", "-j4"]
        done = subprocess.run(
            command, cwd=project, env=env, capture_output=True, text=True, check=False
        )
        log = out / f"{name}.log"
        log.write_text(done.stdout + done.stderr)
        results.append(
            {
                "case": name,
                "expected_error": expected,
                "state": "passed"
                if done.returncode and f"error[{expected}]" in done.stderr
                else "failed",
                "argv": command,
                "returncode": done.returncode,
                "source": str(retained.relative_to(ROOT)),
                "source_sha256": hashlib.sha256(source.encode()).hexdigest(),
                "log": str(log.relative_to(ROOT)),
            }
        )
    receipt = {
        "state": "passed" if all(r["state"] == "passed" for r in results) else "failed",
        "cases": results,
        "lock_sha256": hashlib.sha256((project / "Cargo.lock").read_bytes()).hexdigest(),
    }
    (EVIDENCE / "compile-fail-results.json").write_text(json.dumps(receipt, indent=2) + "\n")
    sys.stdout.write(json.dumps(receipt, indent=2) + "\n")
    raise SystemExit(0 if receipt["state"] == "passed" else 1)


if __name__ == "__main__":
    main()
