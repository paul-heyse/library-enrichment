"""Gates A01-A06, driven through real Codex and Claude Code.

These are `client`-marked and excluded from `just test` on purpose: they need a client binary,
credentials, and a network round trip to a model. `.claude/rules/evidence-truthfulness.md` is
categorical -- "a mocked client is never a pass for A01 or A02" -- so there is no fallback path
here. When a prerequisite is missing the test skips, the acceptance report records `blocked`
with the reason, and the gate does not pass.

Each test asserts on the client's own transcript: which tools it actually called and what it
said afterwards. The harness (`scripts/client-acceptance.py`) does the installing, registering
and running in a throwaway user directory, and these read what it recorded.
"""

from __future__ import annotations

import hashlib
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any

import pytest
from client_events import SERVICE, Trace, parse
from client_fixture import validate_summary

ROOT = Path(__file__).resolve().parents[2]

HARNESS = ROOT / "scripts/client-acceptance.py"
ADAPTER = ROOT / ".venv/bin/library-enrichment-mcp"

pytestmark = pytest.mark.client


@pytest.fixture(scope="session")
def transcripts() -> dict[str, Any]:
    """Run every scenario once, and hand each test its own transcript."""
    retained = ROOT / ".dev-state/logs/clients"
    retained.mkdir(parents=True, exist_ok=True)
    out = Path(tempfile.mkdtemp(prefix="run-", dir=retained))
    argv = [sys.executable, str(HARNESS), "--apply", "--out", str(out)]
    if os.environ.get("LIBENR_CLIENT_USE_OPERATOR_CREDENTIALS"):
        argv.append("--use-operator-credentials")
    if installation := os.environ.get("LIBENR_CLIENT_INSTALLATION"):
        argv.extend(["--installed", installation])
    result = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True, check=False)

    summary_path = out / "summary.json"
    if not summary_path.is_file():
        pytest.fail(f"the client harness produced no summary: {result.stderr.strip()[-600:]}")
    summary = json.loads(summary_path.read_text())
    expected = {f"A{i:02d}" for i in range(1, 7)} | {
        f"{journey}-{client}"
        for journey in ("discovery", "recovery", "upgrade", "runtime")
        for client in ("codex", "claude")
    }
    validate_summary(summary, result.returncode, expected)
    files = {}
    for path in sorted(out.rglob("*")):
        if path.is_file():
            with path.open("rb") as stream:
                digest = hashlib.file_digest(stream, "sha256").hexdigest()
                files[str(path.relative_to(out))] = digest
    return {"summary": summary, "out": out, "evidence": {"root": str(out), "files": files}}


@pytest.fixture(autouse=True)
def record_transcript_evidence(transcripts, json_metadata):
    """Bind the exact persistent client transcripts and independent witnesses to this test."""
    json_metadata["client_evidence"] = transcripts["evidence"]


def trace(transcripts: dict[str, Any], gate: str) -> Trace:
    entry = next(e for e in transcripts["summary"]["results"] if e["gate"] == gate)
    if entry["status"] == "blocked":
        pytest.skip(str(entry["detail"]))
    assert entry["status"] == "passed", entry
    path = ROOT / str(entry["trace"])
    if not path.is_file():
        path = transcripts["out"] / Path(str(entry["trace"])).name
    recorded = json.loads(path.read_text())
    parsed = parse(str(entry["client"]), recorded["stdout"])
    assert parsed.successful_turn and parsed.answer.strip()
    return parsed


def tools_called(recorded: Trace) -> set[str]:
    return {
        call.tool
        for call in recorded.calls
        if call.server == SERVICE and recorded.successful(call.tool)
    }


def test_the_operator_user_directories_were_not_touched(transcripts: dict[str, Any]) -> None:
    """The precondition for every gate below: a controlled test user directory (§12.3)."""
    summary = transcripts["summary"]
    assert summary["real_user_directories_unchanged"] is True, summary["changed"]


def test_a01_codex_connects_over_real_stdio_and_calls_the_service(
    transcripts: dict[str, Any],
) -> None:
    recorded = trace(transcripts, "A01")
    called = tools_called(recorded)
    assert {"service_status", "resolve_library"} <= called, called
    result = recorded.answer_for(recorded.successful("resolve_library")[0])
    assert result["context_id"] and result["data"]["snapshot"]
    assert {"search_evidence", "read_artifact"} <= called


def test_a02_claude_code_connects_over_real_stdio_and_calls_the_service(
    transcripts: dict[str, Any],
) -> None:
    recorded = trace(transcripts, "A02")
    called = tools_called(recorded)
    assert {"service_status", "resolve_library"} <= called, called
    result = recorded.answer_for(recorded.successful("resolve_library")[0])
    assert result["context_id"] and result["data"]["snapshot"]
    assert {"search_evidence", "read_artifact"} <= called


def test_a03_an_unfamiliar_library_question_goes_breadth_then_depth(
    transcripts: dict[str, Any],
) -> None:
    """The skill's two-pass shape (§11.2), visible in which tools were called and in what order."""
    recorded = trace(transcripts, "A03")
    called = tools_called(recorded)
    assert "library_overview" in called, f"no breadth pass: {called}"
    depth = [
        c
        for c in recorded.calls
        if c.tool in {"search_evidence", "inspect_symbol"} and recorded.successful(c.tool)
    ]
    breadth_end = recorded.usable_at(recorded.successful("library_overview")[0])
    assert depth and breadth_end is not None and breadth_end < depth[0].started


def test_a04_a_version_mismatched_context7_answer_is_narrowed_or_verified(
    transcripts: dict[str, Any],
) -> None:
    recorded = trace(transcripts, "A04")
    assert "resolve_library" in tools_called(recorded), (
        "a version-uncertain Context7 answer has to be checked against an exact release"
    )
    assert recorded.successful("resolve-library-id", server="context7")
    assert recorded.successful("query-docs", server="context7")
    assert "1.0.228" in recorded.answer, "the exact version has to appear in the answer"


def test_a05_a_known_symbol_question_does_not_enumerate_the_library(
    transcripts: dict[str, Any],
) -> None:
    recorded = trace(transcripts, "A05")
    called = tools_called(recorded)
    assert "inspect_symbol" in called, called
    assert "library_overview" not in called, (
        f"a single known symbol does not need a whole-library overview: {called}"
    )


def test_a06_an_unavailable_service_is_stated_rather_than_invented(
    transcripts: dict[str, Any],
) -> None:
    """The skill says so itself: "do not fabricate tool results" when the service is absent."""
    recorded = trace(transcripts, "A06")
    assert not any(c.server == SERVICE for c in recorded.calls)
    from client_fixture import assess

    assess("A06", recorded)


@pytest.mark.parametrize("client", ["codex", "claude"])
@pytest.mark.parametrize("journey", ["discovery", "recovery", "upgrade", "runtime"])
def test_research_journeys_complete_in_both_clients(transcripts, client, journey):
    from client_fixture import assess

    gate = f"{journey}-{client}"
    recorded = trace(transcripts, gate)
    assess(gate, recorded)
