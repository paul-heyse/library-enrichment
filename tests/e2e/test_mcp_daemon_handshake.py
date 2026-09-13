"""The minimal MCP/daemon handshake, end to end.

``AGENT_HANDOFF.md``: "Start with the compatibility matrix and a minimal tested MCP/daemon
handshake" and "do not leave the actual MCP path until the end."

This is the only test that exercises the whole Phase 0 path at once: a real
``library-enrichmentd`` process, a real Unix socket carrying bounded NDJSON-RPC, a real
``library-enrichment-mcp`` subprocess over real stdio, and a real FastMCP client on top. No
layer is mocked -- ``rules/no-mocks-in-acceptance-tiers.yml`` forbids it here, and a mocked
layer would leave the actual boundary untested.

If the daemon binary has not been built, these tests **skip** rather than pass: an unexecuted
test is `not_run`, never a pass. Build it with `cargo build -p enrichment-daemon`.
"""

from __future__ import annotations

import asyncio
import json
import os
import socket
import subprocess
import sys
import time
from collections.abc import Iterator
from pathlib import Path

import pytest
from fastmcp import Client
from fastmcp.client.transports import StdioTransport
from jsonschema import Draft202012Validator

from wire_corpus import wire_corpus

ROOT = Path(__file__).resolve().parents[2]
DAEMON_BIN = ROOT / "target/debug/library-enrichmentd"
GENERATED_SCHEMA = ROOT / "schemas/generated/research-envelope.schema.json"

pytestmark = pytest.mark.skipif(
    not DAEMON_BIN.exists(),
    reason=f"{DAEMON_BIN} is not built; run `cargo build -p enrichment-daemon`",
)


@pytest.fixture
def running_daemon(tmp_path: Path) -> Iterator[dict[str, str]]:
    """A real daemon on a private socket, stopped on teardown.

    State is redirected into `tmp_path`, so this never touches `$LIBENR_HOME` or a real XDG
    path -- blueprint §2.3, the premise gate C20 proves.
    """
    state = tmp_path / "state"
    socket_path = state / "run" / "d.sock"
    env = dict(os.environ)
    env.update(
        {
            "LIBENR_HOME": str(state),
            "LIBENR_CACHE_HOME": str(state / "cache"),
            "LIBENR_DATA_HOME": str(state / "data"),
            # Named explicitly rather than left to LIBENR_HOME: the session-wide isolation
            # fixture in conftest.py sets LIBENR_SOCKET, and that outranks LIBENR_HOME in the
            # resolver, so an inherited value would send this daemon somewhere else entirely.
            "LIBENR_SOCKET": str(socket_path),
            "PYTHONPATH": str(ROOT / "python"),
        }
    )

    process = subprocess.Popen(  # a first-party binary at a known path
        [str(DAEMON_BIN), "start"],
        env=env,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.PIPE,
    )
    try:
        _await_socket(socket_path, process)
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


def _await_socket(path: Path, process: subprocess.Popen[bytes], timeout: float = 15.0) -> None:
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


def _transport(env: dict[str, str]) -> StdioTransport:
    return StdioTransport(
        command=sys.executable, args=["-m", "enrichment_mcp"], env=env, cwd=str(ROOT)
    )


async def test_service_status_traverses_mcp_rpc_and_daemon(
    running_daemon: dict[str, str],
) -> None:
    """MCP stdio -> adapter -> NDJSON-RPC over a Unix socket -> daemon, and back."""
    async with Client(_transport(running_daemon)) as client:
        result = await client.call_tool("service_status", {})

    payload = result.data
    assert isinstance(payload, dict)

    # The daemon answered, so this is a full result rather than the adapter-local fallback.
    assert payload["status"] == "ok"
    assert payload["error"] is None
    assert payload["data"]["versions"]["schema"] == "1.0"

    # And it is still truthful about what is not installed.
    producers = payload["data"]["producers"]
    assert producers, "the daemon must enumerate its producers"
    assert all(not p["available"] for p in producers), (
        "no producer is implemented in phase 0, so none may report itself available"
    )
    assert payload["data"]["health"]["cache_ready"] is False


