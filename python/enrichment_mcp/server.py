"""The FastMCP 4 adapter: nine tools over stdio.

A thin boundary (blueprint §2.1). It validates inputs, calls the daemon, emits MCP results, and
maps structured errors. It does not index libraries, own persistent state, or reimplement any
part of the core.

**Nothing expensive happens at import or during ``initialize``.** Acceptance gate C14 requires
the tool list to appear without a package fetch, a build, or an LSP start, and §2.2 says so
directly: "Normal stdio startup must register the tool catalog promptly. Do not initialize
language servers, download packages, run compilation, or refresh registries during MCP
initialization." The daemon connection is opened per call, never here.

**stdout is the protocol channel.** Every log, warning and traceback goes to stderr. A stray
``print()`` corrupts the session, which is why ruff's ``T20`` is enabled and
``rules/mcp-stdout-protocol-only.yml`` scans for it in the edit loop.

All nine tools are registered with their real input schemas and every one is now wired to a
daemon method. What a tool cannot answer, the daemon says so in the envelope -- a typed error or
a ``partial`` with the gap named, never a fabricated success and never an empty ``ok``.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path
from time import monotonic
from typing import Any
from uuid import uuid4

from fastmcp import FastMCP
from fastmcp.exceptions import ValidationError as ToolValidationError
from fastmcp.resources.base import ResourceResult
from fastmcp.server.middleware import CallNext, Middleware, MiddlewareContext
from fastmcp.tools import Tool, ToolResult
from jsonschema import Draft202012Validator, validators
from mcp.types import (
    CallToolRequestParams,
    CallToolResult,
    ReadResourceRequestParams,
    TextContent,
    ToolAnnotations,
)

from enrichment_mcp import envelope, framing, presentation
from enrichment_mcp._generated.research_envelope_schema import Code, Coverage, Diagnostic
from enrichment_mcp.daemon_client import (
    DaemonClient,
    DaemonUnavailableError,
)

__all__ = ["TOOL_NAMES", "build_server", "log"]

#: Names, contracts and annotations are emitted from the Rust operation declaration.
TOOL_NAMES = tuple(name for name, entry in presentation.bindings().items() if entry["published"])


# Retrieval reads a published snapshot: no network, so a short bound is enough to tell a
# wedged daemon from a slow one.
RETRIEVAL_TIMEOUT_SECONDS = 30.0


def log(message: str) -> None:
    """Write a diagnostic to stderr.

    Never stdout: that is the MCP protocol channel and a stray write corrupts the session.
    Written with an explicit stream write rather than ``print`` so that ruff's ``T20`` can stay
    on with no suppression anywhere in this package.
    """
    sys.stderr.write(f"{message}\n")


def _emit(result: dict[str, Any], *, tool: str | None = None) -> dict[str, Any]:
    """Validate an envelope on the way out, then return it.

    When ``tool`` is given and the result carries data, that data is also checked against the
    tool's own payload schema (`tool-data.schema.json`), so a drifted field name in the core is
    caught here rather than by the calling agent.

    Every tool return passes through here. Blueprint §7.4: "Ensure the actual output matches the
    declared schema" -- FastMCP publishes an output schema derived from the wire contract, and a
    response that did not conform would break that promise silently, at the one boundary a
    calling agent actually reads.

    The check is against the schema generated from the Rust wire types, so it is the same
    definition the CLI and RPC boundaries enforce. Validating a single small envelope costs far
    less than the `inline_result_bytes` budget it sits inside.
    """
    valid, reason = envelope.validate_document(json.dumps(result))
    if valid and tool is not None:
        try:
            presentation.validate_output(tool, result)
        except ValueError as exc:
            valid, reason = False, str(exc)
    if valid:
        return result

    # Our own output failed our own contract. That is an internal defect, so it is reported as
    # one rather than returned as if it were evidence -- and the replacement is built by the
    # error constructor, which cannot itself produce a non-conforming envelope.
    log(f"library-enrichment: emitted a non-conforming envelope: {reason}")
    return envelope.error(
        Code.INTERNAL_ERROR,
        "the service produced a response that does not match its own wire schema",
        "This is a bug in the service, not in the request. Report it with the daemon log.",
        retryable=False,
    )


def _boundary_result(result: dict[str, Any], *, tool: str | None = None) -> ToolResult:
    """Expose a local protocol/availability failure when no native delivery exists.

    This projection has no evidence selection, truncation or terminal-job policy. Native
    deliveries already contain their authoritative content and bypass this constructor.
    """
    result = _emit(result, tool=tool)
    return ToolResult.from_mcp_result(
        CallToolResult(
            content=[TextContent(type="text", text=json.dumps(result, ensure_ascii=False))],
            structured_content=result,
            is_error=result["status"] == "error",
        )
    )


_StrictArguments = validators.extend(
    Draft202012Validator,
    type_checker=Draft202012Validator.TYPE_CHECKER.redefine(
        "integer", lambda _checker, value: type(value) is int
    ),
)


class OperationBoundary(Middleware):
    """Observe original arguments and validate them before FastMCP's Python binding."""

    async def on_read_resource(
        self,
        context: MiddlewareContext[ReadResourceRequestParams],
        call_next: CallNext[ReadResourceRequestParams, ResourceResult],
    ) -> ResourceResult:
        started = monotonic()
        correlation = f"resource_{uuid4().hex}"
        try:
            return await call_next(context)
        except Exception as exc:
            log(
                f"library-enrichment: request_id={correlation} resource_read "
                f"exception={type(exc).__name__}"
            )
            raise
        finally:
            # Resource URI parameters can contain caller text; record the operation kind only.
            elapsed_ms = int((monotonic() - started) * 1000)
            log(
                f"library-enrichment: request_id={correlation} "
                f"resource_read elapsed_ms={elapsed_ms}"
            )

    async def on_call_tool(
        self,
        context: MiddlewareContext[CallToolRequestParams],
        call_next: CallNext[CallToolRequestParams, ToolResult],
    ) -> ToolResult:
        started = monotonic()
        name = context.message.name
        if context.fastmcp_context is None:
            return await call_next(context)
        tool = await context.fastmcp_context.fastmcp.get_tool(name)
        if tool is None:
            return await call_next(context)
        schema = {**tool.parameters, "additionalProperties": False}
        errors = list(_StrictArguments(schema).iter_errors(context.message.arguments or {}))
        if errors:
            error = errors[0]
            path = ".".join(str(part) for part in error.absolute_path) or "arguments"
            # Do not echo the supplied values (snippets can contain private source).
            return _boundary_result(
                envelope.error(
                    Code.UNSUPPORTED_FORMAT,
                    f"Invalid {name} input at {path}: {error.validator}",
                    "Use the published input schema and correct the named field.",
                ),
                tool=name,
            )
        correlation = "pre-admission"
        try:
            result = await call_next(context)
            if result.structured_content is not None:
                correlation = str(result.structured_content.get("request_id", correlation))
                job = result.structured_content.get("job")
                if isinstance(job, dict):
                    try:
                        await context.fastmcp_context.report_progress(
                            progress=0, message=str(job["stage"])
                        )
                    except Exception as exc:
                        # Progress is advisory; transport failure cannot replace a durable
                        # job receipt that the operation has already returned.
                        log(
                            f"library-enrichment: request_id={correlation} "
                            f"progress_exception={type(exc).__name__}"
                        )
            return result
        except ToolValidationError:
            return _boundary_result(
                envelope.error(
                    Code.UNSUPPORTED_FORMAT,
                    f"Invalid {name} argument binding",
                    "Use the published input schema and exact JSON field types.",
                ),
                tool=name,
            )
        except Exception as exc:
            failure = envelope.error(
                Code.INTERNAL_ERROR,
                "The adapter could not complete the operation.",
                "Report this request_id with the adapter and daemon logs.",
            )
            correlation = failure["request_id"]
            log(f"library-enrichment: request_id={correlation} exception={type(exc).__name__}")
            return _boundary_result(failure, tool=name)
        finally:
            log(
                f"library-enrichment: request_id={correlation} tool={name} "
                f"elapsed_ms={int((monotonic() - started) * 1000)}"
            )


