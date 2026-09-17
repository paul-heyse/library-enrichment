"""Opt-in real crates.io/docs.rs acquisition, followed by offline stdio replay."""

import pytest
from fastmcp import Client

from support import daemon

pytestmark = [pytest.mark.live, daemon.requires_daemon_binary]


async def test_real_rust_crate_cold_then_offline(tmp_path):
    state = tmp_path / "state"
    config = tmp_path / "service.toml"
    config.write_text('[policy]\nenabled_profiles = ["static"]\n')
    env = daemon.daemon_env(state, config)
    with daemon.running(env):
        async with Client(daemon.transport(env)) as client:
            resolved = (
                await client.call_tool(
                    "resolve_library",
                    {
                        "ecosystem": "rust",
                        "name": "itoa",
                        "freshness": "revalidate",
                    },
                )
            ).structured_content
            resolved = await daemon.read_complete_answer(client, resolved)
            resolved = await daemon.wait_for_answer(client, resolved)
            if (
                resolved["status"] == "error"
                and resolved["error"]["code"] == "UPSTREAM_UNAVAILABLE"
            ):
                pytest.skip(f"Real registry/docs prerequisite unavailable: {resolved['error']}")
            assert resolved["status"] in {"ok", "partial"}, resolved
            version = resolved["data"]["release"]["version"]
            context = resolved["context_id"]
            snapshot = resolved["snapshot_id"]
            search = (
                await client.call_tool(
                    "search_evidence",
                    {
                        "context_id": context,
                        "snapshot_id": snapshot,
                        "query": "Buffer",
                    },
                )
            ).structured_content
            search = await daemon.read_complete_answer(client, search)
            assert search["data"]["page"]["returned"] > 0
            inspected = (
                await client.call_tool(
                    "inspect_symbol",
                    {
                        "context_id": context,
                        "snapshot_id": snapshot,
                        "symbol_path": "itoa::Buffer",
                        "selection": {
                            "mode": "explicit",
                            "aspects": [
                                {"aspect": name}
                                for name in ("signature", "availability", "documentation", "source")
                            ],
                        },
                    },
                )
            ).structured_content
            inspected = await daemon.read_complete_answer(client, inspected)
            assert inspected["data"]["source"], inspected
            artifact = next(
                a["artifact_id"] for a in resolved["data"]["artifacts"] if a["kind"] == "readme"
            )
            read = (
                await client.call_tool("read_artifact", {"artifact_id": artifact})
            ).structured_content
            assert read["status"] == "ok"
    # Preserve registry identity for exact offline reuse. The controlled upstream-down
    # fixture independently proves offline operation when that same origin is unavailable.
    with daemon.running(env):
        async with Client(daemon.transport(env)) as client:
            replay = (
                await client.call_tool(
                    "resolve_library",
                    {
                        "ecosystem": "rust",
                        "name": "itoa",
                        "version": version,
                        "mode": "upstream",
                        "freshness": "offline",
                    },
                )
            ).structured_content
            assert replay["status"] in {"ok", "partial"}, replay
            assert replay["snapshot_id"] == snapshot
            assert replay["context_id"] == context
            replay = await daemon.read_complete_answer(client, replay)
            assert replay["data"]["answered_from_cache"]
            assert not replay["freshness"]["latest_verified"]
            inspected_again = (
                await client.call_tool(
                    "inspect_symbol",
                    {
                        "context_id": context,
                        "snapshot_id": snapshot,
                        "symbol_path": "itoa::Buffer",
                        "selection": {
                            "mode": "explicit",
                            "aspects": [
                                {"aspect": name}
                                for name in ("signature", "availability", "documentation", "source")
                            ],
                        },
                    },
                )
            ).structured_content
            read_again = (
                await client.call_tool("read_artifact", {"artifact_id": artifact})
            ).structured_content
            inspected_again = await daemon.read_complete_answer(client, inspected_again)
            assert inspected_again["data"] == inspected["data"]
            assert read_again["data"] == read["data"]
