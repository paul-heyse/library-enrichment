"""Operational metrics and structured logs (blueprint §14.3).

§14.3 asks for cache hits and misses, producer duration, queue depth, fetched and response
bytes, LSP startups and reuse, verification outcomes and evidence gaps -- as **engineering
diagnostics, not a benchmarking project**. So what is tested here is not a number, which would
be a benchmark, but that each counter tracks the fact it claims to:

- a second request that the cache answers is a hit, not a miss;
- an answer that names a gap is counted as a gap and not as an error;
- bytes reused from the cache are not reported as bytes downloaded;
- the counters are scoped to one process, and `uptime_seconds` says which window.

The metrics have no gate ID -- the frozen acceptance plan has none for them -- so they are
verified here and run as part of `just test`.
"""

from __future__ import annotations

import hashlib
import json
import zipfile
from pathlib import Path
from typing import Any

from fastmcp import Client

from support import daemon
from support.fixture_upstream import serve

pytestmark = daemon.requires_daemon_binary

PACKAGE = "metrics-demo"
MODULE = "metrics_demo"


def upstream_fixture(root: Path) -> None:
    (root / "static").mkdir(parents=True, exist_ok=True)
    (root / f"pypi/{PACKAGE}").mkdir(parents=True, exist_ok=True)
    name = f"{MODULE}-1.0-py3-none-any.whl"
    wheel = root / "static" / name
    with zipfile.ZipFile(wheel, "w") as archive:
        archive.writestr(f"{MODULE}/__init__.py", "def thrice(v: int) -> int:\n    return v * 3\n")
        archive.writestr(f"{MODULE}/py.typed", "")
        archive.writestr(
            f"{MODULE}-1.0.dist-info/METADATA",
            f"Metadata-Version: 2.4\nName: {PACKAGE}\nVersion: 1.0\nRequires-Python: >=3.10\n\n",
        )
        archive.writestr(
            f"{MODULE}-1.0.dist-info/WHEEL",
            "Wheel-Version: 1.0\nRoot-Is-Purelib: true\nTag: py3-none-any\n",
        )
        archive.writestr(f"{MODULE}-1.0.dist-info/RECORD", "")
    (root / f"pypi/{PACKAGE}/1.0.json").write_text(
        json.dumps(
            {
                "info": {"name": PACKAGE, "version": "1.0"},
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
    (root / f"pypi/{PACKAGE}/index.json").write_text(json.dumps({"versions": ["1.0"]}))


def configuration(path: Path, base: str) -> None:
    # A long registry TTL is the point of the cache half of this test: the second resolve must
    # be answerable without asking the upstream again.
    path.write_text(f"""[policy]
enabled_profiles=["static"]
[limits]
inline_wait_seconds=30
[freshness]
registry_ttl_seconds=3600
[producers.python]
pypi_url="{base}/pypi"
simple_url="{base}/simple"
""")


async def call(client, tool: str, **params: Any) -> dict[str, Any]:
    result = await daemon.read_complete_answer(
        client, (await client.call_tool(tool, params, raise_on_error=False)).structured_content
    )
    return await daemon.wait_for_answer(client, result) if tool == "resolve_library" else result


async def health(client) -> dict[str, Any]:
    return (await call(client, "service_status"))["data"]["health"]


async def test_the_counters_track_what_they_claim_to(tmp_path: Path):
    """Each §14.3 counter moves for the fact it names, and stays put for the others."""
    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    with serve(tmp_path / "upstream") as upstream:
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                # A status call is itself a request, so the baseline is taken after it and the
                # deltas below are measured against a number that already includes one.
                start = await health(client)
                assert start["uptime_seconds"] >= 0
                # Zero, not one: a request is counted after its response is built, so a status
                # call never reports itself. That is the only self-consistent choice -- the
                # alternative would have `response_bytes` include a length it cannot know yet.
                assert start["evidence"]["requests"] == 0, start["evidence"]
                assert start["fetch"] == {
                    "hits": 0,
                    "revalidated": 0,
                    "misses": 0,
                    "failures": 0,
                    "fetched_bytes": 0,
                }, "nothing has been fetched yet"

                first = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name=PACKAGE,
                    version="1.0",
                    python_version="3.14",
                )
                assert first["status"] != "error", first
                cold = await health(client)
                native = cold["native_queries"]
                assert native["executions"] > start["native_queries"]["executions"]
                assert native["completed"] > 0 and native["admitted"] == 0
                assert native["executions"] == native["completed"] + native["incomplete"]
                assert native["planning_micros"] > 0 and native["elapsed_micros"] > 0
                assert native["concurrency_limit"] > 0
                assert native["managed_memory_limit_bytes"] > 0

                # Real bodies came over the wire, and the counter says how many bytes.
                assert cold["fetch"]["misses"] > 0, cold["fetch"]
                assert cold["fetch"]["fetched_bytes"] > 0, cold["fetch"]
                assert cold["fetch"]["failures"] == 0, cold["fetch"]
                # A producer ran, and it took a measurable amount of time.
                assert cold["single_flight"]["started"] >= 1, cold["single_flight"]
                assert cold["single_flight"]["inflight"] == 0, cold["single_flight"]
                # Retrieval never starts a language server (§9.2, and gate C17's counter).
                assert cold["lsp"]["started"] == 0, cold["lsp"]

                downloaded = cold["fetch"]["fetched_bytes"]
                second = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name=PACKAGE,
                    version="1.0",
                    python_version="3.14",
                )
                assert second["status"] != "error", second
                warm = await health(client)

    # The second resolve was answered without downloading the wheel again. This is the
    # assertion that makes `fetched_bytes` mean "transferred" rather than "served": if reused
    # cache bytes were counted, this number would have grown.
    assert warm["fetch"]["fetched_bytes"] == downloaded, (cold["fetch"], warm["fetch"])

    # Requests and response bytes are cumulative across every method, including status itself.
    assert warm["evidence"]["requests"] > cold["evidence"]["requests"]
    assert warm["evidence"]["response_bytes"] > cold["evidence"]["response_bytes"]
    # Every answer above was ok or partial; none was an error, and none was a verification.
    assert warm["evidence"]["errors"] == 0, warm["evidence"]
    assert warm["verification"] == {"succeeded": 0, "failed": 0, "unresolved": 0}

    # Gaps are counted separately from outcomes: an answer that names what is missing is still
    # a correct answer to a bounded question, so a gap is never more than one per request and
    # never implies a failure.
    assert warm["evidence"]["gaps"] <= warm["evidence"]["requests"], warm["evidence"]
    assert (
        warm["evidence"]["ok"]
        + warm["evidence"]["partial"]
        + warm["evidence"]["pending"]
        + warm["evidence"]["errors"]
        == warm["evidence"]["requests"]
    ), "every request lands in exactly one outcome bucket"

    # And the upstream agrees: it was asked for the wheel once, not twice.
    wheel_requests = [line for line in upstream.request_log if line.endswith(".whl")]
    assert len(wheel_requests) == 1, upstream.request_log


async def test_a_verification_the_service_refused_is_counted_as_unresolved(tmp_path: Path):
    """A probe the service could not run is `unresolved`, never `failed` and never silence.

    The failure this guards: every pre-probe refusal — no enabled profile, no admitted image, an
    unqualified host, a quarantined supervisor — returns before a probe exists. If none of them
    were counted, an operator on an unqualified host would read
    `{succeeded: 0, failed: 0, unresolved: 0}`, which is indistinguishable from nobody having
    asked. That is the one question the counter exists to answer.

    And it must not land in `failed`: a probe the service could not run says nothing about the
    caller's code, so attributing it to their library would be a false observation.
    """
    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    with serve(tmp_path / "upstream") as upstream:
        # `static` only: `verify_usage` needs `build`, so this is refused by policy in the core
        # before any capsule, image or container is involved.
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                context = (
                    await call(
                        client,
                        "resolve_library",
                        ecosystem="python",
                        name=PACKAGE,
                        version="1.0",
                        python_version="3.14",
                    )
                )["context_id"]
                before = await health(client)
                answer = await call(
                    client,
                    "verify_usage",
                    context_id=context,
                    snippet="from metrics_demo import thrice\nvalue: int = thrice(2)\n",
                    mode="typecheck",
                    profile="build",
                )
                assert answer["status"] == "error", answer
                assert answer["error"]["code"] == "POLICY_DENIED", answer["error"]
                after = await health(client)

    assert after["verification"]["unresolved"] == before["verification"]["unresolved"] + 1, (
        before["verification"],
        after["verification"],
    )
    assert after["verification"]["failed"] == 0, (
        "the snippet never ran, so nothing was observed about it"
    )
    assert after["verification"]["succeeded"] == 0


async def test_an_error_is_counted_as_an_error_and_not_as_a_gap(tmp_path: Path):
    """A rejected request still counts as a request, and never as evidence with a gap.

    The failure this guards: counting only successful answers would make `requests` exclude
    exactly the failures an operator opens `service_status` to find.
    """
    config = tmp_path / "service.toml"
    configuration(config, "http://127.0.0.1:1")
    env = daemon.daemon_env(tmp_path / "state", config)
    with daemon.running(env):
        async with Client(daemon.transport(env)) as client:
            before = await health(client)
            # A context id that was never issued. Rejected by the core, with no network and no
            # producer run.
            answer = await call(client, "library_overview", context_id="ctx_" + "0" * 16)
            assert answer["status"] == "error", answer
            after = await health(client)

    assert after["evidence"]["errors"] == before["evidence"]["errors"] + 1, (
        before["evidence"],
        after["evidence"],
    )
    assert after["evidence"]["gaps"] == before["evidence"]["gaps"], (
        "an error is not an answer with a gap"
    )
    assert after["single_flight"]["started"] == before["single_flight"]["started"], (
        "a rejected request must not start a producer run"
    )
    assert after["fetch"]["misses"] == 0, "a rejected request must not open a socket"


async def test_the_daemon_logs_one_structured_line_per_request(tmp_path: Path):
    """Diagnostics go to stderr as JSON, and carry no request payload.

    stdout is the MCP protocol channel; §7.4's habit is that the whole service keeps off it.
    The line deliberately records the method and the outcome, not what was asked about: a log
    that records which libraries someone researched is a different artifact than one that
    records that the service answered.
    """
    config = tmp_path / "service.toml"
    configuration(config, "http://127.0.0.1:1")
    env = daemon.daemon_env(tmp_path / "state", config)
    with daemon.running(env) as running:
        async with Client(daemon.transport(env)) as client:
            await call(client, "service_status")
        stderr = running.log

    lines = [
        json.loads(line)
        for line in stderr.splitlines()
        if line.startswith("{") and '"event":"rpc"' in line
    ]
    assert lines, f"no structured rpc lines in daemon stderr:\n{stderr}"
    status_lines = [line for line in lines if line["method"] == "service.status"]
    assert status_lines, [line["method"] for line in lines]
    entry = status_lines[0]
    assert entry["status"] in {"ok", "partial"}, entry
    assert entry["response_bytes"] > 0, entry
    assert entry["duration_ms"] >= 0, entry
    assert set(entry) == {
        "at",
        "event",
        "method",
        "status",
        "duration_ms",
        "response_bytes",
    }, "the log line carries no request payload"
