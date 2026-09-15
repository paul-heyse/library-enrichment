"""Acceptance gate C14, in-process leg: the tool catalog is registered promptly and cheaply.

    C14 | Tool initializes a stdio connection
       | Tool list appears without package fetch, build, or LSP startup.

Blueprint §14.1 asks for both boundaries -- "Test FastMCP through its in-process client as well
as a real stdio subprocess". This module is the in-process half; ``test_stdio_startup.py`` is
the subprocess half, which is where the no-fetch/no-build/no-LSP claim is actually *measured*
rather than asserted.

The FastMCP server here is the real one, driven by the real in-process client. Nothing is
mocked: ``rules/no-mocks-in-acceptance-tiers.yml`` forbids it in this tier, and a mocked
boundary would test nothing.
"""

from __future__ import annotations

import json
from pathlib import Path

import pytest
from fastmcp import Client
from jsonschema import Draft202012Validator
from jsonschema.protocols import Validator

from enrichment_mcp import presentation
from enrichment_mcp.server import TOOL_NAMES, build_server

ROOT = Path(__file__).resolve().parents[2]
GENERATED_SCHEMA = ROOT / "schemas/generated/research-envelope.schema.json"


@pytest.fixture(scope="module")
def envelope_validator() -> Validator:
    """Validate against the *generated* schema, which is emitted from the Rust wire types.

    Using the generated schema rather than the frozen one is deliberate: it proves the adapter
    emits what the authoritative types describe, which is the half `just schema-conformance`
    cannot see.
    """
    if not GENERATED_SCHEMA.exists():
        pytest.fail(f"{GENERATED_SCHEMA} is missing. Run `just schemas-generate`.")
    return Draft202012Validator(json.loads(GENERATED_SCHEMA.read_text()))


async def test_all_nine_tools_are_registered() -> None:
    async with Client(build_server()) as client:
        listed = {tool.name for tool in await client.list_tools()}
    assert listed == set(TOOL_NAMES), "the catalog must match the skill's tool contract"


def test_inline_success_cannot_use_a_receipt_or_an_empty_payload() -> None:
    sample = json.loads((ROOT / "contracts/research-v2/examples/ok.fixture.json").read_text())
    for tool in TOOL_NAMES:
        invalid = sample | {"data": {}}
        with pytest.raises(ValueError):
            presentation.validate_output(tool, invalid)
        assert list(Draft202012Validator(presentation.output_schema(tool)).iter_errors(invalid))


async def test_workflow_examples_use_the_registered_tool_contract() -> None:
    import re

    async with Client(build_server()) as client:
        tools = {tool.name: tool for tool in await client.list_tools()}
        workflow = await client.read_resource("library-evidence://workflow")
    assert workflow
    text = (ROOT / "skills/library-research/references/tool-contract.md").read_text()
    examples = [json.loads(value) for value in re.findall(r"```json\n(.*?)\n```", text, re.S)]
    assert examples
    for example in examples:
        tool = tools[example["tool"]]
        Draft202012Validator(tool.input_schema).validate(example["arguments"])


async def test_every_tool_declares_an_input_schema() -> None:
    async with Client(build_server()) as client:
        tools = await client.list_tools()
    canonical = json.loads(GENERATED_SCHEMA.read_text())

    for tool in tools:
        assert tool.input_schema, f"{tool.name} has no input schema"
        assert tool.output_schema["type"] == "object"
        assert sorted(tool.output_schema["required"]) == sorted(canonical["required"])
        assert tool.output_schema["properties"]["data"].get("anyOf")
        assert tool.output_schema["additionalProperties"] is False
        assert tool.description, f"{tool.name} has no description"
    by_name = {tool.name: tool for tool in tools}
    inspection = by_name["inspect_symbol"].input_schema["properties"]
    assert "selection" in inspection and "depth" not in inspection and "aspects" not in inspection
    assert "alternative_cursor" in by_name["compare_releases"].input_schema["properties"]
    assert by_name["resolve_library"].annotations.open_world_hint is True
    assert by_name["job_control"].annotations.read_only_hint is False


async def test_verify_usage_is_not_advertised_as_read_only() -> None:
    """Blueprint §7.4: it "must not be portrayed as a pure read-only operation"."""
    async with Client(build_server()) as client:
        tools = {tool.name: tool for tool in await client.list_tools()}

    for name in ("verify_usage", "inspect_symbol"):
        annotations = tools[name].annotations
        assert annotations is not None, f"{name} must disclose explicit execution behaviour"
        assert annotations.read_only_hint is not True


async def test_cached_reads_state_their_world_openness_explicitly() -> None:
    """``openWorldHint`` is read as **true** when absent, so silence is the permissive answer.

    A cached read must therefore say ``false`` rather than leave it unset.
    """
    async with Client(build_server()) as client:
        tools = {tool.name: tool for tool in await client.list_tools()}

    annotations = tools["service_status"].annotations
    assert annotations is not None
    assert annotations.open_world_hint is False


