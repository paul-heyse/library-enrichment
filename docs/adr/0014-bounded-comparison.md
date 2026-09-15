---
id: ADR-0014
title: Compare immutable evidence without overstating compatibility
status: accepted
date: 2026-09-13
deciders: [paul-heyse]
level: decision
principles: [DM-08, DM-14, DM-20, DM-32, DM-43]
design: [§3.1, §6.3, §7.1, §7.3]
review: docs/design_review/reviews/design_review_bounded-comparison_2026-09-13.md
evidence: Proposed
supersedes: []
superseded-by: null
revisit: A comparison consumer needs a compatibility guarantee beyond the normalized observations.
verification: R08; P06; C01; C02; C03; C08; C09; C15; schema-conformance
---

# ADR-0014: Compare immutable evidence without overstating compatibility

## Context

The comparison MCP tool currently returns unsupported. The blueprint requires additive API,
documentation, configuration and behavior changes, including when signatures do not change.
Current response budgets count selected payload pieces rather than the complete envelope.

## Decision

Add Rust-owned comparison request/result records and generate their schemas. The primary request
names before/after contexts with optional pinned snapshots, scopes, pagination and byte budget.
Retain the existing ecosystem/name/version convenience inputs as an explicit acquisition path;
require exactly one input form. Pinned comparisons read immutable local snapshots without fetches.

Compare normalized public paths, signatures/overloads, deprecation, documentation, relationships,
configuration, extras/dependency declarations, examples and release notes. Keep raw before/after
observations and locators. Additive changes are detected by set difference independently of any
breaking-change interpreter. A removed path or changed signature is only potentially breaking;
a successful diff never proves runtime, semantic or project compatibility. Unsupported producer
normalizers and incomplete API coverage preclude a complete API comparison.

Compare environment knowledge, declared target/toolchain/features/defaults/extras, dependency
resolution and observed build configuration first. Separate differing or unknown environments
from release changes and name the confounders. Identical unknown environments do not prove
identical effective build configurations. Same-release/different-configuration results are
configuration comparisons, never release-change claims.

All result budgets measure serialized complete envelopes. Keep mandatory envelope fields and
return whole entries with deterministic cursors; when a single item cannot fit, return a stored
artifact handle and a bounded projection or a typed budget error. Cursors bind both snapshots,
query/scopes/order and budget. No raw SQL, arbitrary filesystem access or implicit execution.

## Evidence

| Source | Retrieved | Requirement |
|---|---|---|
| docs/blueprint/IMPLEMENTATION_BLUEPRINT.md §§7.1,7.3,13 | 2026-09-13 | Before/after contexts, environment differences, complete bounded responses and non-API deltas |
| docs/design/DESIGN.md §§5.2,7.3 | 2026-09-13 | Normalized set differences independently of Griffe compatibility checks; whole JSON objects |
| crates/enrichment-core/src/evidence/model.rs; identity/mod.rs | 2026-09-13 | Existing immutable records carry observations and explicit environment knowledge |

## Consequences

Comparisons expose evidence changes with scoped interpretation rather than a binary compatibility
verdict. Existing envelope 1.0 and published snapshots remain readable. The additional comparison
payload is derived from canonical records. Mutable fetch revalidation remains a producer concern,
not a side effect of pinned comparison reads.

## Verification

Added Python function, unchanged Rust API with changed behavior note, source/stub conflicts,
different configurations/toolchains, missing API producers, deterministic pagination, oversized
Unicode/escaped strings, complete-envelope byte checks and old snapshot reads.

## Boundaries preserved

Rust authority, separate static Python worker, immutable publication, six epistemic classes,
explicit execution policy, separate Context7 and no project edits remain unchanged.

## Status history

- 2026-09-13 — accepted at Proposed strength after the scoped contract review.
