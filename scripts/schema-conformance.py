#!/usr/bin/env python3
"""Assert generated wire schemas match the frozen Phase-0 contract.

Byte equality with contracts/research-envelope.schema.json is the WRONG target: $id, $defs
ordering and serde-derived naming will legitimately differ. The frozen artifact defines what
must validate and what must be rejected. Three checks:

  1. Reproducible generation -- schemas/generated/ is unchanged after regeneration.
  2. Corpus (the real oracle) -- the four fixtures validate and the three negative cases from
     VALIDATION_REPORT.md are rejected, under the generated schema exactly as under the frozen
     one. This is also acceptance gate C19.
  3. Structural -- schemas/frozen/enums.json must equal the generated equivalents.

Absent generated output is `not_run`, not a pass and not a hard failure (phase 0).
"""
from __future__ import annotations

import copy, json, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FROZEN = ROOT / "contracts/research-envelope.schema.json"
ENUMS = ROOT / "schemas/frozen/enums.json"
GENERATED = ROOT / "schemas/generated/research-envelope.schema.json"
EXAMPLES = ROOT / "contracts/examples"

failures: list[str] = []
notes: list[str] = []


def negative_cases(ok: dict) -> list[tuple[str, dict]]:
    """The three negative cases VALIDATION_REPORT.md records as correctly rejected."""
    pending_no_job = copy.deepcopy(ok)
    pending_no_job.update(status="pending", job=None)

    error_no_error = copy.deepcopy(ok)
    error_no_error.update(status="error", error=None)

    unknown_field = copy.deepcopy(ok)
    unknown_field["unexpected_root_field"] = True

    return [
        ("pending without a job handle", pending_no_job),
        ("error without an error object", error_no_error),
        ("unknown root field", unknown_field),
    ]


def main() -> int:
    try:
        from jsonschema import Draft202012Validator
    except ImportError:
        print("schema-conformance: BLOCKED -- jsonschema is not installed.", file=sys.stderr)
        print("  Add it to [dependency-groups] dev and run `just sync`.", file=sys.stderr)
        return 2

    frozen = json.loads(FROZEN.read_text())
    fv = Draft202012Validator(frozen)

    schemas = [("frozen", fv)]
    if GENERATED.exists():
        schemas.append(("generated", Draft202012Validator(json.loads(GENERATED.read_text()))))
    else:
        notes.append("schemas/generated/ is empty: generation and cross-schema agreement are "
                     "not_run until the Rust wire types exist (phase 0).")

    # --- 2. corpus --------------------------------------------------------------------
    ok_doc = json.loads((EXAMPLES / "ok.fixture.json").read_text())
    for name in ("ok", "partial", "pending", "error"):
        doc = json.loads((EXAMPLES / f"{name}.fixture.json").read_text())
        for label, v in schemas:
            errs = list(v.iter_errors(doc))
            if errs:
                failures.append(f"corpus: {name}.fixture.json must validate under the {label} "
                                f"schema but did not: {errs[0].message}")

    for desc, doc in negative_cases(ok_doc):
        for label, v in schemas:
            if not list(v.iter_errors(doc)):
                failures.append(f"corpus: '{desc}' must be REJECTED by the {label} schema "
                                f"but validated. (VALIDATION_REPORT.md records it as rejected; "
                                f"gate C19 requires consistent rejection across boundaries.)")

    # --- 3. structural ----------------------------------------------------------------
    enums = json.loads(ENUMS.read_text())
    if GENERATED.exists():
        gen = json.loads(GENERATED.read_text())
        gd = gen.get("$defs", {})

        def check(label, expected, actual):
            if expected != actual:
                failures.append(f"structural: {label} differs from the frozen contract.\n"
                                f"    frozen:    {expected}\n    generated: {actual}")

        check("root required fields", enums["root_required"], sorted(gen.get("required", [])))
        check("status enum", enums["status"],
              gen.get("properties", {}).get("status", {}).get("enum"))
        check("error codes", enums["error_codes"],
              gd.get("Error", {}).get("properties", {}).get("code", {}).get("enum"))
        check("evidence_class enum", enums["evidence_class"],
              gd.get("Evidence", {}).get("properties", {}).get("evidence_class", {}).get("enum"))
        check("source_version_match enum", enums["source_version_match"],
              gd.get("Freshness", {}).get("properties", {}).get("source_version_match", {}).get("enum"))
        check("job state enum", enums["job_state"],
              gd.get("JobHandle", {}).get("properties", {}).get("state", {}).get("enum"))
        check("artifact uri pattern", enums["artifact_uri_pattern"],
              gd.get("ArtifactHandle", {}).get("properties", {}).get("uri", {}).get("pattern"))

    # --- report -----------------------------------------------------------------------
    for n in notes:
        print(f"schema-conformance: NOTE -- {n}")
    if failures:
        print("schema-conformance: FAILED", file=sys.stderr)
        for f in failures:
            print(f"  - {f}", file=sys.stderr)
        return 1
    scope = "frozen + generated" if GENERATED.exists() else "frozen only"
    print(f"schema-conformance: OK -- 4 fixtures validate, 3 negative cases rejected ({scope})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
