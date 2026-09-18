"""Retained probe: verify and rebuild the copied research archive from its explicit inputs."""

from __future__ import annotations

import hashlib
import json
import os
import shutil
import subprocess
import sys
import tarfile
from pathlib import Path

HERE = Path(__file__).resolve().parent


def main() -> int:
    archive = HERE / "bundles/research.tar.gz"
    destination = HERE / "transfer" / f"research-{os.getpid()}"
    destination.mkdir(parents=True)
    with tarfile.open(archive) as bundle:
        bundle.extractall(destination, filter="data")
    interpreter = str(Path(sys.executable).resolve())
    ast_grep = shutil.which("ast-grep")
    if ast_grep is None:
        raise RuntimeError("Research rebuild requires ast-grep")
    environment = {"PATH": str(Path(ast_grep).parent) + ":/usr/bin:/bin", "LANG": "C.UTF-8"}
    outputs = []
    for name, script in [("manifest", "scripts/verify_bundle.py"), ("rebuild", "build/verify.py")]:
        done = subprocess.run(
            [interpreter, "-S", str(destination / script)],
            cwd="/",
            env=environment,
            capture_output=True,
            text=True,
            check=False,
        )
        (HERE / f"research-{name}.log").write_text(done.stdout + done.stderr)
        outputs.append({"check": name, "returncode": done.returncode})
        if done.returncode:
            break
    passed = len(outputs) == 2 and all(result["returncode"] == 0 for result in outputs)
    result = {
        "state": "passed" if passed else "failed",
        "commands": outputs,
        "archive_sha256": hashlib.sha256(archive.read_bytes()).hexdigest(),
        "cwd": "/",
        "interpreter": "resolved base Python, site hooks disabled",
        "scope": (
            "Reader manifest/lookups plus full deterministic regeneration from bundled "
            "exact reference inputs. Cargo dependencies/toolchains are not bundled or rebuilt."
        ),
    }
    (HERE / "research-qualification.json").write_text(json.dumps(result, indent=2) + "\n")
    sys.stdout.write(json.dumps(result, indent=2) + "\n")
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
