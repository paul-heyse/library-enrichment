"""Construct response envelopes at the MCP boundary.

The envelope shape is owned by the Rust wire types (blueprint §6.3). The Pydantic models
imported here are generated from the schemas those types emit, so this module composes them --
it never restates the contract. If a field looks wrong, fix the Rust type and regenerate.

Blueprint §7.2's conditional rule is preserved by construction. :class:`_Outcome` mirrors the
Rust ``wire::Outcome``: ``status`` is derived from the outcome rather than passed alongside it,
and only :class:`_Failed` carries an error object. A caller cannot ask for ``status="error"``
with no error, or ``status="ok"`` with one, because neither is expressible.

**The generated Pydantic DTO is not sufficient to validate an envelope.** It enforces field
types and rejects unknown fields, but ``datamodel-codegen`` does not translate the contract's
root ``allOf`` conditionals into model validators, so the DTO alone accepts a ``pending``
envelope with a null job — which the Rust types, the frozen schema and the generated schema all
reject. :func:`validate_document` therefore validates against the generated JSON Schema, which
does carry those conditionals. That keeps one definition rather than restating the rule here;
gate C19's cross-boundary corpus is what measures the agreement.
"""

from __future__ import annotations

import json
import uuid
from dataclasses import dataclass
from functools import cache
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator
from jsonschema.protocols import Validator

from enrichment_mcp._generated.research_envelope_schema import (
    Code,
    Coverage,
    Error,
    Freshness,
    LibraryEnrichmentResponseEnvelope,
    Pagination,
    SchemaVersion,
    SourceVersionMatch,
    Status,
)

__all__ = [
    "SCHEMA_PATH",
    "error",
    "new_request_id",
    "ok",
    "partial",
    "validate_document",
    "validate_tool_data",
]

#: The schema emitted from the Rust wire types by `just schemas-generate`.
SCHEMA_PATH = (
    Path(__file__).resolve().parents[2] / "schemas/generated/research-envelope.schema.json"
)


#: The per-tool `data` payload schema, emitted alongside the envelope schema.
TOOL_DATA_SCHEMA_PATH = SCHEMA_PATH.with_name("tool-data.schema.json")


@cache
def _tool_data_validator() -> Validator:
    """Load the generated tool-data schema once per process."""
    return Draft202012Validator(json.loads(TOOL_DATA_SCHEMA_PATH.read_text()))


def validate_tool_data(tool: str, data: dict[str, Any]) -> tuple[bool, str | None]:
    """Validate one tool's ``data`` payload against the schema the Rust wire types emit.

    The envelope schema deliberately leaves ``data`` open (it is the tool-specific half of the
    result); this closes it per tool. The generated schema is a tagged union keyed by ``tool``,
    so the payload is checked as that variant. Only ``ok`` and ``partial`` results carry data
    worth checking; an error envelope's ``data`` is empty by construction.
    """
    tagged = {"tool": tool, **data}
    errors = sorted(_tool_data_validator().iter_errors(tagged), key=lambda e: list(e.absolute_path))
    if errors:
        return False, errors[0].message
    return True, None


@cache
def _validator() -> Validator:
    """Load the generated schema once per process.

    Cached because the adapter is a long-lived stdio process and re-reading the file per call
    would put filesystem I/O on the response path.
    """
    # `Draft202012Validator` is a variable bound to a dynamically created class, not a class
    # statement, so it cannot be used as a type annotation -- hence the `Validator` protocol.
    return Draft202012Validator(json.loads(SCHEMA_PATH.read_text()))


def validate_document(document: str) -> tuple[bool, str | None]:
    """Validate a candidate envelope, returning ``(valid, reason)``.

    This is the MCP boundary's half of acceptance gate C19. It uses the generated JSON Schema
    rather than the generated Pydantic DTO for the reason in the module docstring: only the
    schema carries the root conditionals.
    """
    try:
        parsed = json.loads(document)
    except json.JSONDecodeError as exc:
        return False, f"not valid JSON: {exc}"

    errors = sorted(_validator().iter_errors(parsed), key=lambda e: list(e.absolute_path))
    if errors:
        return False, errors[0].message
    return True, None


