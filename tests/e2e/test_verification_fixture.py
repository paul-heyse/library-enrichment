"""Real MCP -> durable Rust jobs -> admitted container -> scoped verification evidence."""

import hashlib
import json
import os
import shlex
import sys
import threading
import time
import zipfile
from pathlib import Path

import anyio
import pytest
from fastmcp import Client

from support import daemon, execution
from support.fixture_upstream import serve

ROOT = os.environ.get("LIBENR_EXECUTION_TEST_ROOT")
IMAGE = os.environ.get("LIBENR_EXECUTION_TEST_PYTHON")
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


def upstream_fixture(root: Path, requirements: tuple[str, ...] = ()) -> None:
    (root / "static").mkdir(parents=True)
    (root / "pypi/probe-demo").mkdir(parents=True)
    name = "probe_demo-1.0-py3-none-any.whl"
    wheel = root / "static" / name
    with zipfile.ZipFile(wheel, "w") as archive:
        archive.writestr(
            "probe_demo/__init__.py", "def twice(value: int) -> int:\n    return value * 2\n"
        )
        archive.writestr("probe_demo/py.typed", "")
        archive.writestr(
            "probe_demo-1.0.dist-info/METADATA",
            "Metadata-Version: 2.4\nName: probe-demo\nVersion: 1.0\nRequires-Python: >=3.10\n"
            + "".join(f"Requires-Dist: {requirement}\n" for requirement in requirements)
            + "\n",
        )
        archive.writestr(
            "probe_demo-1.0.dist-info/WHEEL",
            "Wheel-Version: 1.0\nRoot-Is-Purelib: true\nTag: py3-none-any\n",
        )
        archive.writestr("probe_demo-1.0.dist-info/RECORD", "")
    record = {
        "info": {"name": "probe-demo", "version": "1.0"},
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
    (root / "pypi/probe-demo/1.0.json").write_text(json.dumps(record))
    (root / "pypi/probe-demo/index.json").write_text(json.dumps({"versions": ["1.0"]}))


def configuration(path: Path, base: str) -> None:
    path.write_text(f'''[policy]
enabled_profiles=["static","build","runtime"]
[limits]
inline_wait_seconds=0
[execution]
storage_root={json.dumps(ROOT)}
python_image={json.dumps(IMAGE)}
deadline_seconds=60
[producers.python]
pypi_url="{base}/pypi"
simple_url="{base}/simple"
worker_python={json.dumps(sys.executable)}
''')


async def call(client, tool, **params):
    result = await daemon.read_complete_answer(
        client, (await client.call_tool(tool, params, raise_on_error=False)).structured_content
    )
    return await daemon.wait_for_answer(client, result) if tool == "resolve_library" else result


async def finish(client, pending):
    return await daemon.wait_for_answer(client, pending)


async def test_verification_fixture_typecheck_runtime_and_derived_environment(tmp_path: Path):
    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    canary = tmp_path / "studied-repository"
    canary.mkdir()
    (canary / "keep.py").write_text("original bytes\n")
    before = hashlib.sha256((canary / "keep.py").read_bytes()).hexdigest()
    with serve(tmp_path / "upstream") as upstream:
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                resolved = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="probe-demo",
                    version="1.0",
                    python_version="3.14",
                )
                assert resolved["status"] != "error", resolved
                context = resolved["context_id"]
                valid = await call(
                    client,
                    "verify_usage",
                    context_id=context,
                    snippet="from probe_demo import twice\nvalue: int = twice(3)\n",
                    mode="typecheck",
                    profile="build",
                )
                assert valid["status"] == "pending", valid
                receipt = valid
                valid = await finish(client, valid)
                assert valid["status"] == "ok", valid
                assert valid["data"]["snippet_origin"] == "agent"
                assert valid["data"]["environment"]["resolution"] == "resolved"
                assert valid["data"]["derived_context"]["context_id"] != context
                assert "3.14.7" in valid["data"]["environment"]["toolchain"]
                assert valid["data"]["observations"][-1]["command"][0] == "/opt/producers/bin/ty"
                invalid = await finish(
                    client,
                    await call(
                        client,
                        "verify_usage",
                        context_id=context,
                        snippet="from probe_demo import twice\nvalue: str = twice(3)\n",
                        mode="typecheck",
                        profile="build",
                    ),
                )
                assert invalid["status"] == "error", invalid
                assert invalid["error"]["code"] == "VERIFICATION_FAILED"
                runtime = await finish(
                    client,
                    await call(
                        client,
                        "verify_usage",
                        context_id=context,
                        snippet="from probe_demo import twice\nassert twice(3) == 7\n",
                        mode="runtime",
                        profile="runtime",
                        test_intent="twice returns seven",
                    ),
                )
                assert runtime["status"] == "error", runtime
                assert "AssertionError" in runtime["data"]["observations"][-1]["stderr"]
                assert runtime["data"]["mode"] == "runtime"
                replay = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="probe-demo",
                    version="1.0",
                    python_version="3.14",
                    freshness="offline",
                )
                assert replay["context_id"] == context
                assert replay["data"]["environment"]["resolution"] == "declared"
                job_id = receipt["job"]["job_id"]
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                persisted = await call(client, "job_control", job_id=job_id)
                assert persisted["data"]["state"] == "succeeded"
                recovered = await daemon.read_terminal_answer(client, persisted)
                assert (
                    recovered["data"]["result_artifact_id"] == valid["data"]["result_artifact_id"]
                )
    assert hashlib.sha256((canary / "keep.py").read_bytes()).hexdigest() == before
    assert list((tmp_path / "state/cache/capsules").iterdir()) == []


