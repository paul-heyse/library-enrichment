---
paths:
  - "schemas/generated/**"
  - "python/enrichment_mcp/_generated/**"
  - "crates/*/src/generated/**"
---

# Generated artifacts are never hand-edited

Rust wire types are authoritative (blueprint §6.3). JSON Schemas are generated from them, and
the Pydantic boundary DTOs are generated from those schemas. Editing a generated file makes the
three drift silently, which is exactly the failure the generation pipeline exists to prevent.

To change a schema: change the Rust type, run `just schemas-generate`, and commit both sides in
the same change. `just schema-conformance` runs `git diff --exit-code` over `schemas/generated/`
to prove generation is reproducible.

Codegen fidelity is not the oracle. The oracle is the conformance corpus: the four fixtures in
`contracts/examples/` must validate, and the three negative cases from `VALIDATION_REPORT.md`
(pending without a job handle, error without an error object, unknown root field) must be
rejected, under the generated schema exactly as under the frozen one.

Preserve upstream raw JSON in content-addressed blobs so normalization can improve without
refetching. Persist upstream IDs as local producer identifiers, never as globally meaningful
symbols — rustdoc's item IDs in particular are not cross-release identities.
