"""Mechanical validation against the Rust-generated MCP binding catalog."""

from __future__ import annotations

import json
from functools import cache
from typing import Any

from jsonschema import Draft202012Validator
from jsonschema.protocols import Validator

from enrichment_mcp.envelope import SCHEMA_PATH


@cache
def bindings() -> dict[str, dict[str, Any]]:
    """Load the finite operation declaration packaged with its native request schema."""
    schema = json.loads(SCHEMA_PATH.with_name("request.schema.json").read_text())
    return {entry["name"]: entry for entry in schema["x-enrichment-operations"]}


def output_schema(tool: str) -> dict[str, Any]:
    """The daemon's generated response contract, including all native conditional rules."""
    return bindings()[tool]["output_schema"]


@cache
def _output_validator(tool: str) -> Validator:
    return Draft202012Validator(output_schema(tool))


def validate_output(tool: str, result: dict[str, Any]) -> None:
    """Validate without coercing typed native values or inventing defaults."""
    errors = list(_output_validator(tool).iter_errors(result))
    if errors:
        raise ValueError(errors[0].message)


def is_error(result: dict[str, Any]) -> bool:
    """A retrieved terminal job failure must remain visible to an MCP client."""
    if result["status"] == "error":
        return True
    payload = result.get("data", {})
    terminal = payload.get("result")
    return isinstance(terminal, dict) and terminal.get("outcome", {}).get("status") == "error"


def error_preview(result: dict[str, Any], *, compact: bool = False) -> dict[str, Any]:
    """Project native recovery for hosts that discard structured content on MCP errors."""
    payload = result["data"]
    terminal = payload.get("result")
    failed_job = isinstance(terminal, dict) and terminal.get("outcome", {}).get("status") == "error"
    error = terminal["outcome"]["error"] if failed_job else result["error"]
    preview = {
        "format": "research-error-preview/1",
        "request_id": result["request_id"],
        "code": error["code"],
        "retryable": error["retryable"],
        "cause": error["diagnostic"]["cause"],
        "stage": error["diagnostic"]["stage"],
    }
    if failed_job:
        preview["job_id"] = payload["job_id"]
        if terminal["delivery"]["mode"] == "artifact":
            preview["read"] = terminal["delivery"]["read"]
    if compact:
        # Whole recovery IDs/actions are retained; an omitted explanation is explicitly
        # labeled. A failed artifact write cannot supply a result-read action.
        preview["details_omitted"] = True
        preview["action_kinds"] = [action["kind"] for action in error["diagnostic"]["actions"]]
        if len(json.dumps(error["next_action"]).encode()) <= 256:
            preview["next_action"] = error["next_action"]
    else:
        preview["message"] = error["message"]
        preview["next_action"] = error["next_action"]
    return preview
