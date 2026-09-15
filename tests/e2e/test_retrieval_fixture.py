"""The retrieval tools over the real MCP path, against the fixture upstream.

MCP stdio -> adapter -> NDJSON-RPC -> daemon -> a published snapshot. Measured from the
client's side: the blueprint's Phase 1 gate (a release works cold from a clean cache, then
offline from the same snapshot, through every tool), the resource templates returning the
same bytes as the tools, and gate **C13** (the adapter runs from an unrelated directory,
uses the configured state, and leaves that directory untouched).
"""

from __future__ import annotations

import hashlib
import json
import os
import shutil
from collections.abc import Iterator
from pathlib import Path
from typing import Any

import pytest
from fastmcp import Client

from support import daemon
from support.fixture_upstream import FixtureUpstream, serve

pytestmark = daemon.requires_daemon_binary

FIXTURE_ROOT = daemon.ROOT / "tests/fixtures/upstream"


def _write_config(path: Path, upstream: FixtureUpstream) -> None:
    path.write_text(
        "\n".join(
            [
                'config_version = "1.0"',
                "[policy]",
                'enabled_profiles = ["static"]',
                "[producers.rust]",
                f'crates_io_index_url = "{upstream.base_url}/index"',
                f'crates_io_api_url = "{upstream.base_url}/api/v1"',
                f'docs_rs_url = "{upstream.base_url}"',
                "",
            ]
        )
    )


@pytest.fixture
def upstream() -> Iterator[FixtureUpstream]:
    if not (FIXTURE_ROOT / "docsrs/enr-fixture-0.2.0.json.zst").is_file():
        pytest.fail("fixture upstream documents are missing; run `just fixtures-build`")
    with serve(FIXTURE_ROOT) as server:
        yield server


async def _call(
    env: dict[str, str], tool: str, cwd: Path | None = None, **arguments: Any
) -> dict[str, Any]:
    async with Client(daemon.transport(env, cwd=cwd)) as client:
        result = await client.call_tool(tool, arguments, raise_on_error=False)
        payload = await daemon.read_complete_answer(client, result.structured_content)
        payload = await daemon.wait_for_answer(client, payload)
    assert isinstance(payload, dict)
    return payload


async def _read(env: dict[str, str], uri: str) -> dict[str, Any]:
    async with Client(daemon.transport(env)) as client:
        contents = await client.read_resource(uri)
        text = getattr(contents[0], "text", None)
        assert isinstance(text, str), contents
        payload = await daemon.read_complete_answer(client, json.loads(text))
        assert isinstance(payload, dict)
        return payload


async def _round(env: dict[str, str], context_id: str, readme_id: str) -> list[dict[str, Any]]:
    """One pass over the four retrieval tools; the `data` payloads."""
    overview = await _call(env, "library_overview", context_id=context_id)
    search = await _call(env, "search_evidence", context_id=context_id, query="perimeter")
    inspect = await _call(
        env, "inspect_symbol", context_id=context_id, symbol_path="enr_fixture::Shape"
    )
    artifact = await _call(env, "read_artifact", artifact_id=readme_id)
    for name, payload in [
        ("overview", overview),
        ("search", search),
        ("inspect", inspect),
        ("artifact", artifact),
    ]:
        assert payload["status"] != "error", f"{name}: {payload['summary']}"
    return [overview["data"], search["data"], inspect["data"], artifact["data"]]


def _readme_id(resolved: dict[str, Any]) -> str:
    return next(a["artifact_id"] for a in resolved["data"]["artifacts"] if a["kind"] == "readme")


async def test_every_tool_answers_cold_then_offline_from_the_same_snapshot(
    tmp_path: Path,
) -> None:
    state = tmp_path / "state"
    config = tmp_path / "service.toml"
    with serve(FIXTURE_ROOT) as upstream:
        _write_config(config, upstream)
        env = daemon.daemon_env(state, config)
        with daemon.running(env):
            resolved = await _call(
                env, "resolve_library", ecosystem="rust", name="enr-fixture", version="0.2.0"
            )
            assert resolved["status"] == "ok", resolved["summary"]
            snapshot_id = resolved["snapshot_id"]
            assert snapshot_id.startswith("snap_")
            context_id = resolved["context_id"]
            readme_id = _readme_id(resolved)
            cold = await _round(env, context_id, readme_id)

    # The upstream is gone; a fresh daemon over the same state answers everything.
    with daemon.running(env):
        offline = await _call(
            env,
            "resolve_library",
            ecosystem="rust",
            name="enr-fixture",
            version="0.2.0",
            freshness="offline",
        )
        assert offline["status"] == "ok", offline["summary"]
        assert offline["snapshot_id"] == snapshot_id
        assert offline["data"]["answered_from_cache"] is True
        warm = await _round(env, context_id, readme_id)
        pinned = await _call(
            env, "library_overview", context_id=context_id, snapshot_id=snapshot_id
        )

    assert cold == warm
    assert pinned["data"] == cold[0]
    assert (state / "data/snapshots" / snapshot_id / "manifest.json").is_file()


