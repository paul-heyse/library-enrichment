"""The shared corpus for acceptance gate C19.

Defined in one place so the contract tier (CLI, both schemas, the Pydantic DTO) and the
e2e tier (the `wire.validate` RPC method) submit *identical* documents to different
boundaries and compare verdicts. Three validators that each reject something would not
establish the gate's "consistently"; agreeing on the same documents does.
"""

from __future__ import annotations

import json
from pathlib import Path

# "Rejected CONSISTENTLY through CLI/RPC/MCP boundaries" is the assertion, and consistency is
# the half that needs measuring. Defining the corpus once, here, is what lets the contract and
# e2e tiers submit identical documents to different boundaries and compare verdicts -- rather
# than each tier inventing its own cases and agreeing only by coincidence.

_CONTRACTS = Path(__file__).resolve().parent.parent / "contracts/research-v2"

_ERROR_OBJECT = {
    "code": "POLICY_DENIED",
    "message": "m",
    "retryable": False,
    "next_action": "n",
    "diagnostic": {
        "cause": "policy_denied",
        "stage": "admission",
        "affected_ids": [],
        "rule": None,
        "observed": None,
        "allowed": None,
        "correlation_id": None,
        "actions": [{"kind": "operator_setup", "reason": "n"}],
    },
}
_JOB_OBJECT = {"job_id": "job_x", "state": "queued", "stage": "s", "poll_after_ms": 1000}


def _ok_fixture() -> dict[str, object]:
    text = (_CONTRACTS / "examples" / "ok.fixture.json").read_text()
    loaded: dict[str, object] = json.loads(text)
    return loaded


def _mutate(**changes: object) -> str:
    doc = _ok_fixture()
    doc.update(changes)
    return json.dumps(doc)


def _without(field: str) -> str:
    doc = _ok_fixture()
    doc.pop(field)
    return json.dumps(doc)


def wire_corpus() -> list[tuple[str, str, bool]]:
    """`(name, document, is_valid)` triples for every boundary to agree on.

    The first four are the delivered fixtures, which every boundary must **accept**. Without
    them a validator that rejected everything would pass a rejection-only corpus trivially.
    """
    cases: list[tuple[str, str, bool]] = []
    for name in ("ok", "partial", "pending", "error"):
        text = (_CONTRACTS / "examples" / f"{name}.fixture.json").read_text()
        cases.append((f"{name}_fixture", text, True))

    cases += [
        # The three that scripts/schema-conformance.py builds in negative_cases().
        ("pending_without_job", _mutate(status="pending", job=None), False),
        ("error_without_error_object", _mutate(status="error", error=None), False),
        ("unknown_root_field", _mutate(unexpected_root_field=True), False),
        # The rest of the frozen contract's root `allOf`, which those three do not reach.
        ("ok_with_error_object", _mutate(error=_ERROR_OBJECT), False),
        ("partial_with_error_object", _mutate(status="partial", error=_ERROR_OBJECT), False),
        (
            "pending_with_error_object",
            _mutate(status="pending", job=_JOB_OBJECT, error=_ERROR_OBJECT),
            False,
        ),
        # Enum and structural violations.
        ("unknown_status", _mutate(status="finished"), False),
        (
            "unknown_error_code",
            _mutate(status="error", error={**_ERROR_OBJECT, "code": "NO_SUCH_CODE"}),
            False,
        ),
        ("missing_required_root_field", _without("coverage"), False),
        ("wrong_schema_version", _mutate(schema_version="1.0"), False),
        ("not_an_object", json.dumps([]), False),
    ]
    artifact = {
        "mode": "artifact",
        "artifact_id": "art_0123456789abcdef",
        "sections": [],
        "read": {
            "kind": "read_artifact",
            "artifact_id": "art_0123456789abcdef",
            "section": None,
            "cursor": None,
        },
        "limits": {"requested_max_bytes": None, "effective_max_bytes": 4096},
    }
    cases += [
        ("artifact_receipt", _mutate(delivery=artifact, data={}), True),
        ("artifact_with_inline_data", _mutate(delivery=artifact, data={"rows": []}), False),
        (
            "pending_artifact",
            _mutate(delivery=artifact, data={}, status="pending", job=_JOB_OBJECT),
            False,
        ),
    ]
    for field in ("context_id", "snapshot_id", "job", "error"):
        cases.append((f"missing_nullable_{field}", _without(field), False))
    return cases
