"""The Rust and Python socket resolvers must agree on every branch.

`crates/enrichment-daemon/src/paths.rs` and `python/enrichment_mcp/daemon_client.py` each
implement the resolution order ADR 0006 records. That duplication is deliberate — the adapter
must not shell out to the daemon on its call path just to learn where the daemon is — but it
means the two can drift.

A drift here is quiet and nasty: the adapter would report the daemon as unavailable while the
daemon sat listening on a different path, and `service_status` would truthfully describe a
service that was actually running. So every branch is compared against the real binary via
`library-enrichmentd socket-path`, which is the Rust resolver's own answer.

An earlier revision claimed `tests/e2e/test_mcp_daemon_handshake.py` was the divergence
detector. It was not: that fixture always sets `LIBENR_HOME`, so it exercised one branch of five
and would never have noticed the others.
"""

from __future__ import annotations

import os
import subprocess
from pathlib import Path

import pytest

from enrichment_mcp.daemon_client import socket_path

ROOT = Path(__file__).resolve().parents[2]
DAEMON_BIN = ROOT / "target/debug/library-enrichmentd"

#: The five sources, in the order ADR 0006 fixes, each with an absolute value.
BRANCHES: list[tuple[str, dict[str, str]]] = [
    ("libenr_socket", {"LIBENR_SOCKET": "/tmp/libenr-test/explicit.sock"}),
    ("libenr_home", {"LIBENR_HOME": "/tmp/libenr-test/state"}),
    ("xdg_runtime_dir", {"XDG_RUNTIME_DIR": "/tmp/libenr-test/run"}),
    ("xdg_cache_home", {"XDG_CACHE_HOME": "/tmp/libenr-test/cache"}),
    ("home", {"HOME": "/tmp/libenr-test/home"}),
]

#: Every variable either resolver consults. Cleared before each case so precedence is real.
_ALL = ("LIBENR_SOCKET", "LIBENR_HOME", "XDG_RUNTIME_DIR", "XDG_CACHE_HOME", "HOME")

pytestmark = pytest.mark.skipif(
    not DAEMON_BIN.exists(),
    reason=f"{DAEMON_BIN} is not built; run `cargo build -p enrichment-daemon`",
)


def _rust_resolution(env: dict[str, str]) -> str:
    """Ask the Rust resolver directly."""
    result = subprocess.run(  # a first-party binary at a known path
        [str(DAEMON_BIN), "socket-path"],
        env=env,
        capture_output=True,
        check=True,
        timeout=30,
    )
    return result.stdout.decode().strip()


def _python_resolution(env: dict[str, str], monkeypatch: pytest.MonkeyPatch) -> str:
    for key in _ALL:
        monkeypatch.delenv(key, raising=False)
    for key, value in env.items():
        if key in _ALL:
            monkeypatch.setenv(key, value)
    return str(socket_path())


@pytest.mark.parametrize(("name", "overrides"), BRANCHES, ids=[n for n, _ in BRANCHES])
def test_both_resolvers_agree(
    name: str, overrides: dict[str, str], monkeypatch: pytest.MonkeyPatch
) -> None:
    base = {k: v for k, v in os.environ.items() if k not in _ALL}
    env = {**base, **overrides}

    rust = _rust_resolution(env)
    python = _python_resolution(env, monkeypatch)

    assert rust == python, (
        f"the `{name}` branch resolves differently in Rust and Python: {rust!r} vs {python!r}. "
        f"Both implement ADR 0006's order; fix whichever drifted."
    )
    assert Path(rust).is_absolute(), "a relative socket path may land in a repository under study"


def test_precedence_is_the_same_in_both(monkeypatch: pytest.MonkeyPatch) -> None:
    """With every source set at once, both must pick the same one.

    Agreeing branch-by-branch is not enough — two resolvers can each handle all five sources and
    still order them differently.
    """
    overrides = {k: v for _, o in BRANCHES for k, v in o.items()}
    base = {k: v for k, v in os.environ.items() if k not in _ALL}
    env = {**base, **overrides}

    rust = _rust_resolution(env)
    assert rust == _python_resolution(env, monkeypatch)
    assert rust == "/tmp/libenr-test/explicit.sock", "LIBENR_SOCKET outranks everything"


def test_the_rust_resolver_refuses_a_relative_path() -> None:
    """Python has no equivalent guard, so this pins the boundary that actually enforces it.

    The daemon is what creates the socket, so its refusal is the one that keeps a live socket out
    of a repository under study (§2.3, gate C20).
    """
    base = {k: v for k, v in os.environ.items() if k not in _ALL}
    result = subprocess.run(  # a first-party binary at a known path
        [str(DAEMON_BIN), "socket-path"],
        env={**base, "LIBENR_SOCKET": "relative/d.sock"},
        capture_output=True,
        check=False,
        timeout=30,
    )
    assert result.returncode != 0
    assert b"absolute" in result.stderr, result.stderr


def test_an_empty_environment_is_refused() -> None:
    """No source set means no private directory, which is an error rather than a guess."""
    base = {k: v for k, v in os.environ.items() if k not in _ALL}
    result = subprocess.run(  # a first-party binary at a known path
        [str(DAEMON_BIN), "socket-path"],
        env=base,
        capture_output=True,
        check=False,
        timeout=30,
    )
    assert result.returncode != 0
    assert b"private runtime directory" in result.stderr, result.stderr
