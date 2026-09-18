---
id: ADR-0055
title: Own committed Delta snapshots and share native cache policy
status: proposed
date: 2026-09-17
deciders: [paul-heyse]
level: decision
principles: [DM-02, DM-16, DM-26, DM-32, DM-36, DM-43, DM-50]
design: [§8.4, §17]
review: docs/design_review/reviews/design_review_datafusion-deltalake-caching_2026-09-17.md
evidence: Interface-checked
supersedes: []
superseded-by: null
revisit: The pinned libraries provide shared CDF metadata and accounted snapshot ownership, or a real nonlocal or ListingTable consumer is enabled.
verification: Plan 19 DC01-DC12, CF01-CF10 and Q01-Q13; focused native cache and contract units.
---

# ADR-0055: Own committed Delta snapshots and share native cache policy

## Context

The shared runtime already owns operation materialization and file metadata, but Delta table
opens repeatedly replay logs and verify the same immutable contracts. The provider cache encodes
and decodes the native snapshot on its hot path. CDF constructs a private metadata cache whose
relative paths cannot safely share the ordinary scan's absolute-path namespace without alignment.

## Scope

Plan 19's local native runtime, table state, immutable contract/provider reuse, CDF metadata,
cache policy and observations. This record remains proposed until the revised design and final
application evidence are reviewed. It changes neither durable Delta authority nor read protection.

## Drivers

One owner per value and policy; reuse native incremental snapshots and native byte eviction;
exact version/incarnation semantics; preserve physical ownership after eviction; no cached grants.

## Options

- Repeated cold opens with more frequent checkpoints: improves restart cost but retains repeated
  replay and semantic preparation.
- Serialized in-process snapshots: adds codec allocations and parsing without an interchange need.
- Typed snapshots/contracts with native caches and bounded coordination: selected.
- Global query-result or physical-plan cache: excluded; operation results remain operation-owned.

## Decision

One QueryRuntime configures native caches and table-class policies. DataFusion DefaultCache owns
resident byte eviction, not compute-if-absent or resource authority. A bounded snapshot registry
shares immutable Delta state, refreshes mutable heads with native forward-only update and publishes
acknowledged writer results monotonically. Exact reads never downgrade heads. Current reads check
for foreign commits; cache TTL is not freshness. Contract reuse is positive-only and scoped to the
registry incarnation and full semantic witness. In-process provider values are typed; only retained
replay descriptors are serialized in a restricted, bounded binary format (ADR-0053).

Snapshot/contract/provider allocations retain managed reservations through their last physical
consumer. Accounted occupancy is distinct from evicted live allocations and external estimates.
Cache size declarations remain immutable after insertion. Shared metadata uses a concrete native
cache; unconsumed listing/statistics families are disabled. CDF must use the bound session and the
same canonical root-store file identity as normal scans. Native Parquet predicate limits and Delta
checkpoint cadence are explicit policy, not new execution algorithms.

Every cache hit still requires current identity, semantic contract and retention admission. Root
replacement/removal, maintenance generations, changed witnesses and confirmed physical deletion
invalidate their respective derived caches. Miss/failure never implies absent evidence. Native
mutation, command and publication ownership stays independent of cache membership.

### Consequences

Repeated replay/serialization is eliminated for resident admitted values. Coordination, memory
ownership and maintenance invalidation become explicit responsibilities. No generic cache platform,
private replay algorithm, historical reader or automatic result reuse is introduced.

### Compensating controls

Plan 19 DC01-DC12 include foreign commits, delayed writers, wrong identity, positive-only contracts,
path collisions, failed deletions, cache eviction with live readers, bounded codecs and shutdown.
The CP11 completed-pivot/deletion barrier precedes full application journeys. Local vendor seams
retain exact hashes and upstream pins; no upstream publication is implied by this decision.

## Evidence

| Claim | Primary source | Retrieved | Exact support |
|---|---|---|---|
| Cache values can outlive resident entries | [DataFusion DefaultCache](https://github.com/apache/datafusion/blob/55.1.0/datafusion/execution/src/cache/default_cache.rs) | 2026-09-17 | `let value = entry.value.clone();` |
| Delta refresh is forward-only | [DeltaTable](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs) | 2026-09-17 | `This API is forward-only.` |
| CDF planning already receives a session | [CdfLoadBuilder](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs) | 2026-09-17 | `session: &dyn Session` |
| Kernel memory sizing is an estimate | [Kernel Snapshot](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs) | 2026-09-17 | `Best-effort estimate` |

Exact local source checks and corrections are in Plan 19 §3.2/§9.1. Independent verification is
recorded in `.dev-state/plan19/execution/upstream-verification.md` when returned; its absence is not
an executed receipt. Review fixture timings are not application performance guarantees.

## Verification

Focused contract/cache units and compile/schema checks during implementation; Plan 19 complete
DC/CF/Q application matrices after CP11. No complete package or application gate is closed by this
record. Required evidence remains open until actually executed against the final source.

## Boundaries preserved

§B1–§B13: Rust owns semantics, authority and effects; native DataFusion/Delta execute plans and
persist state; Python remains mechanical transport/extraction; no studied repository mutation,
raw SQL/shell tool, legacy import, cached effect authorization or unqualified publication.

## More information

[Plan 19](../plans/19-unified-runtime-and-delta-cache-completion.md),
[ADR-0052](0052-operation-owned-native-materialization.md),
[ADR-0053](0053-immutable-delta-provider-rebinding.md).

## Status history

- 2026-09-17 — proposed; implementation authorized, full application qualification deferred until the deletion barrier.
