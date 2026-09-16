---
id: ADR-0041
title: Execute service operations through native DataFusion contracts
status: accepted
date: 2026-09-15
deciders: [paul-heyse]
level: decision
principles: [DM-16, DM-17, DM-19, DM-26, DM-52]
design: [§1.1, §6.3, §8.4, §14.3]
review: docs/design_review/reviews/design_review_unified-datafusion-delta-runtime_2026-09-15.md
evidence: Interface-checked
supersedes: [ADR-0024, ADR-0038, ADR-0040]
superseded-by: null
revisit: A required Plan 15 consumer cannot use the qualified native route or preserve its semantic and ownership contract.
verification: Plan 15 native-contracts-check; native-research-check; architecture-check (implementation oracles; not yet qualified).
---

# ADR-0041: Execute service operations through native DataFusion contracts

## Context

The unified runtime review and Delta follow-up identify remaining procedural authorities and native integration gaps. The owner accepted the corrected target in Plan 15 and explicitly authorized its entire implementation on 2026-09-15. The rejected raw-write candidate is not the selected design. Acceptance of this decision does not claim implementation or product qualification.

## Scope

Amends the living design sections cited above. Plan 15 records implementation and deletion sequence. Preserved outcomes of superseded records remain binding; their obsolete mechanisms and compatibility assumptions do not.

## Drivers

One inspectable native authority, exact evidence, durable coherent publication and useful bounded MCP operations.

## Options

- Keep the existing split architecture: rejected by the owner because it retains repeated semantic authorities.
- Wrap the existing procedures: rejected because it conceals transformations from native planning.
- Complete native replacement with typed boundary mechanisms: selected.

## Decision

All data transformations, policy evaluation, research results and job decisions are native Arrow/DataFusion definitions and plans. One concrete SessionState carries the effective policy and shared resources. Compose DeltaExtensionPlanner and finite service extension planners inside RetentionPlanner. Metadata and generated MCP contracts derive from the same enforced definitions. Python only transports MCP and encodes static producer facts as generated Arrow IPC. Do not retain whole-score, normalizer or result-selection procedures behind opaque UDFs.

### Consequences

The implementation and new epoch replace the old mechanisms directly. There is no parallel compatibility runtime. Native upstream capabilities are inherited through qualified public routes, not copied from private modules. Required missing qualification remains visible.

### Compensating controls

Exact dependency lock, native admission and route tests, ownership/recovery faults, preserved source evidence, and Plan 15 deletion and installed MCP oracles.

## Evidence

Verified 2026-09-15 against the exact local checkouts by the upstream-verifier; the referenced reviews retain bounded executed probes. This is API evidence, not full-service certification.

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Native integration boundary | [Exact upstream source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/planner.rs) | 2026-09-15 | how to lower delta-rs custom logical nodes into executable physical plans. |

## Verification

Plan 15 assigns native-contracts-check; native-research-check; architecture-check. These must be implemented and executed on the final source. Prior Plan 14 gates and review probes are not substituted for them.

## Boundaries preserved

Rust owns semantics and publication; Python remains a thin MCP adapter and separate static extraction worker. Exact identities, independent epistemic classes, isolated execution outside studied repositories, typed grants, bounded truthful MCP delivery, ty and rust-analyzer remain required. Storage and execution mechanisms are amended explicitly above. Frozen blueprint/handoff bytes remain provenance.
