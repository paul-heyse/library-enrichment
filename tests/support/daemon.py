"""Start and stop the real daemon binary for the contract and e2e tiers.

Shared by the tests that drive a real `library-enrichmentd` over a real socket, so the way a
daemon is started, waited for and stopped is written once. Nothing here mocks anything.
"""

from __future__ import annotations

import os
import socket
import subprocess
import sys
import time
from collections.abc import Iterator
from contextlib import contextmanager
from pathlib import Path

import pytest
from fastmcp.client.transports import StdioTransport

ROOT = Path(__file__).resolve().parents[2]
DAEMON_BIN = ROOT / "target/debug/library-enrichmentd"

requires_daemon_binary = pytest.mark.skipif(
    not DAEMON_BIN.exists(),
    reason=f"{DAEMON_BIN} is not built; run `cargo build -p enrichment-daemon`",
)


def daemon_env(state: Path, config_path: Path | None = None) -> dict[str, str]:
    """An environment that confines a daemon's state to `state`.

    `LIBENR_SOCKET` is named explicitly: the session-wide isolation fixture in conftest.py sets
    it too, and it outranks `LIBENR_HOME` in the resolver, so an inherited value would send the
    daemon somewhere else entirely.
    """
    env = dict(os.environ)
    env.update(
        {
            "LIBENR_HOME": str(state),
            "LIBENR_CACHE_HOME": str(state / "cache"),
            "LIBENR_DATA_HOME": str(state / "data"),
            "LIBENR_SOCKET": str(state / "run" / "d.sock"),
            "PYTHONPATH": str(ROOT / "python"),
        }
    )
    if config_path is not None:
        env["LIBENR_CONFIG"] = str(config_path)
    return env


def await_socket(path: Path, process: subprocess.Popen[bytes], timeout: float = 15.0) -> None:
    """Wait for the daemon to bind, failing loudly if it died instead."""
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if process.poll() is not None:
            stderr = process.stderr.read().decode() if process.stderr else ""
            pytest.fail(f"the daemon exited during startup: {stderr}")
        if path.exists():
            probe = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
            try:
                probe.connect(str(path))
                return
            except OSError:
                pass
            finally:
                probe.close()
        time.sleep(0.05)
    pytest.fail(f"the daemon did not bind {path} within {timeout}s")


@contextmanager
def running(env: dict[str, str]) -> Iterator[dict[str, str]]:
    """A real daemon under `env`, stopped on exit."""
    process = subprocess.Popen(  # a first-party binary at a known path
        [str(DAEMON_BIN), "start"],
        env=env,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.PIPE,
    )
    try:
        await_socket(Path(env["LIBENR_SOCKET"]), process)
        yield env
    finally:
        subprocess.run(  # same first-party binary
            [str(DAEMON_BIN), "stop"], env=env, capture_output=True, check=False, timeout=10
        )
        try:
            process.wait(timeout=10)
        except subprocess.TimeoutExpired:
            process.kill()
            process.wait(timeout=10)


def transport(env: dict[str, str], cwd: Path | None = None) -> StdioTransport:
    """The adapter as a real stdio subprocess, launched as `python -m enrichment_mcp`."""
    return StdioTransport(
        command=sys.executable,
        args=["-m", "enrichment_mcp"],
        env=env,
        cwd=str(cwd or ROOT),
    )
