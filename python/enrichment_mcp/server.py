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
from typing import Annotated, Any, Literal
from uuid import uuid4

from fastmcp import FastMCP
from fastmcp.exceptions import ValidationError as ToolValidationError
from fastmcp.resources.base import ResourceResult
from fastmcp.server.middleware import CallNext, Middleware, MiddlewareContext
from fastmcp.tools.base import ToolResult
from jsonschema import Draft202012Validator, validators
from mcp.types import (
    CallToolRequestParams,
    CallToolResult,
    ReadResourceRequestParams,
    ResourceLink,
    TextContent,
    ToolAnnotations,
)
from pydantic import BeforeValidator, Field

from enrichment_mcp import envelope, presentation
from enrichment_mcp._generated.request_schema import (
    ArtifactSection,
    DiscoverySelection,
    InspectionOptions,
    ResearchSelection,
)
from enrichment_mcp._generated.research_envelope_schema import Code, Coverage, Diagnostic
from enrichment_mcp.daemon_client import (
    ACQUISITION_TIMEOUT_SECONDS,
    DaemonClient,
    DaemonUnavailableError,
)

__all__ = ["TOOL_NAMES", "build_server", "log"]

#: Every tool the companion skill's contract names, in the order it lists them.
TOOL_NAMES = (
    "resolve_library",
    "library_overview",
    "search_evidence",
    "inspect_symbol",
    "compare_releases",
    "verify_usage",
    "read_artifact",
    "job_control",
    "service_status",
)

Ecosystem = Literal["rust", "python"]
ResearchMode = Literal["project", "upstream", "compare", "revision"]
FreshnessMode = Literal["cache_ok", "revalidate", "offline"]
EvidenceFamily = Literal["api", "docs", "examples", "release_notes", "features", "source"]


def _selection_from_json(value: object) -> ResearchSelection:
    return ResearchSelection.model_validate_json(json.dumps(value), strict=True)


def _discovery_from_json(value: object) -> DiscoverySelection:
    return DiscoverySelection.model_validate_json(json.dumps(value), strict=True)


DiscoveryInput = Annotated[DiscoverySelection, BeforeValidator(_discovery_from_json)]


def _execution_from_json(value: object) -> InspectionOptions:
    return InspectionOptions.model_validate_json(json.dumps(value), strict=True)


def _section_from_json(value: object) -> ArtifactSection:
    return ArtifactSection.model_validate_json(json.dumps(value), strict=True)


SectionInput = Annotated[ArtifactSection, BeforeValidator(_section_from_json)]
SelectionInput = Annotated[ResearchSelection, BeforeValidator(_selection_from_json)]
ExecutionInput = Annotated[InspectionOptions, BeforeValidator(_execution_from_json)]


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


MCP_FRAME_ALLOWANCE_BYTES = 1024


