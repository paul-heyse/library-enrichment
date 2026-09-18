#!/usr/bin/env python3
"""Create deterministic portable reader/research archives without compiler targets or host files."""

from __future__ import annotations

import argparse
import gzip
import hashlib
import io
import json
import sys
import tarfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def selected(path: Path, mode: str) -> bool:
    relative = path.relative_to(ROOT)
    parts = relative.parts
    if relative.as_posix() in {
        "skill_improvement/evidence/implementation/bundles/candidate-pilot.tar.gz",
        "skill_improvement/evidence/implementation/bundles/candidate-pilot.json",
        "skill_improvement/evidence/implementation/baseline-reader.tar.gz",
        "skill_improvement/evidence/implementation/baseline-files.json",
    }:
        return mode == "research"
    if any(
        p
        in {
            "__pycache__",
            ".ruff_cache",
            ".git",
            "target",
            "fixtures",
            "bundles",
            "transfer",
            "baseline",
            "candidate",
        }
        or p.startswith(".build-target")
        for p in parts
    ):
        return False
    if ".cache" in parts:
        return mode == "research" and parts[:2] == ("build", ".cache")
    if parts[0] in {"content", "queries", "scripts", "build", "authoring"}:
        return True
    if len(parts) == 1:
        return path.suffix == ".md" or path.name == "LICENSE"
    if mode == "research":
        return parts[0] == "skill_improvement"
    evidence = "skill_improvement/evidence/"
    name = relative.as_posix()
    if not name.startswith(evidence):
        return False
    tail = name.removeprefix(evidence)
    if tail.startswith(("sources/", "queries/")):
        return True
    if tail in {"source-manifest.json", "rust-probe-run.json", "logs/rust-probes.log"}:
        return True
    if tail.startswith("implementation/"):
        leaf = tail.removeprefix("implementation/")
        return leaf.startswith(("probes/", "source-exports/", "probe-command-")) or leaf in {
            "probe-results.json",
            "probe-profile.json",
            "runtime-registry.json",
        }
    return False


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=["reader", "research"])
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    files = [
        p
        for p in sorted(ROOT.rglob("*"))
        if p.is_file() and not p.is_symlink() and p.resolve() != output and selected(p, args.mode)
    ]
    manifest = {
        "schema_version": 1,
        "mode": args.mode,
        "files": {
            str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in files
        },
        "scope": (
            "Reader: offline lookup and retained evidence. "
            "Research adds cached exact inputs for rebuilding."
        ),
    }
    output.parent.mkdir(parents=True, exist_ok=True)
    with (
        output.open("wb") as raw,
        gzip.GzipFile(fileobj=raw, mode="wb", filename="", mtime=0) as zipped,
        tarfile.open(fileobj=zipped, mode="w") as archive,
    ):
        for path in files:
            info = tarfile.TarInfo(str(path.relative_to(ROOT)))
            data = path.read_bytes()
            info.size = len(data)
            info.mode = 0o644
            archive.addfile(info, io.BytesIO(data))
        data = (json.dumps(manifest, indent=2, sort_keys=True) + "\n").encode()
        info = tarfile.TarInfo("BUNDLE_MANIFEST.json")
        info.size = len(data)
        info.mode = 0o644
        archive.addfile(info, io.BytesIO(data))
    sys.stdout.write(
        json.dumps(
            {
                "mode": args.mode,
                "files": len(files),
                "bytes": output.stat().st_size,
                "sha256": hashlib.sha256(output.read_bytes()).hexdigest(),
            }
        )
        + "\n"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
