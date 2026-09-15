"""Start and stop the real daemon binary for the contract and e2e tiers.

Shared by the tests that drive a real `library-enrichmentd` over a real socket, so the way a
daemon is started, waited for and stopped is written once. Nothing here mocks anything.
"""

from __future__ import annotations

import os
import socket
import subprocess
import sys
import threading
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
        from support.execution import complete_fixture_configuration

        complete_fixture_configuration(config_path)
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


class Running(dict[str, str]):
    """The daemon's environment, plus whatever it has written to stderr so far.

    A `dict` subclass so every existing `with daemon.running(env):` keeps working and the
    yielded value is still usable as an environment. The daemon writes one structured line per
    request to stderr (§14.3), which is both worth asserting on and, unread, enough to fill a
    64 KiB pipe buffer and wedge the process mid-test -- so it is drained continuously rather
    than at exit.
    """

    #: Everything the daemon has written to stderr, drained live on a background thread.
    log: str = ""


@contextmanager
def running(env: dict[str, str], cwd: Path | None = None) -> Iterator[Running]:
    """A real daemon under `env`, stopped on exit.

    `cwd` exists for gate C20: starting the daemon *inside* a canary repository is the sharpest
    form of the working-repository boundary test, because a relative path anywhere in the
    service would then land in the canary rather than somewhere harmless.
    """
    process = subprocess.Popen(  # a first-party binary at a known path
        [str(DAEMON_BIN), "start"],
        env=env,
        cwd=str(cwd) if cwd else None,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.PIPE,
    )
    handle = Running(env)

    def drain() -> None:
        # `await_socket` reads this same pipe when startup fails, which is why the drain only
        # starts once the socket is up: two readers on one pipe would race for the same bytes.
        if process.stderr is None:
            return
        for line in iter(process.stderr.readline, b""):
            handle.log += line.decode(errors="replace")

    reader: threading.Thread | None = None
    try:
        await_socket(Path(env["LIBENR_SOCKET"]), process)
        reader = threading.Thread(target=drain, daemon=True)
        reader.start()
        yield handle
    finally:
        subprocess.run(  # same first-party binary
            [str(DAEMON_BIN), "stop"], env=env, capture_output=True, check=False, timeout=10
        )
        try:
            process.wait(timeout=10)
        except subprocess.TimeoutExpired:
            process.kill()
            process.wait(timeout=10)
        # Let the drain finish the tail the daemon wrote on its way out, so a test that asserts
        # on shutdown diagnostics sees them. Bounded: the pipe closes when the process does.
        if reader is not None:
            reader.join(timeout=5)
        # Preserve daemon failures beside this fixture's state so a closed RPC connection has
        # an inspectable cause after the context manager has reaped its process.
        Path(env["LIBENR_HOME"]).joinpath("daemon-test.log").write_text(handle.log)


def transport(env: dict[str, str], cwd: Path | None = None) -> StdioTransport:
    """The adapter as a real stdio subprocess, launched as `python -m enrichment_mcp`."""
    return StdioTransport(
        command=sys.executable,
        args=["-m", "enrichment_mcp"],
        env=env,
        cwd=str(cwd or ROOT),
    )


async def read_complete_answer(client, result):
    """Follow the real service's overflow artifact protocol without inflating inline results."""
    error = result.get("error")
    if not isinstance(error, dict) or error.get("code") != "BUDGET_EXCEEDED":
        return result
    artifact_id = result.get("data", {}).get("result_artifact_id")
    if artifact_id is None:
        return result
    import json

    cursor = None
    parts = []
    while True:
        page = (
            await client.call_tool("read_artifact", {"artifact_id": artifact_id, "cursor": cursor})
        ).structured_content
        assert page["status"] == "ok", page
        assert page["data"]["encoding"] == "utf8"
        parts.append(page["data"]["content"])
        cursor = page["pagination"]["next_cursor"]
        if cursor is None:
            break
    return json.loads("".join(parts))


async def wait_for_answer(client, result, timeout: float = 300.0):
    """Wait on the real durable job, retaining the original caller's sharing disclosure."""
    deadline = time.monotonic() + timeout
    shared = [
        note
        for note in result.get("coverage", {}).get("limitations", [])
        if "already in flight" in note
    ]
    while result.get("status") == "pending":
        assert time.monotonic() < deadline, result
        job_id = result["job"]["job_id"]
        response = (
            await client.call_tool(
                "job_control",
                {
                    "job_id": job_id,
                    "action": "wait",
                    "wait_seconds": 10,
                },
            )
        ).structured_content
        response = await read_complete_answer(client, response)
        if response["status"] == "error":
            return response
        data = response["data"]
        if data.get("result") is not None:
            result = data["result"]
            break
        assert data["state"] in {"queued", "running", "cancel_requested"}, response
    result = await read_complete_answer(client, result)
    for note in shared:
        if note not in result["coverage"]["limitations"]:
            result["coverage"]["limitations"].append(note)
    return result
