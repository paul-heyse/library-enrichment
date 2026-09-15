"""Real cold comparisons own acquisition interests and replay durable terminal answers."""

import threading
from pathlib import Path

import anyio
from fastmcp import Client

from e2e.test_python_fixture import build_upstream, config_for
from support import daemon
from support.fixture_upstream import serve

pytestmark = daemon.requires_daemon_binary


async def raw(client, tool, **params):
    return (await client.call_tool(tool, params, raise_on_error=False)).structured_content


async def test_cold_comparison_cancellation_preserves_shared_acquisition_and_restarts(
    tmp_path: Path,
):
    fixture = tmp_path / "upstream"
    build_upstream(fixture, tmp_path / "must-not-import")
    config = tmp_path / "service.toml"
    entered, released = threading.Event(), threading.Event()
    wheel = "/static/evidence_demo-1.0-py3-none-any.whl"
    with serve(fixture) as upstream:
        upstream.held_paths[wheel] = entered, released
        config_for(config, upstream.base_url)
        config.write_text(config.read_text() + "\n[limits]\ninline_wait_seconds=0\n")
        env = daemon.daemon_env(tmp_path / "state", config)
        try:
            with daemon.running(env):
                async with (
                    Client(daemon.transport(env)) as owner,
                    Client(daemon.transport(env)) as other,
                ):
                    acquire = await raw(
                        owner,
                        "resolve_library",
                        ecosystem="python",
                        name="evidence-demo",
                        version="1.0",
                    )
                    assert acquire["status"] == "pending", acquire
                    with anyio.fail_after(15):
                        while not entered.is_set():
                            await anyio.sleep(0.02)
                    comparison = await raw(
                        other,
                        "compare_releases",
                        ecosystem="python",
                        name="evidence-demo",
                        from_version="1.0",
                        to_version="2.0",
                    )
                    assert comparison["status"] == "pending", comparison
                    with anyio.fail_after(10):
                        while True:
                            record = await raw(
                                owner, "job_control", job_id=acquire["job"]["job_id"]
                            )
                            if record["data"]["active_interests"] == 2:
                                break
                            await anyio.sleep(0.02)
                    cancelled = await raw(
                        other,
                        "job_control",
                        job_id=comparison["job"]["job_id"],
                        action="cancel",
                        interest_token=comparison["data"]["interest_token"],
                    )
                    assert cancelled["status"] == "ok", cancelled
                    terminal = await daemon.wait_for_answer(other, comparison)
                    assert terminal["status"] == "error", terminal
                    surviving = await raw(owner, "job_control", job_id=acquire["job"]["job_id"])
                    assert surviving["data"]["active_interests"] == 1, surviving
                    released.set()
                    resolved = await daemon.wait_for_answer(owner, acquire)
                    assert resolved["status"] in {"ok", "partial"}, resolved
                    assert resolved["data"]["snapshot"]
                    assert upstream.request_log.count(wheel) == 1
            before = list(upstream.request_log)
            with daemon.running(env):
                async with Client(daemon.transport(env)) as client:
                    replay = await daemon.read_complete_answer(
                        client, await raw(client, "job_control", job_id=comparison["job"]["job_id"])
                    )
                    assert replay["data"]["state"] == "cancelled", replay
                    assert await daemon.read_terminal_answer(client, replay) == terminal
                    assert upstream.request_log == before
        finally:
            released.set()


async def test_one_side_failure_is_durable_and_a_new_comparison_uses_the_retained_side(tmp_path):
    fixture = tmp_path / "upstream"
    build_upstream(fixture, tmp_path / "must-not-import")
    config = tmp_path / "service.toml"
    with serve(fixture) as upstream:
        config_for(config, upstream.base_url)
        config.write_text(config.read_text() + "\n[limits]\ninline_wait_seconds=0\n")
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                pending = await raw(
                    client,
                    "compare_releases",
                    ecosystem="python",
                    name="evidence-demo",
                    from_version="1.0",
                    to_version="404.0",
                )
                assert pending["status"] == "pending", pending
                failed = await daemon.wait_for_answer(client, pending)
                assert failed["status"] == "error", failed
                assert failed["error"]["code"] == "VERSION_NOT_FOUND", failed
        before = list(upstream.request_log)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                record = await raw(client, "job_control", job_id=pending["job"]["job_id"])
                assert record["data"]["state"] == "failed", record
                assert await daemon.read_terminal_answer(client, record) == failed
                assert upstream.request_log == before
                result = await daemon.wait_for_answer(
                    client,
                    await raw(
                        client,
                        "compare_releases",
                        ecosystem="python",
                        name="evidence-demo",
                        from_version="1.0",
                        to_version="2.0",
                    ),
                )
                assert result["status"] in {"ok", "partial"}, result
                assert any(
                    c["subject"] == "different.new_api.batch" for c in result["data"]["changes"]
                )
                assert upstream.request_log.count("/static/evidence_demo-1.0-py3-none-any.whl") == 1