class NativeTool(Tool):
    """One FastMCP execution mechanism for every generated core operation binding."""

    rpc_method: str
    rpc_timeout: float

    async def run(self, arguments: dict[str, Any]) -> ToolResult:
        delivery = framing.current_profile()
        wait = int(arguments.get("wait_seconds", "0")) if self.name == "job_control" else 0
        client = DaemonClient.from_env()
        try:
            response = await client.call(
                self.rpc_method,
                arguments,
                timeout_seconds=self.rpc_timeout + wait,
                delivery=delivery.profile,
            )
        except DaemonUnavailableError as exc:
            # Availability is an adapter-local observation, never a native evidence answer.
            if self.name == "service_status" and exc.cause == "connection":
                return _boundary_result(_unavailable_status(exc))
            return _boundary_result(_transport_error(exc))
        if isinstance(response.get("error"), dict):
            return _boundary_result(_rpc_error_envelope(response["error"]))
        payload = response.get("result")
        if not isinstance(payload, dict):
            raise ValueError("daemon returned no native MCP result")
        structured = payload.get("structuredContent")
        if not isinstance(structured, dict):
            raise ValueError("native MCP result has no evidence envelope")
        valid, reason = envelope.validate_document(json.dumps(structured))
        if not valid:
            raise ValueError(f"invalid native envelope: {reason}")
        presentation.validate_output(self.name, structured)
        return framing.native_result(payload, response.get("delivery_bytes"), delivery)


