"""Warm language-server sessions, over real MCP, against a real ty in a real container.

Gates P08a, P08b and C17.

P08 is retired: ty 0.0.80 implements `textDocument/implementation`, so the original gate's
premise ("unsupported") is false and ADR 0005 replaced it with two successors. P08a is the
nominal case -- a base class query returns mapped subclass locations. P08b is the one worth
reading carefully: ty answers a `typing.Protocol` query with the protocol and its *declared*
subclass and omits structurally compatible classes entirely. That answer is real and incomplete
at the same time, and recording it as "no implementors" would be a false negative about a class
that does satisfy the protocol.

C17 is the counter: a signature-only inspection must leave `health.lsp.started` untouched.
"""

from __future__ import annotations

import hashlib
import json
import os
import zipfile
from pathlib import Path
from typing import Any

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
        not ROOT or not IMAGE,
        reason=(
            "set LIBENR_EXECUTION_TEST_ROOT and LIBENR_EXECUTION_TEST_PYTHON "
            "after execution-image setup"
        ),
    ),
]

#: Both cases in one module, so one capsule and one warm session cover both queries.
#:
#: `Declared` names `Shape` as a base; `Structural` does not, and satisfies it anyway -- which
#: the module proves by assigning one to a `Shape` annotation. That assignment is what makes
#: P08b's omission a real hole rather than a hypothetical one.
MODULE = '''"""Fixture for language-server navigation."""

from typing import Protocol


class Base:
    """A nominal base class."""

    def value(self) -> int:
        return 1


class Child(Base):
    """A nominal subclass."""

    def value(self) -> int:
        return 2


class Shape(Protocol):
    """A structural protocol."""

    def area(self) -> float: ...


class Declared(Shape):
    """Explicitly declares the protocol as a base."""

    def area(self) -> float:
        return 1.0


class Structural:
    """Satisfies the protocol without declaring it."""

    def area(self) -> float:
        return 2.0


accepted: Shape = Structural()
'''


def upstream_fixture(root: Path) -> None:
    (root / "static").mkdir(parents=True)
    (root / "pypi/lsp-demo").mkdir(parents=True)
    name = "lsp_demo-1.0-py3-none-any.whl"
    wheel = root / "static" / name
    with zipfile.ZipFile(wheel, "w") as archive:
        archive.writestr("lsp_demo/__init__.py", MODULE)
        archive.writestr("lsp_demo/py.typed", "")
        archive.writestr(
            "lsp_demo-1.0.dist-info/METADATA",
            "Metadata-Version: 2.4\nName: lsp-demo\nVersion: 1.0\nRequires-Python: >=3.10\n\n",
        )
        archive.writestr(
            "lsp_demo-1.0.dist-info/WHEEL",
            "Wheel-Version: 1.0\nRoot-Is-Purelib: true\nTag: py3-none-any\n",
        )
        archive.writestr("lsp_demo-1.0.dist-info/RECORD", "")
    (root / "pypi/lsp-demo/1.0.json").write_text(
        json.dumps(
            {
                "info": {"name": "lsp-demo", "version": "1.0"},
                "urls": [
                    {
                        "filename": name,
                        "packagetype": "bdist_wheel",
                        "url": "{{BASE_URL}}/static/" + name,
                        "digests": {"sha256": hashlib.sha256(wheel.read_bytes()).hexdigest()},
                        "requires_python": ">=3.10",
                        "yanked": False,
                    }
                ],
            }
        )
    )
    (root / "pypi/lsp-demo/index.json").write_text(json.dumps({"versions": ["1.0"]}))


def configuration(path: Path, base: str) -> None:
    path.write_text(f"""[policy]
enabled_profiles=["static","build","runtime"]
[execution]
storage_root={json.dumps(ROOT)}
python_image={json.dumps(IMAGE)}
[limits]
warm_lsp_sessions=2
[producers.python]
pypi_url="{base}/pypi"
simple_url="{base}/simple"
""")


async def call(client, tool: str, **params: Any) -> dict[str, Any]:
    result = await daemon.read_complete_answer(
        client, (await client.call_tool(tool, params, raise_on_error=False)).structured_content
    )
    return await daemon.wait_for_answer(client, result) if tool == "resolve_library" else result


async def lsp_metrics(client) -> dict[str, Any]:
    return (await call(client, "service_status"))["data"]["health"]["lsp"]


def observation(payload: dict[str, Any], method: str) -> dict[str, Any]:
    matches = [
        o["payload"]["value"]
        for o in payload["data"].get("execution_observations", [])
        if o["payload"]["kind"] == "semantic_query" and o["payload"]["value"]["method"] == method
    ]
    assert len(matches) == 1, payload
    return matches[0]


def lines_of(entry: dict[str, Any]) -> set[int]:
    return {
        target["range"]["start"]["line"]
        for target in entry["locations"]
        if target["kind"] == "artifact"
    }


async def acquire(client):
    return await call(
        client,
        "resolve_library",
        ecosystem="python",
        name="lsp-demo",
        version="1.0",
        python_version="3.14",
    )


async def semantic(client, context, symbol, intent="execute_on_miss", **options):
    result = await call(
        client,
        "inspect_symbol",
        context_id=context,
        symbol_path=f"lsp_demo.{symbol}",
        selection={"mode": "explicit", "aspects": [{"aspect": name} for name in ["semantics"]]},
        execution={"intent": intent, "profile": "build", "methods": ["implementation"], **options},
    )
    return await daemon.wait_for_answer(client, result)


