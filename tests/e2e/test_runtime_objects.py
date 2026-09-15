"""Retained runtime objects coexist with source/stub declarations through real MCP and Python."""

import hashlib
import json
import os
import zipfile
from pathlib import Path

import pytest
from fastmcp import Client

from support import daemon
from support.fixture_upstream import serve

ROOT = os.environ.get("LIBENR_EXECUTION_TEST_ROOT", "")
IMAGE = os.environ.get("LIBENR_EXECUTION_TEST_PYTHON", "")
pytestmark = [
    pytest.mark.sandbox,
    daemon.requires_daemon_binary,
    pytest.mark.skipif(
        not ROOT or not IMAGE, reason="qualified Python image and execution root required"
    ),
]


def upstream_fixture(root, extra=""):
    (root / "static").mkdir(parents=True)
    (root / "pypi/runtime-demo").mkdir(parents=True)
    filename = "runtime_demo-1.0-py3-none-any.whl"
    wheel = root / "static" / filename
    with zipfile.ZipFile(wheel, "w") as archive:
        archive.writestr(
            "runtime_demo/__init__.py",
            '"""Runtime fixture."""\nprint("observed import")\n'
            'def choose(value: int) -> str:\n    """Render a value."""\n    return str(value)\n'
            + extra,
        )
        archive.writestr("runtime_demo/__init__.pyi", "def choose(value: bytes) -> int: ...\n")
        archive.writestr("runtime_demo/py.typed", "")
        archive.writestr(
            "runtime_demo-1.0.dist-info/METADATA",
            "Metadata-Version: 2.4\nName: runtime-demo\nVersion: 1.0\n\n",
        )
        archive.writestr(
            "runtime_demo-1.0.dist-info/WHEEL",
            "Wheel-Version: 1.0\nRoot-Is-Purelib: true\nTag: py3-none-any\n",
        )
        archive.writestr("runtime_demo-1.0.dist-info/RECORD", "")
    (root / "pypi/runtime-demo/1.0.json").write_text(
        json.dumps(
            {
                "info": {"name": "runtime-demo", "version": "1.0"},
                "urls": [
                    {
                        "filename": filename,
                        "packagetype": "bdist_wheel",
                        "url": "{{BASE_URL}}/static/" + filename,
                        "digests": {"sha256": hashlib.sha256(wheel.read_bytes()).hexdigest()},
                        "yanked": False,
                    }
                ],
            }
        )
    )
    (root / "pypi/runtime-demo/index.json").write_text(json.dumps({"versions": ["1.0"]}))


async def call(client, tool, **params):
    result = (await client.call_tool(tool, params, raise_on_error=False)).structured_content
    result = await daemon.read_complete_answer(client, result)
    return await daemon.wait_for_answer(client, result)


