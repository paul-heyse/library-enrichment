"""Run the fixed five-run fixture with exact artifacts and a non-polling wall clock.

Compilation and diagnostics are excluded. An independent watchdog kills the owned process
group on deadline; blocking wait avoids Python's timeout polling adding up to 50 ms per run.
Neither thresholds nor host caches are changed by this command.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import signal
import statistics
import subprocess
import threading
import time
from contextlib import suppress
from datetime import UTC, datetime
from pathlib import Path

from cli import say
from evidence_run import source_digest


def digest(path: Path) -> str:
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def run(argv: list[str], output: Path, number: int) -> dict:
    env = dict(os.environ)
    env.pop("LIBENR_MEASURE_QUERIES", None)
    expired = threading.Event()
    with (output / f"run-{number}.log").open("xb") as log:
        started = time.monotonic()
        process = subprocess.Popen(
            ["/usr/bin/time", "-v", "-o", str(output / f"run-{number}.time"), *argv],
            stdout=log,
            stderr=subprocess.STDOUT,
            env=env,
            start_new_session=True,
        )

        def timeout() -> None:
            expired.set()
            with suppress(ProcessLookupError):
                os.killpg(process.pid, signal.SIGKILL)

        watchdog = threading.Timer(60, timeout)
        watchdog.start()
        try:
            result = process.wait()
            elapsed = time.monotonic() - started
        finally:
            watchdog.cancel()
            watchdog.join()
            if process.poll() is None:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
    return {
        "run": number,
        "exit_code": result,
        "wall_seconds": elapsed,
        "deadline_exceeded": expired.is_set(),
        "one_test_passed": "test result: ok. 1 passed; 0 failed; 0 ignored;"
        in (output / f"run-{number}.log").read_text(),
        "log": f"run-{number}.log",
        "resources": f"run-{number}.time",
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--executable", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--resources", choices=("portable", "workstation"), default="portable")
    args = parser.parse_args()
    executable = args.executable.resolve(strict=True)
    worker = executable.parent.parent / "library-enrichment-native-worker"
    if executable.parent.name != "deps" or not worker.is_file():
        parser.error("provide a precompiled Cargo integration test with its sibling native worker")
    args.output.mkdir(parents=True, exist_ok=False)
    name = "every_tool_answers_offline_from_the_snapshot_published_cold"
    if args.resources == "workstation":
        name = "workstation_" + name
    argv = [
        str(executable),
        "--exact",
        name,
        "--nocapture",
    ]
    receipt = {
        "started_at": datetime.now(UTC).isoformat(),
        "source_before": source_digest(),
        "profile": executable.parent.parent.name,
        "resources": args.resources,
        "command": argv,
        "binary_sha256": digest(executable),
        "native_worker_sha256": digest(worker),
        "host_caches_cleared": False,
        "measurement": "monotonic process wall; blocking wait; independent 60-second watchdog",
        "budgets": {"median": 1.5, "each": 3.0},
        "rss_scope": "time reports per-process/child maxima, not simultaneous tree RSS sum",
    }
    runs = [run(argv, args.output, number) for number in range(1, 6)]
    median = statistics.median(item["wall_seconds"] for item in runs)
    after = source_digest()
    passed = (
        receipt["source_before"] == after
        and median <= 1.5
        and all(
            item["exit_code"] == 0
            and item["one_test_passed"]
            and not item["deadline_exceeded"]
            and item["wall_seconds"] <= 3
            for item in runs
        )
        and receipt["binary_sha256"] == digest(executable)
        and receipt["native_worker_sha256"] == digest(worker)
    )
    receipt.update(
        runs=runs,
        median_seconds=median,
        source_after=after,
        status="passed" if passed else "failed",
        finished_at=datetime.now(UTC).isoformat(),
    )
    (args.output / "measurements.json").write_bytes((json.dumps(receipt, indent=2) + "\n").encode())
    say(json.dumps({"status": receipt["status"], "median_seconds": median, "runs": runs}))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
