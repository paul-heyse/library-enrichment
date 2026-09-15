"""The §4.4 fallback: when docs.rs has no JSON, compile it here -- and say so.

Hosted JSON comes first, always. This exercises what happens when there is none: the caller
opts in, the `build` profile is enabled and qualified, and a dated nightly compiles the crate
inside a capsule. Real daemon, real MCP, real container.

Gate R09 lives here too, and it is the distinction that matters most in this file. The dated
nightly building the documentation is not evidence that the crate compiles on the project's
stable compiler -- so the same crate that documents successfully on nightly must still fail a
stable compile probe, and the result must not claim otherwise.
"""

from __future__ import annotations

import json
import os
from collections.abc import Iterator
from pathlib import Path
from typing import Any

import pytest
from fastmcp import Client

from support import daemon
from support.fixture_upstream import FixtureUpstream, serve

ROOT = os.environ.get("LIBENR_EXECUTION_TEST_ROOT", "")
RUST_IMAGE = os.environ.get("LIBENR_EXECUTION_TEST_RUST", "")
pytestmark = [
    pytest.mark.sandbox,
    daemon.requires_daemon_binary,
    pytest.mark.skipif(
        not ROOT or not RUST_IMAGE,
        reason=(
            "set LIBENR_EXECUTION_TEST_ROOT and LIBENR_EXECUTION_TEST_RUST "
            "after execution-image setup"
        ),
    ),
]

FIXTURE_ROOT = daemon.ROOT / "tests/fixtures/upstream"
#: 0.1.0 deliberately has no hosted rustdoc JSON. That is gate R03's case, and this one's too.
NO_HOSTED_JSON = "0.1.0"


