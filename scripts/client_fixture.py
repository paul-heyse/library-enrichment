"""Fresh upstream bytes and a real daemon for authenticated client acceptance only."""

from __future__ import annotations

import asyncio
import json
import os
import re
import sys
from collections.abc import Iterator
from contextlib import contextmanager
from pathlib import Path

from client_events import SERVICE, Invocation, Trace
from launch_configuration import describe

from enrichment_mcp.daemon_client import DaemonClient
from enrichment_mcp.envelope import validate_document
from enrichment_mcp.presentation import error_preview

ROOT = Path(__file__).resolve().parent.parent


def validate_summary(summary: dict, exit_code: int, expected: set[str]) -> None:
    """A harness failure cannot be hidden behind a duplicated or incomplete result set."""
    entries = summary["results"]
    gates = [entry["gate"] for entry in entries]
    if len(gates) != len(set(gates)) or set(gates) != expected:
        raise ValueError("client summary has duplicate, missing or unexpected scenarios")
    if any(entry["status"] not in {"passed", "failed", "blocked"} for entry in entries):
        raise ValueError("client summary has an invalid scenario outcome")
    succeeded = summary["real_user_directories_unchanged"] and all(
        entry["status"] == "passed" for entry in entries
    )
    if exit_code != (0 if succeeded else 1):
        raise ValueError("client harness exit code contradicts its summary")


@contextmanager
def service(
    root: Path, enabled: bool, *, installation: Path | None = None
) -> Iterator[dict | None]:
    """Use the shared HTTP fixtures and production launch description, with no prewarming."""
    if not enabled:
        yield None
        return
    sys.path.insert(0, str(ROOT / "tests"))
    from e2e.test_python_fixture import build_upstream
    from e2e.test_runtime_objects import upstream_fixture
    from support import daemon
    from support.fixture_upstream import serve

    runtime_root = installation.resolve() if installation is not None else ROOT
    fixtures = root / "fixtures"
    build_upstream(fixtures, root / "must-not-import")
    runtime = root / "runtime-fixtures"
    upstream_fixture(runtime)
    with (
        serve(ROOT / "tests/fixtures/upstream") as rust,
        serve(fixtures) as python,
        serve(runtime) as runtime_python,
    ):
        config = root / "service.toml"
        rust_index = "https://index.crates.io" if root.name == "A04" else f"{rust.base_url}/index"
        rust_api = "https://crates.io/api/v1" if root.name == "A04" else f"{rust.base_url}/api/v1"
        rust_docs = "https://docs.rs" if root.name == "A04" else rust.base_url
        selected = runtime_python if root.name.startswith("runtime-") else python
        config.write_text(
            '[policy]\nenabled_profiles=["static","build","runtime"]\n'
            "[limits]\ninline_wait_seconds=0\n"
            f'[producers.rust]\ncrates_io_index_url="{rust_index}"\n'
            f'crates_io_api_url="{rust_api}"\ndocs_rs_url="{rust_docs}"\n'
            f'[producers.python]\npypi_url="{selected.base_url}/pypi"\n'
            f'simple_url="{selected.base_url}/simple"\n'
            f"worker_python={json.dumps(str(runtime_root / '.venv/bin/python'))}\n"
        )
        execution_root = os.environ.get("LIBENR_EXECUTION_TEST_ROOT")
        if execution_root:
            with config.open("a") as stream:
                stream.write(f"\n[execution]\nstorage_root={json.dumps(execution_root)}\n")
                for language in ("python", "rust"):
                    image = os.environ.get(f"LIBENR_EXECUTION_TEST_{language.upper()}")
                    if image:
                        stream.write(f"{language}_image={json.dumps(image)}\n")
        env = daemon.daemon_env(root / "s", config)
        launch = describe(
            root / "s", config, "release" if installation else "debug", installation=installation
        )
        (root / "launch.json").write_text(json.dumps(launch, indent=2) + "\n")
        with daemon.running(env, cwd=root, binary=Path(launch["daemon"]["command"])) as running:
            try:
                yield launch
            finally:
                (root / "requests.json").write_text(
                    json.dumps(
                        {
                            "rust": rust.request_log,
                            "python": python.request_log,
                            "runtime": runtime_python.request_log,
                        },
                        indent=2,
                    )
                )
                (root / "daemon.log").write_text(running.log)
        if (root / "must-not-import").exists():
            raise ValueError("static acquisition imported the fixture")


