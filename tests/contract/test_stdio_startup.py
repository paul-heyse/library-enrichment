"""Acceptance gate C14, stdio-subprocess leg: the claim is measured, not asserted.

    C14 | Tool initializes a stdio connection
       | Tool list appears without package fetch, build, or LSP startup.

The in-process tests in ``test_mcp_catalog.py`` prove the catalog is *registered*. They cannot
prove it is registered *cheaply*, because an in-process server shares this interpreter. So these
tests run a real ``library-enrichment-mcp`` subprocess over real stdio and check three things
that a fetch, a build, or an LSP start would each violate:

1. **No child processes.** Cargo, rustc, rust-analyzer, ty and uv are all subprocesses. Read
   from ``/proc/<pid>/task/*/children`` while the process is alive.
2. **No service-state writes.** A download or an unpack lands under ``$LIBENR_CACHE_HOME`` or
   ``$LIBENR_DATA_HOME``; both are digested before and after. This is the same technique
   ``scripts/state-leak-check.sh`` and ``tests/conftest.py`` use for gate C20.
3. **Promptness.** Blueprint §2.2 requires the catalog "promptly"; the budget is
   ``limits.inline_wait_seconds`` from the frozen ``config/service.example.toml``.

The daemon is deliberately **not** running, which also exercises the Phase 0 gate's other
clause: startup must not depend on it.
"""

from __future__ import annotations

import hashlib
import os
import sys
import time
from pathlib import Path

import pytest
from fastmcp import Client
from fastmcp.client.transports import StdioTransport

from enrichment_mcp.server import TOOL_NAMES

ROOT = Path(__file__).resolve().parents[2]

#: `limits.inline_wait_seconds` from the frozen config/service.example.toml.
INLINE_WAIT_SECONDS = 2.0

pytestmark = pytest.mark.skipif(
    not sys.platform.startswith("linux"),
    reason="process-tree inspection uses /proc, which is Linux-only",
)


def _descendants(pid: int) -> set[int]:
    """Every child of `pid`, read from procfs.

    Empty means the process spawned nothing -- which is the whole point of C14.
    """
    found: set[int] = set()
    task_dir = Path(f"/proc/{pid}/task")
    if not task_dir.exists():
        return found
    for task in task_dir.iterdir():
        children = task / "children"
        try:
            raw = children.read_text()
        except (OSError, FileNotFoundError):
            continue
        found.update(int(value) for value in raw.split())
    return found


def _digest_tree(root: Path) -> str:
    """A structural digest of a directory: every path plus its size.

    Content is not hashed -- a fetch or an unpack changes the *shape* of the tree, and this
    stays fast enough to run around a subprocess.
    """
    if not root.exists():
        return "absent"
    entries = []
    for path in sorted(root.rglob("*")):
        try:
            entries.append(f"{path.relative_to(root)}:{path.stat().st_size}")
        except OSError:
            entries.append(f"{path.relative_to(root)}:?")
    return hashlib.sha256("\n".join(entries).encode()).hexdigest()


@pytest.fixture
def adapter_env(tmp_path: Path) -> dict[str, str]:
    """A clean environment pointing service state at a private sandbox.

    Nothing here resolves into a working repository or a real XDG path -- blueprint §2.3, and
    the premise gate C20 proves.
    """
    env = dict(os.environ)
    env.update(
        {
            "LIBENR_HOME": str(tmp_path / "state"),
            "LIBENR_CACHE_HOME": str(tmp_path / "state" / "cache"),
            "LIBENR_DATA_HOME": str(tmp_path / "state" / "data"),
            # A socket that deliberately does not exist: startup must not need the daemon.
            "LIBENR_SOCKET": str(tmp_path / "state" / "run" / "absent.sock"),
            "PYTHONPATH": str(ROOT / "python"),
        }
    )
    return env