async def test_verification_fixture_rust_compile_valid_invalid_and_runtime(tmp_path: Path):
    rust_image = os.environ.get("LIBENR_EXECUTION_TEST_RUST")
    if not rust_image:
        pytest.skip("set LIBENR_EXECUTION_TEST_RUST after execution-image setup")
    config = tmp_path / "service.toml"
    with serve(daemon.ROOT / "tests/fixtures/upstream") as upstream:
        configuration(config, upstream.base_url)
        text = config.read_text().replace(
            "deadline_seconds=60", f"rust_image={json.dumps(rust_image)}\ndeadline_seconds=60"
        )
        config.write_text(
            text
            + f'\n[producers.rust]\ncrates_io_index_url="{upstream.base_url}/index"\n'
            + f'crates_io_api_url="{upstream.base_url}/api/v1"\n'
            + f'docs_rs_url="{upstream.base_url}"\n'
        )
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                resolved = await call(
                    client,
                    "resolve_library",
                    ecosystem="rust",
                    name="enr-fixture",
                    version="0.1.0",
                    features=[],
                    default_features=True,
                )
                assert resolved["status"] != "error", resolved
                context = resolved["context_id"]
                valid = await finish(
                    client,
                    await call(
                        client,
                        "verify_usage",
                        context_id=context,
                        snippet="fn main() { let _ = std::mem::size_of::<enr_fixture::Widget>(); }",
                        mode="compile",
                        profile="build",
                    ),
                )
                assert valid["status"] == "ok", valid
                assert valid["data"]["observations"][-1]["command"][1] == "+1.98.1"
                invalid = await finish(
                    client,
                    await call(
                        client,
                        "verify_usage",
                        context_id=context,
                        snippet="fn main() { let _: u32 = false; }",
                        mode="compile",
                        profile="build",
                    ),
                )
                assert invalid["status"] == "error", invalid
                assert invalid["error"]["code"] == "VERIFICATION_FAILED"
                assert "mismatched types" in invalid["data"]["observations"][-1]["stderr"]
                runtime = await finish(
                    client,
                    await call(
                        client,
                        "verify_usage",
                        context_id=context,
                        snippet="fn main() { assert_eq!(2+2, 5); }",
                        mode="runtime",
                        profile="runtime",
                    ),
                )
                assert runtime["status"] == "error", runtime
                assert "panicked" in runtime["data"]["observations"][-1]["stderr"]


