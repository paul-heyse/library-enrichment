"""Retained exact-context facts outlive freshness metadata and producer processes."""

import hashlib
from pathlib import Path

import pytest
from fastmcp import Client

from e2e.test_python_fixture import build_upstream, call, config_for
from e2e.test_resolve_fixture import FIXTURE_ROOT, _write_config
from support import daemon
from support.fixture_upstream import serve

pytestmark = daemon.requires_daemon_binary


def snapshot_bytes(data: Path) -> dict[str, str]:
    return {
        str(path.relative_to(data)): hashlib.sha256(path.read_bytes()).hexdigest()
        for path in data.joinpath("snapshots").rglob("*")
        if path.is_file()
    }


@pytest.mark.parametrize("ecosystem", ["rust", "python"])
async def test_exact_evidence_never_expires_and_new_versions_preserve_it(tmp_path, ecosystem):
    root = FIXTURE_ROOT
    config = tmp_path / "config.toml"
    if ecosystem == "python":
        root = tmp_path / "upstream"
        build_upstream(root, tmp_path / "MUST_NOT_IMPORT")
    name, old, new = (
        ("enr-fixture", "0.1.0", "0.2.0")
        if ecosystem == "rust"
        else ("evidence-demo", "1.0", "2.0")
    )
    with serve(root) as upstream:
        if ecosystem == "rust":
            _write_config(config, upstream)
        else:
            config_for(config, upstream.base_url)
        with config.open("a") as stream:
            stream.write("\n[freshness]\nregistry_ttl_seconds=0\nmutable_docs_ttl_seconds=0\n")
        env = daemon.daemon_env(tmp_path / "state", config)
        data = Path(env["LIBENR_DATA_HOME"])
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                first = await call(
                    client, "resolve_library", ecosystem=ecosystem, name=name, version=old
                )
                assert first["status"] in {"ok", "partial"}, first
                before = snapshot_bytes(data)
                newer = await call(
                    client, "resolve_library", ecosystem=ecosystem, name=name, version=new
                )
                assert newer["status"] in {"ok", "partial"}, newer
                assert newer["context_id"] != first["context_id"]
                assert before.items() <= snapshot_bytes(data).items()
        # TTL is zero above. Native snapshots are never edited to simulate age; elapsed
        # freshness cannot invalidate exact immutable inputs after daemon restart.
        retained = snapshot_bytes(data)

    # Real upstream is gone and the producer process has restarted. Default cache_ok must
    # still answer both versions, without HTTP or re-extraction, even with TTL explicitly zero.
    with daemon.running(env):
        async with Client(daemon.transport(env)) as client:
            for previous, version in [(first, old), (newer, new)]:
                replay = await call(
                    client, "resolve_library", ecosystem=ecosystem, name=name, version=version
                )
                assert replay["status"] in {"ok", "partial"}, replay
                assert replay["data"]["answered_from_cache"]
                assert replay["snapshot_id"] == previous["snapshot_id"]
                assert replay["context_id"] == previous["context_id"]
                assert not replay["freshness"]["latest_verified"]
                overview = await call(
                    client,
                    "library_overview",
                    context_id=replay["context_id"],
                    snapshot_id=replay["snapshot_id"],
                )
                assert overview["status"] in {"ok", "partial"}, overview
            status = await call(client, "service_status")
            assert status["data"]["health"]["single_flight"]["started"] == 0
    assert retained == snapshot_bytes(data)


async def test_identical_readme_bytes_keep_their_release_scoped_citations(tmp_path):
    root = tmp_path / "upstream"
    build_upstream(root, tmp_path / "MUST_NOT_IMPORT")
    config = tmp_path / "config.toml"
    with serve(root) as upstream:
        config_for(config, upstream.base_url)
        with daemon.running(daemon.daemon_env(tmp_path / "state", config)) as env:
            async with Client(daemon.transport(env)) as client:
                for version in ["1.0", "2.0"]:
                    resolved = await call(
                        client,
                        "resolve_library",
                        ecosystem="python",
                        name="evidence-demo",
                        version=version,
                    )
                    assert resolved["status"] in {"ok", "partial"}, resolved
                    result = await call(
                        client,
                        "search_evidence",
                        context_id=resolved["context_id"],
                        query="small static capability",
                    )
                    citations = [
                        item
                        for item in result["evidence"]
                        if "small static capability" in item["excerpt"]
                    ]
                    assert citations, result
                    for citation in citations:
                        assert citation["source_uri"].endswith(
                            f"evidence_demo-{version}-py3-none-any.whl#README.md"
                        ), citation
                        assert citation["source_version_match"] == "exact"


@pytest.mark.parametrize("ecosystem", ["rust", "python"])
async def test_latest_rechecks_selection_without_regenerating_unchanged_evidence(
    tmp_path, ecosystem
):
    config = tmp_path / "config.toml"
    root = FIXTURE_ROOT
    if ecosystem == "python":
        root = tmp_path / "upstream"
        build_upstream(root, tmp_path / "MUST_NOT_IMPORT")
    with serve(root) as upstream:
        if ecosystem == "rust":
            _write_config(config, upstream)
        else:
            config_for(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                arguments = dict(
                    ecosystem=ecosystem,
                    name="enr-fixture" if ecosystem == "rust" else "evidence-demo",
                )
                first = await call(client, "resolve_library", **arguments)
                assert first["status"] in {"ok", "partial"}, first
                before = snapshot_bytes(Path(env["LIBENR_DATA_HOME"]))
                calls = len(upstream.response_log)
                second = await call(client, "resolve_library", **arguments)
                assert second["status"] in {"ok", "partial"}, second
                assert second["data"]["answered_from_cache"]
                assert second["context_id"] == first["context_id"]
                assert second["data"]["snapshot"]["counts"] == first["data"]["snapshot"]["counts"]
                selected_runs = [
                    run
                    for run in second["data"]["producer_runs"]
                    if run["producer"] == "registry-selection"
                ]
                assert selected_runs and all(run["log"] for run in selected_runs)
                checked = upstream.response_log[calls:]
                assert checked, "latest must check its mutable selection"
                assert all(
                    path.startswith(("/index/", "/simple/", "/pypi/")) for path, _, _ in checked
                ), checked
                assert before.items() <= snapshot_bytes(Path(env["LIBENR_DATA_HOME"])).items()
                third = await call(client, "resolve_library", **arguments)
                assert third["data"]["answered_from_cache"]
                assert third["snapshot_id"] == second["snapshot_id"]
                assert third["data"]["snapshot"]["counts"] == first["data"]["snapshot"]["counts"]