def _transport(env: dict[str, str]) -> StdioTransport:
    """Run the adapter the way a client would: a real process over real stdio."""
    return StdioTransport(
        command=sys.executable,
        args=["-m", "enrichment_mcp"],
        env=env,
        cwd=str(ROOT),
    )


async def test_the_tool_list_appears_over_real_stdio(adapter_env: dict[str, str]) -> None:
    async with Client(_transport(adapter_env)) as client:
        tools = await client.list_tools()
    assert {tool.name for tool in tools} == set(TOOL_NAMES)


async def test_listing_tools_starts_no_subprocess(adapter_env: dict[str, str]) -> None:
    """A fetch, a compile, or an LSP start would each appear here as a grandchild process.

    The adapter is spawned as a child of this test process, so it can be identified by diffing
    our own children -- no dependency on FastMCP transport internals, which expose no PID.
    """
    before = _descendants(os.getpid())

    transport = _transport(adapter_env)
    async with Client(transport) as client:
        await client.list_tools()
        adapters = _descendants(os.getpid()) - before
        grandchildren = {pid: _descendants(pid) for pid in adapters}

    assert adapters, "the stdio transport should have spawned an adapter process"
    spawned = {pid: kids for pid, kids in grandchildren.items() if kids}
    assert not spawned, (
        f"the adapter spawned {spawned} while listing tools; C14 requires the catalog to "
        f"appear without a package fetch, a build, or an LSP start"
    )


async def test_listing_tools_writes_no_service_state(adapter_env: dict[str, str]) -> None:
    """A download or an unpack would change the shape of the cache or data tree."""
    cache = Path(adapter_env["LIBENR_CACHE_HOME"])
    data = Path(adapter_env["LIBENR_DATA_HOME"])
    before = (_digest_tree(cache), _digest_tree(data))

    async with Client(_transport(adapter_env)) as client:
        await client.list_tools()

    after = (_digest_tree(cache), _digest_tree(data))
    assert before == after, "listing tools must not touch service state"


async def test_the_catalog_appears_within_the_inline_budget(
    adapter_env: dict[str, str],
) -> None:
    """Blueprint §2.2: "Normal stdio startup must register the tool catalog promptly"."""
    started = time.monotonic()
    async with Client(_transport(adapter_env)) as client:
        await client.list_tools()
    elapsed = time.monotonic() - started

    assert elapsed < INLINE_WAIT_SECONDS, (
        f"startup plus tools/list took {elapsed:.2f}s, over the "
        f"{INLINE_WAIT_SECONDS}s inline budget"
    )


async def test_service_status_works_over_stdio_with_no_daemon(
    adapter_env: dict[str, str],
) -> None:
    """The Phase 0 gate's other clause, through the real transport.

    An absent daemon is reported as absent. It is not a failed tool call, and startup did not
    depend on it.
    """
    async with Client(_transport(adapter_env)) as client:
        result = await client.call_tool("service_status", {})

    payload = result.structured_content
    assert isinstance(payload, dict)
    assert payload["status"] == "partial"
    assert payload["data"]["daemon"]["available"] is False
    assert payload["data"]["adapter"]["available"] is True


async def test_nothing_is_written_to_protocol_stdout(adapter_env: dict[str, str]) -> None:
    """stdout carries MCP frames only.

    If the adapter wrote a log line to stdout, the client's own JSON-RPC parsing would break --
    so a completed handshake plus a well-formed result is the assertion. A stray ``print()``
    during startup makes this test fail rather than corrupt a user's session silently.
    """
    async with Client(_transport(adapter_env)) as client:
        tools = await client.list_tools()
        result = await client.call_tool("service_status", {})

    # Both a list and a call completed over the same stdio pipe, so every byte the client read
    # parsed as a JSON-RPC frame. FastMCP's own startup banner goes to stderr, where it belongs.
    assert len(tools) == len(TOOL_NAMES)
    assert isinstance(result.structured_content, dict)
    assert result.structured_content["schema_version"] == "5.0"
