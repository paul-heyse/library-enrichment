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

All nine tools are registered from the start with their real input schemas, so the contract a
calling agent sees is the contract Phase 1 will fill in. The eight that are not implemented yet
return a typed ``UNSUPPORTED_CAPABILITY`` envelope naming the phase that will implement them --
an honest typed error, never a fabricated success and never an empty ``ok``.
"""

from __future__ import annotations

import json
import sys
from typing import Annotated, Any, Literal

from fastmcp import FastMCP
from mcp.types import ToolAnnotations
from pydantic import Field

from enrichment_mcp import envelope
from enrichment_mcp._generated.research_envelope_schema import Code, Coverage
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

#: The tools that are not wired to a daemon method yet, and where each one lands.
#:
#: Only tools that actually reach `_not_implemented` belong here. Listing a tool that already
#: routes through `_research` makes this look like a registry of service state, which it is not
#: -- the dispatch table is. Keeping a stale entry here is what let the caller-visible message
#: claim a phase the service had already left.
_UNIMPLEMENTED_IN = {
    "compare_releases": 3,
    "verify_usage": 4,
    "job_control": 4,
}

Ecosystem = Literal["rust", "python"]
ResearchMode = Literal["project", "upstream", "compare", "revision"]
FreshnessMode = Literal["cache_ok", "revalidate", "offline"]
EvidenceFamily = Literal["api", "docs", "examples", "release_notes", "features", "source"]
Aspect = Literal[
    "signature", "availability", "relationships", "documentation", "examples", "source", "semantics"
]

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
    if valid and tool is not None and result.get("status") in {"ok", "partial"}:
        raw_data = result.get("data")
        if isinstance(raw_data, dict):
            valid, reason = envelope.validate_tool_data(tool, raw_data)
    if valid:
        return result

    # Our own output failed our own contract. That is an internal defect, so it is reported as
    # one rather than returned as if it were evidence -- and the replacement is built by the
    # error constructor, which cannot itself produce a non-conforming envelope.
    log(f"library-enrichment: emitted a non-conforming envelope: {reason}")
    return envelope.error(
        Code.UNSUPPORTED_FORMAT,
        "the service produced a response that does not match its own wire schema",
        "This is a bug in the service, not in the request. Report it with the daemon log.",
        retryable=False,
    )


def _not_implemented(tool: str) -> dict[str, Any]:
    """The typed answer for a tool whose producers do not exist yet.

    ``UNSUPPORTED_CAPABILITY`` is the honest code from the frozen thirteen: the capability is
    genuinely absent, and the next action says when it arrives. Deliberately not an empty
    ``ok`` -- a successful empty result would assert that the question had been answered.
    """
    phase = _UNIMPLEMENTED_IN[tool]
    return _emit(
        envelope.error(
            Code.UNSUPPORTED_CAPABILITY,
            f"`{tool}` is not implemented in this build",
            f"`{tool}` lands in phase {phase}. "
            f"Call `service_status` to see which producers are installed and which "
            f"tools this build answers.",
            retryable=False,
        )
    )


# Annotations are disclosure, never enforcement -- policy is enforced in the Rust core
# (blueprint §10). They are set explicitly rather than left unset because `destructiveHint` and
# `openWorldHint` are read as **true** when absent, so silence is the permissive answer.
#
# Written with the snake_case field names rather than the camelCase aliases. Both populate the
# model (`populate_by_name=True`) and both serialize to the camelCase wire form, but the field
# names are what the class actually declares, and `ty` cannot see through the alias generator.
_READ_ONLY = ToolAnnotations(
    read_only_hint=True,
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
        instructions=(
            "Evidence service for Rust and Python libraries. Resolve exact release identity "
            "before asking anything else, and read `coverage` on every result: `ok` means "
            "successful within that scope, never complete knowledge of a library."
        ),
    )

    @mcp.tool(
        name="service_status",
        description="Inspect readiness and capabilities, without indexing.",
        annotations=_CACHED_READ,
    )
    async def service_status(
        component: Annotated[
            str | None,
            Field(description="Restrict the report to one producer or component."),
        ] = None,
    ) -> dict[str, Any]:
        """Report what this build can actually do, including what is absent."""
        return _emit(await _service_status(component))

    @mcp.tool(
        name="resolve_library",
        description="Establish exact identity and environment before research.",
        annotations=_READ_ONLY,
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
            Field(description="Features (Rust) or extras (Python) your project enables."),
        ] = None,
        default_features: Annotated[
            bool | None,
            Field(description="Whether your project enables default features, if known."),
        ] = None,
        target: Annotated[
            str | None, Field(description="Your project's target triple or platform, if known.")
        ] = None,
        freshness: Annotated[
            FreshnessMode,
            Field(
                description=(
                    "`cache_ok` reuses a recorded resolution within its TTL; `revalidate` "
                    "always consults the registry; `offline` never opens a socket."
                )
            ),
        ] = "cache_ok",
        revalidate: Annotated[
            bool,
            Field(description="Shorthand for `freshness=revalidate`."),
        ] = False,
    ) -> dict[str, Any]:
        """Resolve a release to a stable context identity."""
        params: dict[str, Any] = {
            "ecosystem": ecosystem,
            "name": name,
            "version": version,
            "mode": mode,
            "features": features,
            "default_features": default_features,
            "target": target,
            "freshness": "revalidate" if revalidate else freshness,
        }
        return _emit(
            await _research("library.resolve", params, timeout=ACQUISITION_TIMEOUT_SECONDS),
            tool="resolve_library",
        )

    @mcp.tool(
        name="library_overview",
        description="Discover unfamiliar capabilities without knowing symbol names.",
        annotations=_READ_ONLY,
    )
    async def library_overview(
        context_id: Annotated[str, Field(description="From `resolve_library`.", min_length=1)],
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
    ) -> dict[str, Any]:
        """Return the module/feature map and documentation headings."""
        return _emit(
            await _research(
                "library.overview",
                {
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
        name="search_evidence",
        description="Search a bounded set of API, docs, examples, source, or release evidence.",
        annotations=_READ_ONLY,
    )
    async def search_evidence(
        context_id: Annotated[str, Field(description="From `resolve_library`.", min_length=1)],
        query: Annotated[str, Field(description="What to look for.", min_length=1)],
        kinds: Annotated[
            list[EvidenceFamily] | None,
            Field(description="Restrict to evidence kinds; all but `source` when omitted."),
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
    ) -> dict[str, Any]:
        """Search bounded evidence for a context."""
        return _emit(
            await _research(
                "evidence.search",
                {
                    "context_id": context_id,
                    "snapshot_id": snapshot_id,
                    "query": query,
                    "kinds": kinds,
                    "cursor": cursor,
                    "max_items": max_items,
                    "max_bytes": max_bytes,
                },
                timeout=RETRIEVAL_TIMEOUT_SECONDS,
            ),
            tool="search_evidence",
        )

    @mcp.tool(
        name="inspect_symbol",
        description="Characterize a known candidate and its deployment requirements.",
        annotations=_READ_ONLY,
    )
    async def inspect_symbol(
        context_id: Annotated[str, Field(description="From `resolve_library`.", min_length=1)],
        symbol_path: Annotated[
            str, Field(description="Fully qualified symbol path.", min_length=1)
        ],
        depth: Annotated[
            Literal["signature", "documentation", "source"],
            Field(description="How much to retrieve. Request `source` only when needed."),
        ] = "documentation",
        aspects: Annotated[
            list[Aspect] | None,
            Field(description="Aspects to include; all supported ones when omitted."),
        ] = None,
        snapshot_id: Annotated[
            str | None,
            Field(description="Read a specific snapshot; the context's current one otherwise."),
        ] = None,
        max_bytes: Annotated[
            int | None,
            Field(description="Inline byte budget; the server caps it.", ge=1024),
        ] = None,
    ) -> dict[str, Any]:
        """Describe one symbol and what deploying it requires."""
        return _emit(
            await _research(
                "symbol.inspect",
                {
                    "context_id": context_id,
                    "snapshot_id": snapshot_id,
                    "symbol_path": symbol_path,
                    "depth": depth,
                    "aspects": aspects,
                    "max_bytes": max_bytes,
                },
                timeout=RETRIEVAL_TIMEOUT_SECONDS,
            ),
            tool="inspect_symbol",
        )

    @mcp.tool(
        name="compare_releases",
        description="Discover additions/removals and non-API changes.",
        annotations=_READ_ONLY,
    )
    async def compare_releases(
        ecosystem: Annotated[Ecosystem, Field(description="Which package ecosystem.")],
        name: Annotated[str, Field(description="Crate or distribution name.", min_length=1)],
        from_version: Annotated[str, Field(description="Baseline version.", min_length=1)],
        to_version: Annotated[str, Field(description="Candidate version.", min_length=1)],
    ) -> dict[str, Any]:
        """Compare two releases across API, configuration and release notes."""
        return _not_implemented("compare_releases")

    @mcp.tool(
        name="verify_usage",
        description="Test a proposed invocation or composition in isolation.",
        annotations=_EXECUTES,
    )
    async def verify_usage(
        context_id: Annotated[str, Field(description="From `resolve_library`.", min_length=1)],
        snippet: Annotated[str, Field(description="The code to verify.", min_length=1)],
        mode: Annotated[
            Literal["typecheck", "compile", "run"],
            Field(description="What to establish. These prove different things."),
        ] = "typecheck",
    ) -> dict[str, Any]:
        """Verify a usage pattern in a service-owned isolated environment."""
        return _not_implemented("verify_usage")

    @mcp.tool(
        name="read_artifact",
        description="Retrieve large result sections without flooding context.",
        annotations=_READ_ONLY,
    )
    async def read_artifact(
        artifact_id: Annotated[
            str, Field(description="From a result's `artifacts`.", min_length=1)
        ],
        section: Annotated[
            str | None, Field(description="Read one named section instead of the whole thing.")
        ] = None,
        cursor: Annotated[str | None, Field(description="Continue a previous read.")] = None,
        max_bytes: Annotated[
            int | None,
            Field(description="Bytes per slice; the server caps it.", ge=1024),
        ] = None,
    ) -> dict[str, Any]:
        """Read a bounded section of a stored artifact, addressed by ID and never by path."""
        return _emit(
            await _read_artifact(artifact_id, section, cursor, max_bytes),
            tool="read_artifact",
        )

    # Resource templates (blueprint §7.4) delegate to the same core reads as the tools, so a
    # client that surfaces resources and one that only surfaces tools see identical bytes.
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
        name="job_control",
        description="Observe or cancel an explicitly submitted long-running operation.",
        annotations=_READ_ONLY,
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
    ) -> dict[str, Any]:
        """Observe or cancel a durable core job."""
        return _not_implemented("job_control")

    return mcp


def _rpc_error_envelope(detail: dict[str, Any]) -> dict[str, Any]:
    """Map a JSON-RPC error to a typed envelope.

    Every RPC error the daemon emits carries the service code and a next action in ``data``
    (ADR 0006); when they are present they are forwarded as-is, so a parameter mistake reaches
    the caller as the code the core chose rather than as a generic transport failure.
    """
    raw_data = detail.get("data")
    data: dict[str, Any] = raw_data if isinstance(raw_data, dict) else {}
    raw_code = data.get("code")
    code = Code(raw_code) if raw_code in Code.__members__.values() else Code.UPSTREAM_UNAVAILABLE
    next_action = str(data.get("next_action") or "Check the daemon log, then retry.")
    return envelope.error(
        code,
        str(detail.get("message", "the daemon returned an error")),
        next_action,
        retryable=code is Code.UPSTREAM_UNAVAILABLE,
    )


async def _research(method: str, params: dict[str, Any], *, timeout: float) -> dict[str, Any]:
    """Call a research method and forward the core's envelope.

    The daemon builds the whole envelope -- identity, coverage, freshness, evidence -- because
    those are evidence-model assertions and §1.1 gives the evidence model to the core. This
    function calls, maps a structured error, and forwards. An unreachable daemon is an
    ``error`` here rather than the ``partial`` `service_status` returns: with no daemon there
    is no evidence at all, and the next action is the same either way.
    """
    client = DaemonClient.from_env()
    try:
        response = await client.call(method, params, timeout_seconds=timeout)
    except DaemonUnavailableError as exc:
        return envelope.error(
            Code.UPSTREAM_UNAVAILABLE,
            f"the daemon is unavailable: {exc}",
            "Start it with `library-enrichmentd start`, then retry.",
            retryable=True,
        )

    detail = response.get("error")
    if isinstance(detail, dict):
        return _rpc_error_envelope(detail)

    result = response.get("result")
    if not isinstance(result, dict):
        return envelope.error(
            Code.UPSTREAM_UNAVAILABLE,
            "the daemon returned no envelope",
            "Check the daemon log, then retry.",
            retryable=True,
        )
    # Forwarded verbatim. `_emit` validates it on the way out.
    return result


async def _read_artifact(
    artifact_id: str, section: str | None, cursor: str | None, max_bytes: int | None
) -> dict[str, Any]:
    """One read, shared by the `read_artifact` tool and the artifact resource template."""
    return await _research(
        "artifact.read",
        {
            "artifact_id": artifact_id,
            "section": section,
            "cursor": cursor,
            "max_bytes": max_bytes,
        },
        timeout=RETRIEVAL_TIMEOUT_SECONDS,
    )


async def _service_status(component: str | None) -> dict[str, Any]:
    """Forward the daemon's envelope, or report its absence.

    The daemon builds the whole envelope -- identity, coverage and freshness included -- because
    those are evidence-model assertions and §1.1 gives the evidence model to the core. This
    function does what §2.1 says an adapter does: calls, maps a structured error, forwards.

    The one envelope composed here is the daemon-unreachable case, and only because the core is
    by definition not around to state it. Even then it is `partial` with the gap named, never an
    empty `ok`: "the daemon is down" and "no producers are installed" are different facts.
    """
    client = DaemonClient.from_env()
    params = {"component": component} if component is not None else {}
    try:
        response = await client.call("service.status", params)
    except DaemonUnavailableError as exc:
        return envelope.partial(
            "The daemon is not running, so only adapter-local facts are available.",
            {
                "daemon": {"available": False, "detail": str(exc)},
                "adapter": {"available": True, "tools": list(TOOL_NAMES)},
            },
            Coverage(
                scope="adapter-local status only",
                indexed=["adapter"],
                missing=["daemon", "producers", "cache"],
                limitations=[
                    "The daemon was unreachable, so producer and cache state are unknown "
                    "rather than absent. Start it with `library-enrichmentd start`."
                ],
            ),
        )

    if response.get("error") is not None:
        detail = response["error"]
        return envelope.error(
            Code.UPSTREAM_UNAVAILABLE,
            str(detail.get("message", "the daemon returned an error")),
            "Check the daemon log, then retry.",
            retryable=True,
        )

    result = response.get("result")
    if not isinstance(result, dict):
        return envelope.error(
            Code.UPSTREAM_UNAVAILABLE,
            "the daemon returned no envelope",
            "Check the daemon log, then retry.",
            retryable=True,
        )
    # Forwarded verbatim. `_emit` validates it on the way out, so a malformed core envelope is
    # caught here rather than reaching a caller.
    return result