async def assert_source_ranges(client, query, expected):
    names = set()
    for target in query["locations"]:
        assert target["kind"] == "artifact", target
        document = await call(client, "read_artifact", artifact_id=target["artifact_id"])
        content = document["data"]["content"]
        assert content == MODULE
        start, end = target["range"]["start"], target["range"]["end"]
        assert start["line"] == end["line"]
        selected = content.splitlines()[start["line"]].encode()[start["byte"] : end["byte"]]
        names.add(selected.decode())
    assert names == expected


async def test_nominal_implementation_lookup_returns_mapped_subclass_locations(tmp_path: Path):
    """P08a: the real ty query names both declarations with validated artifact byte ranges."""
    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    with serve(tmp_path / "upstream") as upstream:
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                resolved = await acquire(client)
                answer = await semantic(client, resolved["context_id"], "Base")
                entry = observation(answer, "implementation")
                assert entry["outcome"] == "results", entry
                assert lines_of(entry) == {5, 12}
                await assert_source_ranges(client, entry, {"Base", "Child"})
                fact = answer["data"]["execution_observations"][0]
                assert fact["source"]["evidence_class"] == "typechecker_observed"
                assert fact["image_id"] == IMAGE
                assert entry["server"].startswith("ty 0.0.80"), entry
                consumer = await call(
                    client, "read_artifact", artifact_id=entry["document_artifact_id"]
                )
                assert consumer["data"]["content"] == "import lsp_demo\nlsp_demo.Base\n"


async def test_a_protocol_lookup_is_recorded_as_incomplete_not_as_no_implementors(tmp_path: Path):
    """P08b: real declared locations survive alongside the structural-coverage limitation."""
    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    with serve(tmp_path / "upstream") as upstream:
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                resolved = await acquire(client)
                answer = await semantic(client, resolved["context_id"], "Shape")
                entry = observation(answer, "implementation")
                assert entry["outcome"] == "incomplete"
                assert lines_of(entry) == {19, 25}
                await assert_source_ranges(client, entry, {"Shape", "Declared"})
                assert any("structural protocol" in note for note in entry["limitations"])
                assert "semantic_queries" in answer["coverage"]["missing"]
                assert answer["status"] == "partial"


@pytest.mark.parametrize("workers", [1, 2])
async def test_a_signature_read_starts_no_server_and_two_semantic_reads_share_one(
    tmp_path, workers
):
    """C17: retained reads start no server; new explicit queries reuse a leased session."""
    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    with serve(tmp_path / "upstream") as upstream:
        configuration(config, upstream.base_url)
        config.write_text(
            config.read_text().replace(
                "warm_lsp_sessions=2",
                f"warm_lsp_sessions=2\nexpensive_worker_concurrency={workers}",
            )
        )
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                resolved = await acquire(client)
                context = resolved["context_id"]
                signature = await call(
                    client,
                    "inspect_symbol",
                    context_id=context,
                    symbol_path="lsp_demo.Base",
                    selection={"mode": "explicit", "aspects": [{"aspect": "signature"}]},
                )
                assert signature["data"]["symbol"]["name"] == "Base"
                assert signature["data"]["execution_observations"] == []
                missing = await call(
                    client,
                    "inspect_symbol",
                    context_id=context,
                    symbol_path="lsp_demo.Base",
                    selection={
                        "mode": "explicit",
                        "aspects": [{"aspect": name} for name in ["semantics"]],
                    },
                )
                assert not missing["data"]["execution_observations"]
                assert (await lsp_metrics(client))["started"] == 0
                first = await semantic(client, context, "Base")
                assert lines_of(observation(first, "implementation")) == {5, 12}
                after_first = await lsp_metrics(client)
                assert after_first["started"] == after_first["warm"] == 1
                second = await semantic(client, context, "Shape")
                assert observation(second, "implementation")["outcome"] == "incomplete"
                after_second = await lsp_metrics(client)
                assert (
                    after_second["started"] == after_second["warm"] == after_second["reused"] == 1
                )
                retained = await semantic(client, first["context_id"], "Base")
                assert (
                    retained["data"]["execution_observations"]
                    == first["data"]["execution_observations"]
                )
                assert await lsp_metrics(client) == after_second


async def test_a_retained_capsule_is_checked_before_reuse_after_restart(tmp_path: Path):
    """Evidence survives cache corruption; only an explicit rerun repairs disposable preparation."""
    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    with serve(tmp_path / "upstream") as upstream:
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                resolved = await acquire(client)
                first = await semantic(client, resolved["context_id"], "Base")
                assert lines_of(observation(first, "implementation")) == {5, 12}
        capsules = tmp_path / "state/cache/capsules"
        installed = list(capsules.glob("*/python/lsp_demo/__init__.py"))
        assert len(installed) == 1
        installed[0].write_text("raise RuntimeError('corrupt disposable source')\n")
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                retained = await semantic(client, first["context_id"], "Base", intent="retained")
                assert (
                    retained["data"]["execution_observations"]
                    == first["data"]["execution_observations"]
                )
                assert (await lsp_metrics(client))["started"] == 0
                rerun = await semantic(client, resolved["context_id"], "Base", intent="rerun")
                assert lines_of(observation(rerun, "implementation")) == {5, 12}
                assert rerun["snapshot_id"] == first["snapshot_id"]
                assert (await lsp_metrics(client))["started"] == 1
                manifests = [json.loads(p.read_text()) for p in capsules.glob("*.json")]
                assert len(manifests) == 1
                selected = capsules / manifests[0]["generation"] / "python/lsp_demo/__init__.py"
                assert selected.read_text() == MODULE
                assert selected != installed[0]
