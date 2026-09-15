"""Authored MCP presentation contracts over generated native evidence types (ADR-0037).

This layer composes tool-specific output and JSON input binding. It owns no evidence queries,
coverage decisions, continuation identities, execution policy or durable results.
"""

from __future__ import annotations

import json
from functools import cache
from typing import Any, Self

from jsonschema import Draft202012Validator
from jsonschema.protocols import Validator
from pydantic import BaseModel, ConfigDict, Field, model_validator

from enrichment_mcp._generated import research_envelope_schema as wire
from enrichment_mcp._generated import tool_data_schema as data
from enrichment_mcp.envelope import TOOL_DATA_SCHEMA_PATH


class EmptyData(BaseModel):
    """No inline payload: a receipt, failure, or durable result descriptor carries the answer."""

    model_config = ConfigDict(extra="forbid")


class AdapterDaemonStatus(BaseModel):
    model_config = ConfigDict(extra="forbid")
    available: bool
    detail: str


class AdapterStatus(BaseModel):
    model_config = ConfigDict(extra="forbid")
    available: bool
    tools: list[str]


class LocalStatus(BaseModel):
    """Only adapter-observed facts when no daemon response exists."""

    model_config = ConfigDict(extra="forbid")
    daemon: AdapterDaemonStatus
    adapter: AdapterStatus


class ResearchResponse[T: BaseModel](BaseModel):
    """MCP composition; native DTOs retain authority over every evidence-bearing field."""

    model_config = ConfigDict(
        extra="forbid",
        json_schema_extra={
            "allOf": [
                {
                    "if": {"properties": {"status": {"const": "pending"}}},
                    "then": {
                        "properties": {"job": {"not": {"type": "null"}}, "error": {"type": "null"}}
                    },
                },
                {
                    "if": {"properties": {"status": {"const": "error"}}},
                    "then": {"properties": {"error": {"not": {"type": "null"}}}},
                },
                {
                    "if": {"properties": {"status": {"enum": ["ok", "partial"]}}},
                    "then": {"properties": {"error": {"type": "null"}}},
                },
            ]
        },
    )
    schema_version: wire.SchemaVersion
    request_id: str = Field(min_length=1)
    status: wire.Status
    summary: str
    context_id: str | None
    snapshot_id: str | None
    data: T | data.JobData | EmptyData
    coverage: wire.Coverage
    freshness: wire.Freshness
    evidence: list[wire.Evidence]
    artifacts: list[wire.ArtifactHandle]
    delivery: wire.DeliveryDescriptor
    job: wire.JobHandle | None
    error: wire.Error | None

    @model_validator(mode="after")
    def validate_outcome(self) -> Self:
        if self.status == wire.Status.pending and self.job is None:
            raise ValueError("pending results require a durable job handle")
        if (self.status == wire.Status.error) != (self.error is not None):
            raise ValueError("only error results require an error diagnostic")
        if self.delivery.root.mode == "artifact" and not isinstance(self.data, EmptyData):
            raise ValueError("artifact delivery requires an empty inline data payload")
        return self


# Each tool advertises its actual data, durable delivery, receipt and failure possibilities.
OUTPUT_MODELS: dict[str, type[BaseModel]] = {
    "resolve_library": ResearchResponse[data.ResolveData],
    "library_overview": ResearchResponse[data.OverviewData],
    "search_evidence": ResearchResponse[data.SearchData],
    "inspect_symbol": ResearchResponse[data.InspectData],
    "compare_releases": ResearchResponse[data.CompareData],
    "verify_usage": ResearchResponse[data.VerificationData],
    "read_artifact": ResearchResponse[data.ArtifactSliceData],
    "job_control": ResearchResponse[data.JobData],
    "service_status": ResearchResponse[data.StatusData | LocalStatus],
    "snapshot_manifest": ResearchResponse[data.ManifestData],
}


def output_schema(tool: str) -> dict[str, Any]:
    """Publish the same serialization contract checked before every ToolResult."""
    schema = OUTPUT_MODELS[tool].model_json_schema(mode="serialization")
    # Code generation retains field types but cannot translate native allOf rules.
    # Carry the native conditional constraints through without restating domain semantics.
    native = json.loads(TOOL_DATA_SCHEMA_PATH.read_text())["$defs"]
    for name in ("JobResult", "Page"):
        if name in schema["$defs"]:
            schema["$defs"][name]["allOf"] = native[name]["allOf"]
            schema["$defs"][name]["required"] = native[name]["required"]
    alternatives = schema["properties"]["data"]["anyOf"]
    empty = {"$ref": "#/$defs/EmptyData"}
    job = {"$ref": "#/$defs/JobData"}
    success = [
        value
        for value in alternatives
        if value != empty and (value != job or tool == "job_control")
    ]
    failure = [empty, *success] if tool == "verify_usage" else [empty]
    schema["allOf"].extend(
        [
            {
                "if": {"properties": {"delivery": {"properties": {"mode": {"const": "artifact"}}}}},
                "then": {"properties": {"data": empty}},
            },
            {
                "if": {"properties": {"status": {"const": "pending"}}},
                "then": {
                    "properties": {
                        "data": job,
                        "delivery": {"properties": {"mode": {"const": "inline"}}},
                    }
                },
            },
            {
                "if": {
                    "properties": {
                        "status": {"enum": ["ok", "partial"]},
                        "delivery": {"properties": {"mode": {"const": "inline"}}},
                    }
                },
                "then": {"properties": {"data": {"anyOf": success}}},
            },
            {
                "if": {
                    "properties": {
                        "status": {"const": "error"},
                        "delivery": {"properties": {"mode": {"const": "inline"}}},
                    }
                },
                "then": {"properties": {"data": {"anyOf": failure}}},
            },
        ]
    )
    return schema


@cache
def _output_validator(tool: str) -> Validator:
    return Draft202012Validator(output_schema(tool))


def validate_output(tool: str, result: dict[str, Any]) -> None:
    """Strict JSON validation preserves enum semantics without Python coercion."""
    errors = list(_output_validator(tool).iter_errors(result))
    if errors:
        raise ValueError(errors[0].message)
    OUTPUT_MODELS[tool].model_validate_json(json.dumps(result), strict=True)


def is_error(result: dict[str, Any]) -> bool:
    """A retrieved terminal job failure must remain visible to an MCP client."""
    if result["status"] == "error":
        return True
    payload = result.get("data", {})
    terminal = payload.get("result")
    return isinstance(terminal, dict) and terminal.get("outcome") == "error"


def error_preview(result: dict[str, Any], *, compact: bool = False) -> dict[str, Any]:
    """Project native recovery for hosts that discard structured content on MCP errors."""
    payload = result["data"]
    terminal = payload.get("result")
    failed_job = isinstance(terminal, dict) and terminal.get("outcome") == "error"
    error = terminal["error"] if failed_job else result["error"]
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
