"""A real PyPI distribution, source inspection, and offline pinned replay through MCP."""

import sys

import pytest
from fastmcp import Client

from support import daemon

pytestmark = [pytest.mark.live, daemon.requires_daemon_binary]


async def test_real_python_distribution_cold_then_offline(tmp_path):
    config = tmp_path / "service.toml"
    config.write_text(
        'config_version = "1.0"\n[policy]\nenabled_profiles = ["static"]\n'
        f'[producers.python]\nworker_python = "{sys.executable}"\n'
    )
    env = daemon.daemon_env(tmp_path / "state", config)
    with daemon.running(env):
        async with Client(daemon.transport(env)) as client:
            resolved = (
                await client.call_tool(
                    "resolve_library",
                    {
                        "ecosystem": "python",
                        "name": "packaging",
                        "python_version": "3.14",
                    },
                )
            ).structured_content
            resolved = await daemon.read_complete_answer(client, resolved)
            resolved = await daemon.wait_for_answer(client, resolved)
            assert resolved["status"] in {"ok", "partial"}, resolved
            version = resolved["data"]["release"]["version"]
            context, snapshot = resolved["context_id"], resolved["snapshot_id"]
            inspected = (
                await client.call_tool(
                    "inspect_symbol",
                    {
                        "context_id": context,
                        "snapshot_id": snapshot,
                        "symbol_path": "packaging.version.Version",
                        "depth": "source",
                    },
                )
            ).structured_content
            inspected = await daemon.read_complete_answer(client, inspected)
            assert inspected["data"]["symbol"]["kind"] == "class", inspected
            assert inspected["data"]["source"]["text"]
            assert resolved["data"]["python"]["worker_artifact_id"]
    # Keep the source registry identity unchanged. The controlled upstream-down fixture
    # separately proves that offline replay succeeds when that endpoint is unavailable.
    with daemon.running(env):
        async with Client(daemon.transport(env)) as client:
            replay = (
                await client.call_tool(
                    "resolve_library",
                    {
                        "ecosystem": "python",
                        "name": "packaging",
                        "version": version,
                        "python_version": "3.14",
                        "mode": "upstream",
                        "freshness": "offline",
                    },
                )
            ).structured_content
            replay = await daemon.read_complete_answer(client, replay)
            assert replay["context_id"] == context
            assert replay["snapshot_id"] == snapshot
            assert replay["data"]["answered_from_cache"]
            again = (
                await client.call_tool(
                    "inspect_symbol",
                    {
                        "context_id": context,
                        "snapshot_id": snapshot,
                        "symbol_path": "packaging.version.Version",
                        "depth": "source",
                    },
                )
            ).structured_content
            again = await daemon.read_complete_answer(client, again)
            assert again["data"] == inspected["data"]
