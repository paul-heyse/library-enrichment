"""Actual socket protocol violations; these are unit checks, not client acceptance gates."""

from __future__ import annotations

import asyncio
import json
from collections.abc import AsyncIterator
from contextlib import asynccontextmanager
from pathlib import Path

import pytest

from enrichment_mcp.daemon_client import RPC_MESSAGE_BYTES, DaemonClient, DaemonUnavailableError


@asynccontextmanager
async def peer(path: Path, response: bytes) -> AsyncIterator[DaemonClient]:
    """Run a real one-exchange socket peer and wait for its connection to close."""
    finished = asyncio.Event()

    async def exchange(reader: asyncio.StreamReader, writer: asyncio.StreamWriter) -> None:
        try:
            await reader.readuntil(b"\n")
            writer.write(response)
            await writer.drain()
        except ConnectionError:
            # An over-limit receiver closes before consuming the deliberately invalid frame.
            pass
        finally:
            writer.close()
            await asyncio.gather(writer.wait_closed(), return_exceptions=True)
            finished.set()

    async with await asyncio.start_unix_server(exchange, path) as server:
        try:
            yield DaemonClient(path)
        finally:
            await asyncio.wait_for(finished.wait(), 2)
            server.close()
            await server.wait_closed()


@pytest.mark.asyncio
async def test_reply_above_default_stream_limit_is_read(tmp_path: Path) -> None:
    expected = {"jsonrpc": "2.0", "id": 1, "result": {"text": "a" * 100_000}}
    async with peer(tmp_path / "d.sock", json.dumps(expected).encode() + b"\n") as client:
        assert await client.call("service.status") == expected


@pytest.mark.asyncio
@pytest.mark.parametrize(
    ("frame", "cause"),
    [
        (b"", "incomplete_frame"),
        (b'{"jsonrpc":"2.0","id":1,"result":{}}', "incomplete_frame"),
        (b"x" * (RPC_MESSAGE_BYTES + 2), "response_limit"),
        (b"x" * (RPC_MESSAGE_BYTES + 1) + b"\n", "response_limit"),
        (b"{\n", "protocol"),
        (b"\xff\n", "protocol"),
        (b"[]\n", "protocol"),
        (b'{"jsonrpc":"1.0","id":1,"result":{}}\n', "protocol"),
        (b'{"jsonrpc":"2.0","id":2,"result":{}}\n', "protocol"),
        (b'{"jsonrpc":"2.0","id":true,"result":{}}\n', "protocol"),
        (b'{"jsonrpc":"2.0","id":1}\n', "protocol"),
        (b'{"jsonrpc":"2.0","id":1,"result":{},"error":{}}\n', "protocol"),
        (b'{"jsonrpc":"2.0","id":1,"error":null}\n', "protocol"),
        (b'{"jsonrpc":"2.0","id":1,"error":{"code":true,"message":"bad"}}\n', "protocol"),
    ],
)
async def test_protocol_failure_has_precise_origin(
    tmp_path: Path, frame: bytes, cause: str
) -> None:
    async with peer(tmp_path / "d.sock", frame) as client:
        with pytest.raises(DaemonUnavailableError) as error:
            await client.call("service.status")
        assert error.value.cause == cause