async def _witness(trace: Trace, socket: Path) -> dict:
    """Check the client's identities against the daemon, without finishing its research for it."""
    client = DaemonClient(socket, timeout_seconds=30)
    snapshots = {}
    artifacts = {}
    failures = {}
    for call in trace.calls:
        if call.server != SERVICE or call.completed is None:
            continue
        preview = call.failure_preview()
        if preview is not None and preview.get("job_id"):
            response = (
                await client.call("job.control", {"job_id": preview["job_id"], "action": "status"})
            )["result"]
            valid, reason = validate_document(json.dumps(response))
            if not valid or response["data"].get("state") != "failed":
                raise ValueError(f"independent failed-job witness failed: {reason}")
            expected = error_preview(response, compact=preview.get("details_omitted", False))
            if {k: v for k, v in preview.items() if k != "request_id"} != {
                k: v for k, v in expected.items() if k != "request_id"
            }:
                raise ValueError("independent failed-job preview differs from native journal")
            failures[preview["job_id"]] = response
        if call.error:
            continue
        answer = trace.answer_for(call)
        if answer["status"] not in {"ok", "partial"}:
            continue
        pairs = []
        data = answer["data"]
        snapshot = data.get("snapshot")
        if isinstance(snapshot, dict):
            pairs.append((answer["context_id"], snapshot["snapshot_id"]))
        for side in ("before", "after"):
            value = data.get(side)
            if call.tool == "compare_releases" and isinstance(value, dict):
                pairs.append((value["context_id"], value["snapshot_id"]))
        if answer.get("snapshot_id") and answer.get("context_id"):
            pairs.append((answer["context_id"], answer["snapshot_id"]))
        if call.arguments.get("snapshot_id") and call.arguments.get("context_id"):
            pairs.append((call.arguments["context_id"], call.arguments["snapshot_id"]))
        for context, snapshot_id in pairs:
            response = (await client.call("snapshot.manifest", {"snapshot_id": snapshot_id}))[
                "result"
            ]
            valid, reason = validate_document(json.dumps(response))
            if (
                not valid
                or response["status"] not in {"ok", "partial"}
                or response["context_id"] != context
                or response["snapshot_id"] != snapshot_id
            ):
                raise ValueError(f"independent snapshot witness failed: {reason}")
            snapshots[snapshot_id] = response
        if call.tool == "read_artifact":
            response = (await client.call("artifact.read", call.arguments))["result"]
            valid, reason = validate_document(json.dumps(response))
            if not valid or {k: v for k, v in response.items() if k != "request_id"} != {
                k: v for k, v in answer.items() if k != "request_id"
            }:
                raise ValueError(f"independent artifact witness failed: {reason}")
            artifacts[data["artifact"]["artifact_id"]] = {
                "sha256": data["artifact"]["sha256"],
                "content_digest": data["content_digest"],
            }
    return {"snapshots": snapshots, "artifacts": artifacts, "failures": failures}


def witness(trace: Trace, launch: dict | None) -> dict:
    if launch is None:
        if any(call.server == SERVICE for call in trace.calls):
            raise ValueError("service-absent scenario invoked an unregistered service")
        return {"service_registered": False}
    return asyncio.run(_witness(trace, Path(launch["daemon"]["env"]["LIBENR_SOCKET"])))


