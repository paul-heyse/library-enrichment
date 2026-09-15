---
id: ADR-0012
title: Preserve declared feature knowledge in environment identities
status: accepted
date: 2026-09-13
deciders: [paul-heyse]
level: decision
principles: [DM-08, DM-14, DM-15, DM-32, DM-51]
design: [§3.1, §6.3, §8.2]
review: docs/design_review/reviews/design_review_phase-1-contract-repairs_2026-09-13.md
evidence: Proposed
supersedes: []
superseded-by: null
revisit: A future model change alters the recorded knowledge or gate dependency.
verification: omitted_and_explicit_empty_features_have_different_identities; recorded_resolution_keeps_its_snapshot_after_current_changes; R05
---

# ADR-0012: Preserve declared feature knowledge in environment identities

## Context

A target-only declaration and explicit empty features collapse to the same feature vector. Inspection then turns explicit emptiness into null. Metadata-only snapshots also need absence of an observed build.

## Scope

The named gate/model contracts only; this is not a phase-completion claim.

## Drivers

Preserve uncertainty, historical evidence, and reproducible phase acceptance.

## Options

Changing the existing vector to nullable would break array consumers. Inferring known-empty from any declared environment invents feature intent. Both rejected.

## Decision

Retain the feature vector and add features_known, default false for legacy records. Include knowledge in new environment identities. Existing serialized IDs remain unchanged; legacy nonempty vectors establish their selection, legacy empty vectors do not. Make snapshot and overview observed_configuration optional. Keep the frozen root envelope unchanged; regenerate data schemas. Bump the Rust normalizer to 2. Replay resolves the snapshot stored alongside its data, never the mutable current pointer. Complete producer provenance before publication.

### Consequences

New evidence uses the corrected contract. Historical records retain their original limitations.

### Compensating controls

Focused negative and legacy tests plus generated-schema conformance.

## Evidence

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Governing requirement | docs/blueprint/IMPLEMENTATION_BLUEPRINT.md §3.1 | 2026-09-13 | "do not mutate the meaning of an old ID" |
| Design is decidable | docs/design_review/reviews/design_review_phase-1-contract-repairs_2026-09-13.md | 2026-09-13 | "Accept proposed contract repair and phase alignment" |

## Verification

omitted_and_explicit_empty_features_have_different_identities; recorded_resolution_keeps_its_snapshot_after_current_changes; R05. Run affected Rust tests, schema conformance and the phase gate.

## Boundaries preserved

All §B1–§B13 remain intact: Rust ownership, independent observations, immutable evidence,
static-first research, thin Python and the repository boundary are unchanged.

## Status history

- 2026-09-13 — accepted following the scoped proposal review; implementation evidence pending.