def write_config(
    path: Path, upstream: FixtureUpstream, profiles: str, broker: str | None = None
) -> None:
    path.write_text(
        "\n".join(
            [
                'config_version = "1.0"',
                "[policy]",
                f"enabled_profiles = {profiles}",
                "[limits]",
                "inline_wait_seconds = 0",
                "[execution]",
                f'storage_root = "{ROOT}"',
                f'rust_image = "{RUST_IMAGE}"',
                "deadline_seconds = 60",
                *([f'broker_path = "{broker}"'] if broker else []),
                "[network]",
                "acquisition_timeout_seconds = 900",
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


async def call(client, tool: str, **params: Any) -> dict[str, Any]:
    result = await daemon.read_complete_answer(
        client, (await client.call_tool(tool, params)).structured_content
    )
    return await daemon.wait_for_answer(client, result) if tool == "resolve_library" else result


async def finish(client, pending: dict[str, Any]) -> dict[str, Any]:
    if pending["status"] != "pending":
        return pending
    for _ in range(60):
        result = await call(
            client, "job_control", job_id=pending["job"]["job_id"], action="wait", wait_seconds=10
        )
        assert result["status"] == "ok", result
        if result["data"]["result"] is not None:
            return result["data"]["result"]
    pytest.fail("the probe exceeded bounded job polling")


async def test_asking_for_a_local_build_without_the_build_profile_is_denied(
    tmp_path: Path, upstream: FixtureUpstream
) -> None:
    """Asking is not granting. Only `static` is enabled, so the fallback must refuse."""
    config = tmp_path / "service.toml"
    write_config(config, upstream, '["static"]')
    env = daemon.daemon_env(tmp_path / "state", config)
    with daemon.running(env):
        async with Client(daemon.transport(env)) as client:
            payload = await call(
                client,
                "resolve_library",
                ecosystem="rust",
                name="enr-fixture",
                version=NO_HOSTED_JSON,
                allow_local_build=True,
            )
    assert payload["status"] == "partial", payload
    gap = next(g for g in payload["data"]["gaps"] if g["reason"] == "policy_denied")
    assert "`build` profile is not enabled" in gap["detail"], gap
    assert gap["planned_fallback"]["enabled"] is False
    # No snapshot was manufactured to paper over the refusal.
    assert payload["data"]["hosted_rustdoc_json"]["state"] == "missing"


async def test_a_local_build_supplies_the_api_and_is_labelled_as_locally_built(
    tmp_path: Path, upstream: FixtureUpstream
) -> None:
    config = tmp_path / "service.toml"
    write_config(config, upstream, '["static","build"]')
    env = daemon.daemon_env(tmp_path / "state", config)
    with daemon.running(env):
        async with Client(daemon.transport(env)) as client:
            payload = await call(
                client,
                "resolve_library",
                ecosystem="rust",
                name="enr-fixture",
                version=NO_HOSTED_JSON,
                allow_local_build=True,
            )
            assert payload["status"] in {"ok", "partial"}, payload
            snapshot = payload["data"].get("snapshot")
            assert snapshot is not None, payload
            assert snapshot["counts"]["symbols"] > 0, snapshot

            # Provenance says where this came from, and it says so in the published manifest
            # rather than only in this reply -- a snapshot has to stay traceable after the call
            # that made it. `docs-rs-rustdoc-json` must not appear: nothing was downloaded.
            contents = await client.read_resource(
                f"library-evidence://snapshots/{snapshot['snapshot_id']}/manifest"
            )
            manifest_text = getattr(contents[0], "text", None)
            assert isinstance(manifest_text, str), contents
            manifest_answer = await daemon.read_complete_answer(client, json.loads(manifest_text))
            producers = manifest_answer["data"]["producer_runs"]
            local = [run for run in producers if run["producer"] == "locally_built_rustdoc"]
            assert len(local) == 1 and local[0]["producer_version"] == "local-rustdoc/2", producers
            assert local[0]["profile"] == "build" and local[0]["outcome"] == "succeeded"
            assert not any(
                run["producer"] == "docs-rs-rustdoc-json" and run["outcome"] == "succeeded"
                for run in producers
            )
            receipt = await call(client, "read_artifact", artifact_id=local[0]["log"])
            attempts = json.loads(receipt["data"]["content"])
            assert any("+nightly-2026-09-13" in attempt["command"] for attempt in attempts)
            assert any("1.100.0-nightly" in attempt["stdout"] for attempt in attempts)
            assert all(attempt["cleanup_confirmed"] for attempt in attempts)

            # And the caller is told what a nightly build does not establish.
            limitation = " ".join(payload["coverage"]["limitations"])
            assert "not evidence that this crate compiles" in limitation, limitation
            assert "809936eac" in limitation, limitation

            # The API is real evidence, readable like any other.
            context = payload["context_id"]
            overview = await call(client, "library_overview", context_id=context)
            assert overview["status"] != "error", overview

            artifact = await call(
                client, "read_artifact", artifact_id=payload["artifacts"][0]["artifact_id"]
            )
            assert artifact["status"] != "error", artifact


async def test_a_nightly_documentation_build_does_not_claim_stable_compatibility(
    tmp_path: Path, upstream: FixtureUpstream
) -> None:
    """R09: documented on nightly, refused by the project's stable compiler, said plainly."""
    config = tmp_path / "service.toml"
    write_config(config, upstream, '["static","build"]')
    env = daemon.daemon_env(tmp_path / "state", config)
    with daemon.running(env):
        async with Client(daemon.transport(env)) as client:
            resolved = await call(
                client,
                "resolve_library",
                ecosystem="rust",
                name="enr-fixture",
                version=NO_HOSTED_JSON,
                allow_local_build=True,
            )
            assert resolved["data"]["snapshot"] is not None, resolved
            context = resolved["context_id"]

            # A consumer using a nightly-only feature. The documentation build above succeeded
            # on 1.100.0-nightly; the compile probe runs the project's stable 1.98.1.
            nightly_only = (daemon.ROOT / "tests/fixtures/probes/nightly-consumer.rs").read_text()
            probe = await finish(
                client,
                await call(
                    client,
                    "verify_usage",
                    context_id=context,
                    snippet=nightly_only,
                    mode="compile",
                    profile="build",
                ),
            )
            assert probe["status"] == "error", probe
            assert probe["error"]["code"] == "VERIFICATION_FAILED", probe

            # The compiler that refused it is named, and it is the stable one -- not the
            # nightly that built the documentation a moment ago.
            observation = probe["data"]["observations"][-1]
            assert "+1.98.1" in observation["command"], observation["command"]
            assert "1.98.1" in probe["data"]["environment"]["toolchain"]
            assert "nightly" not in probe["data"]["environment"]["toolchain"]
            diagnostics = observation["stderr"]
            assert "never_type" in diagnostics or "nightly" in diagnostics, diagnostics

            # Two producers, two compilers, one crate: the snapshot documents it on nightly and
            # the probe says it does not build on stable. Neither result overwrites the other.
            assert probe["data"]["source_context_id"] == context
            assert probe["data"]["limitations"], probe


async def test_rust_navigation_runs_in_a_warm_rust_analyzer_session(
    tmp_path: Path, upstream: FixtureUpstream
) -> None:
    """Blueprint 9.2 asks for navigation in both languages, not only the Python one.

    rust-analyzer indexes on startup, so this is the slowest thing in the suite. It is still
    worth having as a real session: the Rust capsule nests crate sources under a versioned
    directory that rustdoc spans are not relative to, and only an actual query proves the anchor
    resolves through it.
    """
    config = tmp_path.joinpath("service.toml")
    write_config(config, upstream, '["static","build"]')
    env = daemon.daemon_env(tmp_path.joinpath("state"), config)
    with daemon.running(env):
        async with Client(daemon.transport(env)) as client:
            resolved = await call(
                client,
                "resolve_library",
                ecosystem="rust",
                name="enr-fixture",
                version=NO_HOSTED_JSON,
                allow_local_build=True,
            )
            assert resolved["data"]["snapshot"] is not None, resolved
            context = resolved["context_id"]

            answer = await finish(
                client,
                await call(
                    client,
                    "inspect_symbol",
                    context_id=context,
                    symbol_path="enr_fixture::Widget",
                    aspects=["semantics"],
                    execution={
                        "intent": "execute_on_miss",
                        "profile": "build",
                        "methods": ["definition"],
                    },
                ),
            )
            assert answer["status"] in {"ok", "partial"}, answer
            observations = answer["data"]["execution_observations"]
            assert len(observations) == 1, answer
            observed = observations[0]
            definition = observed["payload"]["value"]
            assert definition["method"] == "definition"
            assert definition["server"].startswith("rust-analyzer"), definition
            assert observed["image_id"] == RUST_IMAGE
            assert observed["source"]["evidence_class"] == "compiler_derived"
            assert definition["outcome"] == "results", definition
            assert len(definition["locations"]) == 1, definition
            location = definition["locations"][0]
            assert location["kind"] == "artifact", location
            document = await call(client, "read_artifact", artifact_id=location["artifact_id"])
            source = document["data"]["content"]
            assert (
                source
                == (daemon.ROOT / "tests/fixtures/crates/enr-fixture-0.1.0/src/lib.rs").read_text()
            )
            start, end = location["range"]["start"], location["range"]["end"]
            assert start["line"] == end["line"]
            assert (
                source.splitlines()[start["line"]].encode()[start["byte"] : end["byte"]]
                == b"Widget"
            )
            before = (await call(client, "service_status"))["data"]["health"]["lsp"]
            retained = await call(
                client,
                "inspect_symbol",
                context_id=answer["context_id"],
                symbol_path="enr_fixture::Widget",
                aspects=["semantics"],
                execution={
                    "intent": "execute_on_miss",
                    "profile": "build",
                    "methods": ["definition"],
                },
            )
            assert retained["data"]["execution_observations"] == observations
            assert (await call(client, "service_status"))["data"]["health"]["lsp"] == before


async def test_build_and_proc_macro_activity_without_a_sandbox_is_policy_denied(
    tmp_path: Path, upstream: FixtureUpstream
) -> None:
    """R10, literally: a compile probe is build activity, and without isolation it is denied.

    `cargo check` on this fixture runs its `build.rs`. Blueprint 10 is explicit that "compile
    and typecheck do not mean execute nothing": build scripts and procedural macros belong to
    the `build` profile's sandbox. So with the profile enabled and the container broker missing,
    the answer must be a policy denial -- not a fallback to running the build script on the host.
    """
    config = tmp_path.joinpath("service.toml")
    absent_broker = tmp_path.joinpath("no-such-broker")
    marker = tmp_path.joinpath("build-script-ran")
    write_config(config, upstream, '["static","build"]', broker=str(absent_broker))

    env = daemon.daemon_env(tmp_path.joinpath("state"), config)
    env["ENR_FIXTURE_MARKER_DIR"] = str(tmp_path)
    with daemon.running(env):
        async with Client(daemon.transport(env)) as client:
            resolved = await call(
                client,
                "resolve_library",
                ecosystem="rust",
                name="enr-fixture",
                version=NO_HOSTED_JSON,
            )
            denied = await finish(
                client,
                await call(
                    client,
                    "verify_usage",
                    context_id=resolved["context_id"],
                    snippet="fn main() { let _ = enr_fixture::greet(); }\n",
                    mode="compile",
                    profile="build",
                ),
            )
            assert denied["status"] == "error", denied
            assert denied["error"]["code"] == "POLICY_DENIED", denied
            assert denied["error"]["next_action"], denied

    # The build script never ran anywhere. A host fallback would have left its marker.
    assert not marker.exists()
    assert not absent_broker.exists()


async def test_requested_features_use_hosted_first_then_a_qualified_local_observation(
    tmp_path, upstream
):
    config = tmp_path / "service.toml"
    write_config(config, upstream, '["static","build"]')
    env = daemon.daemon_env(tmp_path / "state", config)
    with daemon.running(env):
        async with Client(daemon.transport(env)) as client:
            request = dict(
                ecosystem="rust",
                name="enr-fixture",
                version="0.2.0",
                target="x86_64-unknown-linux-gnu",
                features=[],
                default_features=False,
                allow_local_build=True,
            )
            result = await call(client, "resolve_library", **request)
            assert result["status"] in {"ok", "partial"}, result
            assert result["data"]["hosted_rustdoc_json"]["state"] == "available"
            assert "/crate/enr-fixture/0.2.0/json" in upstream.request_log
            runs = result["data"]["producer_runs"]
            hosted = next(r for r in runs if r["producer"] == "docs-rs-rustdoc-json")
            local = next(r for r in runs if r["producer"] == "locally_built_rustdoc")
            assert hosted["started_at"] <= local["started_at"]
            assert local["inputs"]["cargo_lock"] and local["inputs"]["configuration"]
            searched = await call(
                client,
                "search_evidence",
                context_id=result["context_id"],
                query="extra_only",
                kinds=["api"],
            )
            assert searched["status"] in {"ok", "partial"}, searched
            assert not searched["data"]["hits"], searched
            before = list(upstream.request_log)
            retained = await call(client, "resolve_library", **request, freshness="offline")
            assert retained["snapshot_id"] == result["snapshot_id"]
            assert upstream.request_log == before
