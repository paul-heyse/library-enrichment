"""`resolve_library` over the real MCP path, against the fixture upstream.

MCP stdio -> adapter -> NDJSON-RPC -> daemon -> real HTTP client -> a loopback stand-in for
crates.io and docs.rs (`tests/support/fixture_upstream.py`). No component is mocked; the only
thing that differs from production is which hosts the configuration names.

Gates measured here from the client's side: **R01** (the requested version is retained while a
newer one exists), **R03** (missing hosted JSON is `partial` with an explicit, policy-aware
fallback), and the offline replay half of the blueprint's Phase 1 gate.
"""

from __future__ import annotations

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
    """A service configuration whose registry and docs endpoints are the fixture upstream.

    Only `static` is enabled, as in development: the planned fallback for missing JSON must
    then report `enabled: false`, which is what R03 wants to see.
    """
    path.write_text(
        "\n".join(
            [
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
    if not (FIXTURE_ROOT / "index/enr-fixture.ndjson").is_file():
        pytest.fail("fixture upstream documents are missing; run `just fixtures-build`")
    with serve(FIXTURE_ROOT) as server:
        yield server


@pytest.fixture
def daemon_env(tmp_path: Path, upstream: FixtureUpstream) -> Iterator[dict[str, str]]:
    state = tmp_path / "state"
    config = tmp_path / "service.toml"
    _write_config(config, upstream)
    with daemon.running(daemon.daemon_env(state, config)) as env:
        yield env


async def _resolve(env: dict[str, str], **arguments: Any) -> dict[str, Any]:
    async with Client(daemon.transport(env)) as client:
        result = await client.call_tool(
            "resolve_library", {"ecosystem": "rust", **arguments}, raise_on_error=False
        )
        payload = await daemon.wait_for_answer(client, result.structured_content)
    assert isinstance(payload, dict)
    return payload


async def test_the_requested_version_is_retained_while_a_newer_release_exists(
    daemon_env: dict[str, str],
) -> None:
    payload = await _resolve(daemon_env, name="enr-fixture", version="0.1.0")

    assert payload["status"] == "partial", payload["summary"]
    assert payload["data"]["release"]["version"] == "0.1.0"
    assert payload["data"]["upstream"]["newest_stable"] == "0.2.0"
    assert payload["data"]["upstream"]["resolved_is_newest_stable"] is False
    assert payload["freshness"]["latest_verified"] is True
    assert payload["context_id"].startswith("ctx_")


async def test_missing_hosted_json_is_partial_with_the_fallback_named(
    daemon_env: dict[str, str],
) -> None:
    payload = await _resolve(daemon_env, name="enr-fixture", version="0.1.0")

    assert payload["status"] == "partial"
    assert "hosted_rustdoc_json" in payload["coverage"]["missing"]
    assert {"registry_metadata", "crate_source", "documentation_build_config"} <= set(
        payload["coverage"]["indexed"]
    )
    assert payload["data"]["hosted_rustdoc_json"]["state"] == "missing"
    gap = next(g for g in payload["data"]["gaps"] if g["kind"] == "hosted_rustdoc_json")
    assert gap["reason"] == "hosted_json_missing"
    # The gap names the producer that would actually run, and the same name appears in the
    # snapshot provenance if it does -- so a caller can tell a locally built API from a hosted one.
    assert gap["planned_fallback"]["producer"] == "locally_built_rustdoc"
    assert gap["planned_fallback"]["enabled"] is False
    assert payload["data"]["observed_configuration"]["all_features"] is True
    assert all(a["uri"].startswith("library-evidence://artifacts/") for a in payload["artifacts"])


async def test_a_recorded_resolution_replays_offline_through_mcp(
    tmp_path: Path,
) -> None:
    """Resolve once online, stop the upstream, then answer offline from the record."""
    state = tmp_path / "state"
    config = tmp_path / "service.toml"
    with serve(FIXTURE_ROOT) as upstream:
        _write_config(config, upstream)
        env = daemon.daemon_env(state, config)
        with daemon.running(env):
            online = await _resolve(env, name="enr-fixture", version="0.2.0")
            assert online["data"]["hosted_rustdoc_json"]["state"] == "available"
            assert online["data"]["answered_from_cache"] is False

    # The upstream is gone. A fresh daemon over the same state answers from the record.
    with daemon.running(env):
        offline = await _resolve(env, name="enr-fixture", version="0.2.0", freshness="offline")
        revalidate = await _resolve(
            env, name="enr-fixture", version="0.2.0", freshness="revalidate"
        )

    assert offline["status"] != "error", offline["summary"]
    assert offline["context_id"] == online["context_id"]
    assert offline["data"]["answered_from_cache"] is True
    assert offline["freshness"]["latest_verified"] is False
    assert offline["data"]["release"] == online["data"]["release"]
    assert any("without age-based expiry" in note for note in offline["coverage"]["limitations"])

    assert revalidate["status"] == "error"
    assert revalidate["error"]["code"] == "UPSTREAM_UNAVAILABLE"
    assert revalidate["error"]["retryable"] is True


async def test_a_bad_request_is_a_typed_error_not_a_transport_failure(
    daemon_env: dict[str, str],
) -> None:
    payload = await _resolve(daemon_env, name="enr-fixture", version="9.9.9")
    assert payload["status"] == "error"
    assert payload["error"]["code"] == "VERSION_NOT_FOUND"
    assert "0.2.0" in payload["error"]["next_action"]
