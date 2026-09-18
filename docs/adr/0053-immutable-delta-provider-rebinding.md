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
revisit: Upstream exposes bounded binary provider restoration with validated root and snapshot ownership, or a portable native snapshot codec is qualified.
verification: Plan 19 descriptor_bijection_digest_and_portable_route, binary codec probe, captured read units and DC08-DC09/Q08-Q09.
---

# ADR-0053: Rebind restricted immutable Delta descriptors under current read protection

## Context

Plan 18 requires an actual restricted DeltaLogicalCodec read consumer. At the pinned commit the
provider hooks serialize a DeltaScan; they do not authorize its requested identity/schema, restore
runtime handles, or support arbitrary logical extensions. The upstream verifier confirmed that
no public accessor/rebinding combination supplied these guarantees without reconstructing the
snapshot. Keep this record proposed until the application conformance review accepts its scope.

## Scope

Typed in-process provider ingredients and retained service search-projection descriptors. Portable
export projections use exact Delta table/version/cohort bindings and native log reconstruction;
they do not serialize absolute source locations or filesystem incarnations. CDF uses its own
explicit interval builder. Complete fresh-process command replay remains Plan 19 CP08 work.

## Drivers

Share native immutable snapshots without hot-path serialization. Preserve exact current read
protection, semantic contracts, portability and physical allocation ownership.

## Options

- Cold opens for every read repeat replay and verification.
- Serialized in-process snapshots add parsing and copies without an interchange requirement.
- Unrestricted logical/physical codecs expose unsupported nodes and runtime-dependent state.
- Typed resident values plus restricted binary descriptors at actual persistence boundaries:
  selected. Portable bundles use native Delta logs because an encoded provider embeds its root.

## Decision

DataFusion DefaultCache holds typed immutable ingredients keyed by namespace incarnation,
exact table/version/cohort, semantic contract, compiled definition and the bound session witness.
Byte eviction has no TTL. Snapshots share the registry's accounted allocation; cache values contain
no lease, effects or operation handle. Every provider entry point checks exact protection before
lookup and attaches current protection to a freshly constructed native provider.

Service projection checkpoints persist `delta-immutable-cbor/1`: a restricted native
DeltaLogicalCodec envelope, Ciborium 0.2.2 and raw Arrow IPC byte strings. Native declarations
supply Binary storage and exact hex transport. Native admission verifies output/descriptor
bijection, digest and route. Cold restoration checks exact durable metadata/history, contract,
namespace, configuration, schema and version, then adopts full native EagerSnapshot ingredients
into the shared registry. It rebuilds from the registry-selected allocation so physical readers
retain the correct reservation even if another producer won the same-version insertion.

The vendor seam refuses nonmaterialized/partial state, skipped statistics, operation predicates,
operation IDs and unsupported hooks. Full-table adoption also refuses file selection and row-index
semantics. There is one current decoder, no old JSON reader or fallback. The upstream JSON provider
codec is replaced at the restricted boundary; Delta log JSON remains the native storage protocol.

A fixed framing preflight uses ciborium-ll before typed decoding to enforce definite lengths,
cardinality, depth and exact document exhaustion. Native Arrow footer/message validation checks
lengths, buffers, rows, nodes and field depth before FileReader allocations; compressed payloads
and unsupported representations refuse. Encoding and decoding reserve bounded native budgets.
These limits are allocation admission, not exact RSS measurement; external-memory qualification
remains CP09/CP12.

Portable export checkpoints explicitly carry no binary read descriptors. Native admission enforces
this property only for `export_rebuild`; ordinary checkpoints require the complete descriptor set.
Their readers reconstruct exact Delta versions under fresh captured protection. An invalid service
descriptor never falls back to reconstruction. This preserves atomic bundle rename and subsequent
copying without weakening service namespace identity or inventing a private snapshot relocation API.

Read-only captures retain a bounded exact dependency vector and the matching permanent root lock;
the lock alone is not table authorization. Native anti-joins apply the same exact table contract as
durable leases. Full reclamation, sealed-bundle and fresh-process qualification remain open.

## Evidence

| Claim | Primary source | Retrieved | Exact support |
|---|---|---|---|
| Logical extension hooks are unimplemented at the pin | [DeltaLogicalCodec](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs) | 2026-09-17 | `todo!("DeltaLogicalCodec")` |
| Runtime scan handles are not serialized | [DeltaScan](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs) | 2026-09-17 | `#[serde(skip)]` on runtime handles |
| Typed decoding accepts a recursion limit | [Ciborium 0.2.2](https://docs.rs/ciborium/0.2.2/ciborium/de/fn.from_reader_with_recursion_limit.html) | 2026-09-17 | `pub fn from_reader_with_recursion_limit` |
| Codec output can use a bounded writer | [Ciborium writer](https://docs.rs/ciborium/0.2.2/ciborium/fn.into_writer.html) | 2026-09-17 | `pub fn into_writer` |
| Snapshot root participates in restored scans | [Pinned DeltaScan](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs) | 2026-09-17 | `snapshot: SnapshotWrapper` |

## Verification

Plan 19 isolated upstream probe exercises binary native snapshot roundtrip, malformed/trailing
CBOR, forged IPC footer refusal and rebound native scan. Pure Arrow tests cover native descriptor
bijection/digest/export routing and exact read scope. Receipts live under
`.dev-state/plan19/execution/`; each records the source and scope actually tested. These are not
full service persistence, export-copy or fresh-process qualification, which remain not_run until
CP11. Plan 18's old JSON-cache receipts are historical and do not certify this implementation.

## Boundaries preserved

§B1–§B13: Rust owns authority, effects and retention. DataFusion/Delta remain execution and storage
engines; Python stays a thin transport/extraction boundary. No repository-under-study mutation,
arbitrary command/SQL interface, legacy data import or cross-operation materialization is added.

## More information

[Plan 19](../plans/19-unified-runtime-and-delta-cache-completion.md),
[cache review](../design_review/reviews/design_review_datafusion-cache-factory_2026-09-17.md).

## Status history

- 2026-09-17 — proposed; restricted native consumer implemented, final conformance open.

- 2026-09-17 — proposed argument updated for typed resident values, restricted binary persistence and portable native export reads; complete application conformance remains open.
