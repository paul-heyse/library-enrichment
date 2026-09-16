---
id: ADR-0043
title: Preserve typed semantics across Arrow and Delta
status: accepted
date: 2026-09-15
deciders: [paul-heyse]
level: decision
principles: [DM-06, DM-07, DM-15, DM-22, DM-42]
design: [§6.2, §6.3]
review: docs/design_review/reviews/design_review_unified-datafusion-delta-runtime_2026-09-15.md
evidence: Interface-checked
supersedes: [ADR-0022, ADR-0031]
superseded-by: null
revisit: A required Plan 15 consumer cannot use the qualified native route or preserve its semantic and ownership contract.
verification: Plan 15 native-evidence-check; delta-mutation-check (implementation oracles; not yet qualified).
---

# ADR-0043: Preserve typed semantics across Arrow and Delta

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

Native Arrow schemas are semantic authority. Map every used type explicitly into lossless Delta storage; reject unsupported representations and invalid values. Kernel Arrow conversion is not proof of losslessness. Store semantic contract metadata and validate/rebind it on exact snapshot reads. Candidate constraints are untrusted until native row/tag, aggregate-key and anti-join reference validation succeeds. Hash canonical typed bytes using declared native ordering and merge semantics; never reconstruct domain objects or JSON trees for identity. Raw source bytes remain exact immutable evidence. Native semantic identity is distinct from table versions/files. Use paired kernel schema/expression patches only at actual kernel boundaries.

### Consequences

The implementation and new epoch replace the old mechanisms directly. There is no parallel compatibility runtime. Native upstream capabilities are inherited through qualified public routes, not copied from private modules. Required missing qualification remains visible.

### Compensating controls

Exact dependency lock, native admission and route tests, ownership/recovery faults, preserved source evidence, and Plan 15 deletion and installed MCP oracles.

## Evidence

Verified 2026-09-15 against the exact local checkouts by the upstream-verifier; the referenced reviews retain bounded executed probes. This is API evidence, not full-service certification.

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Native integration boundary | [Exact upstream source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_conversion/mod.rs) | 2026-09-15 | ArrowDataType::UInt64 => Ok(DataType::LONG) |

## Verification

Plan 15 assigns native-evidence-check; delta-mutation-check. These must be implemented and executed on the final source. Prior Plan 14 gates and review probes are not substituted for them.

## Boundaries preserved

Rust owns semantics and publication; Python remains a thin MCP adapter and separate static extraction worker. Exact identities, independent epistemic classes, isolated execution outside studied repositories, typed grants, bounded truthful MCP delivery, ty and rust-analyzer remain required. Storage and execution mechanisms are amended explicitly above. Frozen blueprint/handoff bytes remain provenance.

## Native dependency policy

The type-universe invariant applies to every Arrow, DataFusion, Parquet, object_store, Delta
and selected kernel crate. The architecture gate checks one version per native interface and
exact anchor versions. Cargo deny reports utility duplicates without a per-crate skip list.
At the pinned Delta revision, alloc-stdlib 0.2.2 is required while the resolved compression
stack also uses 0.3.0; changing upstream APIs merely to remove that duplicate does not improve
native type compatibility. Source restrictions, wildcard bans, licenses and banned dependency
classes remain enforced. Verified from the resolved Cargo.lock and `cargo deny` on 2026-09-15.
