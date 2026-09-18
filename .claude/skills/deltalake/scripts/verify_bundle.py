#!/usr/bin/env python3
"""Verify a copied bundle offline; no cache, network, Cargo or host project imports."""

from __future__ import annotations

import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path
from urllib.parse import unquote

ROOT = Path(__file__).resolve().parents[1]


def main() -> int:
    manifest = json.loads((ROOT / "BUNDLE_MANIFEST.json").read_text())
    for name, expected in manifest["files"].items():
        path = ROOT / name
        if not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            raise ValueError(f"Missing or changed bundle file: {name}")
    for args in [
        ["find", "--task", "custom session UDF fallback", "--limit", "1"],
        ["show", "delta.schema"],
        ["show", "deltalake::DeltaTable", "--member", "scan_table"],
        ["compare", "delta.write", "delta.merge"],
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
    checked_links = 0
    for name in manifest["files"]:
        path = ROOT / name
        authored = (
            path.parent == ROOT
            or name.startswith(("content/capabilities/", "content/routes/", "content/integration/"))
            or name in {"scripts/README.md", "skill_improvement/IMPLEMENTATION_REPORT.md"}
            or name.endswith("/REPORT.md")
        )
        if not authored or path.suffix != ".md":
            continue
        body = re.sub(r"```.*?```", "", path.read_text(), flags=re.S)
        for target in re.findall(r"\[[^\]\n]*\]\(([^)\s]+)\)", body):
            if re.match(r"(?:https?|mailto|app):", target) or target.startswith("#"):
                continue
            linked = path.parent / unquote(target.split("#", 1)[0].strip("<>"))
            if not linked.is_file() or not linked.resolve().is_relative_to(ROOT):
                raise ValueError(f"Broken portable authored link: {name} -> {target}")
            checked_links += 1
    sys.stdout.write(
        json.dumps(
            {
                "state": "passed",
                "mode": manifest["mode"],
                "manifest_files": len(manifest["files"]),
                "lookup_commands": 4,
                "authored_document_links": checked_links,
                "cwd": "/",
                "network_required": False,
            }
        )
        + "\n"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