async def test_the_end_to_end_envelope_conforms_to_the_generated_schema(
    running_daemon: dict[str, str],
) -> None:
    """What crosses the real MCP boundary validates against the authoritative schema.

    `just schema-conformance` checks the schema against fixtures. This checks a live response,
    produced by the real service, against the schema generated from the Rust wire types -- the
    half a fixture corpus cannot reach.
    """
    if not GENERATED_SCHEMA.exists():
        pytest.fail(f"{GENERATED_SCHEMA} is missing. Run `just schemas-generate`.")
    validator = Draft202012Validator(json.loads(GENERATED_SCHEMA.read_text()))

    async with Client(_transport(running_daemon)) as client:
        listed = await client.call_tool("service_status", {})
        unimplemented = await client.call_tool(
            "resolve_library", {"ecosystem": "rust", "name": "serde"}
        )

    for label, result in (("service_status", listed), ("resolve_library", unimplemented)):
        payload = result.data
        assert isinstance(payload, dict)
        errors = list(validator.iter_errors(payload))
        assert not errors, f"{label} emitted a non-conforming envelope: {errors[0].message}"


async def test_a_component_filter_narrows_the_report(running_daemon: dict[str, str]) -> None:
    """A matched filter narrows the report; an unmatched one says so in `coverage`.

    The two must be distinguishable. An unmatched filter returning an empty `ok` with the same
    coverage as a matched one would let a caller read "this build has no such producer" out of
    a result that only means "the filter matched nothing".
    """
    async with Client(_transport(running_daemon)) as client:
        known = await client.call_tool("service_status", {"component": "griffe"})
        unknown = await client.call_tool("service_status", {"component": "no-such-producer"})

    assert isinstance(known.data, dict)
    assert known.data["status"] == "ok"
    assert [p["name"] for p in known.data["data"]["producers"]] == ["griffe"]

    assert isinstance(unknown.data, dict)
    assert unknown.data["status"] == "partial", "an unmatched filter is not a complete answer"
    assert unknown.data["data"]["producers"] == []
    assert unknown.data["coverage"]["missing"], "the gap must be explicit"
    assert any(
        "no-such-producer" in limitation for limitation in unknown.data["coverage"]["limitations"]
    ), "the limitation must name the filter that matched nothing"

    # And the two coverage blocks must not be interchangeable.
    assert known.data["coverage"] != unknown.data["coverage"]


async def _rpc(socket_path: Path, method: str, params: dict[str, object]) -> dict[str, object]:
    """One NDJSON-RPC request over the real socket."""
    reader, writer = await asyncio.open_unix_connection(str(socket_path))
    try:
        request = {"jsonrpc": "2.0", "id": 1, "method": method, "params": params}
        writer.write((json.dumps(request) + "\n").encode())
        await writer.drain()
        line = await reader.readline()
    finally:
        writer.close()
        await asyncio.gather(writer.wait_closed(), return_exceptions=True)
    parsed: dict[str, object] = json.loads(line)
    return parsed


