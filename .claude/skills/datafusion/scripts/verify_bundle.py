#!/usr/bin/env python3
"""Verify a copied bundle offline; no cache, network, Cargo or host project imports."""

from __future__ import annotations

import hashlib
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def main() -> int:
    manifest = json.loads((ROOT / "BUNDLE_MANIFEST.json").read_text())
    for name, expected in manifest["files"].items():
        path = ROOT / name
        if not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            raise ValueError(f"Missing or changed bundle file: {name}")
    for args in [
        ["find", "--task", "provider inexact projection limit", "--limit", "1"],
        ["show", "arrow.cast"],
        ["show", "datafusion::prelude::DataFrame", "--member", "execute_stream"],
        ["compare", "arrow.select", "arrow.filter-reuse"],
    ]:
        done = subprocess.run(
            [sys.executable, "-I", "-S", str(ROOT / "scripts/reference.py"), *args],
            cwd="/",
            capture_output=True,
            text=True,
            check=True,
        )
        result = json.loads(done.stdout)
        if "error" in result:
            raise ValueError(result)
    # Validate content separately from the transport manifest.
    provenance = json.loads((ROOT / "content/PROVENANCE.json").read_text())
    for name, expected in provenance["files"].items():
        assert hashlib.sha256((ROOT / "content" / name).read_bytes()).hexdigest() == expected
    sys.stdout.write(
        json.dumps(
            {
                "state": "passed",
                "mode": manifest["mode"],
                "manifest_files": len(manifest["files"]),
                "lookup_commands": 4,
                "cwd": "/",
                "network_required": False,
            }
        )
        + "\n"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
