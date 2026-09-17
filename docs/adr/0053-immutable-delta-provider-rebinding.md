---
id: ADR-0053
title: Rebind restricted immutable Delta descriptors under current read protection
status: proposed
date: 2026-09-17
deciders: [paul-heyse]
level: decision
principles: [DM-02, DM-06, DM-16, DM-42]
design: [§17]
review: docs/design_review/reviews/design_review_delta-integration-followup_2026-09-15.md
evidence: Implemented
supersedes: []
superseded-by: null
revisit: Upstream exposes validated immutable Delta provider rebinding and bounded codec writers, or a required consumer needs persisted descriptors.
verification: provider_cache unit fixtures, retention exact-vector units and Plan 18 CF07-CF08/Q08-Q09.
---

# ADR-0053: Rebind restricted immutable Delta descriptors under current read protection

## Context

Plan 18 requires an actual restricted DeltaLogicalCodec read consumer. At the pinned commit the
provider hooks serialize a DeltaScan; they do not authorize its requested identity/schema, restore
runtime handles, or support arbitrary logical extensions. The upstream verifier confirmed that
no public accessor/rebinding combination supplied these guarantees without reconstructing the
snapshot. Keep this record proposed until the application conformance review accepts its scope.

## Scope

EvidenceTables' exact immutable version/cohort reads. Durable command replay, persisted descriptor
storage and fresh-process qualification remain separate open Plan 18 work. CDF is reconstructed
from explicit descriptors through its existing builder, never this cache.

## Drivers

Reuse the native snapshot descriptor without repeating table loading. Keep current read authority,
semantic contracts and resource lifetimes independent of cache occupancy.

## Options

- Reopen every snapshot: correct but leaves the native provider codec unused.
- Deserialize and reconstruct another table: loses the useful cached snapshot and repeats loading.
- Unrestricted logical/physical codecs: expose unimplemented hooks and runtime-dependent state.
- Restricted provider hook plus validated rebinding: selected with a narrow recorded vendor seam.

## Decision

Use DataFusion DefaultCache with typed exact keys, bounded bytes and TTL. Keys bind canonical root,
table identity, version, cohort, semantic contract, all effective session options, codec revision
and complete compiled definition. Build identity includes policy, functions, mappings and codecs.
Eviction never grants authority or releases an outstanding reader's managed reservation.

A caller enrolls and verifies its complete table vector before any provider is opened. Durable
protection is checked by a native anti-join against the exact committed dependency records. Cache
values carry no lease or effect handle. Hits validate the native snapshot identity and complete
storage/semantic/CHECK contract, then rebind the owned current LogStore and native session engine.
Both hits and misses attach current retention before returning a read-only native view.

The vendor seam exposes the captured Snapshot and validates/rebinds immutable scans using native
reader-feature checks, root identity and schema reconstruction. It refuses operation-local file
predicates/operation IDs that serialization omits, and preserves serialized file selection. A
bounded writer method delegates to the same native codec; no alternate provider serialization is
implemented. Generic extension and physical codec hooks are not invoked.

Encoding reserves managed memory before buffer growth. Decoding reserves a bounded expansion
budget and retains it through provider/plan/stream ownership. That budget is not a complete
measurement of every external kernel allocation; CP09 qualification remains open. Existing
read-only root guards are namespace-checked; replacing their global protection with exact durable
read-only enrollment remains CP07 work.

## Evidence

| Claim | Primary source | Retrieved | Exact support |
|---|---|---|---|
| Logical extension hooks are unimplemented at the pin | [DeltaLogicalCodec](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs) | 2026-09-17 | `todo!("DeltaLogicalCodec")` |
| Runtime scan handles are not serialized | [DeltaScan](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs) | 2026-09-17 | `#[serde(skip)]` on runtime handles |
| Native bounded cache has an explicit TTL constructor | [DefaultCache](https://github.com/apache/datafusion/blob/55.1.0/datafusion/execution/src/cache/default_cache.rs) | 2026-09-17 | `pub fn new_with_ttl(memory_limit: usize, ttl: Option<Duration>)` |

## Verification

`cargo test -p enrichment-store --lib provider_cache::tests`: isolated native empty-table codec
roundtrip/rebinding, wrong identity/version/root/config, non-Delta refusal, read-only mutation
refusal, allocation rejection and evicted-live-value accounting. Receipt:
`.dev-state/plan18/execution/provider-cache-units.log` (3 passed, 2026-09-17).

Exact protected-vector negatives use native Arrow fixtures. Fresh-process replay, final resource
pressure and installed application journeys remain not_run until Plan 18's deletion barrier.

## Boundaries preserved

§B1–§B13: Rust owns authority, effects and retention. DataFusion/Delta remain execution and storage
engines; Python stays a thin transport/extraction boundary. No repository-under-study mutation,
arbitrary command/SQL interface, legacy data import or cross-operation materialization is added.

## More information

[Plan 18](../plans/18-schema-governed-runtime-and-cache-completion.md),
[cache review](../design_review/reviews/design_review_datafusion-cache-factory_2026-09-17.md).

## Status history

- 2026-09-17 — proposed; restricted native consumer implemented, final conformance open.