def _tool_result(result: dict[str, Any], *, tool: str | None = None) -> ToolResult:
    """Keep evidence in structured output, with a short optional human-readable projection."""
    result = _emit(result, tool=tool)
    summary = str(result.get("summary", "Evidence response"))[:160]
    while len(json.dumps(summary, ensure_ascii=False).encode()) > 160:
        summary = summary[:-1]
    content: list[TextContent | ResourceLink] = [TextContent(type="text", text=summary)]
    failed = presentation.is_error(result)
    if failed:
        content = [TextContent(type="text", text=json.dumps(presentation.error_preview(result)))]
    delivery = result["delivery"]
    if delivery["mode"] == "artifact" and not failed:
        content.append(
            ResourceLink(
                type="resource_link",
                name="Complete result",
                uri=f"library-evidence://artifacts/{delivery['artifact_id']}",
                mime_type="application/json",
            )
        )
    # Measure the complete MCP result; optional blocks never displace the structured handle.
    structured_bytes = len(json.dumps(result, ensure_ascii=False, separators=(",", ":")).encode())
    measured = CallToolResult(content=content, structured_content=result, is_error=failed)
    if len(measured.model_dump_json(by_alias=True).encode()) > (
        structured_bytes + MCP_FRAME_ALLOWANCE_BYTES - 128
    ):
        content = (
            [
                TextContent(
                    type="text", text=json.dumps(presentation.error_preview(result, compact=True))
                )
            ]
            if failed
            else []
        )
    return ToolResult(content=content, structured_content=result, is_error=failed)


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
            return _tool_result(
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
            return _tool_result(
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
            return _tool_result(failure, tool=name)
        finally:
            log(
                f"library-enrichment: request_id={correlation} tool={name} "
                f"elapsed_ms={int((monotonic() - started) * 1000)}"
            )


# Annotations are disclosure, never enforcement -- policy is enforced in the Rust core
# (blueprint §10). They are set explicitly rather than left unset because `destructiveHint` and
# `openWorldHint` are read as **true** when absent, so silence is the permissive answer.
#
# Written with the snake_case field names rather than the camelCase aliases. Both populate the
# model (`populate_by_name=True`) and both serialize to the camelCase wire form, but the field
# names are what the class actually declares, and `ty` cannot see through the alias generator.
_ACQUIRES = ToolAnnotations(
    read_only_hint=False,
    destructive_hint=False,
    idempotent_hint=True,
    open_world_hint=True,
)
_CACHED_READ = ToolAnnotations(
    read_only_hint=True,
    destructive_hint=False,
    idempotent_hint=True,
    open_world_hint=False,
)
# `verify_usage` builds an environment and executes code in an isolated profile. Blueprint §7.4
# is explicit that it "must not be portrayed as a pure read-only operation".
_EXECUTES = ToolAnnotations(
    read_only_hint=False,
    destructive_hint=False,
    idempotent_hint=False,
    open_world_hint=True,
)


def build_server() -> FastMCP:
    """Construct the MCP server and register the tool catalog.

    Pure construction: no socket is opened, no package is fetched, no subprocess is started.
    """
    mcp: FastMCP = FastMCP(
        name="library-enrichment",
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

    @mcp.tool(
        output_schema=presentation.output_schema("service_status"),
        name="service_status",
        description="Inspect readiness and capabilities, without indexing.",
        annotations=_CACHED_READ,
    )
    async def service_status(
        component: Annotated[
            str | None,
            Field(description="Restrict the report to one producer or component."),
        ] = None,
    ) -> ToolResult:
        """Report what this build can actually do, including what is absent."""
        payload, _from_core = await _service_status(component)
        # The payload schema is checked only for an answer the core composed. The
        # daemon-unreachable envelope below is adapter-local by necessity -- there is no core to
        # ask -- and its data is deliberately *not* a status payload. Validating it against one
        # would turn a truthful "the daemon is down" into a reported internal defect.
        return _tool_result(payload, tool="service_status")

    @mcp.tool(
        output_schema=presentation.output_schema("resolve_library"),
        name="resolve_library",
        description="Establish exact identity and environment before research.",
        annotations=_ACQUIRES,
    )
    async def resolve_library(
        ecosystem: Annotated[Ecosystem, Field(description="Which package ecosystem.")],
        name: Annotated[str, Field(description="Crate or distribution name.", min_length=1)],
        version: Annotated[
            str | None,
            Field(description="Exact version. Omit only for an explicit upstream question."),
        ] = None,
        mode: Annotated[
            ResearchMode | None,
            Field(
                description=(
                    "Research mode. Defaults to `project` with a version and `upstream` without."
                )
            ),
        ] = None,
        features: Annotated[
            list[str] | None,
            Field(description="Rust features your project enables; use extras for Python."),
        ] = None,
        default_features: Annotated[
            bool | None,
            Field(description="Whether your project enables default features, if known."),
        ] = None,
        target: Annotated[
            str | None, Field(description="Your project's target triple or platform, if known.")
        ] = None,
        repository: str | None = None,
        revision: str | None = None,
        package_subdir: str | None = None,
        python_version: str | None = None,
        extras: list[str] | None = None,
        allow_prerelease: bool = False,
        allow_yanked: bool = False,
        freshness: Annotated[
            FreshnessMode,
            Field(
                description=(
                    "`cache_ok` retains exact-version evidence indefinitely; "
                    "latest selection has a TTL. "
                    "`revalidate` "
                    "always consults the registry; `offline` never opens a socket."
                )
            ),
        ] = "cache_ok",
        allow_local_build: Annotated[
            bool,
            Field(
                description=(
                    "Accept a local rustdoc build when docs.rs JSON is missing or in an "
                    "unreadable format, or its observed build differs from requested "
                    "features/target. "
                    "Compiles the crate on a dated nightly in an isolated "
                    "capsule and can take minutes. Requires the operator-enabled `build` "
                    "profile; asking never grants it."
                )
            ),
        ] = False,
    ) -> ToolResult:
        """Resolve a release to a stable context identity."""
        params: dict[str, Any] = {
            "ecosystem": ecosystem,
            "name": name,
            "version": version,
            "mode": mode,
            "features": features,
            "default_features": default_features,
            "target": target,
            "repository": repository,
            "revision": revision,
            "package_subdir": package_subdir,
            "python_version": python_version,
            "extras": extras,
            "allow_prerelease": allow_prerelease,
            "allow_yanked": allow_yanked,
            "allow_local_build": allow_local_build,
            "freshness": freshness,
        }
        return _tool_result(
            await _research("library.resolve", params, timeout=ACQUISITION_TIMEOUT_SECONDS),
            tool="resolve_library",
        )

    @mcp.tool(
        output_schema=presentation.output_schema("library_overview"),
        name="library_overview",
        description="Discover unfamiliar capabilities without knowing symbol names.",
        annotations=_CACHED_READ,
    )
    async def library_overview(
        context_id: Annotated[str, Field(description="From `resolve_library`.", min_length=1)],
        discovery: Annotated[
            list[DiscoveryInput] | None,
            Field(
                description=(
                    "Independent feature, README, release-note and example pages. "
                    "Omit for bounded previews; use an empty list for namespace navigation alone."
                ),
                max_length=4,
            ),
        ] = None,
        area: Annotated[
            str | None, Field(description="Narrow the map to one module or feature area.")
        ] = None,
        snapshot_id: Annotated[
            str | None,
            Field(description="Read a specific snapshot; the context's current one otherwise."),
        ] = None,
        max_items: Annotated[
            int | None,
            Field(description="Child entries per namespace; the server caps it.", ge=1),
        ] = None,
        max_bytes: Annotated[
            int | None,
            Field(description="Inline byte budget; the server caps it.", ge=1024),
        ] = None,
    ) -> ToolResult:
        """Return the module/feature map and documentation headings."""
        return _tool_result(
            await _research(
                "library.overview",
                {
                    "discovery": None
                    if discovery is None
                    else [
                        item.model_dump(mode="json", by_alias=True, warnings="error")
                        for item in discovery
                    ],
                    "context_id": context_id,
                    "snapshot_id": snapshot_id,
                    "area": area,
                    "max_items": max_items,
                    "max_bytes": max_bytes,
                },
                timeout=RETRIEVAL_TIMEOUT_SECONDS,
            ),
            tool="library_overview",
        )

    @mcp.tool(
        output_schema=presentation.output_schema("search_evidence"),
        name="search_evidence",
        description="Search a bounded set of API, docs, examples, source, or release evidence.",
        annotations=_CACHED_READ,
    )
    async def search_evidence(
        context_id: Annotated[str, Field(description="From `resolve_library`.", min_length=1)],
        query: Annotated[str, Field(description="What to look for.", min_length=1)],
        kinds: Annotated[
            list[EvidenceFamily] | None,
            Field(description="Restrict to evidence kinds; all but `source` when omitted."),
        ] = None,
        area: Annotated[
            str | None, Field(description="Restrict to this namespace subtree.")
        ] = None,
        cursor: Annotated[
            str | None,
            Field(description="Continue a previous search. Same query, filters and snapshot."),
        ] = None,
        snapshot_id: Annotated[
            str | None,
            Field(description="Read a specific snapshot; the context's current one otherwise."),
        ] = None,
        max_items: Annotated[
            int | None, Field(description="Page size; the server caps it.", ge=1)
        ] = None,
        max_bytes: Annotated[
            int | None,
            Field(description="Inline byte budget; the server caps it.", ge=1024),
        ] = None,
    ) -> ToolResult:
        """Search bounded evidence for a context."""
        return _tool_result(
            await _research(
                "evidence.search",
                {
                    "context_id": context_id,
                    "snapshot_id": snapshot_id,
                    "query": query,
                    "kinds": kinds,
                    "area": area,
                    "cursor": cursor,
                    "max_items": max_items,
                    "max_bytes": max_bytes,
                },
                timeout=RETRIEVAL_TIMEOUT_SECONDS,
            ),
            tool="search_evidence",
        )

    @mcp.tool(
        output_schema=presentation.output_schema("inspect_symbol"),
        name="inspect_symbol",
        description=(
            "Read retained symbol evidence; explicit execution options can run "
            "isolated semantic or runtime inspection."
        ),
        annotations=_EXECUTES,
    )
    async def inspect_symbol(
        context_id: Annotated[str, Field(description="From `resolve_library`.", min_length=1)],
        symbol_path: Annotated[
            str, Field(description="Fully qualified symbol path.", min_length=1)
        ],
        definition_id: Annotated[
            str | None,
            Field(
                description="Select a definition from candidates when the public path is ambiguous."
            ),
        ] = None,
        selection: SelectionInput | None = None,
        snapshot_id: Annotated[
            str | None,
            Field(description="Read a specific snapshot; the context's current one otherwise."),
        ] = None,
        execution: Annotated[
            ExecutionInput | None,
            Field(description="Read retained evidence or explicitly select an execution profile."),
        ] = None,
        max_bytes: Annotated[
            int | None,
            Field(description="Inline byte budget; the server caps it.", ge=1024),
        ] = None,
    ) -> ToolResult:
        """Describe one symbol and what deploying it requires."""
        return _tool_result(
            await _research(
                "symbol.inspect",
                {
                    "context_id": context_id,
                    "snapshot_id": snapshot_id,
                    "symbol_path": symbol_path,
                    "definition_id": definition_id,
                    "selection": selection.model_dump(mode="json", by_alias=True, warnings="error")
                    if selection is not None
                    else {"mode": "default"},
                    "execution": execution.model_dump(mode="json", by_alias=True, warnings="error")
                    if execution is not None
                    else None,
                    "max_bytes": max_bytes,
                },
                timeout=RETRIEVAL_TIMEOUT_SECONDS,
            ),
            tool="inspect_symbol",
        )

    @mcp.tool(
        output_schema=presentation.output_schema("compare_releases"),
        name="compare_releases",
        description="Discover additions/removals and non-API changes.",
        annotations=_ACQUIRES,
    )
    async def compare_releases(
        ecosystem: Ecosystem | None = None,
        name: str | None = None,
        from_version: str | None = None,
        to_version: str | None = None,
        before_context_id: str | None = None,
        after_context_id: str | None = None,
        before_snapshot_id: str | None = None,
        alternative_cursor: str | None = None,
        after_snapshot_id: str | None = None,
        scopes: list[
            Literal["api", "docs", "configuration", "release_notes", "examples", "relationships"]
        ]
        | None = None,
        cursor: str | None = None,
        max_items: Annotated[int | None, Field(ge=1)] = None,
        max_bytes: Annotated[int | None, Field(ge=1024)] = None,
    ) -> ToolResult:
        """Compare pinned contexts locally, or explicitly resolve a version pair first.

        Use exactly one input form. Environment differences and incomplete coverage are reported
        before interpreting API additions/removals, documentation and behavior notes.
        """
        params = {
            "ecosystem": ecosystem,
            "name": name,
            "from_version": from_version,
            "to_version": to_version,
            "before_context_id": before_context_id,
            "after_context_id": after_context_id,
            "before_snapshot_id": before_snapshot_id,
            "alternative_cursor": alternative_cursor,
            "after_snapshot_id": after_snapshot_id,
            "scopes": scopes,
            "cursor": cursor,
            "max_items": max_items,
            "max_bytes": max_bytes,
        }
        return _tool_result(
            await _research("library.compare", params, timeout=ACQUISITION_TIMEOUT_SECONDS),
            tool="compare_releases",
        )

    @mcp.tool(
        output_schema=presentation.output_schema("verify_usage"),
        name="verify_usage",
        description="Test a proposed invocation or composition in isolation.",
        annotations=_EXECUTES,
    )
    async def verify_usage(
        context_id: Annotated[str, Field(description="From `resolve_library`.", min_length=1)],
        snippet: Annotated[str, Field(description="The code to verify.", min_length=1)],
        mode: Annotated[
            Literal["typecheck", "compile", "runtime"],
            Field(description="What to establish. These prove different things."),
        ] = "typecheck",
        profile: Literal["build", "runtime"] = "build",
        snapshot_id: str | None = None,
        test_intent: str | None = None,
        max_bytes: Annotated[int | None, Field(ge=1024)] = None,
    ) -> ToolResult:
        """Verify a usage pattern in a service-owned isolated environment."""
        return _tool_result(
            await _research(
                "usage.verify",
                {
                    "context_id": context_id,
                    "snapshot_id": snapshot_id,
                    "snippet": snippet,
                    "mode": mode,
                    "profile": profile,
                    "test_intent": test_intent,
                    "max_bytes": max_bytes,
                },
                timeout=RETRIEVAL_TIMEOUT_SECONDS,
            ),
            tool="verify_usage",
        )

    @mcp.tool(
        output_schema=presentation.output_schema("read_artifact"),
        name="read_artifact",
        description="Retrieve large result sections without flooding context.",
        annotations=_CACHED_READ,
    )
    async def read_artifact(
        artifact_id: Annotated[
            str, Field(description="From a result's `artifacts`.", min_length=1)
        ],
        section: Annotated[
            SectionInput | None,
            Field(description="Read a typed result section or a Markdown heading."),
        ] = None,
        cursor: Annotated[str | None, Field(description="Continue a previous read.")] = None,
        max_bytes: Annotated[
            int | None,
            Field(description="Bytes per slice; the server caps it.", ge=1024),
        ] = None,
    ) -> ToolResult:
        """Read a bounded section of a stored artifact, addressed by ID and never by path."""
        return _tool_result(
            await _read_artifact(artifact_id, section, cursor, max_bytes),
            tool="read_artifact",
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
    async def artifact_resource(artifact_id: str) -> str:
        return json.dumps(
            _emit(await _read_artifact(artifact_id, None, None, None), tool="read_artifact")
        )

    @mcp.resource(
        "library-evidence://contexts/{context_id}/overview",
        name="context_overview",
        description="The faceted overview of a context; the same read as `library_overview`.",
        mime_type="application/json",
    )
    async def overview_resource(context_id: str) -> str:
        payload = await _research(
            "library.overview",
            {"context_id": context_id},
            timeout=RETRIEVAL_TIMEOUT_SECONDS,
        )
        return json.dumps(_emit(payload, tool="library_overview"))

    @mcp.resource(
        "library-evidence://snapshots/{snapshot_id}/manifest",
        name="snapshot_manifest",
        description="What a snapshot contains: counts, producers and coverage.",
        mime_type="application/json",
    )
    async def manifest_resource(snapshot_id: str) -> str:
        payload = await _research(
            "snapshot.manifest",
            {"snapshot_id": snapshot_id},
            timeout=RETRIEVAL_TIMEOUT_SECONDS,
        )
        return json.dumps(_emit(payload, tool="snapshot_manifest"))

    @mcp.tool(
        output_schema=presentation.output_schema("job_control"),
        name="job_control",
        description="Observe or cancel an explicitly submitted long-running operation.",
        annotations=_EXECUTES,
    )
    async def job_control(
        job_id: Annotated[str, Field(description="From a `pending` result.", min_length=1)],
        action: Annotated[
            Literal["status", "wait", "cancel"],
            Field(description="`cancel` drops your interest; shared work may continue."),
        ] = "status",
        wait_seconds: Annotated[
            int, Field(description="Bounded wait for `wait`.", ge=0, le=10)
        ] = 0,
        interest_token: str | None = None,
        max_bytes: Annotated[int | None, Field(ge=1024)] = None,
    ) -> ToolResult:
        """Observe or cancel a durable core job."""
        return _tool_result(
            await _research(
                "job.control",
                {
                    "job_id": job_id,
                    "action": action,
                    "wait_seconds": wait_seconds,
                    "interest_token": interest_token,
                    "max_bytes": max_bytes,
                },
                timeout=RETRIEVAL_TIMEOUT_SECONDS + wait_seconds,
            ),
            tool="job_control",
        )

    # The frozen template in blueprint 7.4 is `.../jobs/{job_id}/result`, and a resource URI is
    # part of the contract a client binds to -- an adapter that served a different one would be
    # answering a question nobody asked.
    @mcp.resource(
        "library-evidence://jobs/{job_id}/result",
        name="job_result",
        mime_type="application/json",
    )
    async def job_resource(job_id: str) -> str:
        return json.dumps(
            _emit(
                await _research(
                    "job.control", {"job_id": job_id}, timeout=RETRIEVAL_TIMEOUT_SECONDS
                ),
                tool="job_control",
            )
        )

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


async def _research(method: str, params: dict[str, Any], *, timeout: float) -> dict[str, Any]:
    """Call a research method and forward the core's envelope.

    The daemon builds the whole envelope -- identity, coverage, freshness, evidence -- because
    those are evidence-model assertions and §1.1 gives the evidence model to the core. This
    function calls, maps a structured error, and forwards. An unreachable daemon is an
    ``error`` here rather than the ``partial`` `service_status` returns: with no daemon there
    is no evidence at all, and the next action is the same either way.
    """
    tool = {
        "usage.verify": "verify_usage",
        "job.control": "job_control",
        "library.resolve": "resolve_library",
        "library.compare": "compare_releases",
        "library.overview": "library_overview",
        "evidence.search": "search_evidence",
        "symbol.inspect": "inspect_symbol",
        "artifact.read": "read_artifact",
        "snapshot.manifest": "snapshot_manifest",
    }.get(method)
    if tool is not None:
        valid, reason = envelope.validate_request(tool, params)
        if not valid:
            return envelope.error(
                Code.UNSUPPORTED_FORMAT,
                f"invalid {tool} request: {reason}",
                "Use the tool's declared input schema.",
                retryable=False,
            )
    client = DaemonClient.from_env()
    try:
        response = await client.call(method, params, timeout_seconds=timeout)
    except DaemonUnavailableError as exc:
        return _transport_error(exc)

    detail = response.get("error")
    if isinstance(detail, dict):
        return _rpc_error_envelope(detail)

    result = response.get("result")
    if not isinstance(result, dict):
        return envelope.error(
            Code.INTERNAL_ERROR,
            "the daemon returned no envelope",
            "Report the malformed response with the daemon log.",
            retryable=False,
        )
    # Forwarded verbatim. `_emit` validates it on the way out.
    return result


async def _read_artifact(
    artifact_id: str, section: ArtifactSection | None, cursor: str | None, max_bytes: int | None
) -> dict[str, Any]:
    """One read, shared by the `read_artifact` tool and the artifact resource template."""
    return await _research(
        "artifact.read",
        {
            "artifact_id": artifact_id,
            "section": section.model_dump(mode="json", by_alias=True, warnings="error")
            if section is not None
            else None,
            "cursor": cursor,
            "max_bytes": max_bytes,
        },
        timeout=RETRIEVAL_TIMEOUT_SECONDS,
    )


async def _service_status(component: str | None) -> tuple[dict[str, Any], bool]:
    """Forward the daemon's envelope, or report its absence.

    The daemon builds the whole envelope -- identity, coverage and freshness included -- because
    those are evidence-model assertions and §1.1 gives the evidence model to the core. This
    function does what §2.1 says an adapter does: calls, maps a structured error, forwards.

    The one envelope composed here is the daemon-unreachable case, and only because the core is
    by definition not around to state it. Even then it is `partial` with the gap named, never an
    empty `ok`: "the daemon is down" and "no producers are installed" are different facts.

    Returns the envelope and whether the core composed it, because only a core-composed one
    carries a `service_status` payload the generated schema can check.
    """
    client = DaemonClient.from_env()
    params = {"component": component} if component is not None else {}
    try:
        response = await client.call("service.status", params)
    except DaemonUnavailableError as exc:
        if exc.cause != "connection":
            return _transport_error(exc), False
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
        ), False

    if response.get("error") is not None:
        return _rpc_error_envelope(response["error"]), False

    result = response.get("result")
    if not isinstance(result, dict):
        return envelope.error(
            Code.INTERNAL_ERROR,
            "the daemon returned no envelope",
            "Report the malformed response with the daemon log.",
            retryable=False,
        ), False
    # Forwarded verbatim. `_emit` validates both the envelope and, because this one came from
    # the core, its `service_status` payload -- so a drifted field name in the daemon is caught
    # here rather than by the calling agent.
    return result, True
