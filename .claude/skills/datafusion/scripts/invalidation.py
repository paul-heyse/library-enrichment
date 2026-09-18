#!/usr/bin/env python3
"""Report changed contract evidence and neighboring crate surfaces without modifying records."""

from __future__ import annotations

import hashlib
import json
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def surface_hashes(root: Path) -> dict[str, str]:
    grouped = defaultdict(list)
    for row in (root / "content/index/operations.tsv").read_text().splitlines():
        fields = row.split("\t")
        grouped[fields[3]].append([fields[0], fields[2], fields[6]])
    return {
        crate: hashlib.sha256(json.dumps(sorted(rows)).encode()).hexdigest()
        for crate, rows in grouped.items()
    }


def main() -> int:
    state = json.loads((ROOT / "content/capabilities/dependencies.json").read_text())
    surfaces = surface_hashes(ROOT)
    changed_crates = sorted(
        c
        for c in set(surfaces) | set(state["crate_surfaces"])
        if surfaces.get(c) != state["crate_surfaces"].get(c)
    )
    affected = {}
    for capability, files in state["records"].items():
        changed = [
            name
            for name, expected in files.items()
            if not (ROOT / name).exists()
            or hashlib.sha256((ROOT / name).read_bytes()).hexdigest() != expected
        ]
        candidate = sorted(set(changed_crates) & set(state["candidate_crates"][capability]))
        if changed or candidate:
            affected[capability] = {
                "changed_dependencies": changed,
                "candidate_set_review_crates": candidate,
            }
    sys.stdout.write(
        json.dumps(
            {
                "affected": affected,
                "changed_crates": changed_crates,
                "new_crates_need_discovery_review": sorted(
                    set(surfaces) - set(state["crate_surfaces"])
                ),
                "scope": (
                    "Dependency-based review hints; "
                    "no claim that unlinked semantic effects are impossible."
                ),
            },
            indent=2,
        )
        + "\n"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