@pytest.mark.parametrize("attributes", [["choose"], []])
async def test_runtime_observation_retains_static_alternatives_and_replays_without_policy(
    tmp_path, attributes
):
    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    with serve(tmp_path / "upstream") as upstream:
        config.write_text(f'''[policy]
enabled_profiles=["static","runtime"]
[execution]
storage_root={json.dumps(ROOT)}
python_image={json.dumps(IMAGE)}
[producers.python]
pypi_url="{upstream.base_url}/pypi"
simple_url="{upstream.base_url}/simple"
''')
        env = daemon.daemon_env(tmp_path / "state", config)
        selection = {"module": "runtime_demo", "attributes": attributes}
        symbol = ".".join(["runtime_demo", *attributes])
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                resolved = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="runtime-demo",
                    version="1.0",
                    python_version="3.14",
                )
                static = await call(
                    client, "inspect_symbol", context_id=resolved["context_id"], symbol_path=symbol
                )
                assert not static["data"]["execution_observations"]
                if attributes:
                    declarations = static["data"]["observations"]
                    assert {o["origin"] for o in declarations} == {"source", "stub"}
                    assert len({o["payload"]["signature"] for o in declarations}) > 1
                observed = await call(
                    client,
                    "inspect_symbol",
                    context_id=resolved["context_id"],
                    symbol_path=symbol,
                    selection={
                        "mode": "explicit",
                        "aspects": [{"aspect": name} for name in ["runtime"]],
                    },
                    execution={
                        "intent": "execute_on_miss",
                        "profile": "runtime",
                        "runtime": selection,
                    },
                )
                assert observed["status"] in {"ok", "partial"}, observed
                facts = observed["data"]["execution_observations"]
                assert len(facts) == 1
                fact = facts[0]
                runtime = fact["payload"]["value"]
                assert fact["payload"]["kind"] == "runtime_object"
                assert fact["source"]["evidence_class"] == "runtime_observed"
                assert runtime["outcome"] == "results", runtime
                assert runtime["module"] == "runtime_demo" and runtime["selection"] == attributes
                assert runtime["type_name"] == (
                    "builtins.function" if attributes else "builtins.module"
                )
                if attributes:
                    assert "int" in runtime["signature"] and "str" in runtime["signature"]
                    assert "bytes" not in runtime["signature"]
                    assert observed["data"]["observations"] == []
                    documented = await call(
                        client,
                        "inspect_symbol",
                        context_id=observed["context_id"],
                        symbol_path=symbol,
                        selection={
                            "mode": "explicit",
                            "aspects": [
                                {"aspect": name} for name in ["signature", "documentation"]
                            ],
                        },
                    )
                    # Execution derives an exact environment, so its copied static facts
                    # have new environment-bound observation IDs. Content and provenance
                    # survive intact; the original snapshot retains its original identities.
                    preserved = documented["data"]["observations"]
                    identity_fields = {"observation_id", "environment_id"}

                    def contents(rows):
                        return sorted(
                            [
                                {
                                    key: value
                                    for key, value in row.items()
                                    if key not in identity_fields
                                }
                                for row in rows
                            ],
                            key=lambda value: json.dumps(value, sort_keys=True),
                        )

                    assert contents(preserved) == contents(declarations)
                    original = await call(
                        client,
                        "inspect_symbol",
                        context_id=static["context_id"],
                        snapshot_id=static["snapshot_id"],
                        symbol_path=symbol,
                    )
                    assert original["data"]["observations"] == declarations
                else:
                    assert "choose" in runtime["attributes"]
                    assert runtime["signature"] is None
                jobs = sorted(
                    Path(env["LIBENR_DATA_HOME"]).joinpath("jobs/terminal").glob("*.json")
                )
        config.write_text('[policy]\nenabled_profiles=["static"]\n')
    # The upstream is gone and execution is disabled. Runtime evidence is still a read.
    with daemon.running(env):
        async with Client(daemon.transport(env)) as client:
            for intent in ["retained", "execute_on_miss"]:
                retained = await call(
                    client,
                    "inspect_symbol",
                    context_id=observed["context_id"],
                    symbol_path=symbol,
                    selection={
                        "mode": "explicit",
                        "aspects": [{"aspect": name} for name in ["runtime"]],
                    },
                    execution={
                        "intent": intent,
                        "profile": "runtime",
                        "runtime": selection,
                    },
                )
                assert retained["data"]["execution_observations"] == facts
            assert (
                sorted(Path(env["LIBENR_DATA_HOME"]).joinpath("jobs/terminal").glob("*.json"))
                == jobs
            )


@pytest.mark.parametrize(
    "extra,absent_signature",
    [
        ("choose.__doc__ = '🌏\\\"\\n' * 30000\n", False),
        (
            "import inspect\nchoose.__signature__ = inspect.Signature([inspect.Parameter("
            "'value', inspect.Parameter.POSITIONAL_ONLY, default='x' * 10000)])\n",
            True,
        ),
    ],
)
async def test_runtime_report_limits_preserve_truthful_partial_results(
    tmp_path, extra, absent_signature
):
    upstream_fixture(tmp_path / "upstream", extra)
    config = tmp_path / "service.toml"
    with serve(tmp_path / "upstream") as upstream:
        config.write_text(f'''[policy]
enabled_profiles=["static","runtime"]
[execution]
storage_root={json.dumps(ROOT)}
python_image={json.dumps(IMAGE)}
[producers.python]
pypi_url="{upstream.base_url}/pypi"
simple_url="{upstream.base_url}/simple"
''')
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                resolved = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="runtime-demo",
                    version="1.0",
                    python_version="3.14",
                )
                observed = await call(
                    client,
                    "inspect_symbol",
                    context_id=resolved["context_id"],
                    symbol_path="runtime_demo.choose",
                    selection={
                        "mode": "explicit",
                        "aspects": [{"aspect": name} for name in ["runtime"]],
                    },
                    execution={
                        "intent": "execute_on_miss",
                        "profile": "runtime",
                        "runtime": {"module": "runtime_demo", "attributes": ["choose"]},
                    },
                )
                assert observed["status"] == "partial", observed
                fact = observed["data"]["execution_observations"][0]["payload"]["value"]
                assert fact["outcome"] == "incomplete", fact
                assert fact["type_name"] == "builtins.function"
                assert "runtime_api" in observed["coverage"]["missing"]
                if absent_signature:
                    assert fact["signature"] is None
                else:
                    assert fact["signature"] == "(value: 'int') -> 'str'"
                    assert fact["docstring"] and len(fact["docstring"].encode()) < 65536
                run = observed["data"]["producer_runs"][0]
                receipt = await call(client, "read_artifact", artifact_id=run["log"])
                parts = [receipt["data"]["content"]]
                while receipt["data"]["page"]["next_cursor"] is not None:
                    receipt = await call(
                        client,
                        "read_artifact",
                        artifact_id=run["log"],
                        cursor=receipt["data"]["page"]["next_cursor"],
                    )
                    parts.append(receipt["data"]["content"])
                process = json.loads("".join(parts))["transcript"]["runtime"]
                assert process["end"] == "exited" and process["exit_code"] == 0, process
                assert len(process["stdout"].encode()) <= 65536