def assess(gate: str, trace: Trace) -> None:
    """Require usable scenario outcomes from correlated native events."""

    def require(tool: str) -> list[Invocation]:
        calls = trace.successful(tool)
        if not calls:
            raise ValueError(f"client did not obtain a completed usable {tool} result")
        return calls

    if gate in {"A01", "A02"}:
        for tool in ("service_status", "resolve_library", "search_evidence", "read_artifact"):
            require(tool)
    elif gate == "A03" or gate.startswith("discovery-"):
        breadth = require("library_overview")[0]
        depth = trace.successful("search_evidence") + trace.successful("inspect_symbol")
        completed = trace.usable_at(breadth)
        if not depth or completed is None or min(c.started for c in depth) <= completed:
            raise ValueError("research did not finish breadth before beginning depth")
    elif gate == "A04":
        require("resolve_library")
        if (
            not trace.successful("resolve-library-id", server="context7")
            or not trace.successful("query-docs", server="context7")
            or "1.0.228" not in trace.answer
        ):
            raise ValueError("missing real Context7 lookup or exact-version qualification")
    elif gate == "A05":
        require("inspect_symbol")
        if trace.successful("library_overview"):
            raise ValueError("known-symbol lookup unnecessarily enumerated the library")
    elif gate == "A06":
        if any(c.server == SERVICE for c in trace.calls) or not any(
            word in trace.answer.lower()
            for word in (
                "unavailable",
                "not available",
                "not registered",
                "not installed",
                "needs to be registered",
            )
        ):
            raise ValueError("client did not acknowledge the actually absent service")
    elif gate.startswith("upgrade-"):
        result = trace.answer_for(require("compare_releases")[0])["data"]
        if (
            not result["changes"]
            or result["before"]["release"]["version"] != "1.0"
            or result["after"]["release"]["version"] != "2.0"
        ):
            raise ValueError("upgrade research lacks an actual two-release comparison")
    elif gate.startswith("runtime-"):
        results = [trace.answer_for(c) for c in require("inspect_symbol")]
        facts = [o for r in results for o in r["data"]["execution_observations"]]
        if not any(
            o["payload"]["kind"] == "runtime_object"
            and o["payload"]["value"]["outcome"] == "results"
            for o in facts
        ):
            raise ValueError("runtime research lacks a completed runtime object observation")
    elif gate.startswith("recovery-"):
        resolved = [trace.answer_for(c) for c in require("resolve_library")]
        inspected = [trace.answer_for(c) for c in require("inspect_symbol")]
        attempted = any(
            c.tool == "resolve_library" and c.arguments.get("version") == "9.9.9"
            for c in trace.calls
            if c.server == SERVICE
        )
        failures = []
        for c in trace.calls:
            if c.server != SERVICE or c.completed is None:
                continue
            try:
                native = c.envelope()
            except ValueError:
                preview = c.failure_preview()
                if preview is not None:
                    failures.append(preview)
                continue
            if native["status"] == "error" or (
                c.tool == "job_control" and native["data"].get("state") == "failed"
            ):
                failures.append(native)
        recovered = any(
            r["data"]["release"]["version"] == "0.2.0"
            and any(i["context_id"] == r["context_id"] for i in inspected)
            for r in resolved
        )
        if not attempted or not failures or not recovered:
            raise ValueError("recovery lacks the failed request and exact usable replacement")
    if gate not in {"A01", "A02", "A06"}:
        check_citations(trace)


def check_citations(trace: Trace) -> None:
    """Cited service identities must occur in the client's completed validated evidence."""
    kinds = r"ctx|snap|ev_ent|ev|art|obs|frag|fragment|def|bind|symbol|run|rel|env|exec"
    pattern = re.compile(rf"\b(?:{kinds})_[a-f0-9]{{16,64}}\b(?!…|\.\.\.)")
    observed: set[str] = set()
    digests: set[str] = set()
    for call in trace.calls:
        if call.server != SERVICE or call.completed is None or call.error:
            continue
        result = trace.answer_for(call)
        if result["status"] in {"ok", "partial"}:
            observed.update(pattern.findall(json.dumps(result)))
            digests.update(re.findall(r"\b[a-f0-9]{64}\b", json.dumps(result)))
    cited = set(pattern.findall(trace.answer))
    abbreviated = re.findall(
        rf"\b(?:(?P<kind>{kinds})_)?(?P<head>[a-f0-9]{{8,63}})(?:…|\.\.\.)(?P<tail>[a-f0-9]{{0,63}})",
        trace.answer,
    )
    # A displayed short identity remains a citation only when it selects exactly one
    # identity in the completed evidence. Ambiguous prefixes never establish support.
    unique = True
    for kind, head, tail in abbreviated:
        choices = {
            identity.rsplit("_", 1)[1]
            for identity in observed
            if not kind or identity.startswith(kind + "_")
        }
        if not kind:
            choices |= digests
            # A content-addressed artifact's short identity and its full sha256 denote
            # the same digest prefix. Count its maximal observed digest only once.
            choices = {v for v in choices if not any(w != v and w.startswith(v) for w in choices)}
        unique &= sum(v.startswith(head) and v.endswith(tail) for v in choices) == 1
    if not (cited or abbreviated) or not cited <= observed or not unique:
        raise ValueError("research brief cites no actual evidence or invents service identities")
