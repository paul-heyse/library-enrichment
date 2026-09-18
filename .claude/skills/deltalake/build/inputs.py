"""Verify retained raw inputs before any offline generation."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path


def verify(root: Path) -> dict:
    lock = json.loads((root / "build/inputs.lock.json").read_text())
    for name, expected in lock["files"].items():
        path = root / name
        if not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            raise ValueError(
                f"Captured input changed or missing: {name}; acquire/review explicitly"
            )
    return {
        "state": "passed",
        "inputs": len(lock["files"]),
        "protocol_identity": lock["protocol_identity"],
    }
