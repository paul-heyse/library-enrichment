"""Unit fault injection: optional progress must preserve an already returned receipt."""

import json
from pathlib import Path

from fastmcp.server.context import Context
from fastmcp.server.middleware import MiddlewareContext
from fastmcp.tools.base import ToolResult
from mcp.types import CallToolRequestParams

from enrichment_mcp.server import OperationBoundary, build_server


class FailedProgress(Context):
    async def report_progress(
        self, progress: float, total: float | None = None, message: str | None = None
    ) -> None:
        raise ConnectionError("closed notification transport")


async def test_progress_failure_preserves_the_pending_receipt() -> None:
    server = build_server()
    root = Path(__file__).resolve().parents[2]
    receipt = json.loads((root / "contracts/research-v2/examples/pending.fixture.json").read_text())
    expected = ToolResult(structured_content=receipt)
    context = MiddlewareContext(
        message=CallToolRequestParams(name="job_status", arguments={"job_id": "job_fixture"}),
        fastmcp_context=FailedProgress(server),
    )

    async def complete(context: MiddlewareContext[CallToolRequestParams]) -> ToolResult:
        assert context.message.name == "job_status"
        return expected

    actual = await OperationBoundary().on_call_tool(context, complete)
    assert actual is expected
    assert actual.structured_content == receipt