async def test_verification_fixture_crash_recovery_stops_owned_descendants(tmp_path: Path):
    import asyncio
    import subprocess

    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    with serve(tmp_path / "upstream") as upstream:
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        process = subprocess.Popen(
            [str(daemon.DAEMON_BIN), "start"],
            env=env,
            cwd=tmp_path,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
        )
        try:
            daemon.await_socket(Path(env["LIBENR_SOCKET"]), process)
            async with Client(daemon.transport(env)) as client:
                resolved = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="probe-demo",
                    version="1.0",
                    python_version="3.14",
                )
                result = await call(
                    client,
                    "verify_usage",
                    context_id=resolved["context_id"],
                    snippet=(
                        "import os,time\nfrom pathlib import Path\nos.fork()\nwhile True:\n"
                        " Path('/capsule/heartbeat').write_text(str(time.time_ns()))\n"
                        " time.sleep(.05)\n"
                    ),
                    mode="runtime",
                    profile="runtime",
                )
                assert result["status"] == "pending", result
                job_id = result["job"]["job_id"]
                capsule = tmp_path / "state/cache/capsules" / job_id
                root = Path(ROOT or "")
                for _ in range(100):
                    if await execution.heartbeat(root, capsule):
                        break
                    await asyncio.sleep(0.1)
                assert await execution.heartbeat(root, capsule), (
                    "real runtime producer never started"
                )
                container = execution.container_name(root, capsule)
                assert container is not None
                # A second daemon must not reconcile/kill the first daemon's live work.
                duplicate = subprocess.run(
                    [str(daemon.DAEMON_BIN), "start"],
                    env=env,
                    cwd=tmp_path,
                    capture_output=True,
                    timeout=10,
                    check=False,
                )
                assert duplicate.returncode != 0
                assert b"another writer" in duplicate.stderr
                # Different data roots must still exclude competing cache-owner recovery.
                alternate = daemon.daemon_env(tmp_path / "alternate-state", config)
                alternate["LIBENR_CACHE_HOME"] = env["LIBENR_CACHE_HOME"]
                before_duplicate = await execution.heartbeat(root, capsule)
                duplicate_cache = subprocess.run(
                    [str(daemon.DAEMON_BIN), "start"],
                    env=alternate,
                    cwd=tmp_path,
                    capture_output=True,
                    timeout=10,
                    check=False,
                )
                assert duplicate_cache.returncode != 0
                assert b"another writer owns this cache root" in duplicate_cache.stderr
                await asyncio.sleep(0.2)
                assert await execution.heartbeat(root, capsule) != before_duplicate
                process.kill()
                process.wait(timeout=10)
        finally:
            if process.poll() is None:
                process.kill()
                process.wait(timeout=10)
            if process.stderr:
                process.stderr.close()
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                recovered = await call(client, "job_control", job_id=job_id)
                assert recovered["data"]["state"] == "failed", recovered
                assert "interrupted" in recovered["data"]["stage"]
                assert (
                    await execution.broker(root, "container", "exists", container)
                ).returncode == 1
                assert await execution.heartbeat(root, capsule) is None
                assert not any(
                    json.loads(p.read_text()).get("capsule") == str(capsule)
                    for p in (root / "owned").glob("*.json")
                )


def dependency_fixture(root: Path, requirement: str | None = None) -> None:
    name = "child_dep-1.0-py3-none-any.whl"
    wheel = root / "static" / name
    with zipfile.ZipFile(wheel, "w") as archive:
        archive.writestr("child_dep/__init__.py", "def answer() -> int:\n    return 42\n")
        archive.writestr("child_dep/py.typed", "")
        archive.writestr(
            "child_dep-1.0.dist-info/METADATA",
            "Metadata-Version: 2.4\nName: child-dep\nVersion: 1.0\nRequires-Python: >=3.10\n"
            + (f"Requires-Dist: {requirement}\n" if requirement else "")
            + "\n",
        )
        archive.writestr(
            "child_dep-1.0.dist-info/WHEEL",
            "Wheel-Version: 1.0\nRoot-Is-Purelib: true\nTag: py3-none-any\n",
        )
        archive.writestr("child_dep-1.0.dist-info/RECORD", "")
    metadata = root / "pypi/child-dep"
    metadata.mkdir()
    metadata.joinpath("registry.json").write_text(
        json.dumps(
            {
                "releases": {
                    "1.0": [
                        {
                            "filename": name,
                            "packagetype": "bdist_wheel",
                            "url": "{{BASE_URL}}/static/" + name,
                            "digests": {"sha256": hashlib.sha256(wheel.read_bytes()).hexdigest()},
                            "requires_python": ">=3.10",
                            "yanked": False,
                        }
                    ]
                }
            }
        )
    )


