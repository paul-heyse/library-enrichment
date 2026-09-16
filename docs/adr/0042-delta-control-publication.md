---
id: ADR-0042
title: Publish coherent evidence through a Delta control transaction
status: accepted
date: 2026-09-15
deciders: [paul-heyse]
level: decision
principles: [DM-14, DM-19, DM-30, DM-35, DM-43]
design: [§8.2, §8.3]
review: docs/design_review/reviews/design_review_unified-datafusion-delta-runtime_2026-09-15.md
evidence: Interface-checked
supersedes: [ADR-0023]
superseded-by: null
revisit: A required Plan 15 consumer cannot use the qualified native route or preserve its semantic and ownership contract.
verification: Plan 15 delta-mutation-check; delta-control-check; delta-storage-check (implementation oracles; not yet qualified).
---

# ADR-0042: Publish coherent evidence through a Delta control transaction

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

Use delta-rs 58f07cd62bfbce3649a7e1c87c696288068ae184 and its selected Buoyant kernel 8ba063f8f84fec222000f66d40d70911d7c79675 with DataFusion 55.1.0 and Arrow/Parquet 59.3.0. Purpose-specific Delta tables hold evidence; one typed control table holds coordinated claims, publication vectors, context heads and terminal results. Bind preconditions to the commit read snapshot and fence every contested predecessor using application transaction keys. Reconcile command identity after unknown outcomes and before fresh retries; markers alone are not deduplication. Complete logical-input Delta builders enforce writes with an explicit SessionState and RequireSessionState. Raw provider INSERT is unqualified and must not bypass this boundary. Capture and validate exact loaded snapshots before table_provider registration. Remove application Parquet journals/manifests/current pointers and custom transaction writers.

### Consequences

The implementation and new epoch replace the old mechanisms directly. There is no parallel compatibility runtime. Native upstream capabilities are inherited through qualified public routes, not copied from private modules. Required missing qualification remains visible.

### Compensating controls

Exact dependency lock, native admission and route tests, ownership/recovery faults, preserved source evidence, and Plan 15 deletion and installed MCP oracles.

## Evidence

Verified 2026-09-15 against the exact local checkouts by the upstream-verifier; the referenced reviews retain bounded executed probes. This is API evidence, not full-service certification.

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Native integration boundary | [Exact upstream source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs) | 2026-09-15 | Add an additional application transaction to the commit |

## Verification

Plan 15 assigns delta-mutation-check; delta-control-check; delta-storage-check. These must be implemented and executed on the final source. Prior Plan 14 gates and review probes are not substituted for them.

## Boundaries preserved

Rust owns semantics and publication; Python remains a thin MCP adapter and separate static extraction worker. Exact identities, independent epistemic classes, isolated execution outside studied repositories, typed grants, bounded truthful MCP delivery, ty and rust-analyzer remain required. Storage and execution mechanisms are amended explicitly above. Frozen blueprint/handoff bytes remain provenance.
