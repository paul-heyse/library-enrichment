#!/usr/bin/env python3
"""Create deterministic portable reader/research archives without compiler targets or host files."""

from __future__ import annotations

import argparse
import gzip
import hashlib
import io
import json
import re
import sys
import tarfile
from pathlib import Path
from urllib.parse import unquote

ROOT = Path(__file__).resolve().parents[1]


def selected(path: Path, mode: str) -> bool:
    relative = path.relative_to(ROOT)
    parts = relative.parts
    # A bundle cannot contain its own later hash/trace receipt. Keep distribution
    # qualification beside the archives, separate from bundled reference evidence.
    if relative.as_posix() in {
        "skill_improvement/evidence/implementation/qualification.json",
        "skill_improvement/evidence/implementation/transfer-qualification.json",
        "skill_improvement/evidence/implementation/qualification-run.log",
        "skill_improvement/evidence/implementation/research-rebuild.log",
        "skill_improvement/evidence/implementation/research-syscalls.log",
        "skill_improvement/evidence/implementation/transfer-syscalls.log",
    }:
        return False
    if any(
        p in {"__pycache__", ".ruff_cache", ".git", "target", "bundles"}
        or p.startswith(
            (
                ".build-target",
                ".evaluation",
                ".qualification",
                ".research-qualification",
                ".compile-fail",
            )
        )
        for p in parts
    ):
        return False
    if ".cache" in parts:
        return mode == "research" and parts[:2] == ("build", ".cache")
    if parts[0] == "build":
        return (
            mode == "research"
            or relative.as_posix() == "build/manifests/deltalake.json"
            or (
                len(parts) == 2
                and parts[1] in {"test_contracts.py", "fetch.py", "inputs.lock.json"}
            )
        )
    if parts[0] in {"content", "queries", "scripts", "authoring"}:
        return True
    if len(parts) == 1:
        return path.suffix == ".md" or path.name.startswith("LICENSE")
    if relative.as_posix() == "skill_improvement/IMPLEMENTATION_REPORT.md":
        return True
    if mode == "research":
        return parts[0] == "skill_improvement"
    name = relative.as_posix()
    prefix = "skill_improvement/evidence/"
    if not name.startswith(prefix):
        return False
    tail = name.removeprefix(prefix)
    if tail.startswith(("sources/", "queries/")):
        return True
    if tail in {"source-manifest.json", "acquisition.Cargo.lock"}:
        return True
    if tail.startswith("implementation/"):
        leaf = tail.removeprefix("implementation/")
        return leaf.startswith(("probes/", "logs/", "compile-fail/")) or leaf in {
            "probe-results.json",
            "runtime-profile.json",
            "additional-sources.json",
            "compile-fail-results.json",
            "source-controls.json",
        }
    return False


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=["reader", "research"])
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    files = {
        p
        for p in sorted(ROOT.rglob("*"))
        if p.is_file() and not p.is_symlink() and p.resolve() != output and selected(p, args.mode)
    }
    # Preserve local document navigation without making reader use depend on the
    # research archive. Follow existing authored links, never external/hidden data.
    pending = [p for p in files if p.suffix == ".md"]
    while pending:
        document = pending.pop()
        body = re.sub(r"```.*?```", "", document.read_text(), flags=re.S)
        for target in re.findall(r"\[[^\]\n]*\]\(([^)\s]+)\)", body):
            if re.match(r"(?:https?|mailto|app):", target) or target.startswith("#"):
                continue
            linked = (document.parent / unquote(target.split("#", 1)[0].strip("<>"))).resolve()
            if not linked.is_relative_to(ROOT) or linked in files or not linked.is_file():
                continue
            if not selected(linked, "research"):
                continue
            relative = linked.relative_to(ROOT)
            if any(p.startswith(".") for p in relative.parts) or "bundles" in relative.parts:
                continue
            if "target" in relative.parts or linked.suffix in {".gz", ".zst"}:
                continue
            if relative.parts[0] == "build" and not selected(linked, args.mode):
                continue
            files.add(linked)
            if linked.suffix == ".md":
                pending.append(linked)
    files = sorted(files)
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
