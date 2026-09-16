"""Acceptance gate C19: **consistency** across boundaries, measured document by document.

    C19 | Inputs violate wire schema
       | Rejected consistently through CLI/RPC/MCP boundaries.

"Consistently" is the content of this gate, and it is not established by three validators that
each happen to reject something. They could disagree about *which* documents are valid and
nothing would notice. So one corpus — defined once in ``tests/wire_corpus.py`` — is submitted
to every boundary, and the verdicts are compared per document:

1. **CLI** — `library-enrichmentd validate`, a real subprocess reading the document;
2. **Candidate schema** — `python/enrichment_mcp/_schemas/research-envelope.schema.json`;
3. **Generated schema** — emitted from the Rust wire types by `emit-schemas`;
4. **Adapter** — `enrichment_mcp.envelope.validate_document`, which `server._emit` runs over
   every tool response.

Be precise about what that buys, because the count flatters itself: these four are **three
transports over two independent implementations**. The CLI and the RPC leg both call one
`enrichment_daemon::validate::validate`; the adapter and the generated-schema leg both build a
validator over the same emitted file. The genuinely independent pair is Rust serde +
``TryFrom`` on one side and jsonschema over the *frozen* contract on the other -- that is where
a real disagreement could surface, and it is the reason the frozen leg is not dropped as
redundant.

Two limitations, recorded rather than glossed:

- **No corpus document crosses the real MCP stdio transport.** Phase 0 exposes no tool that
  accepts a wire document, so there is nothing to send one through. The MCP path is covered on
  the *output* side by ``tests/e2e`` instead.
- The RPC leg needs a running daemon and therefore lives in
  ``tests/e2e/test_mcp_daemon_handshake.py``.

The corpus deliberately includes the four delivered fixtures as **accept** cases. Without them
a validator that rejected everything would pass a rejection-only corpus trivially.
"""

from __future__ import annotations

import json
import subprocess
from pathlib import Path

import pytest
from jsonschema import Draft202012Validator

from enrichment_mcp._generated.research_envelope_schema import (
    LibraryEnrichmentResponseEnvelope,
)
from enrichment_mcp.envelope import validate_document
from wire_corpus import wire_corpus

ROOT = Path(__file__).resolve().parents[2]
DAEMON_BIN = ROOT / "target/debug/library-enrichmentd"
PACKAGED_SCHEMA = ROOT / "python/enrichment_mcp/_schemas/research-envelope.schema.json"
GENERATED_SCHEMA = ROOT / "schemas/generated/research-envelope.schema.json"

CORPUS = wire_corpus()
CORPUS_IDS = [name for name, _, _ in CORPUS]


def _schema_accepts(schema_path: Path, document: str) -> bool:
    validator = Draft202012Validator(json.loads(schema_path.read_text()))
    try:
        parsed = json.loads(document)
    except json.JSONDecodeError:
        return False
    return not list(validator.iter_errors(parsed))


def _adapter_accepts(document: str) -> bool:
    """The adapter's own validator, the one `server._emit` runs on every tool response.

    Note this is NOT the generated Pydantic DTO. That model enforces field types and rejects
    unknown fields, but `datamodel-codegen` does not translate the contract's root `allOf`
    conditionals into validators, so it accepts a `pending` envelope with a null job -- which
    every other boundary rejects. `test_the_generated_dto_alone_is_not_a_sufficient_validator`
    pins that limitation so nobody mistakes the DTO for a complete check.
    """
    valid, _ = validate_document(document)
    return valid


def _cli_accepts(document: str) -> tuple[bool, dict[str, object]]:
    result = subprocess.run(  # a first-party binary at a known path
        [str(DAEMON_BIN), "validate"],
        input=document.encode(),
        capture_output=True,
        check=False,
        timeout=30,
    )
    verdict: dict[str, object] = json.loads(result.stdout)
    # The exit code and the printed verdict must not disagree, or a shell branching on `$?`
    # would see something different from a caller parsing the JSON.
    assert (result.returncode == 0) == verdict["valid"], (
        f"exit code {result.returncode} contradicts {verdict}"
    )
    return bool(verdict["valid"]), verdict


@pytest.mark.skipif(
    not DAEMON_BIN.exists(),
    reason=f"{DAEMON_BIN} is not built; run `cargo build -p enrichment-daemon`",
)
@pytest.mark.parametrize(("name", "document", "is_valid"), CORPUS, ids=CORPUS_IDS)
def test_every_boundary_agrees_on_every_document(name: str, document: str, is_valid: bool) -> None:
    """The same document, four boundaries, one verdict.

    A disagreement here is the exact failure C19 exists to catch: it would mean a document the
    service emits is accepted at one boundary and refused at another.
    """
    if not GENERATED_SCHEMA.exists():
        pytest.fail(f"{GENERATED_SCHEMA} is missing. Run `just schemas-generate`.")

    cli, verdict = _cli_accepts(document)
    verdicts = {
        "cli": cli,
        "packaged_schema": _schema_accepts(PACKAGED_SCHEMA, document),
        "generated_schema": _schema_accepts(GENERATED_SCHEMA, document),
        "adapter": _adapter_accepts(document),
    }

    # Pin the boundary set: deleting a key above would otherwise silently drop a boundary and
    # leave this test passing with less coverage than its name claims.
    assert set(verdicts) == {"cli", "packaged_schema", "generated_schema", "adapter"}

    disagreements = {b: v for b, v in verdicts.items() if v != is_valid}
    assert not disagreements, (
        f"`{name}` should be {'accepted' if is_valid else 'rejected'} everywhere, but "
        f"{disagreements} disagreed. Full verdicts: {verdicts}. CLI said: {verdict}"
    )


