"""Pinned SDK framing facts and mechanical verification of native MCP output."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from fastmcp.resources import ResourceContent, ResourceResult
from fastmcp.server.dependencies import get_context
from fastmcp.tools import ToolResult
from jsonschema import Draft202012Validator
from mcp_types import CallToolResult, JSONRPCResponse, ReadResourceResult, TextResourceContents
from mcp_types.methods import serialize_server_result
from mcp_types.version import KNOWN_PROTOCOL_VERSIONS, MODERN_PROTOCOL_VERSIONS

from enrichment_mcp.presentation import delivery_contract


@dataclass(frozen=True)
class Framing:
    """Transport measurements, with no evidence or delivery-policy decisions."""

    protocol: str
    bytes: int
    uri: str | None = None

    @property
    def profile(self) -> dict[str, Any]:
        value = {
            "kind": "mcp_stdio",
            "era": "modern" if self.protocol in MODERN_PROTOCOL_VERSIONS else "classic",
            "framing_bytes": str(self.bytes),
        }
        if self.uri is not None:
            value |= {"kind": "mcp_resource_stdio", "uri": self.uri}
        Draft202012Validator(delivery_contract()["profile_schema"]).validate(value)
        return value


def measure(protocol: str, request_id: str | int) -> Framing:
    """Measure the actual stdio JSON-RPC wrapper, including the line terminator."""
    if protocol not in KNOWN_PROTOCOL_VERSIONS:
        raise ValueError("unqualified MCP protocol version")
    encoded = JSONRPCResponse(jsonrpc="2.0", id=request_id, result={}).model_dump_json(
        by_alias=True, exclude_unset=True
    )
    framing = len(encoded.encode()) + 1 - 2
    if framing > delivery_contract()["max_framing_bytes"]:
        raise ValueError("MCP request ID exceeds the admitted framing bound")
    return Framing(protocol, framing)


def current_profile(*, resource: bool = False) -> Framing:
    """Obtain the raw typed ID; Context.request_id loses integer/string identity."""
    current = get_context()
    context = current.request_context
    if context is None or current.transport != "stdio":
        raise ValueError("native delivery requires an admitted stdio request context")
    raw_id = context._srctx.request_id
    if raw_id is None or isinstance(raw_id, bool):
        raise ValueError("tool delivery requires a JSON-RPC request ID")
    frame = measure(context.protocol_version, raw_id)
    if resource:
        uri = (context._srctx.params or {}).get("uri")
        if not isinstance(uri, str):
            raise ValueError("resource delivery requires the exact requested URI")
        return Framing(frame.protocol, frame.bytes, uri)
    return frame


def native_result(payload: dict[str, Any], declared_bytes: object, framing: Framing) -> ToolResult:
    """Refuse serializer drift; never repair, trim or choose a different native result."""
    model = CallToolResult.model_validate(payload, strict=True)
    shaped = serialize_server_result(
        "tools/call",
        framing.protocol,
        model.model_dump(mode="json", by_alias=True, exclude_none=True),
    )
    _verify(payload, shaped, declared_bytes, framing)
    return ToolResult.from_mcp_result(model)


def native_resource(
    payload: dict[str, Any], declared_bytes: object, framing: Framing
) -> ResourceResult:
    """Forward native text bytes and metadata through FastMCP's canonical resource result."""
    model = ReadResourceResult.model_validate(payload, strict=True)
    if framing.uri is None or len(model.contents) != 1:
        raise ValueError("native resource requires one exact URI-bound content block")
    content = model.contents[0]
    if not isinstance(content, TextResourceContents) or str(content.uri) != framing.uri:
        raise ValueError("native resource URI or content kind changed")
    result = ResourceResult(
        [ResourceContent(content.text, mime_type=content.mime_type, meta=content.meta)],
        meta=model.meta,
    )
    shaped = serialize_server_result(
        "resources/read",
        framing.protocol,
        result.to_mcp_result(framing.uri).model_dump(mode="json", by_alias=True, exclude_none=True),
    )
    _verify(payload, shaped, declared_bytes, framing)
    return result


def _verify(
    payload: dict[str, Any], shaped: dict[str, Any], declared_bytes: object, framing: Framing
) -> None:
    if framing.protocol in MODERN_PROTOCOL_VERSIONS:
        contract = delivery_contract()
        expected_meta = {
            "io.modelcontextprotocol/serverInfo": {
                "name": contract["server_name"],
                "version": contract["server_version"],
            }
        }
        if payload.get("_meta") != expected_meta:
            raise ValueError("native MCP result lacks the exact server identity stamp")
    if shaped != payload:
        raise ValueError("MCP serializer changed the native delivery contract")
    # Use the stdio transport's serializer, not stdlib JSON's float/string conventions.
    encoded = JSONRPCResponse(jsonrpc="2.0", id=0, result=shaped).model_dump_json(
        by_alias=True, exclude_unset=True
    )
    actual = len(encoded.encode()) + 1 - measure(framing.protocol, 0).bytes
    if type(declared_bytes) is not int or actual + framing.bytes != declared_bytes:
        raise ValueError("native and SDK delivery byte measurements disagree")
