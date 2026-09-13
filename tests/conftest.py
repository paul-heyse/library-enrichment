"""Shared pytest configuration.

The autouse fixture below is load-bearing: it guarantees no test resolves the real XDG service
directories. Without it, a directory-policy bug would quietly write into the developer's home
instead of failing a test, and acceptance gate C20's premise -- that the service never touches
state it was not pointed at -- would go unproven on the Python side.
"""

from __future__ import annotations

import os
from collections.abc import Iterator
from pathlib import Path

import pytest

# Environment variables that redirect the service's directory resolver.
_STATE_VARS = ("LIBENR_HOME", "LIBENR_CACHE_HOME", "LIBENR_DATA_HOME", "LIBENR_CONFIG")


def _real_xdg_paths() -> list[Path]:
    cache = Path(os.environ.get("XDG_CACHE_HOME", Path.home() / ".cache"))
    data = Path(os.environ.get("XDG_DATA_HOME", Path.home() / ".local/share"))
    return [cache / "library-enrichment", data / "library-enrichment"]


def _digest(paths: list[Path]) -> list[tuple[str, int]]:
    """Cheap structural digest: every entry under each path, with its size."""
    out: list[tuple[str, int]] = []
    for root in paths:
        if not root.exists():
            out.append((str(root), -1))
            continue
        for p in sorted(root.rglob("*")):
            try:
                out.append((str(p), p.stat().st_size if p.is_file() else -2))
            except OSError:
                out.append((str(p), -3))
    return out


@pytest.fixture(autouse=True, scope="session")
def isolate_service_state(tmp_path_factory: pytest.TempPathFactory) -> Iterator[Path]:
    """Point the service's state directories at a temporary root, and prove it held.

    Fails the session if the real XDG service paths changed while tests ran.
    """
    root = tmp_path_factory.mktemp("libenr-state")
    before = _digest(_real_xdg_paths())

    previous = {k: os.environ.get(k) for k in _STATE_VARS}
    os.environ["LIBENR_HOME"] = str(root)
    os.environ["LIBENR_CACHE_HOME"] = str(root / "cache")
    os.environ["LIBENR_DATA_HOME"] = str(root / "data")
    os.environ["LIBENR_TEST_ROOT"] = str(root)

    try:
        yield root
    finally:
        for key, value in previous.items():
            if value is None:
                os.environ.pop(key, None)
            else:
                os.environ[key] = value
        os.environ.pop("LIBENR_TEST_ROOT", None)

    after = _digest(_real_xdg_paths())
    if before != after:
        changed = set(map(str, after)) ^ set(map(str, before))
        raise AssertionError(
            "Tests modified the real XDG service directories. Development state belongs in "
            "$LIBENR_HOME; something resolved the production directory policy instead of the "
            f"configured root. Changed entries: {sorted(changed)[:10]}"
        )
