---
id: ADR-0044
title: Own external effects through native command execution
status: accepted
date: 2026-09-15
deciders: [paul-heyse]
level: decision
principles: [DM-19, DM-20, DM-28, DM-29, DM-35]
design: [§8.1, §8.3, §9.1]
review: docs/design_review/reviews/design_review_unified-datafusion-delta-runtime_2026-09-15.md
evidence: Interface-checked
supersedes: [ADR-0032]
superseded-by: null
revisit: A required Plan 15 consumer cannot use the qualified native route or preserve its semantic and ownership contract.
verification: Plan 15 native-effect-check; native-resources-check (implementation oracles; not yet qualified).
---

# ADR-0044: Own external effects through native command execution

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

External effects enter through finite native command/source/sink extensions, never scalar UDFs or planning callbacks. Native plans select work and shape facts; OS/network/archive/compiler/LSP drivers perform only mechanisms. Durable attempt/fence, interests and cleanup obligations own the effect. Stream drop, timeout and lease expiry do not release ownership until physical cleanup is observed. Reconcile unknown execution and commit outcomes before retrying. Use one shared native runtime with explicit accounting for writer, decoder, kernel and child-process resources outside its memory pool. Preserve isolated Rust/rust-analyzer and Python/ty behavior and source/stub/runtime distinctions.

### Consequences

The implementation and new epoch replace the old mechanisms directly. There is no parallel compatibility runtime. Native upstream capabilities are inherited through qualified public routes, not copied from private modules. Required missing qualification remains visible.

### Compensating controls

Exact dependency lock, native admission and route tests, ownership/recovery faults, preserved source evidence, and Plan 15 deletion and installed MCP oracles.

## Evidence

Verified 2026-09-15 against the exact local checkouts by the upstream-verifier; the referenced reviews retain bounded executed probes. This is API evidence, not full-service certification.

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Native integration boundary | [Exact upstream source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs) | 2026-09-15 | The provided `session` should wrap a concrete `datafusion::execution::context::SessionState`. |

## Verification

Plan 15 assigns native-effect-check; native-resources-check. These must be implemented and executed on the final source. Prior Plan 14 gates and review probes are not substituted for them.

## Boundaries preserved

Rust owns semantics and publication; Python remains a thin MCP adapter and separate static extraction worker. Exact identities, independent epistemic classes, isolated execution outside studied repositories, typed grants, bounded truthful MCP delivery, ty and rust-analyzer remain required. Storage and execution mechanisms are amended explicitly above. Frozen blueprint/handoff bytes remain provenance.
