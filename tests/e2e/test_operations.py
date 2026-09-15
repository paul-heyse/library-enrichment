"""Operational hardening: sharing, cancelling, crashing and keeping.

Gates C04-C07. All four are about what happens when a service is used by more than one caller,
or interrupted in the middle of something, which is when a design either holds or quietly
produces evidence nobody should trust.

Each drives a real daemon over real MCP against the loopback upstream. The upstream counts its
own requests, so "one producer job" is measured at the far end rather than inferred from a
counter the service keeps about itself.
"""

from __future__ import annotations

import asyncio
import hashlib
import json
import os
import signal
import subprocess
import threading
import time
import zipfile
from pathlib import Path
from typing import Any

import pytest
from fastmcp import Client

from support import daemon
from support.fixture_upstream import serve

pytestmark = daemon.requires_daemon_binary

EXECUTION_ROOT = os.environ.get("LIBENR_EXECUTION_TEST_ROOT", "")
PYTHON_IMAGE = os.environ.get("LIBENR_EXECUTION_TEST_PYTHON", "")
#: C05 needs a job to share, and a job needs an execution profile with a qualified image.
needs_execution = pytest.mark.skipif(
    not EXECUTION_ROOT or not PYTHON_IMAGE,
    reason=(
        "a shared job needs an enabled build profile and a qualified image; run "
        "just execution-images --apply and just execution-qualify --apply"
    ),
)