def build_server() -> FastMCP:
    """Construct the MCP server and register the tool catalog.

    Pure construction: no socket is opened, no package is fetched, no subprocess is started.
    """
    mcp: FastMCP = FastMCP(
        name=presentation.delivery_contract()["server_name"],
        version=presentation.delivery_contract()["server_version"],
        strict_input_validation=True,
        mask_error_details=True,
        middleware=[OperationBoundary()],
        instructions=(
            "Evidence service for Rust and Python libraries. Resolve exact release identity "
            "before asking anything else, and read `coverage` on every result: `ok` means "
            "successful within that scope, never complete knowledge of a library. For Rust, "
            "empty cfg_hints mean no condition was recorded; they do not establish availability "
            "under default features. Keep docs.rs observed build configuration separate from "
            "the requested project configuration. Preserve project_availability_unverified "
            "until matching configuration evidence or qualified execution establishes it."
        ),
    )

    for name in TOOL_NAMES:
        binding = presentation.bindings()[name]
        mcp.add_tool(
            NativeTool(
                name=name,
                description=binding["description"],
                parameters=binding["input_schema"],
                output_schema=binding["output_schema"],
                annotations=ToolAnnotations.model_validate(binding["annotations"]),
                rpc_method=binding["rpc"],
                rpc_timeout=float(binding["timeout_seconds"]),
            )
        )

    # Resource templates (blueprint §7.4) delegate to the same core reads as the tools, so a
    # client that surfaces resources and one that only surfaces tools see identical bytes.
    @mcp.resource(
        "library-evidence://workflow",
        name="research_workflow",
        description="Current selection, coverage, jobs, recovery and result-reading guidance.",
        mime_type="text/markdown",
    )
    async def workflow_resource() -> str:
        guidance = Path(__file__).with_name("_guidance").joinpath("tool-contract.md")
        return "Registered tools: " + ", ".join(TOOL_NAMES) + "\n\n" + guidance.read_text()

    @mcp.resource(
        "library-evidence://artifacts/{artifact_id}",
        name="artifact",
        description="A stored artifact, paged; the same read as the `read_artifact` tool.",
        mime_type="application/json",
    )
    async def artifact_resource(artifact_id: str) -> ResourceResult:
        return await _native_resource("artifact.read", {"artifact_id": artifact_id})

    @mcp.resource(
        "library-evidence://contexts/{context_id}/overview",
        name="context_overview",
        description="The faceted overview of a context; the same read as library_overview.",
        mime_type="application/json",
    )
    async def overview_resource(context_id: str) -> ResourceResult:
        return await _native_resource("library.overview", {"context_id": context_id})

    @mcp.resource(
        "library-evidence://snapshots/{snapshot_id}/manifest",
        name="snapshot_manifest",
        description="What a snapshot contains: counts, producers and coverage.",
        mime_type="application/json",
    )
    async def manifest_resource(snapshot_id: str) -> ResourceResult:
        return await _native_resource("snapshot.manifest", {"snapshot_id": snapshot_id})

    @mcp.resource(
        "library-evidence://jobs/{job_id}/result",
        name="job_result",
        mime_type="application/json",
    )
    async def job_resource(job_id: str) -> ResourceResult:
        return await _native_resource("job.control", {"job_id": job_id})

    return mcp


