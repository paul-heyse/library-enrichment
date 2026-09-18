"""Run locked local Rust planning probes and retain profile, logs and source digests."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
from datetime import UTC, datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
HERE = ROOT / "skill_improvement/evidence/implementation"
PROBES = HERE / "probes"
KERNEL = "8ba063f8f84fec222000f66d40d70911d7c79675"
DELTA = "58f07cd62bfbce3649a7e1c87c696288068ae184"


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--resolve", action="store_true", help="Initial resolution from retained acquisition lock"
    )
    parser.add_argument("--label", default="runtime")
    args = parser.parse_args()
    env = dict(os.environ)
    env["CARGO_TARGET_DIR"] = str(HERE.parent / ".build-target")
    env["CARGO_INCREMENTAL"] = "0"
    for key in ["RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTDOCFLAGS"]:
        env.pop(key, None)
    if args.resolve and not (PROBES / "Cargo.lock").exists():
        (PROBES / "Cargo.lock").write_bytes((HERE.parent / "acquisition.Cargo.lock").read_bytes())
    commands = [
        [
            "cargo",
            "+1.98.1",
            "metadata",
            "--offline",
            "--format-version",
            "1",
            *([] if args.resolve else ["--locked"]),
        ],
        ["rustc", "+1.98.1", "-Vv"],
        [
            "cargo",
            "+1.98.1",
            "test",
            "--offline",
            "--locked",
            "-j",
            "4",
            "--",
            "--show-output",
            "--test-threads=1",
        ],
    ]
    runs = []
    started = datetime.now(UTC).isoformat()
    state = "failed"
    tests = []
    (HERE / "logs").mkdir(exist_ok=True)
    for i, command in enumerate(commands):
        done = subprocess.run(
            command, cwd=PROBES, env=env, capture_output=True, text=True, check=False
        )
        log = HERE / "logs" / f"{args.label}-{i}.log"
        log.write_text(done.stdout + done.stderr)
        runs.append(
            {"argv": command, "returncode": done.returncode, "log": str(log.relative_to(HERE))}
        )
        if done.returncode:
            sys.stderr.write(done.stderr[-4000:])
            break
        if i == 0:
            metadata = json.loads(done.stdout)
            packages = {p["id"]: p for p in metadata["packages"]}
            profile = [
                {
                    "name": packages[n["id"]]["name"],
                    "version": packages[n["id"]]["version"],
                    "source": packages[n["id"]]["source"],
                    "features": n["features"],
                }
                for n in metadata["resolve"]["nodes"]
            ]
            for name in ["buoyant_kernel", "buoyant_kernel_engine", "buoyant_kernel_derive"]:
                hits = [p for p in profile if p["name"] == name]
                if len(hits) != 1 or not hits[0]["source"].endswith("#" + KERNEL):
                    raise RuntimeError(f"Kernel pin drift: {hits}")
            for name in ["deltalake", "deltalake-core"]:
                hits = [p for p in profile if p["name"] == name]
                if len(hits) != 1 or not hits[0]["source"].endswith("#" + DELTA):
                    raise RuntimeError(f"Delta pin drift: {hits}")
            for name, version in [
                ("datafusion", "55.1.0"),
                ("arrow-array", "59.3.0"),
                ("object_store", "0.13.2"),
            ]:
                if [p["version"] for p in profile if p["name"] == name] != [version]:
                    raise RuntimeError(f"Incoherent profile for {name}")
            (HERE / "runtime-profile.json").write_text(json.dumps(profile, indent=2) + "\n")
        elif i == 2:
            tests = re.findall(r"^test (\w+) \.\.\. ok$", done.stdout, re.MULTILINE)
            expected = {
                name
                for p in (PROBES / "tests").glob("*.rs")
                for name in re.findall(
                    r"#\[(?:tokio::)?test\]\s*(?:async )?fn (\w+)", p.read_text()
                )
            }
            if set(tests) != expected or len(tests) != len(expected):
                raise RuntimeError(
                    f"Executed controls differ: expected {sorted(expected)}, found {tests}"
                )
            state = "passed"
            sys.stdout.write(done.stdout)
    record = {
        "state": state,
        "started_at": started,
        "ended_at": datetime.now(UTC).isoformat(),
        "commands": runs,
        "passed_tests": tests,
        "target": "evidence/.build-target",
        "runner_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "profile_sha256": hashlib.sha256((HERE / "runtime-profile.json").read_bytes()).hexdigest()
        if (HERE / "runtime-profile.json").exists()
        else None,
        "sources": {
            str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest()
            for p in sorted(PROBES.rglob("*"))
            if p.is_file()
        },
        "scope": (
            "Local memory/filesystem, controlled stale snapshots and fixt"
            "ures; no remote cloud, crash durability or workload performa"
            "nce qualification."
        ),
    }
    (HERE / "probe-results.json").write_text(json.dumps(record, indent=2) + "\n")
    raise SystemExit(0 if state == "passed" else 1)


if __name__ == "__main__":
    main()