def upstream_fixture(root: Path, package: str = "ops-demo") -> None:
    module = package.replace("-", "_")
    (root / "static").mkdir(parents=True, exist_ok=True)
    (root / f"pypi/{package}").mkdir(parents=True, exist_ok=True)
    name = f"{module}-1.0-py3-none-any.whl"
    wheel = root / "static" / name
    with zipfile.ZipFile(wheel, "w") as archive:
        archive.writestr(f"{module}/__init__.py", "def twice(v: int) -> int:\n    return v * 2\n")
        archive.writestr(f"{module}/py.typed", "")
        archive.writestr(
            f"{module}-1.0.dist-info/METADATA",
            f"Metadata-Version: 2.4\nName: {package}\nVersion: 1.0\nRequires-Python: >=3.10\n\n",
        )
        archive.writestr(
            f"{module}-1.0.dist-info/WHEEL",
            "Wheel-Version: 1.0\nRoot-Is-Purelib: true\nTag: py3-none-any\n",
        )
        archive.writestr(f"{module}-1.0.dist-info/RECORD", "")
    (root / f"pypi/{package}/1.0.json").write_text(
        json.dumps(
            {
                "info": {"name": package, "version": "1.0"},
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
    (root / f"pypi/{package}/index.json").write_text(json.dumps({"versions": ["1.0"]}))


def configuration(path: Path, base: str, extra: str = "", profiles: str = '["static"]') -> None:
    path.write_text(f"""[policy]
enabled_profiles={profiles}
[limits]
inline_wait_seconds=0
[freshness]
registry_ttl_seconds=0
{extra}[producers.python]
pypi_url="{base}/pypi"
simple_url="{base}/simple"
""")


async def call(client, tool: str, **params: Any) -> dict[str, Any]:
    result = await daemon.read_complete_answer(
        client, (await client.call_tool(tool, params)).structured_content
    )
    return await daemon.wait_for_answer(client, result) if tool == "resolve_library" else result


async def resolve(client, package: str = "ops-demo") -> dict[str, Any]:
    return await call(
        client,
        "resolve_library",
        ecosystem="python",
        name=package,
        version="1.0",
        python_version="3.14",
    )


async def test_two_clients_asking_for_one_extraction_get_one_producer_run(tmp_path: Path):
    """C04: two adapters, one acquisition, equivalent evidence for both."""
    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    with serve(tmp_path / "upstream") as upstream:
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            # Two independent MCP clients, each its own adapter subprocess over its own stdio --
            # the two-client shape the gate is about, not two calls down one connection.
            async with (
                Client(daemon.transport(env)) as first,
                Client(daemon.transport(env)) as second,
            ):
                left, right = await asyncio.gather(resolve(first), resolve(second))

            assert left["status"] != "error", left
            assert right["status"] != "error", right

            # Equivalent evidence: the same release, the same context, the same snapshot.
            assert left["context_id"] == right["context_id"]
            assert left["snapshot_id"] == right["snapshot_id"]
            assert left["data"]["release"] == right["data"]["release"]

            # One producer run. Measured two ways, and both have to agree: the service's own
            # counter, and the number of times the upstream was actually asked for the wheel.
            async with Client(daemon.transport(env)) as client:
                counts = (await call(client, "service_status"))["data"]["health"]["single_flight"]
            assert counts["started"] == 1, counts
            assert counts["shared"] == 1, counts
            assert counts["inflight"] == 0, counts

    wheel_requests = [
        line for line in upstream.request_log if line.endswith("ops_demo-1.0-py3-none-any.whl")
    ]
    assert len(wheel_requests) == 1, upstream.request_log

    # And the caller that attached to someone else's run is told so, rather than believing it
    # caused the acquisition it is reading.
    shared = [answer for answer in (left, right) if "already in flight" in str(answer["coverage"])]
    assert len(shared) == 1, (left["coverage"], right["coverage"])


@pytest.mark.sandbox
@needs_execution
@pytest.mark.parametrize("operation", ["verify", "inspect"])
async def test_one_caller_cancelling_leaves_another_callers_work_alone(
    tmp_path: Path, operation: str
):
    """C05: detaching one interest from a shared job never cancels it for the other holder."""
    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    with serve(tmp_path / "upstream") as upstream:
        configuration(
            config,
            upstream.base_url,
            extra=(
                f"[execution]\nstorage_root={json.dumps(EXECUTION_ROOT)}\n"
                f"python_image={json.dumps(PYTHON_IMAGE)}\ndeadline_seconds=60\n"
            ),
            profiles='["static","build"]',
        )
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with (
                Client(daemon.transport(env)) as client,
                Client(daemon.transport(env)) as survivor,
            ):
                context = (await resolve(client))["context_id"]

                # Two submissions of identical work share one job and get distinct interests.
                tool, arguments = (
                    (
                        "verify_usage",
                        dict(
                            context_id=context,
                            snippet="from ops_demo import twice\nvalue: int = twice(2)\n",
                            mode="typecheck",
                            profile="build",
                        ),
                    )
                    if operation == "verify"
                    else (
                        "inspect_symbol",
                        dict(
                            context_id=context,
                            symbol_path="ops_demo.twice",
                            aspects=["semantics"],
                            execution={
                                "intent": "execute_on_miss",
                                "profile": "build",
                                "methods": ["definition"],
                            },
                        ),
                    )
                )
                submissions = [
                    await call(adapter, tool, **arguments) for adapter in (client, survivor)
                ]
                first, second = submissions
                assert first["status"] == "pending", first
                assert second["status"] == "pending", second
                assert first["job"]["job_id"] == second["job"]["job_id"], "one shared job"
                assert first["data"]["interest_token"] != second["data"]["interest_token"], (
                    "each caller gets its own interest"
                )

                job_id = first["job"]["job_id"]
                dropped = await call(
                    client,
                    "job_control",
                    job_id=job_id,
                    action="cancel",
                    interest_token=first["data"]["interest_token"],
                )
                assert dropped["status"] == "ok", dropped
                # One interest remains, so the work is still wanted and still running.
                assert dropped["data"]["active_interests"] == 1, dropped
                assert dropped["data"]["state"] in {"queued", "running"}, dropped

                # Cancelling with someone else's token is refused outright.
                forged = await call(
                    client,
                    "job_control",
                    job_id=job_id,
                    action="cancel",
                    interest_token="interest_not_yours",
                )
                assert forged["status"] == "error", forged

                completed = await daemon.wait_for_answer(survivor, second)
                assert completed["status"] in {"ok", "partial"}, completed
                if operation == "verify":
                    assert completed["data"]["observations"][-1]["exit_code"] == 0
                    assert completed["data"]["observations"][-1]["cleanup_confirmed"]
                else:
                    observations = completed["data"]["execution_observations"]
                    assert len(observations) == 1, completed
                    assert observations[0]["payload"]["value"]["outcome"] == "results"
                terminal = await call(survivor, "job_control", job_id=job_id)
                assert terminal["data"]["state"] in {"succeeded", "partial"}, terminal
                assert terminal["data"]["result"]["snapshot_id"] == completed["snapshot_id"]


async def test_a_crash_during_publication_leaves_no_half_published_snapshot(tmp_path: Path):
    """C06: kill the daemon mid-acquisition; no partial snapshot is ever readable."""
    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    state = tmp_path / "state"
    with serve(tmp_path / "upstream") as upstream:
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(state, config)

        # Hold the wheel request open so the kill lands *during* acquisition rather than
        # whenever a sleep happens to expire. Racing a fast fixture would make this test pass
        # by finishing early, which proves nothing about a crash.
        entered, release = threading.Event(), threading.Event()
        upstream.held_paths["/static/ops_demo-1.0-py3-none-any.whl"] = (entered, release)

        process = subprocess.Popen(
            [str(daemon.DAEMON_BIN), "start"],
            env=env,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
        )
        try:
            daemon.await_socket(Path(env["LIBENR_SOCKET"]), process)
            async with Client(daemon.transport(env)) as client:
                task = asyncio.create_task(resolve(client))
                assert await asyncio.to_thread(entered.wait, 30), "the wheel was never requested"
                # SIGKILL with the acquisition mid-flight. No shutdown path runs, so what is on
                # disk afterwards is exactly what a crash leaves.
                process.send_signal(signal.SIGKILL)
                # The in-flight call cannot complete, and the adapter says so rather than
                # raising: an unreachable daemon is a typed error with a next action, which is
                # what a caller needs at exactly the moment the service went away.
                interrupted = await asyncio.wait_for(task, timeout=30)
                assert interrupted["status"] == "error", interrupted
                assert interrupted["error"]["code"] == "UPSTREAM_UNAVAILABLE", interrupted
                assert interrupted["error"]["next_action"], interrupted
        finally:
            release.set()
            upstream.held_paths.clear()
            process.kill()
            process.wait(timeout=10)

        snapshots = state / "data" / "snapshots"
        published = sorted(p.name for p in snapshots.iterdir()) if snapshots.is_dir() else []
        staging = state / "data" / "staging"
        left_behind = sorted(p.name for p in staging.iterdir()) if staging.is_dir() else []

        # Whatever the crash caught, a published snapshot directory is complete or absent --
        # never a directory of tables missing its manifest.
        for name in published:
            assert (snapshots / name / "manifest.json").is_file(), (
                f"{name} is published without a manifest, so a reader would see a half-written "
                "snapshot"
            )

        # Restart. Staging is swept, and every published snapshot still reads.
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                answer = await resolve(client)
                assert answer["status"] != "error", answer
                if answer["snapshot_id"]:
                    overview = await call(
                        client,
                        "library_overview",
                        context_id=answer["context_id"],
                        snapshot_id=answer["snapshot_id"],
                    )
                    assert overview["status"] != "error", overview
        swept = sorted(p.name for p in staging.iterdir()) if staging.is_dir() else []
        assert swept == [], f"staging left behind after restart: {left_behind} -> {swept}"


async def test_a_later_snapshot_leaves_a_pinned_one_readable_and_unchanged(tmp_path: Path):
    """C07: enrichment adds a snapshot; the one a caller pinned keeps its exact bytes."""
    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    state = tmp_path / "state"
    with serve(tmp_path / "upstream") as upstream:
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(state, config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                first = await resolve(client)
                assert first["status"] != "error", first
                pinned = first["snapshot_id"]
                assert pinned, first
                context = first["context_id"]

                before = await call(
                    client, "library_overview", context_id=context, snapshot_id=pinned
                )
                assert before["status"] != "error", before

                snapshots = state / "data" / "snapshots"
                pinned_dir = next(p for p in snapshots.iterdir() if pinned.endswith(p.name[-16:]))
                digest = {
                    f.relative_to(pinned_dir).as_posix(): hashlib.sha256(f.read_bytes()).hexdigest()
                    for f in sorted(pinned_dir.rglob("*"))
                    if f.is_file()
                }
                mtimes = {path: os.stat(pinned_dir / path).st_mtime_ns for path in digest}

                # A latest lookup adds its real registry-selection provenance to the same
                # exact context. This must publish a distinct snapshot, without extracting
                # the unchanged wheel again or rewriting the snapshot already pinned.
                later = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="ops-demo",
                    python_version="3.14",
                    mode="project",
                )
                assert later["status"] in {"ok", "partial"}, later
                assert later["context_id"] == context
                assert later["snapshot_id"] != pinned
                assert later["data"]["answered_from_cache"]
                assert any(
                    run["producer"] == "registry-selection" and run["log"]
                    for run in later["data"]["producer_runs"]
                ), later
                current = await call(client, "library_overview", context_id=context)
                assert current["snapshot_id"] == later["snapshot_id"], current
                assert (
                    sum(
                        path.endswith("ops_demo-1.0-py3-none-any.whl")
                        for path in upstream.request_log
                    )
                    == 1
                )

                after = await call(
                    client, "library_overview", context_id=context, snapshot_id=pinned
                )
                assert after["status"] != "error", after
                assert after["data"] == before["data"], "the pinned snapshot's evidence changed"

                # Byte-for-byte, not merely equivalent. An immutable snapshot that was rewritten
                # with the same content would still have broken the promise.
                unchanged = {
                    f.relative_to(pinned_dir).as_posix(): hashlib.sha256(f.read_bytes()).hexdigest()
                    for f in sorted(pinned_dir.rglob("*"))
                    if f.is_file()
                }
                assert unchanged == digest, "a pinned snapshot's bytes changed"
                assert all(
                    os.stat(pinned_dir / path).st_mtime_ns == mtime
                    for path, mtime in mtimes.items()
                ), "a pinned snapshot's files were rewritten"


async def test_durable_acquisition_interests_cancel_only_the_detached_caller(tmp_path: Path):
    """Two real adapters share one held acquisition; one cancellation leaves a real result."""
    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    with serve(tmp_path / "upstream") as upstream:
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        entered, released = threading.Event(), threading.Event()
        upstream.held_paths["/static/ops_demo-1.0-py3-none-any.whl"] = (entered, released)
        with daemon.running(env):
            async with (
                Client(daemon.transport(env)) as first,
                Client(daemon.transport(env)) as second,
            ):
                params = {"ecosystem": "python", "name": "ops-demo", "version": "1.0"}
                left = (await first.call_tool("resolve_library", params)).structured_content
                right = (await second.call_tool("resolve_library", params)).structured_content
                assert left["status"] == right["status"] == "pending"
                assert left["job"]["job_id"] == right["job"]["job_id"]
                assert left["data"]["interest_token"] != right["data"]["interest_token"]
                assert await asyncio.to_thread(entered.wait, 30)
                try:
                    detached = await call(
                        first,
                        "job_control",
                        job_id=left["job"]["job_id"],
                        action="cancel",
                        interest_token=left["data"]["interest_token"],
                    )
                    assert detached["data"]["active_interests"] == 1
                    assert detached["data"]["state"] == "running"
                finally:
                    released.set()
                    upstream.held_paths.clear()
                completed = await daemon.wait_for_answer(second, right)
                assert completed["status"] in {"ok", "partial"}, completed
                assert completed["data"]["release"]["version"] == "1.0"
                assert completed["snapshot_id"]
                journal = json.loads(
                    (
                        Path(env["LIBENR_DATA_HOME"])
                        / "jobs/terminal"
                        / f"{right['job']['job_id']}.json"
                    ).read_text()
                )
                stage = journal["resolution"]
                assert stage["release_id"] == completed["data"]["release"]["release_id"]
                assert stage["context_id"] == completed["context_id"]
                assert stage["result_artifact_id"] in stage["input_artifact_ids"]
                assert journal["state"] in {"succeeded", "partial"}
                before = len(upstream.request_log)
                replay = await call(first, "resolve_library", **params)
                assert replay["snapshot_id"] == completed["snapshot_id"]
                assert replay["data"]["answered_from_cache"]
                assert len(upstream.request_log) == before
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                stored = await call(
                    client, "job_control", job_id=right["job"]["job_id"], action="status"
                )
                assert stored["data"]["result"]["snapshot_id"] == completed["snapshot_id"]
                assert (await call(client, "service_status"))["data"]["health"]["single_flight"][
                    "started"
                ] == 0


@pytest.mark.parametrize(
    "point",
    [
        "snapshot_files_durable",
        "snapshot_renamed",
        "catalog_files_durable",
        "catalog_root_durable",
        "journal_terminal_durable",
    ],
)
async def test_sigkill_at_each_publication_boundary_recovers_without_repeating_producers(
    tmp_path, point
):
    """C06: real SIGKILL at finite debug barriers, with exact post-restart visibility."""
    import uuid

    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    state = tmp_path / "state"
    with serve(tmp_path / "upstream") as upstream:
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(state, config)
        with (tmp_path / "crashed-daemon.log").open("wb") as log:
            process = subprocess.Popen(
                [str(daemon.DAEMON_BIN), "start"], env=env, stdout=subprocess.DEVNULL, stderr=log
            )
            try:
                daemon.await_socket(Path(env["LIBENR_SOCKET"]), process)
                control = state / "data/.publication-probe"
                control.mkdir()
                token = uuid.uuid4().hex
                (control / "armed.json").write_text(json.dumps({"point": point, "token": token}))
                async with Client(daemon.transport(env)) as client:
                    pending = (
                        await client.call_tool(
                            "resolve_library",
                            {
                                "ecosystem": "python",
                                "name": "ops-demo",
                                "version": "1.0",
                            },
                        )
                    ).structured_content
                    assert pending["status"] == "pending", pending
                    deadline = time.monotonic() + 40
                    while not (control / "reached.json").exists():
                        assert process.poll() is None, (tmp_path / "crashed-daemon.log").read_text()
                        assert time.monotonic() < deadline, (
                            tmp_path / "crashed-daemon.log"
                        ).read_text()
                        await asyncio.sleep(0.025)
                    reached = json.loads((control / "reached.json").read_text())
                    assert reached == {"point": point, "token": token}
                    process.send_signal(signal.SIGKILL)
                    process.wait(timeout=10)
            finally:
                if process.poll() is None:
                    process.kill()
                    process.wait(timeout=10)
        job_id = pending["job"]["job_id"]
        journal = json.loads((state / "data/jobs/active" / f"{job_id}.json").read_text())
        assert journal["resolution"]["context_id"]
        requests = list(upstream.request_log)
        committed = point in {"catalog_root_durable", "journal_terminal_durable"}
        assert (state / "data/catalog/current.json").exists() == committed
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                recovered = await call(client, "job_control", job_id=job_id, action="status")
                assert recovered["status"] == "ok", recovered
                result = recovered["data"]["result"]
                if committed:
                    assert recovered["data"]["state"] in {"succeeded", "partial"}
                    assert result["context_id"] == journal["resolution"]["context_id"]
                    assert result["data"]["release"]["version"] == "1.0"
                    overview = await call(
                        client,
                        "library_overview",
                        context_id=result["context_id"],
                        snapshot_id=result["snapshot_id"],
                    )
                    assert overview["status"] in {"ok", "partial"}, overview
                else:
                    assert recovered["data"]["state"] == "failed"
                    assert "restarted" in result["error"]["message"]
                    overview = await call(
                        client, "library_overview", context_id=journal["resolution"]["context_id"]
                    )
                    assert overview["status"] == "error"
                assert upstream.request_log == requests, "restart repeated a producer"
                assert (await call(client, "service_status"))["data"]["health"]["single_flight"][
                    "started"
                ] == 0
                assert not list((state / "data/jobs/active").glob("*.json"))
                assert not list((state / "data/staging").iterdir())