@pytest.mark.parametrize(
    "tool,arguments",
    [
        ("verify_usage", {"context_id": "ctx_x", "snippet": "value: int = 1"}),
        ("job_control", {"job_id": "job_x"}),
    ],
)
async def test_execution_tools_report_unreachable_daemon_truthfully(
    tool, arguments, tmp_path, monkeypatch, envelope_validator
):
    monkeypatch.setenv("LIBENR_SOCKET", str(tmp_path / "absent.sock"))
    async with Client(build_server()) as client:
        response = await client.call_tool(tool, arguments, raise_on_error=False)
        assert response.is_error
        result = response.structured_content
    assert not list(envelope_validator.iter_errors(result))
    assert result["status"] == "error"
    assert result["error"]["code"] == "UPSTREAM_UNAVAILABLE"
    assert result["error"]["next_action"]


async def test_service_status_is_truthful_when_the_daemon_is_absent(
    envelope_validator: Validator, tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    """The Phase 0 gate: "`service_status` truthfully reports absent components".

    Pointing at a socket that does not exist is not a mock -- it is the real, ordinary condition
    of a stopped daemon, which is exactly when a caller most needs a usable answer.
    """
    monkeypatch.setenv("LIBENR_SOCKET", str(tmp_path / "absent.sock"))

    async with Client(build_server()) as client:
        result = await client.call_tool("service_status", {})

    payload = result.structured_content
    assert isinstance(payload, dict)
    errors = list(envelope_validator.iter_errors(payload))
    assert not errors, f"non-conforming envelope: {errors[0].message}"

    # Not an error: an unreachable daemon is a fact to report, not a failed call.
    assert payload["status"] == "partial"
    assert payload["error"] is None
    assert payload["data"]["daemon"]["available"] is False
    # And the gap is explicit rather than silently absent.
    assert "daemon" in payload["coverage"]["missing"]
    assert payload["coverage"]["limitations"], "a partial result must say what is missing"


async def test_invalid_tool_input_is_rejected() -> None:
    """Gate C19 at the MCP boundary: input violating the declared schema is refused."""
    async with Client(build_server()) as client:
        with pytest.raises(Exception, match=r"(?i)valid|error|input"):
            # `ecosystem` is a Literal["rust", "python"]; "haskell" is not in the schema.
            await client.call_tool("resolve_library", {"ecosystem": "haskell", "name": "serde"})


async def test_a_missing_required_argument_is_rejected() -> None:
    async with Client(build_server()) as client:
        with pytest.raises(Exception, match=r"(?i)valid|error|required|input"):
            await client.call_tool("resolve_library", {"ecosystem": "rust"})


def test_rust_generated_request_contract_rejects_unknown_and_invalid_types():
    from enrichment_mcp.envelope import validate_request

    assert validate_request(
        "resolve_library",
        {"ecosystem": "python", "name": "some.pkg", "python_version": "3.14", "extras": []},
    )[0]
    assert not validate_request("resolve_library", {"name": "pkg", "python_version": 314})[0]
    assert not validate_request("resolve_library", {"name": "pkg", "install": True})[0]
    assert not validate_request("inspect_symbol", {"depth": "execute"})[0]
    assert not validate_request("search_evidence", {"max_items": -1})[0]


def test_terminal_result_and_page_keep_native_cross_field_constraints() -> None:
    schema = presentation.output_schema("job_control")
    fixture = json.loads((ROOT / "contracts/research-v2/examples/ok.fixture.json").read_text())
    terminal = {
        key: fixture[key]
        for key in ("summary", "context_id", "snapshot_id", "coverage", "delivery", "error")
    }
    terminal["outcome"] = "ok"
    result_schema = {"$defs": schema["$defs"], "$ref": "#/$defs/JobResult"}
    validator = Draft202012Validator(result_schema)
    validator.validate(terminal)
    for outcome in ("pending", "error", None):
        assert list(validator.iter_errors(terminal | {"outcome": outcome}))
    del terminal["outcome"]
    assert list(validator.iter_errors(terminal))
    search = presentation.output_schema("search_evidence")
    validator = Draft202012Validator({"$defs": search["$defs"], "$ref": "#/$defs/Page"})
    page = {
        "returned": 1,
        "count": {"kind": "unknown"},
        "has_more": True,
        "next_cursor": "rows2_example",
    }
    validator.validate(page)
    for changes in (
        {"next_cursor": None},
        {"returned": 0},
        {"has_more": False},
        {"next_cursor": "🌍"},
        {"next_cursor": "x" * 32769},
    ):
        assert list(validator.iter_errors(page | changes))
