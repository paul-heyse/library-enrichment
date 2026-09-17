"""Mechanical SDK framing; native recovery decisions have Rust unit and codec probes."""

import json

import pytest
from mcp_types import JSONRPCResponse

from enrichment_mcp.framing import measure, native_result
from enrichment_mcp.presentation import delivery_contract


@pytest.mark.parametrize("protocol", ["2025-11-25", "2026-07-28"])
@pytest.mark.parametrize("request_id", [7, "7", 'é🦀"\n', "x" * 4096])
def test_native_content_is_preserved_with_exact_sdk_bytes(
    protocol: str, request_id: str | int
) -> None:
    contract = delivery_contract()
    payload = {
        "content": [{"type": "text", "text": 'Required recovery: é "quoted"\n'}],
        "structuredContent": {"request_id": "req_fixture", "recovery": "exact"},
        "isError": True,
    }
    if protocol == "2026-07-28":
        payload |= {
            "resultType": "complete",
            "_meta": {
                "io.modelcontextprotocol/serverInfo": {
                    "name": contract["server_name"],
                    "version": contract["server_version"],
                }
            },
        }
    frame = (
        JSONRPCResponse(jsonrpc="2.0", id=request_id, result=payload)
        .model_dump_json(by_alias=True, exclude_unset=True)
        .encode()
        + b"\n"
    )
    facts = measure(protocol, request_id)
    result = native_result(payload, len(frame), facts)
    assert result.is_error
    assert result.structured_content == payload["structuredContent"]
    assert (
        json.loads(result.to_mcp_result().model_dump_json(by_alias=True, exclude_none=True))[
            "content"
        ]
        == payload["content"]
    )
    with pytest.raises(ValueError, match="measurements disagree"):
        native_result(payload, len(frame) + 1, facts)


def test_rpc_identity_and_transport_bounds_are_exact() -> None:
    assert measure("2025-11-25", "7").bytes - measure("2025-11-25", 7).bytes == 2
    with pytest.raises(ValueError, match="framing bound"):
        measure("2025-11-25", "x" * 17000)
    with pytest.raises(ValueError, match="protocol"):
        measure("unknown-protocol", 7)


def test_missing_modern_stamp_is_refused_before_sdk_can_add_bytes() -> None:
    with pytest.raises(ValueError, match="server identity stamp"):
        native_result(
            {"content": [], "structuredContent": {}, "isError": False, "resultType": "complete"},
            0,
            measure("2026-07-28", 7),
        )
