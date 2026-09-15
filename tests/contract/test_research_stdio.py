"""Real fixture acquisition through raw MCP frames, including durable result retrieval."""

from __future__ import annotations

import asyncio
import hashlib
import json
import os
import sys
import tempfile
from contextlib import asynccontextmanager
from pathlib import Path

from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[2]
BIN = ROOT / "target/debug/library-enrichmentd"


class WireClient:
    def __init__(self, process: asyncio.subprocess.Process):
        self.process = process
        self.sequence = 0
        self.schemas: dict[str, dict] = {}
        self.frames: list[bytes] = []

    async def rpc(self, method: str, params: dict) -> dict:
        self.sequence += 1
        request = {"jsonrpc": "2.0", "id": self.sequence, "method": method, "params": params}
        assert self.process.stdin and self.process.stdout
        self.process.stdin.write((json.dumps(request) + "\n").encode())
        await self.process.stdin.drain()
        while True:
            raw = await asyncio.wait_for(self.process.stdout.readline(), 45)
            assert raw.endswith(b"\n"), "MCP must emit complete JSON frames"
            self.frames.append(raw)
            message = json.loads(raw)
            assert message["jsonrpc"] == "2.0"
            if message.get("id") == self.sequence:
                return message

    async def call(self, tool: str, arguments: dict) -> dict:
        response = await self.rpc("tools/call", {"name": tool, "arguments": arguments})
        assert "error" not in response, response
        result = response["result"]
        value = result["structuredContent"]
        Draft202012Validator(self.schemas[tool]).validate(value)
        error = value["status"] == "error" or (
            tool == "job_control"
            and isinstance(value["data"].get("result"), dict)
            and value["data"]["result"]["outcome"] == "error"
        )
        assert result.get("isError", False) is error
        cap = value["delivery"]["limits"]["effective_max_bytes"]
        if cap is not None:
            encoded = json.dumps(value, ensure_ascii=False, separators=(",", ":")).encode()
            assert len(encoded) <= cap
            assert len(self.frames[-1]) <= cap + 1024, "measured complete MCP frame allowance"
        return value

    async def expand(self, value: dict) -> dict:
        if value["delivery"]["mode"] != "artifact":
            return value
        artifact_id = value["delivery"]["artifact_id"]
        chunks = []
        cursor = None
        for _ in range(4096):
            page = await self.call(
                "read_artifact",
                {
                    "artifact_id": artifact_id,
                    "cursor": cursor,
                    "max_bytes": 65536,
                },
            )
            assert page["status"] == "ok", page
            data = page["data"]
            assert data["encoding"] == "utf8"
            chunk = data["content"].encode()
            assert hashlib.sha256(chunk).hexdigest() == data["content_digest"]
            chunks.append(chunk)
            following = data["page"]["next_cursor"]
            if following is None:
                raw = b"".join(chunks)
                assert hashlib.sha256(raw).hexdigest() == data["artifact"]["sha256"]
                document = json.loads(raw)
                assert document["index"]["format"] == "research-result/2"
                return document["result"]
            assert following != cursor and chunk
            cursor = following
        raise AssertionError("bounded fixture result never completed")

    async def finish(self, value: dict) -> dict:
        if value["status"] != "pending":
            return await self.expand(value)
        for _ in range(45):
            poll = await self.call(
                "job_control",
                {
                    "job_id": value["job"]["job_id"],
                    "action": "wait",
                    "wait_seconds": 1,
                },
            )
            terminal = poll["data"].get("result")
            if terminal:
                return await self.expand(poll | {"delivery": terminal["delivery"]})
        raise AssertionError("fixture job never became terminal")


