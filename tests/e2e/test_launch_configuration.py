"""Use the generated launch configuration from an unrelated working directory."""

from __future__ import annotations

import importlib.util
import os
import sys
from pathlib import Path

from fastmcp import Client
from fastmcp.client.transports import StdioTransport

from e2e.test_python_fixture import build_upstream
from support import daemon
from support.fixture_upstream import serve

SCRIPTS = Path(__file__).resolve().parents[2] / "scripts"
sys.path.insert(0, str(SCRIPTS))
SPEC = importlib.util.spec_from_file_location("tested_launch", SCRIPTS / "launch_configuration.py")
assert SPEC and SPEC.loader
launch = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(launch)


async def test_absolute_locked_adapter_daemon_and_worker_from_unrelated_cwd(tmp_path):
    config = tmp_path / "service.toml"
    description = launch.describe(tmp_path / "state", config, "debug")
    command = description["mcpServers"]["library-enrichment"]
    assert all(Path(path).is_absolute() for path in description["native_executables"].values())
    assert "--frozen" in command["args"]
    assert "--no-sync" in command["args"]
    unrelated = tmp_path / "unrelated"
    unrelated.mkdir()
    (unrelated / "unchanged").write_bytes(b"caller-owned bytes\r\n")
    canary = tmp_path / "import-side-effect"
    root = tmp_path / "upstream"
    build_upstream(root, canary)
    with serve(root) as upstream:
        config.write_text(
            description["configuration_toml"]
            + f'pypi_url = "{upstream.base_url}/pypi"\n'
            + f'simple_url = "{upstream.base_url}/simple"\n'
        )
        env = {key: value for key, value in os.environ.items() if key != "PYTHONPATH"}
        env.update(description["daemon"]["env"])
        with daemon.running(env, cwd=unrelated):
            transport = StdioTransport(
                command=command["command"],
                args=command["args"],
                env=env,
                cwd=str(unrelated),
            )
            async with Client(transport) as client:
                status = (await client.call_tool("service_status", {})).structured_content
                assert status["status"] == "ok", status
                result = (
                    await client.call_tool(
                        "resolve_library",
                        {"ecosystem": "python", "name": "evidence-demo", "version": "1.0"},
                    )
                ).structured_content
                result = await daemon.read_complete_answer(client, result)
                result = await daemon.wait_for_answer(client, result)
                assert result["status"] in {"ok", "partial"}, result
                assert result["context_id"] and result["snapshot_id"]
                inspected = (
                    await client.call_tool(
                        "inspect_symbol",
                        {
                            "context_id": result["context_id"],
                            "symbol_path": "different.PublicThing",
                        },
                    )
                ).structured_content
                inspected = await daemon.read_complete_answer(client, inspected)
                assert inspected["status"] in {"ok", "partial"}, inspected
                assert inspected["snapshot_id"] == result["snapshot_id"]
    assert not canary.exists()
    assert [path.name for path in unrelated.iterdir()] == ["unchanged"]
    assert (unrelated / "unchanged").read_bytes() == b"caller-owned bytes\r\n"
