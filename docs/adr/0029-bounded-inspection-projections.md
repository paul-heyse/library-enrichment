---
id: ADR-0029
title: Distinguish selected API projections from complete retained observations
status: accepted
date: 2026-09-14
deciders: [paul-heyse]
level: decision
principles: [DM-07, DM-27, DM-30, DM-35]
design: [§7.1, §7.3]
review: docs/design_review/reviews/design_review_native-architecture-completion_2026-09-14.md
evidence: Proposed
supersedes: []
superseded-by: null
revisit: A caller needs a paged selection of additional observation payload fields.
verification: signature_projection_retains_qualified_ids_without_documentation; artifacts_are_read_by_handle_in_bounded_slices; escaped_unicode_answers_fit_complete_envelope_and_overflow_is_retrievable
---

# ADR-0029: Distinguish selected API projections from complete retained observations

## Context

Plan 11 W3 requires signature-only inspection to avoid loading documentation. Returning a
modified ApiObservation would break its content identity; loading the complete observation just
to clear its docs would retain the corpus-payload problem. The wire projection must state its
scope separately from the complete stored fact.

## Scope

Amend inspection's generated tool payload in design §7.1 and its bounded delivery behavior in
§7.3. The immutable Arrow observation contract and stable root envelope do not change.

## Drivers

Exact source identities, explicit omission, complete qualified alternatives and bounded reads.

## Options

- Clear docs in a purported complete ApiObservation: rejected; its semantic digest no longer matches.
- Load full observations before formatting: rejected; unrelated text determines request cost.
- Return a typed ApiObservationProjection with an explicit docs_included flag: selected.

## Decision

Each inspection observation is a presentation projection carrying the original observation_id,
subject, origin, environment and source. docs_included=false means payload.docs was excluded
before hydration; NULL in that projection is not evidence of missing documentation. A complete
documentation request sets docs_included=true and retains the original payload. These projections
are never persisted or validated as new ApiObservations. The admitted native provider remains
responsible for validation of complete retained facts.

Exact-path/suffix predicates, fragment kinds, complete-alternative bounds and payload projections
run in DataFusion before DTO decoding. A sentinel detects oversized qualified alternative sets
and returns an explicit resource failure rather than silently dropping facts. Overview citations
may select a bounded sample; their complete facets use separately bounded native aggregates.

Inline replies and durable journals share one overflow encoder. Escaped JSON size is counted
without another result-sized allocation, before conversion to an owned canonical JSON tree.
The immutable result limit is 32 MiB; journal inline limit is 1 MiB; per-request inline limits
remain configurable with a 1024-byte minimum. Content-addressed overflow excludes request_id;
its delivery envelope preserves the current request identity. No old encoder remains selectable.

### Consequences

Generated inspection DTOs gain docs_included. Consumers must distinguish omitted docs from
absent docs; a projection's source identity is a reference to the complete fact. There is no
historical DTO adapter. Large requests can fail explicitly under declared resource limits.

### Compensating controls

Typed generated schemas, native projection regressions, sentinel failures, overflow roundtrip
and independent checks of the actual executed Parquet projection.

## Evidence

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Documentation belongs to the immutable payload | [ApiPayload](../../crates/enrichment-core/src/evidence/relational.rs) | 2026-09-14 | `pub docs: Option<String>` |
| Retained observations validate their content identity | [Observation decoder](../../crates/enrichment-store/src/projection/decode.rs) | 2026-09-14 | `observation.validate().map_err(invalid)?` |
| Inspection should start small | [Living design §7.1](../design/DESIGN.md#71-tool-behavior-requirements) | 2026-09-14 | “Inspection defaults to public API plus a small documentation section” |

Exact DataFusion API/scan evidence is recorded in the dated
[compatibility matrix](../architecture/compatibility-matrix.md). A named struct does not itself
prove leaf pruning; whole-parent references can defeat projection. Measurements must inspect
the executed scan, not infer performance from expression spelling.

## Verification

signature_projection_retains_qualified_ids_without_documentation proves explicit omission and
same source identities. artifacts_are_read_by_handle_in_bounded_slices and
escaped_unicode_answers_fit_complete_envelope_and_overflow_is_retrievable prove bounded transport.
W7 must additionally verify executed scan leaves for a large unrelated documentation value.
Required tests named here are not claims that they have run.

## Boundaries preserved

§B1–§B13 remain intact: Rust owns query/identity/storage; Python remains a thin generated adapter;
complete facts remain typed Arrow evidence with unchanged epistemic classes. Neither public SQL,
a second evidence store nor a legacy data path is introduced.

## More information

[Plan 11](../plans/11-arrow-datafusion-completion-and-legacy-removal.md), W2/W3/W7 and D5/D6.

## Status history

- 2026-09-14 — proposed; current-source tests and scoped design review pending.

- 2026-09-14 — accepted after scoped native architecture review; remaining actual producer,
  client and final performance qualification stays in Plan 12.