async def test_resources_return_the_same_payloads_as_the_tools(
    tmp_path: Path, upstream: FixtureUpstream
) -> None:
    state = tmp_path / "state"
    config = tmp_path / "service.toml"
    _write_config(config, upstream)
    env = daemon.daemon_env(state, config)
    with daemon.running(env):
        resolved = await _call(
            env, "resolve_library", ecosystem="rust", name="enr-fixture", version="0.2.0"
        )
        assert resolved["status"] == "ok", resolved["summary"]
        context_id = resolved["context_id"]
        snapshot_id = resolved["snapshot_id"]
        readme_id = _readme_id(resolved)

        overview_tool = await _call(env, "library_overview", context_id=context_id)
        overview_res = await _read(env, f"library-evidence://contexts/{context_id}/overview")
        artifact_tool = await _call(env, "read_artifact", artifact_id=readme_id)
        artifact_res = await _read(env, f"library-evidence://artifacts/{readme_id}")
        manifest_res = await _read(env, f"library-evidence://snapshots/{snapshot_id}/manifest")

    assert overview_res["data"] == overview_tool["data"]
    assert artifact_res["data"] == artifact_tool["data"]
    assert manifest_res["status"] == "ok", manifest_res["summary"]
    assert manifest_res["data"]["is_current"] is True
    assert manifest_res["data"]["manifest"]["snapshot_id"] == snapshot_id
    assert manifest_res["data"]["manifest"]["counts"] == resolved["data"]["snapshot"]["counts"]


def _digest(root: Path) -> str:
    """A digest over every path and byte under `root`, in a stable order."""
    h = hashlib.sha256()
    for path in sorted(root.rglob("*")):
        h.update(str(path.relative_to(root)).encode())
        if path.is_file():
            h.update(path.read_bytes())
    return h.hexdigest()


async def test_tools_run_from_an_unrelated_directory_and_leave_it_untouched(
    tmp_path: Path, upstream: FixtureUpstream
) -> None:
    """Gate C13: the configured state is used; the current directory is not."""
    state = tmp_path / "state"
    config = tmp_path / "service.toml"
    _write_config(config, upstream)
    canary = tmp_path / "some-other-project"
    (canary / "src").mkdir(parents=True)
    (canary / "src/main.rs").write_text("fn main() {}\n")
    (canary / "Cargo.toml").write_text('[package]\nname = "canary"\nversion = "0.0.0"\n')
    before = _digest(canary)

    env = daemon.daemon_env(state, config)
    with daemon.running(env):
        resolved = await _call(
            env,
            "resolve_library",
            cwd=canary,
            ecosystem="rust",
            name="enr-fixture",
            version="0.2.0",
        )
        assert resolved["status"] == "ok", resolved["summary"]
        overview = await _call(
            env, "library_overview", cwd=canary, context_id=resolved["context_id"]
        )
        assert overview["status"] == "ok"

    assert _digest(canary) == before, "the current directory was written to"
    assert os.getcwd() != str(canary)
    blobs = state / "data/blobs"
    assert blobs.is_dir() and any(blobs.rglob("*")), "state landed in the configured root"
    assert (state / "data/snapshots" / resolved["snapshot_id"]).is_dir()


async def test_mismatched_hosted_version_is_not_admitted_as_requested_api(tmp_path: Path) -> None:
    root = tmp_path / "upstream"
    shutil.copytree(FIXTURE_ROOT, root)
    shutil.copyfile(
        root / "docsrs/enr-fixture-0.2.0.json.zst", root / "docsrs/enr-fixture-0.1.0.json.zst"
    )
    config = tmp_path / "service.toml"
    with serve(root) as upstream:
        _write_config(config, upstream)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            resolved = await _call(
                env, "resolve_library", ecosystem="rust", name="enr-fixture", version="0.1.0"
            )
            assert resolved["status"] == "partial", resolved
            assert resolved["freshness"]["source_version_match"] == "mismatched"
            assert any("declares version" in g["detail"] for g in resolved["data"]["gaps"])
            overview = await _call(env, "library_overview", context_id=resolved["context_id"])
            assert "public_api" in overview["coverage"]["missing"]
            assert overview["data"]["snapshot"]["counts"]["symbols"] == 0
            assert _readme_id(resolved)
