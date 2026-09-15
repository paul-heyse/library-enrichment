"""Probe scope: what a verification establishes, and what it deliberately does not.

Gates C16, P09, P10 and R10. Each drives a real daemon over real MCP against a real container;
the point of every one of them is a distinction that is easy to collapse by accident -- a
resolved environment overwriting a declared one, a typecheck pass being read as runtime
correctness, a probe borrowing the service's own interpreter, or a missing sandbox quietly
becoming host execution.
"""

import hashlib
import json
import os
import sys
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
        not ROOT or not IMAGE,
        reason=(
            "set LIBENR_EXECUTION_TEST_ROOT and LIBENR_EXECUTION_TEST_PYTHON "
            "after execution-image setup"
        ),
    ),
]


def upstream_fixture(root: Path) -> None:
    (root / "static").mkdir(parents=True)
    (root / "pypi/scope-demo").mkdir(parents=True)
    name = "scope_demo-1.0-py3-none-any.whl"
    wheel = root / "static" / name
    with zipfile.ZipFile(wheel, "w") as archive:
        archive.writestr(
            "scope_demo/__init__.py", "def halve(value: int) -> int:\n    return value // 2\n"
        )
        archive.writestr("scope_demo/py.typed", "")
        archive.writestr(
            "scope_demo-1.0.dist-info/METADATA",
            "Metadata-Version: 2.4\nName: scope-demo\nVersion: 1.0\nRequires-Python: >=3.10\n\n",
        )
        archive.writestr(
            "scope_demo-1.0.dist-info/WHEEL",
            "Wheel-Version: 1.0\nRoot-Is-Purelib: true\nTag: py3-none-any\n",
        )
        archive.writestr("scope_demo-1.0.dist-info/RECORD", "")
    (root / "pypi/scope-demo/1.0.json").write_text(
        json.dumps(
            {
                "info": {"name": "scope-demo", "version": "1.0"},
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
    (root / "pypi/scope-demo/index.json").write_text(json.dumps({"versions": ["1.0"]}))


def configuration(path: Path, base: str, broker: str | None = None) -> None:
    extra = f"broker_path={json.dumps(broker)}\n" if broker else ""
    path.write_text(f"""[policy]
enabled_profiles=["static","build","runtime"]
[limits]
inline_wait_seconds=0
[execution]
storage_root={json.dumps(ROOT)}
python_image={json.dumps(IMAGE)}
deadline_seconds=60
{extra}[producers.python]
pypi_url="{base}/pypi"
simple_url="{base}/simple"
""")


async def call(client, tool, **params):
    result = await daemon.read_complete_answer(
        client, (await client.call_tool(tool, params)).structured_content
    )
    return await daemon.wait_for_answer(client, result) if tool == "resolve_library" else result


async def finish(client, pending):
    if pending["status"] != "pending":
        return pending
    for _ in range(30):
        result = await call(
            client, "job_control", job_id=pending["job"]["job_id"], action="wait", wait_seconds=5
        )
        assert result["status"] == "ok", result
        if result["data"]["result"] is not None:
            return result["data"]["result"]
    pytest.fail("verification exceeded bounded job polling")


async def resolve(client):
    resolved = await call(
        client,
        "resolve_library",
        ecosystem="python",
        name="scope-demo",
        version="1.0",
        python_version="3.14",
    )
    assert resolved["status"] != "error", resolved
    return resolved


async def test_a_resolved_environment_derives_a_readable_context_and_leaves_the_original(
    tmp_path: Path,
):
    """C16: resolving an environment creates a new ID; it never redefines the old one."""
    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    with serve(tmp_path / "upstream") as upstream:
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                declared = await resolve(client)
                original = declared["context_id"]
                original_snapshot = declared["snapshot_id"]
                assert declared["data"]["environment"]["resolution"] == "declared"

                verified = await finish(
                    client,
                    await call(
                        client,
                        "verify_usage",
                        context_id=original,
                        snippet="from scope_demo import halve\nvalue: int = halve(4)\n",
                        mode="typecheck",
                        profile="build",
                    ),
                )
                assert verified["status"] == "ok", verified
                derived = verified["data"]["derived_context"]["context_id"]
                assert derived != original
                assert (
                    verified["data"]["environment"]["environment_id"]
                    != declared["data"]["environment"]["environment_id"]
                )
                assert verified["data"]["environment"]["resolution"] == "resolved"

                # The child owns a coherent successor with newly bound static declarations
                # and the executed observation. The source context remains pinned below.
                overview = await call(client, "library_overview", context_id=derived)
                assert overview["status"] != "error", overview
                assert overview["snapshot_id"] == verified["snapshot_id"]
                assert overview["snapshot_id"] != original_snapshot

                # And the original still means exactly what it meant before.
                again = await resolve(client)
                assert again["context_id"] == original
                assert again["snapshot_id"] == original_snapshot
                assert again["data"]["environment"]["resolution"] == "declared"


async def test_typecheck_success_and_runtime_failure_remain_distinct_observations(tmp_path: Path):
    """P09: the same snippet type-checks and then fails at run time. Both facts survive."""
    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    with serve(tmp_path / "upstream") as upstream:
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                context = (await resolve(client))["context_id"]
                snippet = "from scope_demo import halve\nvalue: int = halve(5)\nassert value == 3\n"

                typechecked = await finish(
                    client,
                    await call(
                        client,
                        "verify_usage",
                        context_id=context,
                        snippet=snippet,
                        mode="typecheck",
                        profile="build",
                    ),
                )
                assert typechecked["status"] == "ok", typechecked
                assert typechecked["data"]["evidence_class"] == "typechecker_observed"

                at_runtime = await finish(
                    client,
                    await call(
                        client,
                        "verify_usage",
                        context_id=context,
                        snippet=snippet,
                        mode="runtime",
                        profile="runtime",
                        test_intent="halve(5) is three",
                    ),
                )
                assert at_runtime["status"] == "error", at_runtime
                assert at_runtime["error"]["code"] == "VERIFICATION_FAILED"
                assert at_runtime["data"]["evidence_class"] == "runtime_observed"
                assert "AssertionError" in at_runtime["data"]["observations"][-1]["stderr"]

                # Two different classes over one snippet. A type-check pass is not a runtime
                # pass, and neither result is allowed to stand in for the other.
                assert typechecked["data"]["evidence_class"] != at_runtime["data"]["evidence_class"]
                assert (
                    typechecked["data"]["snippet_artifact_id"]
                    == (at_runtime["data"]["snippet_artifact_id"])
                ), "the same stored snippet, so the difference is the probe and not the input"
                assert typechecked["data"]["limitations"], "scope is always stated"


async def test_the_probe_records_its_own_interpreter_and_never_the_service_environment(
    tmp_path: Path,
):
    """P10: the consumer runs on the capsule's interpreter, with no leak from this process."""
    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    with serve(tmp_path / "upstream") as upstream:
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                context = (await resolve(client))["context_id"]
                result = await finish(
                    client,
                    await call(
                        client,
                        "verify_usage",
                        context_id=context,
                        snippet=(
                            "import json, os, sys\n"
                            "leaked = sorted(k for k in os.environ if k.startswith('LIBENR_'))\n"
                            "print(json.dumps({\n"
                            "    'executable': sys.executable,\n"
                            "    'version': list(sys.version_info[:3]),\n"
                            "    'path': sys.path,\n"
                            "    'leaked': leaked,\n"
                            "    'virtual_env': os.environ.get('VIRTUAL_ENV'),\n"
                            "    'pythonpath': os.environ.get('PYTHONPATH'),\n"
                            "}))\n"
                        ),
                        mode="runtime",
                        profile="runtime",
                        test_intent="record the selected interpreter",
                    ),
                )
                assert result["status"] == "ok", result
                observed = json.loads(result["data"]["observations"][-1]["stdout"])

                assert observed["executable"] == "/usr/local/bin/python3"
                assert observed["version"] == [3, 14, 7]
                assert observed["leaked"] == [], "no service environment variable reaches a probe"
                assert observed["virtual_env"] is None
                assert observed["pythonpath"] is None

                # Every import root is the capsule's or the image's own stdlib -- which includes
                # `python314.zip`, a real stdlib entry rather than a leak.
                for entry in observed["path"]:
                    assert entry.startswith(("/capsule", "/usr/local/lib/python3")), entry
                assert "/capsule/python" in observed["path"]

                # The decisive half is the negative one: nothing from this process's world.
                for leak in (str(Path(sys.executable).parent), str(Path.cwd()), str(ROOT)):
                    assert not any(leak in entry for entry in observed["path"]), leak

                # And the recorded environment names that interpreter, not ours.
                assert "3.14.7" in result["data"]["environment"]["toolchain"]


async def test_build_activity_without_a_working_sandbox_is_policy_denied(tmp_path: Path):
    """R10: an enabled profile plus a broken isolation backend is a denial, never host execution."""
    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    marker = tmp_path / "host-marker"
    marker.write_text("untouched")
    absent_broker = tmp_path / "no-such-broker"
    with serve(tmp_path / "upstream") as upstream:
        # `build` and `runtime` are enabled in configuration; the container broker does not
        # exist. Policy is enabled, isolation is not, and the two must not be confused.
        configuration(config, upstream.base_url, broker=str(absent_broker))
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                context = (await resolve(client))["context_id"]
                denied = await finish(
                    client,
                    await call(
                        client,
                        "verify_usage",
                        context_id=context,
                        snippet=(
                            f"import pathlib\n"
                            f"pathlib.Path({str(marker)!r}).write_text('host execution happened')\n"
                        ),
                        mode="runtime",
                        profile="runtime",
                    ),
                )
                assert denied["status"] == "error", denied
                assert denied["error"]["code"] == "POLICY_DENIED", denied
                assert not denied["error"]["retryable"]
                assert denied["error"]["next_action"]

    # The decisive assertion: nothing ran anywhere. A fallback to the host would have written
    # this file, and an ENVIRONMENT_UNRESOLVED would have described the wrong problem.
    assert marker.read_text() == "untouched"
    assert not absent_broker.exists()