@pytest.mark.parametrize("hostile", [False, True])
async def test_verification_fixture_admits_transitive_metadata_before_following_sources(
    tmp_path: Path, hostile: bool
):
    root = tmp_path / "upstream"
    upstream_fixture(root, ("child-dep>=1; python_version >= '3.10'",))
    config = tmp_path / "service.toml"
    with serve(root) as upstream:
        dependency_fixture(
            root, f"forbidden @ {upstream.base_url}/_test/never" if hostile else None
        )
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                resolved = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="probe-demo",
                    version="1.0",
                    python_version="3.14",
                )
                result = await finish(
                    client,
                    await call(
                        client,
                        "verify_usage",
                        context_id=resolved["context_id"],
                        snippet="from child_dep import answer\nassert answer() == 42\n",
                        mode="runtime",
                        profile="runtime",
                    ),
                )
                if hostile:
                    assert result["status"] == "error", result
                    assert result["error"]["code"] == "ENVIRONMENT_UNRESOLVED"
                    assert "admitted" in result["error"]["message"]
                else:
                    assert result["status"] == "ok", result
                    lock = await call(
                        client, "read_artifact", artifact_id=result["data"]["lock_artifact_id"]
                    )
                    assert "child-dep" in json.dumps(lock)
                assert not any(
                    "forbidden" in path or "never" in path for path in upstream.request_log
                )
                assert not any(
                    "uv" in argument and "compile" in observation["command"]
                    for observation in result.get("data", {}).get("observations", [])
                    for argument in observation["command"]
                )


async def test_verification_fixture_cancels_during_dependency_transfer(tmp_path: Path):
    root = tmp_path / "upstream"
    upstream_fixture(root, ("child-dep>=1",))
    dependency_fixture(root)
    entered, release = threading.Event(), threading.Event()
    config = tmp_path / "service.toml"
    with serve(root) as upstream:
        upstream.held_paths["/static/child_dep-1.0-py3-none-any.whl"] = (entered, release)
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        try:
            with daemon.running(env):
                async with Client(daemon.transport(env)) as client:
                    resolved = await call(
                        client,
                        "resolve_library",
                        ecosystem="python",
                        name="probe-demo",
                        version="1.0",
                        python_version="3.14",
                    )
                    pending = await call(
                        client,
                        "verify_usage",
                        context_id=resolved["context_id"],
                        snippet="import child_dep\n",
                        mode="runtime",
                        profile="runtime",
                    )
                    with anyio.fail_after(15):
                        while not entered.is_set():
                            await anyio.sleep(0.02)
                    started = time.monotonic()
                    cancelled = await call(
                        client,
                        "job_control",
                        job_id=pending["job"]["job_id"],
                        action="cancel",
                        interest_token=pending["data"]["interest_token"],
                    )
                    assert cancelled["status"] == "ok", cancelled
                    result = await finish(client, pending)
                    assert result["error"]["code"] == "VERIFICATION_FAILED", result
                    terminal = await call(client, "job_control", job_id=pending["job"]["job_id"])
                    assert terminal["data"]["state"] == "cancelled", terminal
                    assert time.monotonic() - started < 3
                    assert not release.is_set(), "download remained blocked during cancellation"
                    assert list((tmp_path / "state/cache/capsules").iterdir()) == []
        finally:
            release.set()