def new_request_id() -> str:
    """A fresh opaque request identifier.

    Kept separate from any job or content key: blueprint §8.3 is explicit that request IDs and
    reusable job/content keys must not be conflated, because jobs are shared between callers.
    """
    return f"req_{uuid.uuid4().hex}"


def _empty_pagination() -> Pagination:
    return Pagination(returned=0, total_matches=None, truncated=False, next_cursor=None)


def _unverified_freshness() -> Freshness:
    """Freshness for a result that consulted no registry.

    ``latest_verified=False`` is the honest default: a result that never asked cannot claim a
    release is still current, and a cache hit alone does not establish it either.
    """
    return Freshness(
        registry_checked_at=None,
        source_version_match=SourceVersionMatch.unknown,
        latest_verified=False,
    )


@dataclass(frozen=True, slots=True)
class _Succeeded:
    """Successful within the declared coverage."""


@dataclass(frozen=True, slots=True)
class _Partial:
    """Usable evidence together with explicit gaps."""


@dataclass(frozen=True, slots=True)
class _Failed:
    """A typed failure. The error object is required, not optional."""

    error: Error


#: The status/error pair as one value, mirroring the Rust `wire::Outcome`.
#:
#: `status` is derived from this rather than passed beside it, so the combinations the frozen
#: contract's root `allOf` forbids -- `error` status with no error object, `ok`/`partial` with
#: one -- are not expressible here at all.
_Outcome = _Succeeded | _Partial | _Failed


def _envelope(
    *,
    outcome: _Outcome,
    summary: str,
    data: dict[str, Any],
    coverage: Coverage,
) -> dict[str, Any]:
    match outcome:
        case _Succeeded():
            status, error_detail = Status.ok, None
        case _Partial():
            status, error_detail = Status.partial, None
        case _Failed(error=detail):
            status, error_detail = Status.error, detail

    envelope = LibraryEnrichmentResponseEnvelope(
        schema_version=SchemaVersion.field_1_0,
        request_id=new_request_id(),
        status=status,
        summary=summary,
        context_id=None,
        snapshot_id=None,
        data=data,
        coverage=coverage,
        freshness=_unverified_freshness(),
        evidence=[],
        artifacts=[],
        pagination=_empty_pagination(),
        job=None,
        error=error_detail,
    )
    # by_alias so the wire names are emitted, and no exclusion of None: the contract requires
    # all fourteen root fields to be present, with nullability expressed by value.
    return envelope.model_dump(mode="json", by_alias=True)


def ok(summary: str, data: dict[str, Any], coverage: Coverage) -> dict[str, Any]:
    """A result that succeeded within its declared coverage.

    ``ok`` never means complete knowledge of a library -- see ``coverage`` for what was actually
    looked at.
    """
    return _envelope(outcome=_Succeeded(), summary=summary, data=data, coverage=coverage)


def partial(summary: str, data: dict[str, Any], coverage: Coverage) -> dict[str, Any]:
    """A result carrying usable evidence together with explicit gaps."""
    return _envelope(outcome=_Partial(), summary=summary, data=data, coverage=coverage)


def error(
    code: Code,
    message: str,
    next_action: str,
    *,
    retryable: bool = False,
    summary: str | None = None,
) -> dict[str, Any]:
    """A typed failure carrying a concrete next action.

    Blueprint §7.2: retryability and a next action belong in the error record, not just a code.
    """
    return _envelope(
        outcome=_Failed(
            error=Error(
                code=code,
                message=message,
                retryable=retryable,
                next_action=next_action,
            )
        ),
        summary=summary or message,
        data={},
        coverage=Coverage(
            scope="no evidence was produced for this request",
            indexed=[],
            missing=[],
            limitations=[message],
        ),
    )
