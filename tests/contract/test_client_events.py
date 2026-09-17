"""Native trace parser regressions; these never count as actual client acceptance."""

import hashlib
import json
import runpy
from contextlib import contextmanager
from pathlib import Path

import pytest
from client_events import SERVICE, Invocation, Trace, parse, result_object
from client_fixture import check_citations, validate_summary

ROOT = Path(__file__).resolve().parents[2]


def stream(*events):
    return "\n".join(json.dumps(e) for e in events)


def event(payload, *, status="completed", arguments=None):
    return {
        "type": "item.completed",
        "item": {
            "id": "call-1",
            "type": "mcp_tool_call",
            "server": SERVICE,
            "tool": "resolve_library",
            "arguments": arguments or {},
            "status": status,
            "result": {"content": [{"type": "text", "text": json.dumps(payload)}]},
        },
    }


def test_model_mentions_and_pending_native_calls_cannot_establish_completion():
    pending = json.loads((ROOT / "tests/fixtures/wire/pending.fixture.json").read_text())
    parsed = parse(
        "codex",
        stream(
            {
                "type": "item.completed",
                "item": {
                    "type": "agent_message",
                    "text": '"resolve_library" mcp__library-enrichment__inspect_symbol',
                },
            },
            event(pending),
            {"type": "turn.completed"},
        ),
    )
    assert parsed.successful_turn
    assert not parsed.successful("resolve_library")
    assert not parsed.successful("inspect_symbol")


def test_conflicting_native_identity_duplicate_completion_and_malformed_result_reject():
    pending = json.loads((ROOT / "tests/fixtures/wire/pending.fixture.json").read_text())
    with pytest.raises(ValueError, match="conflicting"):
        parse("codex", stream(event(pending), event(pending, arguments={"version": "different"})))
    with pytest.raises(ValueError, match="duplicate"):
        parse("codex", stream(event(pending), event(pending)))
    invalid = parse("codex", stream(event({"status": "ok"})))
    with pytest.raises(ValueError, match="envelope"):
        invalid.successful("resolve_library")
    failed = parse("codex", stream(event(pending, status="failed")))
    assert not failed.successful("resolve_library")


def test_claude_results_correlate_by_id_and_cannot_borrow_another_result():
    pending = (ROOT / "tests/fixtures/wire/pending.fixture.json").read_text()
    invocation = {
        "type": "assistant",
        "message": {
            "content": [
                {
                    "type": "tool_use",
                    "id": "one",
                    "name": "mcp__library-enrichment__resolve_library",
                    "input": {},
                }
            ]
        },
    }
    other = {
        "type": "user",
        "message": {"content": [{"type": "tool_result", "tool_use_id": "two", "content": pending}]},
    }
    parsed = parse("claude", stream(invocation, other))
    assert parsed.calls[0].completed is None
    assert not parsed.successful("resolve_library")

    other["message"]["content"][0]["tool_use_id"] = "one"
    parsed = parse(
        "claude",
        stream(
            invocation,
            other,
            {"type": "result", "subtype": "success", "is_error": False, "result": "Waiting."},
        ),
    )
    assert parsed.calls[0].completed is not None and parsed.successful_turn
    assert not parsed.successful("resolve_library")


