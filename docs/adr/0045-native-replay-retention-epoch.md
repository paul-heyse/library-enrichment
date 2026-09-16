---
id: ADR-0045
title: Rebuild native plans and retain only target-epoch evidence
status: accepted
date: 2026-09-15
deciders: [paul-heyse]
level: decision
principles: [DM-24, DM-31, DM-33, DM-35, DM-44, DM-51]
design: [§2.3, §7.3, §8.4]
review: docs/design_review/reviews/design_review_unified-datafusion-delta-runtime_2026-09-15.md
evidence: Interface-checked
supersedes: []
superseded-by: null
revisit: A required Plan 15 consumer cannot use the qualified native route or preserve its semantic and ownership contract.
verification: Plan 15 native-replay-check; delta-incremental-check; delta-retention-check; native-removal-check (implementation oracles; not yet qualified).
---

# ADR-0045: Rebuild native plans and retain only target-epoch evidence

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

Activate a fresh target epoch with no old-state/artifact importer, compatibility schema or alternate runtime. Persist typed commands and exact input/publication descriptors, then rebuild native physical plans on replay. DeltaLogicalCodec is limited to validated read-provider payloads; unsupported logical hooks and deprecated DeltaPhysicalCodec are excluded. CDF providers are reconstructed from bounded version-window descriptors. Maintain actual search projections with CDF and select output plus offset in one control transaction. Native full recomputation handles rule revisions and expired history explicitly. One retention definition covers files, logs, CDF, command replay, artifacts and reader/result leases. Marker expiry never authorizes duplicate effects. Qualify conditional storage, synchronization and native reclamation; exclude racy ConditionalPutShim. Remove the enumerated retired runtime and historical service artifacts after candidate qualification.

### Consequences

The implementation and new epoch replace the old mechanisms directly. There is no parallel compatibility runtime. Native upstream capabilities are inherited through qualified public routes, not copied from private modules. Required missing qualification remains visible.

### Compensating controls

Exact dependency lock, native admission and route tests, ownership/recovery faults, preserved source evidence, and Plan 15 deletion and installed MCP oracles.

## Evidence

Verified 2026-09-15 against the exact local checkouts by the upstream-verifier; the referenced reviews retain bounded executed probes. This is API evidence, not full-service certification.

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Native integration boundary | [Exact upstream source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs) | 2026-09-15 | Provide a snapshot to use for the table provider |

## Verification

Plan 15 assigns native-replay-check; delta-incremental-check; delta-retention-check; native-removal-check. These must be implemented and executed on the final source. Prior Plan 14 gates and review probes are not substituted for them.

## Boundaries preserved

Rust owns semantics and publication; Python remains a thin MCP adapter and separate static extraction worker. Exact identities, independent epistemic classes, isolated execution outside studied repositories, typed grants, bounded truthful MCP delivery, ty and rust-analyzer remain required. Storage and execution mechanisms are amended explicitly above. Frozen blueprint/handoff bytes remain provenance.
