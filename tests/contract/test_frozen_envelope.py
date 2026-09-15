"""The frozen response envelope accepts and rejects exactly what it is specified to.

contracts/research-v2/research-envelope.schema.json is the candidate hard-pivot contract.
Generated schemas must agree on which documents validate -- so this
corpus is the oracle `just schema-conformance` uses, and it is also the Python-side half of
acceptance gate C19 (schema violations rejected consistently across boundaries).
"""

from __future__ import annotations

import json
from pathlib import Path

import pytest
from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[2]
SCHEMA = json.loads((ROOT / "contracts/research-v2/research-envelope.schema.json").read_text())
EXAMPLES = ROOT / "contracts/research-v2/examples"
VALIDATOR = Draft202012Validator(SCHEMA)


def _ok_document() -> dict:
    return json.loads((EXAMPLES / "ok.fixture.json").read_text())


@pytest.mark.parametrize("name", ["ok", "partial", "pending", "error"])
def test_delivered_fixtures_validate(name: str) -> None:
    document = json.loads((EXAMPLES / f"{name}.fixture.json").read_text())
    assert list(VALIDATOR.iter_errors(document)) == []


def test_pending_without_a_job_handle_is_rejected() -> None:
    """A `pending` status promises a job to poll; without one the caller has nothing to do."""
    document = _ok_document()
    document.update(status="pending", job=None)
    assert list(VALIDATOR.iter_errors(document))


def test_error_without_an_error_object_is_rejected() -> None:
    """An `error` status must carry a typed code and a next action, not just a summary."""
    document = _ok_document()
    document.update(status="error", error=None)
    assert list(VALIDATOR.iter_errors(document))


def test_unknown_root_field_is_rejected() -> None:
    """The envelope is closed, so a typo in a field name fails loudly rather than being ignored."""
    document = _ok_document()
    document["unexpected_root_field"] = True
    assert list(VALIDATOR.iter_errors(document))


def test_structural_extract_matches_the_frozen_schema() -> None:
    """contracts/research-v2/enums.json is what generated schemas are compared against.

    If it drifts from the contract it was extracted from, every downstream conformance check is
    measuring the wrong thing.
    """
    frozen = json.loads((ROOT / "contracts/research-v2/enums.json").read_text())
    defs = SCHEMA["$defs"]

    assert frozen["root_required"] == sorted(SCHEMA["required"])
    assert frozen["status"] == SCHEMA["properties"]["status"]["enum"]
    assert frozen["error_codes"] == defs["Error"]["properties"]["code"]["enum"]
    assert frozen["evidence_class"] == defs["Evidence"]["properties"]["evidence_class"]["enum"]
    assert frozen["job_state"] == defs["JobHandle"]["properties"]["state"]["enum"]
    assert frozen["artifact_uri_pattern"] == defs["ArtifactHandle"]["properties"]["uri"]["pattern"]


def test_error_codes_are_complete() -> None:
    """Blueprint §7.2 enumerates thirteen stable error codes; dropping one silently narrows the
    contract in a way no fixture would catch."""
    required = {
        "VERSION_NOT_FOUND",
        "AMBIGUOUS_PACKAGE",
        "ARTIFACT_UNAVAILABLE",
        "UNSUPPORTED_FORMAT",
        "ENVIRONMENT_UNRESOLVED",
        "ENVIRONMENT_MISMATCH",
        "POLICY_DENIED",
        "UNSUPPORTED_CAPABILITY",
        "UPSTREAM_UNAVAILABLE",
        "EXTRACTION_FAILED",
        "VERIFICATION_FAILED",
        "BUDGET_EXCEEDED",
        "INVALID_CURSOR",
        "QUERY_FAILED",
        "INTERNAL_ERROR",
    }
    assert set(SCHEMA["$defs"]["Error"]["properties"]["code"]["enum"]) == required