async def test_cancelled_jobs_wait_only_for_their_own_confirmed_cleanup(tmp_path: Path):
    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    armed, release, refused = (tmp_path / name for name in ("armed", "release", "refused"))
    broker = tmp_path / "broker.sh"
    broker.write_text(
        '#!/bin/sh\nfor arg in "$@"; do\n'
        f' if [ "$arg" = rm ] && [ -e {shlex.quote(str(armed))} ]'
        f" && [ ! -e {shlex.quote(str(release))} ]; then\n"
        f"  touch {shlex.quote(str(refused))}; exit 1\n fi\ndone\n"
        'exec /usr/bin/podman "$@"\n'
    )
    broker.chmod(0o755)
    with serve(tmp_path / "upstream") as upstream:
        configuration(config, upstream.base_url)
        config.write_text(
            config.read_text()
            .replace(
                "inline_wait_seconds=0", "inline_wait_seconds=0\nexpensive_worker_concurrency=1"
            )
            .replace("[execution]", f"[execution]\nbroker_path={json.dumps(str(broker))}")
        )
        env = daemon.daemon_env(tmp_path / "state", config)
        assert ROOT is not None
        with execution.qualified_override(config, Path(ROOT)), daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                resolved = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="probe-demo",
                    version="1.0",
                    python_version="3.14",
                )
                busy = await call(
                    client,
                    "verify_usage",
                    context_id=resolved["context_id"],
                    snippet=(
                        "import time\nfrom pathlib import Path\nwhile True:\n"
                        " Path('/capsule/heartbeat').write_text(str(time.time_ns()))\n"
                        " time.sleep(.02)\n"
                    ),
                    mode="runtime",
                    profile="runtime",
                )
                assert busy["status"] == "pending", busy
                job = busy["job"]["job_id"]
                capsule = tmp_path / "state/cache/capsules" / job
                root = Path(ROOT or "")
                try:
                    with anyio.fail_after(15):
                        while await execution.heartbeat(root, capsule) is None:
                            await anyio.sleep(0.02)
                    queued = await call(
                        client,
                        "verify_usage",
                        context_id=resolved["context_id"],
                        snippet="value: int = 1\n",
                        mode="typecheck",
                        profile="build",
                    )
                    assert queued["status"] == "pending", queued
                    await call(
                        client,
                        "job_control",
                        job_id=queued["job"]["job_id"],
                        action="cancel",
                        interest_token=queued["data"]["interest_token"],
                    )
                    with anyio.fail_after(2):
                        await finish(client, queued)
                    status = await call(client, "job_control", job_id=queued["job"]["job_id"])
                    assert status["data"]["state"] == "cancelled", status
                    before = await execution.heartbeat(root, capsule)
                    await anyio.sleep(0.06)
                    assert await execution.heartbeat(root, capsule) != before, (
                        "queued cancellation stopped unrelated work"
                    )

                    armed.touch()
                    await call(
                        client,
                        "job_control",
                        job_id=job,
                        action="cancel",
                        interest_token=busy["data"]["interest_token"],
                    )
                    with anyio.fail_after(5):
                        while not refused.exists():
                            await anyio.sleep(0.02)
                    status = await call(client, "job_control", job_id=job)
                    assert status["data"]["state"] == "cancel_requested", status
                    assert status["data"]["result"] is None
                    assert capsule.exists(), "a live mount was deleted before confirmed absence"
                    assert any(
                        json.loads(path.read_text()).get("capsule") == str(capsule)
                        for path in (Path(ROOT or "") / "owned").glob("*.json")
                    )
                finally:
                    release.touch()
                await finish(client, busy)
                status = await call(client, "job_control", job_id=job)
                assert status["data"]["state"] == "cancelled", status
                assert not any(
                    json.loads(path.read_text()).get("capsule") == str(capsule)
                    for path in (Path(ROOT or "") / "owned").glob("*.json")
                )
                with anyio.fail_after(2):
                    while capsule.exists():
                        await anyio.sleep(0.02)


async def test_a_long_operation_returns_pending_and_finishes_through_ordinary_tools(
    tmp_path: Path,
):
    """C18: a pending result is completed with `job_control`, not with a native MCP task.

    The blueprint is explicit (7.4): "The default adapter does not depend on MCP background-task
    support. Expensive operations submit durable core jobs and return an ordinary `pending`
    envelope; `job_control` works with clients supporting ordinary tools." So the assertions
    here are about shape as much as outcome -- the handle a caller needs, on an envelope any MCP
    client can read, answered by a tool in the ordinary catalogue.
    """
    upstream_fixture(tmp_path.joinpath("upstream"))
    config = tmp_path.joinpath("service.toml")
    with serve(tmp_path.joinpath("upstream")) as upstream:
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path.joinpath("state"), config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                catalogue = {tool.name for tool in await client.list_tools()}
                assert "job_control" in catalogue, catalogue
                assert not {name for name in catalogue if "task" in name.lower()}, (
                    "job control is an ordinary tool; a native task surface would be a second "
                    "scheduler"
                )

                resolved = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="probe-demo",
                    version="1.0",
                    python_version="3.14",
                )
                pending = await call(
                    client,
                    "verify_usage",
                    context_id=resolved["context_id"],
                    snippet="from probe_demo import twice\nvalue: int = twice(3)\n",
                    mode="typecheck",
                    profile="build",
                )

                assert pending["status"] == "pending", pending
                handle = pending["job"]
                assert handle["job_id"].startswith("job_"), handle
                assert handle["state"] in {"queued", "running"}, handle
                assert handle["stage"], handle
                assert handle["poll_after_ms"] > 0, handle
                # The interest token is how one caller cancels without killing shared work.
                assert pending["data"]["interest_token"].startswith("interest_"), pending["data"]

                # Status, then wait, then the result -- all through the ordinary tool.
                status = await call(client, "job_control", job_id=handle["job_id"])
                assert status["status"] == "ok", status
                assert status["data"]["job_id"] == handle["job_id"]

                finished = await finish(client, pending)
                assert finished["status"] == "ok", finished
                assert finished["data"]["evidence_class"] == "typechecker_observed"

                terminal = await call(client, "job_control", job_id=handle["job_id"])
                assert terminal["data"]["state"] == "succeeded", terminal
                assert terminal["data"]["result"] is not None
