#!/usr/bin/env python3
"""Exercise a packaged reader from / with an empty environment and trace filesystem/network use."""

from __future__ import annotations

import hashlib
import json
import os
import re
import subprocess
import sys
import tarfile
from datetime import UTC, datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE = ROOT / "skill_improvement/evidence/implementation"


def main() -> int:
    archive = EVIDENCE / "bundles/reader.tar.gz"
    receipt_path = EVIDENCE / "transfer-qualification.json"
    started = datetime.now(UTC).isoformat()
    receipt_path.write_text(json.dumps({"state": "failed", "started_at": started}) + "\n")
    # Distinct extraction on every invocation; existing evidence is never cleaned broadly.
    destination = EVIDENCE / ".qualification" / f"reader-{os.getpid()}"
    destination.mkdir(parents=True)
    with tarfile.open(archive) as bundle:
        bundle.extractall(destination, filter="data")
    assert not (destination / "build/.cache").exists()
    trace = EVIDENCE / "transfer-syscalls.log"
    interpreter = str(Path(sys.executable).resolve())
    command = [
        "strace",
        "-f",
        "-qq",
        "-s",
        "4096",
        "-e",
        "trace=file,network",
        "-o",
        str(trace),
        interpreter,
        "-I",
        "-S",
        str(destination / "scripts/verify_bundle.py"),
    ]
    done = subprocess.run(
        command,
        cwd="/",
        env={"PATH": "/usr/bin:/bin", "LANG": "C.UTF-8"},
        capture_output=True,
        text=True,
        check=False,
    )
    if done.returncode:
        raise RuntimeError(done.stdout + done.stderr)
    log = trace.read_text()
    socket_calls = [
        line
        for line in log.splitlines()
        if re.search(r"\b(socket|connect|sendto|recvfrom)\(", line)
    ]
    connections = [line for line in socket_calls if "AF_INET" in line]
    local_name_cache = [line for line in socket_calls if "/var/run/nscd/socket" in line]
    other_connects = [
        line for line in socket_calls if "connect(" in line and line not in local_name_cache
    ]
    unexpected = [
        line
        for line in log.splitlines()
        if re.search(r"\b(open|openat|execve)\(", line)
        and str(ROOT.parents[2]) in line
        and str(destination) not in line
    ]
    assert not connections, connections
    assert not other_connects, other_connects
    assert not unexpected, unexpected
    assert "build/.cache" not in log
    result = json.loads(done.stdout)

    def invalidation() -> dict:
        run = subprocess.run(
            [interpreter, "-I", "-S", str(destination / "scripts/invalidation.py")],
            cwd="/",
            env={"PATH": "/usr/bin:/bin"},
            capture_output=True,
            text=True,
            check=True,
        )
        return json.loads(run.stdout)

    assert not invalidation()["affected"]
    authored = destination / "authoring/capabilities/delta.schema.json"
    original = authored.read_bytes()
    try:
        authored.write_bytes(original + b"\n")
        assert set(invalidation()["affected"]) == {"delta.schema"}
    finally:
        authored.write_bytes(original)
    index = destination / "content/index/operations.tsv"
    original_index = index.read_bytes()
    try:
        index.write_bytes(
            original_index
            + b"deltalake_core::qualification_control\tfixture\tfunction\tdeltalake-core\t"
            b"fixture.json\tfixture.md\tfn fixture()\n"
        )
        changed = invalidation()
        assert changed["changed_crates"] == ["deltalake-core"]
        assert changed["affected"]["delta.schema"]["candidate_set_review_crates"] == [
            "deltalake-core"
        ]
    finally:
        index.write_bytes(original_index)
    controls = []
    for name, expected in [
        ("content/profile.json", None),
        (
            "skill_improvement/evidence/sources/delta-rs/crates/core/src/kernel/mod.rs",
            {"delta.kernel"},
        ),
        (
            "content/operations/deltalake_core.operations.vacuum.VacuumBuilder.json",
            {"delta.retention"},
        ),
    ]:
        path = destination / name
        original = path.read_bytes()
        try:
            path.write_bytes(original + b"\n")
            affected = set(invalidation()["affected"])
            assert affected and (expected is None or affected == expected), (name, affected)
            controls.append({"changed": name, "affected": sorted(affected)})
        finally:
            path.write_bytes(original)
    assert not invalidation()["affected"]
    result.update(
        started_at=started,
        ended_at=datetime.now(UTC).isoformat(),
        archive_sha256=hashlib.sha256(archive.read_bytes()).hexdigest(),
        network_syscalls=len(connections),
        system_name_cache_attempts=len(local_name_cache),
        outside_skill_accesses=len(unexpected),
        source_cache_present=False,
        environment="PATH and LANG only; resolved base Python -I -S",
        invalidation_controls=(
            "passed: authoring, API/default contract, source, profile and new alternative"
        ),
        targeted_controls=controls,
        scope=(
            "No original host-project files opened; "
            "Python/system runtime reads and ancestor path metadata are expected."
        ),
    )
    receipt_path.write_text(json.dumps(result, indent=2) + "\n")
    sys.stdout.write(json.dumps(result, indent=2) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
