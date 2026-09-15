---
id: ADR-0011
title: Align comparison gates with blueprint Phase 3
status: accepted
date: 2026-09-13
deciders: [paul-heyse]
level: decision
principles: [DM-59, DM-60]
design: [§13]
review: docs/design_review/reviews/design_review_phase-1-contract-repairs_2026-09-13.md
evidence: Proposed
supersedes: []
superseded-by: null
revisit: A future model change alters the recorded knowledge or gate dependency.
verification: test_comparison_gate_phases_follow_blueprint
---

# ADR-0011: Align comparison gates with blueprint Phase 3

## Context

R08 and P06 currently assign comparison assertions to static-extraction phases. Blueprint §13 assigns changes to Phase 3. The user explicitly chose blueprint sequencing.

## Scope

The named gate/model contracts only; this is not a phase-completion claim.

## Drivers

Preserve uncertainty, historical evidence, and reproducible phase acceptance.

## Options

Keep existing assignments and implement comparison early; rejected because it changes the user-selected slice order.

## Decision

Set only the R08 and P06 phase fields to 3. Keep IDs, scenarios, assertions, and retirement semantics unchanged.

### Consequences

New evidence uses the corrected contract. Historical records retain their original limitations.

### Compensating controls

Focused negative and legacy tests plus generated-schema conformance.

## Evidence

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Governing requirement | docs/blueprint/IMPLEMENTATION_BLUEPRINT.md §13 | 2026-09-13 | "Implement normalized API additions/removals" |
| Design is decidable | docs/design_review/reviews/design_review_phase-1-contract-repairs_2026-09-13.md | 2026-09-13 | "Accept proposed contract repair and phase alignment" |

## Verification

test_comparison_gate_phases_follow_blueprint. Run affected Rust tests, schema conformance and the phase gate.

## Boundaries preserved

All §B1–§B13 remain intact: Rust ownership, independent observations, immutable evidence,
static-first research, thin Python and the repository boundary are unchanged.

## Status history

- 2026-09-13 — accepted following the scoped proposal review; implementation evidence pending.
