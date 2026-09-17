#!/usr/bin/env python3
"""Refresh local hashes in an existing pinned vendor manifest; preview by default."""

import argparse
import hashlib
import json
import sys
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("vendor", type=Path)
    parser.add_argument("--apply", action="store_true")
    args = parser.parse_args()
    root = args.vendor.resolve(strict=True)
    manifest = root / "PROVENANCE.json"
    data = json.loads(manifest.read_text())
    changes = []
    paths = set()
    for entry in data["files"]:
        relative = Path(entry["path"])
        path = (root / relative).resolve(strict=True)
        if relative.is_absolute() or not path.is_relative_to(root) or relative in paths:
            raise ValueError(f"invalid or duplicate vendor path: {relative}")
        paths.add(relative)
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        if entry["sha256"] != digest:
            changes.append(str(relative))
            entry["sha256"] = digest
    if args.apply:
        manifest.write_text(json.dumps(data, indent=2) + "\n")
    sys.stdout.write(
        json.dumps({"applied": args.apply, "pin": data["pin"], "changed": changes}, indent=2) + "\n"
    )


if __name__ == "__main__":
    main()
