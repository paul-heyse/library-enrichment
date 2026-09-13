---
paths:
  - "tests/**"
  - "docs/reports/**"
  - "crates/enrichment-core/src/evidence/**"
  - "crates/enrichment-core/src/change/**"
---

# Evidence truthfulness

Six epistemic classes, kept distinct: `declared`, `statically_extracted`, `compiler_derived`,
`typechecker_observed`, `runtime_observed`, `agent_inferred`. They are categories, not a
confidence scale. An API signature can be compiler-derived while "this replaces our custom
orchestration" is agent-inferred, and the two never merge.

Contradictions are retained as conflicting observations with their provenance. Never overwrite
a runtime observation with a stub annotation, or a project-configuration result with docs.rs'
expanded documentation build.

`ok` means successful within the declared scope — not complete knowledge of a library. A valid
empty search is distinct from an extraction failure and from a missing index (C01). When a
producer fails after other evidence succeeded, return `partial` with explicit gaps, not an
empty `ok` and not total evidence loss (C15).

Availability is several separate states: `documented_available` in a recorded documentation
build is not `project_availability_unverified` resolved. Do not invent a feature predicate for
every symbol — conditional compilation removes inactive branches before extraction, and
documentation annotations are not a complete boolean model.

## Test discipline

Four states: `passed`, `failed`, `blocked`, `not_run`. `passed` requires a command that ran and
a log that was written; `just acceptance-check` enforces that mechanically. A missing tool or
credential is `blocked` with the prerequisite named. Never attempted is `not_run`.

**No mocking of producers, the daemon, or MCP transport in the `contract`, `integration`, or
`e2e` tiers.** Those tiers exist to test the real boundary. A mocked client is never a pass
for A01 or A02. Unit tests may mock freely.

Fixture tests must stay stable without network access. Live tests are opt-in, select and record
exact versions at run time, and report separately so a live failure is never mistaken for a
deterministic regression.

Do not convert gate results into a universal quality percentage. Compare against concrete task
outcomes: the capability was discovered, the exact environment was respected, the integration
requirement was identified, the material claims were supported.