async def test_the_rpc_boundary_agrees_with_the_cli_on_the_shared_corpus(
    running_daemon: dict[str, str],
) -> None:
    """Gate C19's RPC leg, measured against the same documents every other boundary saw.

    `wire.validate` and `library-enrichmentd validate` both call
    `enrichment_daemon::validate::validate`, so this asserts a property the shared
    implementation is meant to guarantee rather than hoping two code paths coincide. A
    divergence would mean the socket and the command line disagree about what conforms.
    """
    corpus = wire_corpus()
    assert corpus, "the shared corpus must not be empty"

    socket_path = Path(running_daemon["LIBENR_SOCKET"])
    disagreements = []

    for name, document, is_valid in corpus:
        response = await _rpc(socket_path, "wire.validate", {"document": document})
        assert response.get("error") is None, f"`{name}`: {response.get('error')}"
        result = response["result"]
        assert isinstance(result, dict)
        rpc_says = bool(result["valid"])

        cli = subprocess.run(  # a first-party binary at a known path
            [str(DAEMON_BIN), "validate"],
            input=document.encode(),
            capture_output=True,
            check=False,
            timeout=30,
        )
        cli_says = bool(json.loads(cli.stdout)["valid"])

        if not (rpc_says == cli_says == is_valid):
            disagreements.append((name, {"rpc": rpc_says, "cli": cli_says, "want": is_valid}))

    assert not disagreements, f"boundaries disagreed: {disagreements}"


async def test_the_configured_rpc_limit_is_the_one_enforced(tmp_path: Path) -> None:
    """A limit set in `LIBENR_CONFIG` reaches the socket, not just `service_status`.

    Reporting a configured value while enforcing a compiled-in one would be worse than not
    reading configuration at all, so this starts a daemon with a deliberately tiny
    `rpc_message_bytes` and checks the wire refuses a frame above it.
    """
    state = tmp_path / "state"
    config = tmp_path / "service.toml"
    config.write_text("[limits]\nrpc_message_bytes = 2048\n")
    socket_path = state / "run" / "d.sock"

    env = dict(os.environ)
    env.update(
        {
            "LIBENR_HOME": str(state),
            "LIBENR_SOCKET": str(socket_path),
            "LIBENR_CONFIG": str(config),
            "PYTHONPATH": str(ROOT / "python"),
        }
    )

    process = subprocess.Popen(  # a first-party binary at a known path
        [str(DAEMON_BIN), "start"],
        env=env,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.PIPE,
    )
    try:
        _await_socket(socket_path, process)

        reader, writer = await asyncio.open_unix_connection(str(socket_path))
        try:
            padding = "x" * 8192
            frame = json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "service.status",
                    "params": {"pad": padding},
                }
            )
            writer.write((frame + "\n").encode())
            await writer.drain()
            line = await reader.readline()
        finally:
            writer.close()
            await asyncio.gather(writer.wait_closed(), return_exceptions=True)

        response = json.loads(line)
        assert response["error"]["data"]["code"] == "BUDGET_EXCEEDED"
        assert "2048" in response["error"]["message"], (
            "the enforced limit must be the configured one, not the built-in default"
        )
    finally:
        subprocess.run(  # same first-party binary
            [str(DAEMON_BIN), "stop"], env=env, capture_output=True, check=False, timeout=10
        )
        try:
            process.wait(timeout=10)
        except subprocess.TimeoutExpired:
            process.kill()
            process.wait(timeout=10)


async def test_the_rpc_validator_rejects_a_malformed_request(
    running_daemon: dict[str, str],
) -> None:
    """`wire.validate` without a document is a typed parameter error, not a crash."""
    socket_path = Path(running_daemon["LIBENR_SOCKET"])
    response = await _rpc(socket_path, "wire.validate", {})

    error = response["error"]
    assert isinstance(error, dict)
    data = error["data"]
    assert isinstance(data, dict)
    assert data["code"] == "UNSUPPORTED_FORMAT"


def test_the_daemon_cli_reports_status_and_stops(running_daemon: dict[str, str]) -> None:
    """`library-enrichmentd status` against a live daemon (blueprint §2.2)."""
    result = subprocess.run(  # a first-party binary at a known path
        [str(DAEMON_BIN), "status"],
        env=running_daemon,
        capture_output=True,
        check=False,
        timeout=10,
    )
    assert result.returncode == 0, result.stderr.decode()
    reported = json.loads(result.stdout)
    # The daemon answers with a full wire envelope, so the status payload sits under `data`.
    assert reported["result"]["status"] == "ok"
    assert reported["result"]["data"]["versions"]["schema"] == "1.0"