@asynccontextmanager
async def live_service():
    assert BIN.is_file(), "build all three workspace binaries before this contract test"
    with tempfile.TemporaryDirectory(prefix="enr13-") as root_name:
        root = Path(root_name)
        processes = []
        env = dict(os.environ) | {
            "LIBENR_HOME": str(root / "state"),
            "LIBENR_CACHE_HOME": str(root / "state/cache"),
            "LIBENR_DATA_HOME": str(root / "state/data"),
            "LIBENR_SOCKET": str(root / "run/d.sock"),
            "LIBENR_CONFIG": str(root / "service.toml"),
            "PYTHONPATH": str(ROOT / "python"),
        }
        with (root / "process.log").open("wb") as log:
            try:
                upstream = await asyncio.create_subprocess_exec(
                    sys.executable,
                    str(ROOT / "tests/support/fixture_upstream.py"),
                    "--root",
                    str(ROOT / "tests/fixtures/upstream"),
                    cwd=root,
                    stdout=asyncio.subprocess.PIPE,
                    stderr=log,
                )
                processes.append(upstream)
                assert upstream.stdout
                announcement = (await upstream.stdout.readline()).decode().strip()
                assert announcement.startswith("listening on ")
                base = announcement.removeprefix("listening on ")
                (root / "service.toml").write_text(
                    '[policy]\nenabled_profiles = ["static"]\n'
                    "[limits]\ninline_wait_seconds = 0\n"
                    "[producers.rust]\n"
                    f'crates_io_index_url = "{base}/index"\n'
                    f'crates_io_api_url = "{base}/api/v1"\n'
                    f'docs_rs_url = "{base}"\n'
                )
                daemon = await asyncio.create_subprocess_exec(
                    str(BIN),
                    "start",
                    env=env,
                    cwd=root,
                    stdout=log,
                    stderr=log,
                )
                processes.append(daemon)
                for _ in range(200):
                    if Path(env["LIBENR_SOCKET"]).exists():
                        break
                    assert daemon.returncode is None, (root / "process.log").read_text()
                    await asyncio.sleep(0.025)
                assert Path(env["LIBENR_SOCKET"]).exists()
                adapter = await asyncio.create_subprocess_exec(
                    sys.executable,
                    "-m",
                    "enrichment_mcp",
                    env=env,
                    cwd=root,
                    stdin=asyncio.subprocess.PIPE,
                    stdout=asyncio.subprocess.PIPE,
                    stderr=log,
                    limit=1024 * 1024 + 1,
                )
                processes.append(adapter)
                client = WireClient(adapter)
                initialized = await client.rpc(
                    "initialize",
                    {
                        "protocolVersion": "2025-11-25",
                        "capabilities": {},
                        "clientInfo": {"name": "research-contract", "version": "2.0"},
                    },
                )
                assert "result" in initialized, initialized
                assert adapter.stdin
                adapter.stdin.write(b'{"jsonrpc":"2.0","method":"notifications/initialized"}\n')
                await adapter.stdin.drain()
                tools = (await client.rpc("tools/list", {}))["result"]["tools"]
                client.schemas = {tool["name"]: tool["outputSchema"] for tool in tools}
                yield client
            finally:
                for process in reversed(processes):
                    if process.returncode is None:
                        process.terminate()
                    try:
                        await asyncio.wait_for(process.wait(), 10)
                    except TimeoutError:
                        process.kill()
                        await process.wait()


async def test_research_journey_over_raw_stdio_and_actual_native_service():
    async with live_service() as client:
        status = await client.call("service_status", {})
        assert status["status"] == "ok"
        receipt = await client.call(
            "resolve_library", {"ecosystem": "rust", "name": "enr-fixture", "version": "0.2.0"}
        )
        assert receipt["status"] == "pending", receipt
        resolved = await client.finish(receipt)
        assert resolved["status"] == "ok", resolved
        context = resolved["context_id"]
        inspected = await client.call(
            "inspect_symbol",
            {
                "context_id": context,
                "symbol_path": "enr_fixture::Widget",
            },
        )
        inspected = await client.expand(inspected)
        assert inspected["status"] in {"ok", "partial"}, inspected
        assert inspected["data"]["observations"]
        overview = await client.call(
            "library_overview",
            {
                "context_id": context,
                "discovery": [{"kind": "documentation", "cursor": "foreign"}, {"kind": "features"}],
            },
        )
        overview = await client.expand(overview)
        assert overview["status"] == "partial"
        assert overview["data"]["discovery"][0]["state"] == "failed"
        unknown = await client.call(
            "job_control", {"job_id": "job_00000000000000000000000000000000"}
        )
        assert unknown["error"]["diagnostic"]["cause"] == "not_found"
        assert unknown["error"]["retryable"] is False
        rejected = await client.call(
            "resolve_library", {"ecosystem": "rust", "name": "enr-fixture", "version": "9.9.9"}
        )
        failed = await client.finish(rejected)
        assert failed["status"] == "error"
        assert failed["error"]["code"] == "VERSION_NOT_FOUND"
        previews = [
            json.loads(block["text"])
            for frame in client.frames
            for block in json.loads(frame).get("result", {}).get("content", [])
            if block.get("type") == "text" and "research-error-preview/1" in block["text"]
        ]
        portable = next(p for p in previews if p.get("job_id") == rejected["job"]["job_id"])
        assert portable["code"] == failed["error"]["code"]
        assert portable["next_action"] == failed["error"]["next_action"]
        assert portable["read"]["artifact_id"]
        assert portable["retryable"] is False
        old = await client.rpc(
            "tools/call",
            {
                "name": "inspect_symbol",
                "arguments": {"context_id": context, "symbol_path": "Widget", "depth": "signature"},
            },
        )
        assert old.get("error") or old["result"].get("isError")
        resource = await client.rpc("resources/read", {"uri": "library-evidence://workflow"})
        assert resource["result"]["contents"]
