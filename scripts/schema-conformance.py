#!/usr/bin/env python3
"""Validate the native wire epoch from Rust-generated schemas and independent outcome fixtures."""

from __future__ import annotations

import copy
import json
from pathlib import Path

from cli import say, warn

ROOT = Path(__file__).resolve().parent.parent
PACKAGED = ROOT / "python/enrichment_mcp/_schemas/research-envelope.schema.json"
ENUMS = ROOT / "tests/fixtures/wire/vocabulary.json"
GENERATED = ROOT / "schemas/generated/research-envelope.schema.json"
EXAMPLES = ROOT / "tests/fixtures/wire"

failures: list[str] = []
notes: list[str] = []


def negative_cases(ok: dict) -> list[tuple[str, dict]]:
    """Independent invalid envelopes, including the exact native page wire contract."""
    pending_no_job = copy.deepcopy(ok)
    pending_no_job.update(status="pending", job=None)

    error_no_error = copy.deepcopy(ok)
    error_no_error.update(status="error", error=None)

    unknown_field = copy.deepcopy(ok)
    unknown_field["unexpected_root_field"] = True
    obsolete_epoch = copy.deepcopy(ok)
    obsolete_epoch["schema_version"] = "2.0"
    bare_handle = copy.deepcopy(ok)
    bare_handle["artifacts"] = [
        {
            "artifact_id": "art_" + "0" * 64,
            "media_type": "text/plain",
            "uri": "library-evidence://artifacts/example",
            "description": "receipt required",
        }
    ]
    empty_continuation = copy.deepcopy(ok)
    empty_continuation["data"]["page"].update(
        returned="0", has_more=True, next_cursor="next"
    )
    numeric_page = copy.deepcopy(ok)
    numeric_page["data"]["page"]["returned"] = 1
    missing_page_presence = copy.deepcopy(ok)
    del missing_page_presence["data"]["page"]["next_cursor"]

    return [
        ("pending without a job handle", pending_no_job),
        ("error without an error object", error_no_error),
        ("unknown root field", unknown_field),
        ("obsolete wire epoch", obsolete_epoch),
        ("artifact handle without receipt", bare_handle),
        ("continuing page without progress", empty_continuation),
        ("retired numeric page count", numeric_page),
        ("missing page cursor presence", missing_page_presence),
    ]


def main() -> int:
    try:
        from jsonschema import Draft202012Validator
    except ImportError:
        warn("schema-conformance: BLOCKED -- jsonschema is not installed.")
        warn("  Add it to [dependency-groups] dev and run `just sync`.")
        return 2

    packaged = json.loads(PACKAGED.read_text())
    schemas = [("packaged", Draft202012Validator(packaged))]
    if GENERATED.exists():
        schemas.append(("generated", Draft202012Validator(json.loads(GENERATED.read_text()))))
    else:
        notes.append(
            "schemas/generated/ is empty: generation and cross-schema agreement are "
            "not_run until the Rust wire types exist (phase 0)."
        )

    # --- 2. corpus --------------------------------------------------------------------
    ok_doc = json.loads((EXAMPLES / "ok.fixture.json").read_text())
    for name in ("ok", "partial", "pending", "error"):
        doc = json.loads((EXAMPLES / f"{name}.fixture.json").read_text())
        for label, v in schemas:
            errs = list(v.iter_errors(doc))
            if errs:
                failures.append(
                    f"corpus: {name}.fixture.json must validate under the {label} "
                    f"schema but did not: {errs[0].message}"
                )

    for desc, doc in negative_cases(ok_doc):
        for label, v in schemas:
            if not list(v.iter_errors(doc)):
                failures.append(
                    f"corpus: '{desc}' must be REJECTED by the {label} schema "
                    f"but validated. (VALIDATION_REPORT.md records it as rejected; "
                    f"gate C19 requires consistent rejection across boundaries.)"
                )

    # --- 3. structural ----------------------------------------------------------------
    enums = json.loads(ENUMS.read_text())
    if GENERATED.exists():
        gen = json.loads(GENERATED.read_text())
        gd = gen.get("$defs", {})

        def check(label: str, expected: object, actual: object) -> None:
            if expected != actual:
                failures.append(
                    f"structural: {label} differs from the native vocabulary.\n"
                    f"    expected:    {expected}\n    generated: {actual}"
                )

        check("root required fields", enums["root_required"], sorted(gen.get("required", [])))
        check(
            "status enum", enums["status"], gen.get("properties", {}).get("status", {}).get("enum")
        )
        check(
            "error codes",
            enums["error_codes"],
            gd.get("Error", {}).get("properties", {}).get("code", {}).get("enum"),
        )
        check(
            "evidence_class enum",
            enums["evidence_class"],
            gd.get("FactSource", {}).get("properties", {}).get("evidence_class", {}).get("enum"),
        )
        check(
            "source_version_match enum",
            enums["source_version_match"],
            gd.get("Freshness", {})
            .get("properties", {})
            .get("source_version_match", {})
            .get("enum"),
        )
        check(
            "job state enum",
            enums["job_state"],
            gd.get("JobHandle", {}).get("properties", {}).get("state", {}).get("enum"),
        )
        check(
            "artifact uri pattern",
            enums["artifact_uri_pattern"],
            gd.get("ArtifactHandle", {}).get("properties", {}).get("uri", {}).get("pattern"),
        )

    # --- report -----------------------------------------------------------------------
    for n in notes:
        warn(f"schema-conformance: NOTE -- {n}")
    if failures:
        say("schema-conformance: FAILED")
        for f in failures:
            warn(f"  - {f}")
        return 1
    scope = "packaged + generated" if GENERATED.exists() else "packaged only"
    say(
        f"schema-conformance: OK -- 4 fixtures validate, "
        f"{len(negative_cases(ok_doc))} negative cases rejected ({scope})"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