def test_teardown_failure_replaces_success_instead_of_hiding_behind_it(tmp_path, monkeypatch):
    api = runpy.run_path(str(ROOT / "scripts/client-acceptance.py"))
    main = api["main"]
    scope = main.__globals__
    root = tmp_path / "sandbox"
    root.mkdir()

    class IsolatedSandbox:
        client = "codex"

        def __init__(self, path):
            self.root = path

        @classmethod
        def create(cls, path):
            return cls(path)

        def untouched(self):
            return True, []

    @contextmanager
    def failing_service(*_args, **_kwargs):
        yield None
        raise OSError("daemon teardown failed")

    monkeypatch.setitem(scope, "Sandbox", IsolatedSandbox)
    monkeypatch.setitem(scope, "service", failing_service)
    monkeypatch.setitem(scope, "prerequisites", lambda *_: [])
    monkeypatch.setitem(scope, "register", lambda *_: None)
    monkeypatch.setitem(scope, "drive", lambda *_: {"gate": "A01", "status": "passed"})
    monkeypatch.setitem(scope, "sandbox_root", lambda: root)
    monkeypatch.setitem(scope, "out_dir", lambda _: tmp_path / "out")
    monkeypatch.setattr("sys.argv", ["client-acceptance", "--apply", "--keep", "--gate", "A01"])
    assert main() == 1
    summary = json.loads((tmp_path / "out/summary.json").read_text())
    assert len(summary["results"]) == 1
    assert summary["results"][0]["status"] == "failed"
    assert "daemon teardown failed" in summary["results"][0]["detail"]
    validate_summary(summary, 1, {"A01"})
    with pytest.raises(ValueError, match="contradicts"):
        validate_summary(summary, 0, {"A01"})
    summary["results"].insert(0, {"gate": "A01", "status": "passed"})
    with pytest.raises(ValueError, match="duplicate"):
        validate_summary(summary, 1, {"A01"})


def test_complete_result_pages_can_restart_with_a_smaller_page_size():
    document = json.loads((ROOT / "tests/fixtures/wire/ok.fixture.json").read_text())
    raw = json.dumps(document).encode()
    artifact = {
        "artifact_id": "art_result",
        "sha256": hashlib.sha256(raw).hexdigest(),
        "media_type": "application/json",
        "size_bytes": len(raw),
        "kind": "other",
        "source_uri": "service:bounded-result",
        "final_url": None,
        "retrieved_at": "2026-09-14T00:00:00.000000Z",
        "etag": None,
        "last_modified": None,
        "compression": None,
    }

    def page(start, end, cursor, following, position):
        chunk = raw[start:end]
        envelope = document | {
            "data": {
                "artifact": artifact,
                "encoding": "utf8",
                "start": start,
                "end": end,
                "total": len(raw),
                "content": chunk.decode(),
                "content_digest": hashlib.sha256(chunk).hexdigest(),
                "remaining": len(raw) - end,
                "section": "envelope",
                "page": {
                    "returned": 1,
                    "count": {"kind": "unknown"},
                    "has_more": following is not None,
                    "next_cursor": following,
                },
            },
        }
        return Invocation(
            f"page-{position}",
            SERVICE,
            "read_artifact",
            {
                "artifact_id": "art_result",
                "section": {"kind": "result", "name": "envelope"},
                "cursor": cursor,
            },
            position,
            position + 1,
            envelope,
        )

    part = len(raw) // 3
    trace = Trace(
        calls=[
            page(0, part * 2, None, "abandoned", 1),
            page(0, part, None, "restart-1", 3),
            page(part, part * 2, "restart-1", "restart-2", 5),
            page(part * 2, len(raw), "restart-2", None, 7),
        ]
    )
    assert trace.result_artifact({"request_id": "overflow"}, "art_result") == (
        document | {"request_id": "overflow"},
        8,
    )
    trace.calls.pop()
    with pytest.raises(ValueError, match="complete"):
        trace.result_artifact({"request_id": "overflow"}, "art_result")


