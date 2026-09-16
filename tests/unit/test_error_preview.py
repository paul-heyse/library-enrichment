"""Presentation regressions; raw stdio journeys separately prove live recovery."""

import json
from pathlib import Path

from mcp.types import CallToolResult, TextContent

from enrichment_mcp import presentation
from enrichment_mcp.server import MCP_FRAME_ALLOWANCE_BYTES, _tool_result

ROOT = Path(__file__).resolve().parents[2]


def test_large_diagnostic_preserves_bounded_error_and_native_recovery() -> None:
    result = json.loads((ROOT / "tests/fixtures/wire/error.fixture.json").read_text())
    result["error"]["message"] = 'Escaped diagnostic: "\\\n🌍' * 1000
    actual = _tool_result(result, tool="inspect_symbol")
    assert actual.is_error
    assert actual.structured_content == result
    assert len(actual.content) == 1
    content = actual.content[0]
    assert isinstance(content, TextContent)
    preview = json.loads(content.text)
    assert preview["details_omitted"] is True
    assert preview["code"] == result["error"]["code"]
    assert preview["next_action"] == result["error"]["next_action"]
    frame = CallToolResult(
        content=actual.content, structured_content=result, is_error=actual.is_error
    )
    structured = json.dumps(result, ensure_ascii=False, separators=(",", ":")).encode()
    assert len(frame.model_dump_json(by_alias=True).encode()) + 128 <= (
        len(structured) + MCP_FRAME_ALLOWANCE_BYTES
    )


def test_failed_job_without_an_artifact_preserves_its_inline_recovery() -> None:
    result = json.loads((ROOT / "tests/fixtures/wire/error.fixture.json").read_text())
    terminal = {
        "outcome": "error",
        "error": result["error"],
        "delivery": result["delivery"],
    }
    result |= {"status": "ok", "error": None, "data": {"job_id": "job_fixture", "result": terminal}}
    for compact in (False, True):
        preview = presentation.error_preview(result, compact=compact)
        assert preview["job_id"] == "job_fixture"
        assert preview["code"] == "POLICY_DENIED"
        assert preview["next_action"] == terminal["error"]["next_action"]
        assert "read" not in preview
