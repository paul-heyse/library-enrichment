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
from enrichment_mcp.daemon_client import DaemonClient, DaemonUnavailableError

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

#: Which phase implements each tool that Phase 0 does not.
_PHASE_OF = {
    "resolve_library": 1,
    "library_overview": 1,
    "search_evidence": 1,
    "inspect_symbol": 1,
    "read_artifact": 1,
    "compare_releases": 3,
    "verify_usage": 4,
    "job_control": 4,
}

Ecosystem = Literal["rust", "python"]


def log(message: str) -> None:
    """Write a diagnostic to stderr.

    Never stdout: that is the MCP protocol channel and a stray write corrupts the session.
    Written with an explicit stream write rather than ``print`` so that ruff's ``T20`` can stay
    on with no suppression anywhere in this package.
    """
    sys.stderr.write(f"{message}\n")


def _emit(result: dict[str, Any]) -> dict[str, Any]:
    """Validate an envelope on the way out, then return it.

    Every tool return passes through here. Blueprint §7.4: "Ensure the actual output matches the
    declared schema" -- FastMCP publishes an output schema derived from the wire contract, and a
    response that did not conform would break that promise silently, at the one boundary a
    calling agent actually reads.

    The check is against the schema generated from the Rust wire types, so it is the same
    definition the CLI and RPC boundaries enforce. Validating a single small envelope costs far
    less than the `inline_result_bytes` budget it sits inside.
    """
    valid, reason = envelope.validate_document(json.dumps(result))
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
    phase = _PHASE_OF[tool]
    return _emit(
        envelope.error(
            Code.UNSUPPORTED_CAPABILITY,
            f"`{tool}` is not implemented in this build",
            f"This service is at phase 0; `{tool}` lands in phase {phase}. "
            f"Call `service_status` to see which producers are installed.",
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
        revalidate: Annotated[
            bool,
            Field(description="Re-check the registry rather than trusting a cached answer."),
        ] = False,
    ) -> dict[str, Any]:
        """Resolve a release to a stable context identity."""
        return _not_implemented("resolve_library")

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
    ) -> dict[str, Any]:
        """Return the module/feature map and documentation headings."""
        return _not_implemented("library_overview")

    @mcp.tool(
        name="search_evidence",
        description="Search a bounded set of API, docs, examples, source, or release evidence.",
        annotations=_READ_ONLY,
    )
    async def search_evidence(
        context_id: Annotated[str, Field(description="From `resolve_library`.", min_length=1)],
        query: Annotated[str, Field(description="What to look for.", min_length=1)],
        kinds: Annotated[
            list[str] | None,
            Field(description="Restrict to evidence kinds, e.g. `api`, `docs`, `examples`."),
        ] = None,
        cursor: Annotated[
            str | None,
            Field(description="Continue a previous search. Same query, filters and snapshot."),
        ] = None,
    ) -> dict[str, Any]:
        """Search bounded evidence for a context."""
        return _not_implemented("search_evidence")

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
    ) -> dict[str, Any]:
        """Describe one symbol and what deploying it requires."""
        return _not_implemented("inspect_symbol")

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
    ) -> dict[str, Any]:
        """Read a bounded section of a stored artifact, addressed by ID and never by path."""
        return _not_implemented("read_artifact")

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


async def _service_status(component: str | None) -> dict[str, Any]:
    """Ask the daemon for its status, and answer truthfully when it is not running.

    A stopped daemon is reported as a fact, not raised as an error: the Phase 0 gate asks that
    ``service_status`` "truthfully reports absent components", and a tool call that fails tells
    a caller nothing about what is installed.
    """
    client = DaemonClient.from_env()
    try:
        response = await client.call("service.status")
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

    result: dict[str, Any] = response.get("result", {})
    if component is None:
        return envelope.ok(
            "Service status as reported by the daemon.",
            result,
            Coverage(
                scope="installed components and their availability",
                indexed=["adapter", "daemon", "producers", "features"],
                missing=[],
                limitations=[
                    "Reports what is installed, not whether library evidence has been indexed."
                ],
            ),
        )

    result, matched = _filter_component(result, component)
    if matched:
        return envelope.ok(
            f"Status for `{component}`.",
            result,
            Coverage(
                scope=f"components matching `{component}`",
                indexed=["adapter", "daemon", "producers", "features"],
                missing=[],
                limitations=[
                    "Reports what is installed, not whether library evidence has been indexed."
                ],
            ),
        )

    # An empty `ok` here would be indistinguishable from "this build has no such producer",
    # which is a different and much stronger claim than "the filter matched nothing". The
    # coverage block has to carry that difference, or a caller cannot tell them apart --
    # `.claude/rules/evidence-truthfulness.md`'s C01 distinction, in miniature.
    return envelope.partial(
        f"No component named `{component}` is known to this build.",
        result,
        Coverage(
            scope=f"components matching `{component}`",
            indexed=["adapter", "daemon"],
            missing=[f"producers matching `{component}`", f"features matching `{component}`"],
            limitations=[
                f"`{component}` did not match any component this build reports. That is not "
                f"evidence that no such component exists -- call `service_status` with no "
                f"filter to see the full list."
            ],
        ),
    )


def _filter_component(result: dict[str, Any], component: str) -> tuple[dict[str, Any], bool]:
    """Narrow a status report to one named component.

    Returns the filtered result and whether anything matched, so the caller can distinguish a
    narrowed answer from an unmatched filter. Those are different facts and must not share a
    coverage block.
    """
    filtered = dict(result)
    matched = False
    for key in ("producers", "features"):
        entries = result.get(key, [])
        if isinstance(entries, list):
            kept = [e for e in entries if e.get("name") == component]
            matched = matched or bool(kept)
            filtered[key] = kept
    return filtered, matched
