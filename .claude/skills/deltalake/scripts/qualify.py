#!/usr/bin/env python3
"""Package twice, trace a copied reader, and regenerate a copied research bundle offline."""

from __future__ import annotations

import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tarfile
from datetime import UTC, datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE = ROOT / "skill_improvement/evidence/implementation"
PYTHON = str(Path(sys.executable).resolve())


def main() -> None:
    bundles = EVIDENCE / "bundles"
    bundles.mkdir(exist_ok=True)
    result = {"state": "failed", "started_at": datetime.now(UTC).isoformat(), "bundles": {}}
    receipt_path = EVIDENCE / "qualification.json"
    receipt_path.write_text(json.dumps(result, indent=2) + "\n")
    for mode in ["reader", "research"]:
        first = bundles / f"{mode}.tar.gz"
        second = bundles / f"{mode}-repeat.tar.gz"
        runs = []
        for out in [first, second]:
            done = subprocess.run(
                [PYTHON, "-I", "-S", str(ROOT / "scripts/package.py"), mode, "--output", str(out)],
                cwd="/",
                capture_output=True,
                text=True,
                check=False,
            )
            if done.returncode:
                raise RuntimeError(done.stdout + done.stderr)
            runs.append(json.loads(done.stdout))
        assert first.read_bytes() == second.read_bytes()
        result["bundles"][mode] = runs[0] | {"repeated_identical": True}
    done = subprocess.run(
        [PYTHON, "-I", "-S", str(ROOT / "scripts/qualify_transfer.py")],
        cwd="/",
        capture_output=True,
        text=True,
        check=False,
    )
    if done.returncode:
        raise RuntimeError(done.stdout + done.stderr)
    result["reader"] = json.loads(done.stdout)
    dest = EVIDENCE / ".research-qualification" / str(os.getpid())
    dest.mkdir(parents=True)
    with tarfile.open(bundles / "research.tar.gz") as t:
        t.extractall(dest, filter="data")
    before = {
        str(p.relative_to(dest / "content")): hashlib.sha256(p.read_bytes()).hexdigest()
        for p in (dest / "content").rglob("*")
        if p.is_file()
    }
    (dest / "content").rename(dest / ".expected-content")
    # Start empty: every emitted artifact must be recreated from bundled input/authoring.
    # Imports resolve within the copied build; its tool binary is explicit.
    binary_dir = dest / ".tools"
    binary_dir.mkdir()
    ast = shutil.which("ast-grep")
    assert ast is not None
    (binary_dir / "ast-grep").symlink_to(Path(ast).resolve())
    trace = EVIDENCE / "research-syscalls.log"
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
        PYTHON,
        "-S",
        str(dest / "build/build.py"),
    ]
    done = subprocess.run(
        command,
        cwd="/",
        env={"PATH": str(binary_dir) + ":/usr/bin:/bin", "LANG": "C.UTF-8"},
        capture_output=True,
        text=True,
        check=False,
    )
    (EVIDENCE / "research-rebuild.log").write_text(done.stdout + done.stderr)
    assert done.returncode == 0, done.stderr[-2000:]
    after = {
        str(p.relative_to(dest / "content")): hashlib.sha256(p.read_bytes()).hexdigest()
        for p in (dest / "content").rglob("*")
        if p.is_file()
    }
    assert before == after, [p for p in before if before[p] != after.get(p)]
    log = trace.read_text()
    network = [
        line
        for line in log.splitlines()
        if re.search(r"\b(socket|connect)\(", line) and "AF_INET" in line
    ]
    host = [
        line
        for line in log.splitlines()
        if re.search(r"\b(open|openat|execve)\(", line)
        and str(ROOT.parents[2]) in line
        and str(dest) not in line
    ]
    assert not network, network
    assert not host, host
    result["research"] = {
        "state": "passed",
        "generated_output_started_empty": True,
        "identical_content_files": len(before),
        "internet_syscalls": len(network),
        "original_project_opens": len(host),
        "scope": "Reference regeneration; Cargo compilation is not bundled or qualified here",
    }
    result["ended_at"] = datetime.now(UTC).isoformat()
    result["state"] = "passed"
    result["qualifier_sha256"] = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    receipt_path.write_text(json.dumps(result, indent=2) + "\n")
    sys.stdout.write(json.dumps(result, indent=2) + "\n")


if __name__ == "__main__":
    main()