def _rpc_error_envelope(detail: dict[str, Any]) -> dict[str, Any]:
    """Map a JSON-RPC error to a typed envelope.

    Every RPC error the daemon emits carries the service code and a next action in ``data``
    (ADR 0006); when they are present they are forwarded as-is, so a parameter mistake reaches
    the caller as the code the core chose rather than as a generic transport failure.
    """
    raw_data = detail.get("data")
    data: dict[str, Any] = raw_data if isinstance(raw_data, dict) else {}
    try:
        code = Code(data["code"])
        diagnostic = Diagnostic.model_validate_json(json.dumps(data["diagnostic"]), strict=True)
        next_action = str(data["next_action"])
    except (KeyError, ValueError):
        return envelope.error(
            Code.INTERNAL_ERROR,
            "The daemon returned an invalid RPC diagnostic.",
            "Report the adapter request_id and daemon log.",
        )
    return envelope.error(
        code,
        str(detail.get("message", "the daemon returned an error")),
        next_action,
        retryable=data.get("retryable") is True,
        diagnostic=diagnostic,
    )


def _transport_error(exc: DaemonUnavailableError) -> dict[str, Any]:
    """Report the observed transport condition with a follow-up that can change it."""
    code, cause, action, reason = {
        "connection": (
            Code.UPSTREAM_UNAVAILABLE,
            "transport",
            "operator_setup",
            "Start the configured daemon with library-enrichmentd start.",
        ),
        "timeout": (
            Code.UPSTREAM_UNAVAILABLE,
            "deadline",
            "change_request",
            "Check service_status or poll an existing job handle. A timeout does not cancel work.",
        ),
        "request_limit": (
            Code.BUDGET_EXCEEDED,
            "capacity",
            "change_request",
            "Reduce the request's encoded size and submit it again.",
        ),
        "response_limit": (
            Code.INTERNAL_ERROR,
            "capacity",
            "report_defect",
            "Report the response-limit defect with the daemon log.",
        ),
        "incomplete_frame": (
            Code.UPSTREAM_UNAVAILABLE,
            "transport",
            "report_defect",
            "Check the daemon exit log and report the interrupted frame.",
        ),
        "protocol": (
            Code.INTERNAL_ERROR,
            "transport",
            "report_defect",
            "Report the malformed daemon response with the daemon log.",
        ),
    }[exc.cause]
    return envelope.error(
        code,
        str(exc),
        reason,
        retryable=False,
        diagnostic=envelope.boundary_diagnostic(cause, "daemon_transport", action, reason),
    )


async def _native_resource(method: str, params: dict[str, Any]) -> ResourceResult:
    """Native evidence composition with exact resource text/URI/framing measurement."""
    delivery = framing.current_profile(resource=True)
    client = DaemonClient.from_env()
    response = await client.call(
        method,
        params,
        timeout_seconds=RETRIEVAL_TIMEOUT_SECONDS,
        delivery=delivery.profile,
    )
    if response.get("error") is not None:
        raise ToolValidationError("The daemon rejected the resource request.")
    payload = response.get("result")
    if not isinstance(payload, dict):
        raise ValueError("daemon returned no native resource result")
    result = framing.native_resource(payload, response.get("delivery_bytes"), delivery)
    document = result.contents[0].content
    if not isinstance(document, str):
        raise ValueError("native evidence resource requires JSON text")
    valid, reason = envelope.validate_document(document)
    if not valid:
        raise ValueError(f"invalid native resource envelope: {reason}")
    tool = next(name for name, entry in presentation.bindings().items() if entry["rpc"] == method)
    presentation.validate_output(tool, json.loads(document))
    return result


def _unavailable_status(exc: DaemonUnavailableError) -> dict[str, Any]:
    """Only local availability facts exist when the daemon cannot answer."""
    return envelope.partial(
        "The daemon is not running, so only adapter-local facts are available.",
        {
            "daemon": {"available": False, "detail": str(exc)},
            "adapter": {"available": True, "tools": list(TOOL_NAMES)},
        },
        Coverage(
            details=None,
            assessments=[],
            scope="adapter-local status only",
            indexed=["adapter"],
            missing=["daemon", "producers", "cache"],
            limitations=[
                "The daemon was unreachable, so producer and cache state are unknown "
                "rather than absent. Start it with `library-enrichmentd start`."
            ],
        ),
    )
