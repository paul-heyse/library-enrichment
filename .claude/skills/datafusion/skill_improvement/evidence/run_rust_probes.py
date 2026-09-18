"""Run the isolated Arrow contract probes and retain commands, logs and environment."""

from __future__ import annotations

import hashlib
import json
import os
import subprocess
from datetime import UTC, datetime
from pathlib import Path

HERE = Path(__file__).resolve().parent


def main() -> None:
    env = dict(os.environ)
    env["CARGO_TARGET_DIR"] = str(HERE / ".build-target")
    env["CARGO_INCREMENTAL"] = "0"
    args = ["cargo", "+1.98.1", "test", "--offline", "--manifest-path", "probes/Cargo.toml"]
    if (HERE / "probes/Cargo.lock").exists():
        args.append("--locked")
    start = datetime.now(UTC).isoformat()
    done = subprocess.run(args, cwd=HERE, env=env, capture_output=True, text=True, check=False)
    log = HERE / "logs/rust-probes.log"
    log.write_text(done.stdout + done.stderr)
    record = {
        "started_at": start,
        "ended_at": datetime.now(UTC).isoformat(),
        "argv": args,
        "cwd": "evidence",
        "exit_code": done.returncode,
        "status": "passed" if done.returncode == 0 else "failed",
        "environment_overrides": {
            "CARGO_TARGET_DIR": "evidence/.build-target",
            "CARGO_INCREMENTAL": "0",
        },
        "log": "logs/rust-probes.log",
        "log_sha256": hashlib.sha256(log.read_bytes()).hexdigest(),
    }
    lock = HERE / "probes/Cargo.lock"
    if lock.exists():
        record["lock_sha256"] = hashlib.sha256(lock.read_bytes()).hexdigest()
    (HERE / "rust-probe-run.json").write_text(json.dumps(record, indent=2) + "\n")
    print(done.stdout)
    print(done.stderr[-4000:])
    raise SystemExit(done.returncode)


if __name__ == "__main__":
    main()
