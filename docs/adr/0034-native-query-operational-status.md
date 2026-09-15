---
id: ADR-0034
title: Expose bounded native query diagnostics in service status
status: accepted
date: 2026-09-14
deciders: [paul-heyse]
level: decision
principles: [DM-09, DM-39, DM-57]
design: [§6.3, §14.3]
review: docs/design_review/reviews/design_review_native-query-status_2026-09-14.md
evidence: Tested
supersedes: []
superseded-by: null
revisit: An operator needs a diagnostic not derivable from these bounded process counters.
verification: test_the_counters_track_what_they_claim_to; schemas-generate; schema-conformance
---

# ADR-0034: Expose bounded native query diagnostics in service status

## Context

Plan 12 FN4 requires usable native query and admission diagnostics. Physical-plan observations
already exist in the single store runtime, but service status does not expose them.

## Scope

Add typed `health.native_queries` to the existing service status payload. It is null when no
runtime is open. Rust owns its schema; generated Python DTOs remain projections.

## Drivers

Make query activity and configured capacity visible without confusing operational counters with
library completeness, adding a query language, or exporting unbounded physical plans.

## Options

- Keep counters accessible only to integration tests: leaves operator diagnosis incomplete.
- Expose complete plan text in every status response: adds large, unnecessary payloads.
- Expose bounded scalar counters and limits: selected.

## Decision

Publish recorded execution/completion/interruption counts, summed planning and elapsed microseconds,
current admission permits in use, concurrency and configured memory/spill/metadata-cache limits.
Counters reset on daemon restart and share the existing uptime scope. Elapsed durations may overlap.
A physical execution is recorded when its trace drops; failures before plan creation are outside
these execution counts. Admission reports current permit holders. Configured limits are not
measured allocations, peak RSS, successful producer capacity or library Coverage.

### Consequences

Status gains one typed optional object. No new tool, persisted relation or engine path is added.
Recent plan text remains bounded internal diagnostics; no SQL or arbitrary plan interface is exposed.

### Compensating controls

The real daemon metrics test observes counters after fresh acquisition. Generated schema conformance
checks the cross-language boundary. The existing native runtime owns all increments and limits.

## Evidence

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Operational diagnosis is required | [Plan 12 FN4](../plans/12-architecture-first-completion.md#fn4--finish-operational-product-behavior) | 2026-09-14 | “Connect useful bounded query/admission/cache diagnostics to operational inspection” |
| Recorded durations are not whole-process wall time | [Native runtime diagnostics](../../crates/enrichment-store/src/query_diagnostics.rs) | 2026-09-14 | “concurrent durations overlap and are not wall time” |

No new upstream API or dependency is introduced: this projection consumes existing runtime counters.

## Verification

`test_the_counters_track_what_they_claim_to` checks real acquisition increments, completed work,
released permits and configured limits. `just schema-conformance` validates regenerated contracts.
Results belong in the Plan 12 ledger; implementation alone does not establish acceptance.

## Boundaries preserved

§B1–§B13 remain intact. Rust owns operational state and schemas; Python only forwards validated
status. Evidence identities, epistemic classes, execution permission and publication are unchanged.

## More information

[Design §14.3](../design/DESIGN.md#143-operational-metrics), [Plan 12](../plans/12-architecture-first-completion.md).

## Status history

- 2026-09-14 — proposed; bounded projection implemented, integration verification pending.

- 2026-09-14 — accepted after the scoped review and real operational metrics/schema checks.
