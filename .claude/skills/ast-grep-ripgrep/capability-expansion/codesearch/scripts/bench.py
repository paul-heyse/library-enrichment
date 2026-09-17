"""Measure one-shot query latency against the serving decision's exit criterion.

The design chose a one-shot query process over a resident session, and recorded a falsifiable
criterion rather than an opinion: **if p95 exceeds 400 ms the decision is revisited** in favour of
a resident mode over a socket.

This measures the *whole process* -- spawn, session construction, Delta log replay, plan, execute,
exit -- because that is what the criterion is about. Timing from inside the binary would omit
exactly the costs that make one-shot questionable, and would report a comfortable number for a
decision it cannot actually inform.

Reports the four states the surrounding project uses. A missing binary or catalog is `blocked`
with the prerequisite named, never a quiet pass.
"""

from __future__ import annotations

import os
import statistics
import subprocess
import sys
import time
from pathlib import Path

THRESHOLD_MS = 400.0
DEFAULT_ITERATIONS = 30


def percentile(values: list[float], fraction: float) -> float:
    """Nearest-rank percentile. Exact on small samples, where interpolation would invent
    precision the sample does not have."""
    if not values:
        return float("nan")
    ordered = sorted(values)
    rank = max(1, min(len(ordered), round(fraction * len(ordered))))
    return ordered[rank - 1]


def main() -> int:
    out = sys.stdout.write
    binary = os.environ.get("CODESEARCH_QUERY_BIN")
    if not binary:
        candidate = Path(os.environ.get("CARGO_TARGET_DIR", "target")) / "debug/codesearch-query"
        binary = str(candidate)
    home = os.environ.get("CODESEARCH_HOME")

    if not Path(binary).exists():
        out(f"bench: blocked -- no query binary at {binary}. Run `just build` first.\n")
        return 0
    if not home or not (Path(home) / "catalog").exists():
        out("bench: blocked -- no catalog. Run `just catalog` first.\n")
        return 0

    iterations = int(sys.argv[1]) if len(sys.argv) > 1 else DEFAULT_ITERATIONS
    command = [binary, "--home", home, "--json", "describe", "mech:rg/cli/--pcre2"]

    # One untimed run so the measurement is of warm page cache rather than of first-touch I/O.
    subprocess.run(command, capture_output=True, check=False)

    samples: list[float] = []
    for _ in range(iterations):
        started = time.perf_counter()
        completed = subprocess.run(command, capture_output=True, check=False)
        samples.append((time.perf_counter() - started) * 1000.0)
        if completed.returncode != 0:
            out(f"bench: failed -- query exited {completed.returncode}\n")
            out(completed.stderr.decode("utf-8", "replace"))
            return 1

    p50 = percentile(samples, 0.50)
    p95 = percentile(samples, 0.95)
    out(f"bench: {iterations} whole-process runs of `describe`\n")
    out(f"  p50   {p50:7.1f} ms\n")
    out(f"  p95   {p95:7.1f} ms\n")
    out(f"  min   {min(samples):7.1f} ms   max {max(samples):7.1f} ms\n")
    out(f"  mean  {statistics.mean(samples):7.1f} ms\n\n")

    if p95 > THRESHOLD_MS:
        out(
            f"bench: failed -- p95 {p95:.1f} ms exceeds the {THRESHOLD_MS:.0f} ms criterion.\n"
            "The one-shot serving decision should be revisited; the resident-session fallback is\n"
            "designed in PLAN-V2 section 8 and is not built.\n"
        )
        return 1
    out(f"bench: passed -- p95 {p95:.1f} ms is within the {THRESHOLD_MS:.0f} ms criterion.\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