@pytest.mark.skipif(
    not DAEMON_BIN.exists(),
    reason=f"{DAEMON_BIN} is not built; run `cargo build -p enrichment-daemon`",
)
def test_the_cli_reports_a_typed_code_on_rejection() -> None:
    """A rejection names a code from the frozen thirteen, not just a boolean."""
    rejected = [(n, d) for n, d, valid in CORPUS if not valid]
    assert rejected, "the corpus must contain rejection cases"

    frozen = json.loads(PACKAGED_SCHEMA.read_text())
    codes = set(frozen["$defs"]["Error"]["properties"]["code"]["enum"])

    for name, document in rejected:
        _, verdict = _cli_accepts(document)
        assert verdict.get("code") in codes, f"`{name}` rejected without a frozen error code"
        assert verdict.get("detail"), f"`{name}` rejected without a reason"


def test_every_builder_emits_a_conforming_envelope() -> None:
    """The adapter's constructors cannot produce a document the contract rejects.

    `_Outcome` makes the forbidden status/error pairs unexpressible, mirroring the Rust
    `wire::Outcome`. This checks the property end to end rather than trusting the type: each
    builder's output goes through the same validator the corpus above uses.
    """
    from enrichment_mcp import envelope
    from enrichment_mcp._generated.research_envelope_schema import Code, Coverage

    coverage = Coverage(
        scope="s", indexed=[], missing=[], limitations=[], assessments=[], details=None
    )
    built = {
        "ok": envelope.ok("summary", {}, coverage),
        "partial": envelope.partial("summary", {}, coverage),
        "error": envelope.error(Code.POLICY_DENIED, "m", "n"),
    }

    for name, document in built.items():
        valid, reason = validate_document(json.dumps(document))
        assert valid, f"the `{name}` builder emitted a non-conforming envelope: {reason}"

    assert built["ok"]["error"] is None
    assert built["partial"]["error"] is None
    assert built["error"]["error"]["code"] == "POLICY_DENIED"


def test_the_corpus_covers_every_root_conditional() -> None:
    """The corpus must reach all three `allOf` blocks, in both directions.

    `scripts/schema-conformance.py` builds three negative cases; the frozen contract has three
    conditionals with more than three ways to violate them. This asserts the corpus did not
    quietly shrink back to the easy subset.
    """
    names = {name for name, _, _ in CORPUS}
    required = {
        "pending_without_job",
        "error_without_error_object",
        "unknown_root_field",
        "ok_with_error_object",
        "partial_with_error_object",
        "pending_with_error_object",
        "artifact_receipt",
        "artifact_with_inline_data",
        "pending_artifact",
    }
    assert required <= names, f"corpus is missing {sorted(required - names)}"
    assert sum(1 for _, _, valid in CORPUS if valid) == 5, (
        "all four outcome fixtures and the artifact receipt must be accepted, or a reject-all "
        "validator would pass this corpus trivially"
    )


def test_the_generated_dto_alone_is_not_a_sufficient_validator() -> None:
    """A pinned limitation, so it cannot quietly become an assumption.

    `datamodel-codegen` renders properties, types and `additionalProperties: false`, but not the
    root `allOf` conditionals. The generated DTO therefore accepts documents the contract
    forbids. This is why `enrichment_mcp.envelope.validate_document` validates against the
    generated *schema* instead, and why nothing in the adapter treats a successful
    `model_validate` as proof that an envelope conforms.

    If a future `datamodel-codegen` does emit those validators, this test fails -- at which
    point the DTO becomes sufficient and the indirection can go.
    """
    pending_without_job = next(doc for name, doc, _ in CORPUS if name == "pending_without_job")

    # The contract rejects it...
    assert not _adapter_accepts(pending_without_job)
    # ...but the DTO alone does not.
    LibraryEnrichmentResponseEnvelope.model_validate_json(pending_without_job)


def test_the_adapter_validates_every_response_it_emits() -> None:
    """`validate_document` is wired into the response path, not merely available.

    An earlier revision had this function documented as "which the adapter uses" while it had no
    call sites outside tests -- the MCP leg of C19 was a claim about code nobody ran. This
    asserts the wiring itself, so the claim cannot drift back into being decorative.
    """
    from enrichment_mcp import server

    # Exercise the real tool projection, with and without a per-tool payload check. Every one of
    # the nine tools is wired to a daemon method now, so there is no unimplemented-tool path left
    # to exercise -- a stub kept alive only for this assertion would be testing nothing.
    for emitted in (
        server._tool_result({"schema_version": "1.0"}),
        server._tool_result({"schema_version": "1.0"}, tool="job_control"),
    ):
        structured = emitted.structured_content
        valid, reason = validate_document(json.dumps(structured))
        assert valid, reason
        assert structured is not None
        assert structured["status"] == "error"

    # And a malformed envelope is replaced rather than returned.
    replaced = server._emit({"schema_version": "1.0"})
    valid, reason = validate_document(json.dumps(replaced))
    assert valid, f"the replacement envelope must itself conform: {reason}"
    assert replaced["status"] == "error"
    assert replaced["error"]["code"] == "INTERNAL_ERROR"