def test_direct_data_section_requires_its_own_complete_digest_checked_cursor_chain():
    document = json.loads((ROOT / "tests/fixtures/wire/ok.fixture.json").read_text())
    raw = json.dumps(document["data"]).encode()
    offset, split = 123, len(raw) // 2
    artifact = {
        "artifact_id": "art_section",
        "sha256": "a" * 64,
        "media_type": "application/json",
        "size_bytes": len(raw) + 200,
        "kind": "other",
        "source_uri": "service:bounded-result",
        "final_url": None,
        "retrieved_at": "2026-09-15T00:00:00.000000Z",
        "etag": None,
        "last_modified": None,
        "compression": None,
    }
    calls = []
    for index, (start, end, cursor, following) in enumerate(
        [(0, split, None, "continued"), (split, len(raw), "continued", None)]
    ):
        chunk = raw[start:end]
        calls.append(
            Invocation(
                f"section-{index}",
                SERVICE,
                "read_artifact",
                {
                    "artifact_id": "art_section",
                    "section": {"kind": "result", "name": "data"},
                    "cursor": cursor,
                },
                index * 2,
                index * 2 + 1,
                document
                | {
                    "data": {
                        "artifact": artifact,
                        "encoding": "utf8",
                        "start": offset + start,
                        "end": offset + end,
                        "total": artifact["size_bytes"],
                        "content": chunk.decode(),
                        "content_digest": hashlib.sha256(chunk).hexdigest(),
                        "remaining": len(raw) - end,
                        "section": "data",
                        "page": {
                            "returned": 1,
                            "count": {"kind": "unknown"},
                            "has_more": following is not None,
                            "next_cursor": following,
                        },
                    }
                },
            )
        )
    trace = Trace(calls=calls)
    decoded, completed = trace.result_artifact(document, "art_section")
    assert completed == 3
    assert decoded == document | {
        "delivery": {
            "mode": "inline",
            "limits": {"requested_max_bytes": None, "effective_max_bytes": None},
        }
    }
    calls[1].arguments["section"]["name"] = "coverage"
    with pytest.raises(ValueError, match="complete"):
        trace.result_artifact(document, "art_section")
    calls[1].arguments["section"]["name"] = "data"
    assert isinstance(calls[1].result, dict)
    calls[1].result["data"]["content_digest"] = "b" * 64
    with pytest.raises(ValueError, match="invalid native result"):
        trace.result_artifact(document, "art_section")


def test_resource_link_presentation_cannot_replace_or_conflict_with_the_envelope():
    document = json.loads((ROOT / "tests/fixtures/wire/ok.fixture.json").read_text())
    link = {
        "type": "text",
        "text": "[Resource link: Complete result] library-evidence://artifacts/x",
    }
    payload = {"type": "text", "text": json.dumps(document)}
    assert result_object([link, payload]) == document
    for invalid in ([link], [payload, payload]):
        with pytest.raises(ValueError, match="one explicit JSON envelope"):
            result_object(invalid)


def test_short_citations_must_select_one_identity_in_completed_evidence(monkeypatch):
    first = "fragment_12345678" + "a" * 56
    second = "fragment_12345678" + "b" * 56
    evidence = {"status": "ok", "identities": [first]}
    monkeypatch.setattr(Trace, "answer_for", lambda *_: evidence)
    trace = Trace(
        calls=[Invocation("call", SERVICE, "library_overview", {}, 0, 1)],
        answer="README.md:1, fragment `12345678…`",
    )
    check_citations(trace)
    evidence["identities"].append(second)
    with pytest.raises(ValueError, match="actual evidence"):
        check_citations(trace)
    trace.answer = "fragment `fedcba98…`"
    with pytest.raises(ValueError, match="actual evidence"):
        check_citations(trace)


def test_only_native_authentication_failures_name_a_blocked_prerequisite():
    text = "Failed to authenticate: OAuth session expired and could not be refreshed"
    message = {"type": "assistant", "message": {"content": [{"type": "text", "text": text}]}}
    assert parse("claude", stream(message)).prerequisite_error is None
    native = message | {"error": "authentication_failed", "is_api_error_message": True}
    trace = parse("claude", stream(native))
    assert trace.prerequisite_error and not trace.successful_turn


def test_pre_admission_failure_is_not_a_terminal_job_observation():
    preview = {"format": "research-error-preview/1", "code": "UNSUPPORTED_FORMAT"}
    call = Invocation(
        "call",
        SERVICE,
        "job_control",
        {"job_id": "job_example", "wait_seconds": 30},
        0,
        1,
        [{"type": "text", "text": json.dumps(preview)}],
        "tool failed",
    )
    assert call.failure_preview() is None
    preview |= {"job_id": "job_example", "code": "VERSION_NOT_FOUND"}
    call.result = [{"type": "text", "text": json.dumps(preview)}]
    assert call.failure_preview() == preview
    preview["job_id"] = "job_other"
    call.result = [{"type": "text", "text": json.dumps(preview)}]
    with pytest.raises(ValueError, match="native job invocation"):
        call.failure_preview()
