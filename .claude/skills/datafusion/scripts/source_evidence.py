#!/usr/bin/env python3
"""Extract selected files from an exact cached .crate, retaining archive and file digests."""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
import tarfile
from pathlib import Path, PurePosixPath

ROOT = Path(__file__).resolve().parents[1]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--package", required=True)
    parser.add_argument("--version", required=True)
    parser.add_argument(
        "--file", action="append", required=True, help="Archive-relative path; repeatable"
    )
    parser.add_argument("--cache", type=Path, default=ROOT / "build/.cache/crates")
    args = parser.parse_args()
    stem = f"{args.package}-{args.version}"
    if "/" in stem or ".." in stem:
        parser.error("Invalid package/version")
    archive = args.cache / f"{stem}.crate"
    destination = ROOT / "skill_improvement/evidence/implementation/source-exports" / stem
    destination.mkdir(parents=True, exist_ok=True)
    records = []
    with tarfile.open(archive) as source:
        names = {m.name: m for m in source.getmembers()}
        requested = set(args.file) | {"Cargo.toml", ".cargo_vcs_info.json"}
        requested |= {
            n.removeprefix(stem + "/")
            for n in names
            if n.startswith(stem + "/") and PurePosixPath(n).name.startswith(("LICENSE", "NOTICE"))
        }
        for relative in sorted(requested):
            path = PurePosixPath(relative)
            if path.is_absolute() or ".." in path.parts:
                parser.error("Files must be safe archive-relative paths")
            member = names.get(f"{stem}/{relative}")
            if member is None:
                if relative in args.file:
                    parser.error(f"Missing file: {relative}")
                continue
            if not member.isfile():
                parser.error(f"Not an ordinary file: {relative}")
            stream = source.extractfile(member)
            if stream is None:
                raise ValueError(f"Cannot read {member.name}")
            data = stream.read()
            output = destination / relative
            output.parent.mkdir(parents=True, exist_ok=True)
            output.write_bytes(data)
            records.append(
                {"path": relative, "sha256": hashlib.sha256(data).hexdigest(), "size": len(data)}
            )
    manifest = {
        "package": args.package,
        "version": args.version,
        "archive_sha256": hashlib.sha256(archive.read_bytes()).hexdigest(),
        "origin": f"https://crates.io/api/v1/crates/{args.package}/{args.version}/download",
        "files": records,
        "scope": "Selected source, not the entire archive. Missing cfg evidence remains unknown.",
    }
    (destination / "SOURCE_MANIFEST.json").write_text(json.dumps(manifest, indent=2) + "\n")
    sys.stdout.write(json.dumps(manifest, indent=2) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
