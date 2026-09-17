---
title: Complete the unified runtime with governed DataFusion and Delta caching
status: draft
date: 2026-09-17
adrs: [ADR-0041, ADR-0042, ADR-0044, ADR-0045, ADR-0047, ADR-0048, ADR-0049, ADR-0050, ADR-0051, ADR-0052, ADR-0053, ADR-0054]
phase: 6
---

# Complete the unified runtime with governed DataFusion and Delta caching

## 1. Purpose, scope and execution discipline

This plan combines:

1. The [DataFusion/Delta caching review](../design_review/reviews/design_review_datafusion-deltalake-caching_2026-09-17.md), including F0–F8, its recommendations, limitations and [recorded probes](../design_review/reviews/evidence/datafusion-deltalake-caching-2026-09-17/README.md).
2. Additional opportunities and necessary corrections established from the current implementation and pinned library sources in this planning pass (§3–§4).
3. **Every remaining obligation in [Plan 18](18-schema-governed-runtime-and-cache-completion.md)**, reconciled against its [execution ledger](18-schema-governed-runtime-and-cache-completion.md#10-execution-ledger-and-completion-rule).

**Use Plan 19 as the combined remaining sequence when implementation resumes.** CP00–CP13 retain their identifiers and complete inherited responsibilities. This plan changes their cache implementation, prerequisites and regression coverage; it does not create a separate cache project or discard the unfinished runtime pivot. Plan 18 preserves dated implementation receipts. Plan 17's complete FP00–FP16, Q01–Q13, SC01–SC10 and integration dispositions remain incorporated through Plan 18. L01–L25 and CF01–CF10 remain binding, with additions below. No requirement is waived by a shorter summary here.

**This pass is documentation-only.** New architecture is **Proposed**, source findings are **Interface-checked**, and review timings remain historical fixture measurements. No implementation, acceptance execution, installation, state removal or activation is authorized by this planning artifact alone. Prior implementation authorization and the current request to prepare this document are separate: this turn stops after the plan is written and checked.

### 1.1 Required destination

- DataFusion expresses transformations, selection, policy, eligibility, results and command plans. Delta is the durable authority for service state, publication, contracts and retention. Bounded Rust I/O, synchronization and process drivers execute admitted work.
- One finite native declaration supplies schema, identities, references, rules, operation bindings and generated transport. Python remains the thin FastMCP adapter and mechanical producer worker.
- One shared native runtime configures caches. Library caches and Delta snapshots retain derived values; **cache membership never grants read, effect, publication or retention authority**.
- Preserve all nine tools, five resource registrations, Rust/Python insights, qualified execution, durable jobs/cancellation, offline reuse, complete retained results, export and reproducible exact-version reads.
- Hard pivot: remove replaced source, schemas, fixtures, readers, packages, launch paths, installations, state and registrations. No old/new mode, migration, alias, historical internal decoder or executable rollback copy. Preserve frozen provenance, requested design evidence and the Delta history required by the target service.
- Published evidence/results remain until explicit removal. Disposable caches are not retention roots. Current source artifacts, command witnesses and historical versions selected by target retention remain functionality, not legacy compatibility.

### 1.2 Test order

**Do not run full integrations, service publication/CDF/export journeys, real clients, installed qualification or full `just ci`/`just test` until CP11 proves the completed architecture and all legacy deletions, including installed state and client registrations.** Intermediate discontinuity and downtime are accepted.

Before CP11, implement the architecture and delete replaced paths. Use meaningful units, pure Arrow/native fixtures, isolated upstream characterization, affected compile/Clippy/static checks and schema generation. A service storage/CDF/restart/MCP journey remains an integration test regardless of its filename or whether it lives in a library test module. Author such tests now; execute them in CP12. The supplied review's previous probe run is evidence to inspect, not permission to replay its service-level portions during this pivot.

After CP11, run the complete final matrices and actual client/producer/storage journeys, repair the target, rebuild and requalify affected source. Activate fresh state only after qualification. Do not add recurring full qualification before ordinary edits or commits.

## 2. Current boundary: preserve implementation, finish consumers

Reviewed **2026-09-17**, HEAD `ccf5739a333d29e6fbabe6e054c942f1c5da8b0b` plus the shared dirty tree. That tree contains prior Plan 17/18 work and concurrent capability/evidence work; this plan claims no exclusive attribution. Preserve `.dev-state/plan18/{planning,execution}/baseline.json` and the checkpoint receipts. Do not reset or broadly clean the tree.

Current source identities are **state 11, snapshot 10.0, wire 5.0, native-json/3**. These are source identities, not evidence of installation. Derive the next identities from actual schema/configuration/descriptor changes; refuse incompatible state and start fresh without conversion. Plan 18's opening state-10/wire-4 baseline is historical; its §10 is the current checkpoint.

| Existing implementation to retain | Remaining boundary carried into this plan |
|---|---|
| Native schema/union/field/identity declarations, semantic analyzer, exact codecs, checked Page | All-family declaration/bounds/witness coverage and full operator/storage/mutation matrices: CP01/CP02 |
| Shared runtime, immutable catalog/factory, composed contract/retention/command/Delta planners | Native provider property truth, all mutation/feature routes, new cache construction and Delta state owner: CP02 |
| 23 Delta control families, exact table-vector anti-join, durable guards, owned native tasks | All retention consumers, exact process grants, warm LSP, crash/unknown-commit and physical-exit closure: CP03/CP07 |
| OperationCacheFactory; SearchIndex, ComparisonKeys, OverviewChildren, OverviewNamespaces; owned single fill, native spill, native counts | Complete consumer/pressure/lifecycle matrix; shared session witness and optional measured memory tier: CP04/CP09/CP12 |
| Restricted DefaultCache provider cache with identity checks, bounded JSON codec and fresh protection | Replace hot-path serialization with shared typed snapshots; persist only restricted binary descriptors; fresh-process replay: CP02/CP08 |
| Native delivery-size UDF, DataFusion inline/retained selection, wire 5.0, Rust tool/error/resource presentation, exact SDK framing | Handler page/aspect/window/recovery composition, artifact prefix selection, all cursor witnesses and actual routes: CP06 |
| Format-61 Rust facts, Python extraction, source/registry/revision captures and native normalizers | Acquisition/runtime/LSP facts, producer fidelity, exact environment/effect consumers and final workers: CP05 |
| Retention horizons, checkpoint/CDF/maintenance helpers and kernel metric mapping | Explicit removal/reclamation, read-only/export enrollment, CDF/replay consumers, resource/diagnostic shutdown: CP07–CP09 |
| Deleted eager CompletedIndex/count channel/custom IPC path and Python semantic preview/fit loop | All remaining L01–L25 plus new cache replacements; stale typed fixtures, packages and installed retirement: CP10/CP11 |

**All CP packages remain open.** Plan 18's focused units, serializer probes and production compile/Clippy receipts retain their exact source and scope. They do not close complete Q/SC/CF matrices. The all-target compile receipt still records stale typed-ID, request-default and removed-resource-API fixtures. Fix those fixtures to the target design, not by restoring deleted conversions.

Keep the unresolved evidence in Plan 17 §2.3 visible: navigation List metadata, execution/export artifact Struct metadata, Decimal diagnostics, nullable document kinds, result clock/window admission, diagnostic pressure/shutdown, and fixtures expecting retired ownership JSON. Their final matching-worker oracles belong to CP12. Do not claim that these old failures were reproduced in this planning pass.

**Newly identified prerequisite:** the review reproduced a warm contract-lookup failure. Source still passes the five-column `native_contract::changes` relation to the one-identity-column `QueryRuntime::require_empty` consumer. Repair this shape contract before populating a positive cache; caching must not conceal the defect (§5, CP02-A).

## 3. Cache decisions and additional opportunities

### 3.1 Complete review disposition

F identifiers here refer to the **2026-09-17 DataFusion/Delta caching review**, not older design reviews.

| Finding | Chosen target | Implementation / proof |
|---|---|---|
| F0 warm contract lookup refuses | Keep native manifest comparison; project a declared violation identity or introduce an explicitly typed violation consumer. Do not weaken emptiness checking or treat failure as absence | CP02-A; DC01, Q01/Q02 |
| F1 repeated full Delta replay | Shared, bounded per-namespace snapshot registry; incremental refresh of retained state, exact-version lookup, post-commit publication and bounded per-key single-flight | CP02-C/CP03; DC02–DC04 |
| F2 repeated contract replay/query | Positive-only `DefaultCache` of verified immutable contracts, scoped to registry incarnation and declaration witness; no time-based correctness and no cached absence | CP02-D; DC05 |
| F3 private CDF metadata cache | Replace the builder-owned 1 MiB cache on the service route with the bound session's shared cache, with canonical file identity and current reader policy | CP02-B/CP08; DC06/DC07 |
| F4 JSON provider hit path | Typed immutable provider ingredients share the registry's snapshot allocation. Restricted binary/IPC-native descriptors only at persistence/replay boundaries | CP02-E/CP08; DC08/DC09 |
| F5 checkpoint cadence implicit | Declare checkpoint interval by table class; initial control interval 10, other classes 100, then qualify cost. Use native checkpoint hooks and retention horizons | CP02-B/CP07/CP12; DC04/DC11 |
| F6 unused statistics/listing caches | Configure both limits to zero, remove their active status variants, regenerate current wire; retain their explicit unused disposition in documentation | CP02-B/CP09/CP10; DC10 |
| F7 occupancy/invalidation inaccessible | Retain concrete metadata `DefaultCache` handle; expose accounted occupancy without copying all entries; invalidate exact canonical paths after confirmed physical deletion | CP02-B/CP07/CP09; DC07/DC10 |
| F8 engine and repeated options setup | Reuse the operation-neutral engine for a stable owned log store; compute an immutable effective-session witness once per bound configuration, reuse it in all cache bindings | CP01/CP02/CP04; DC03/DC12 |

Keep the operation CacheFactory and native spill route. Keep the composed NativePlanner and `SessionFallbackPolicy::RequireSessionState`; do not replace them with a convenience Delta session that loses application analyzers, UDFs or extension planners. Keep Parquet pruning/page-index/Bloom/read-policy capabilities with actual consumer evidence.

### 3.2 Corrections to apply when integrating the review

1. **CDF API:** the pinned `CdfLoadBuilder` has `build`/`build_with_metrics(&dyn Session, ...)`, not the proposed `with_session_state` method. Wire the reader at that actual scan boundary and thread it through add/remove/CDC and deletion-vector access-plan paths. A new setter is unnecessary merely to pass an already available session.
2. **CDF file identity:** ordinary next scans use full file-URL paths with the runtime's root store; CDF uses action-relative paths with the table-prefixed store. Injecting one path-keyed cache without normalizing these namespaces can collide across tables and cannot reliably warm the ordinary-scan entry. This is a design risk established from source, not a reproduced corruption claim.
3. **Predicate cache:** DataFusion 55.1 forwards `max_predicate_cache_size` only when `Some`; Parquet 59.3 defaults to **100 MiB per row group**. `None` therefore does not mean unlimited in this concrete route. Set the limit explicitly and account for simultaneously active readers, rather than claim to fix an observed unlimited allocation.
4. **Memory:** `DefaultCache::memory_used()` is exact for its declared key/value size accounting, not allocator/RSS measurement. Kernel `estimated_owned_heap_size_bytes()` is explicitly best-effort, O(log-file count), and excludes shared schemas/CRC. Delta's inner kernel snapshot/materialized-batch accessors are private. Do not promise direct public access, exact heap measurement, or preallocation enforcement from this estimate.
5. **Freshness:** retaining the last locally committed table does not prove it is the latest when another process can commit. “Zero I/O current read” requires a proven exclusive-writer generation. Default mutable-head capture performs incremental refresh; only an exact already captured version is reusable without head discovery.
6. **Cold load count:** “one replay per table per process” is valid only for an uninterrupted resident, uninvalidated working set. Eviction, root replacement, restart or an uncached historical version can require another cold load. Test one cold fill per resident key and refresh cohort, not an impossible lifetime bound.
7. **Checkpoint cadence:** use a **smaller** interval for more frequent checkpoints. The review's alternative saying “raise the interval” reverses that relationship. At the pin the hook checks `(version + 1) % interval`, and lazy `require_files=false` tables skip automatic checkpoint creation.
8. **Authority:** retained snapshots/contracts are derived copies of durable authority. They do not retain or recreate authority themselves. Log-floor failure is not the only invalidator: root incarnation, explicit removal, reader policy and maintenance generation also govern admission.
9. **Typed snapshot entry points:** `SnapshotWrapper` is defined under a private module. Its conversions explain the native implementation, but application code should use the public `TableProviderBuilder::with_snapshot` / `with_eager_snapshot` and scan-config methods, not import a private wrapper.

### 3.3 Additional deployment opportunities

| ID | Additional opportunity / required refinement | Chosen implementation and boundary | Oracle |
|---|---|---|---|
| CX01 | Unify physical metadata identity across scans and CDF | One canonical local root-store path and metadata convention for next scan, CDF and reclamation; no table-relative key in the shared cache | DC06/DC07 |
| CX02 | Coalesce concurrent snapshot/contract misses and refreshes | Bounded coordination per namespace/table/load class; native `DefaultCache` still owns eviction. Serialize refresh/publish transitions without a daemon-global I/O lock | DC02/DC03/DC05 |
| CX03 | Share one immutable snapshot allocation across head, exact-version and provider reuse | Accounted Arc-owned snapshot value; provider entry retains a reference plus scan contract, not a second serialized inventory or independent file list | DC02/DC08/DC10 |
| CX04 | Make immutable reuse deterministic | No TTL for exact snapshots, verified contracts or typed provider descriptors; byte eviction plus exact incarnation/version/semantic witnesses. Mutable heads have explicit refresh, not TTL freshness | DC04/DC05/DC08 |
| CX05 | Use metadata-only load mode where the consumer needs no active files | A declared identity/metadata read class uses `without_files`; full/stats-bearing class remains the query/write default. Never serve a lazy or stats-stripped value as a full scan/write snapshot | DC04/DC11 |
| CX06 | Reuse static semantic preparation with the same contract cache | Positive verified value retains `StorageContract` and immutable derived manifest/storage-schema preparation under the full compiler/semantic witness. Table-specific CHECK/features and current protection are still verified | DC01/DC05/SC08 |
| CX07 | Bound predicate reuse as part of reader policy | Explicit per-reader/row-group cap; derive concurrency exposure and charge external decoder work. Preserve pushdown, reordering and page/Bloom pruning for qualified inputs | DC10/DC11 |
| CX08 | Reuse engine/log-store and effective-session setup safely | Operation-neutral handles per owned namespace/runtime; per-call task tracking/effects/diagnostics remain current. Session witness includes final options and function/contract revisions, not just a broad build digest | DC03/DC12 |
| CX09 | Prevent cache observation from becoming a new unbounded workload | Constant-size counters and native typed status summaries; no periodic `list_entries()` cloning of all snapshot values, no recursive cache-telemetry commits | DC10, Q11 |
| CX10 | Make retention invalidation cover the whole cache dependency vector | Native maintenance/root-removal decisions invalidate heads, exact versions, contracts/providers and physical metadata at their own boundaries; cached state never rescues a revoked version | DC07/DC09, Q09 |

These are implementation obligations in this plan, not an invitation to add a general cache framework. A narrow `Cache` adapter for namespace/accounting is acceptable where the upstream contract needs it; eviction, TTL, file parsing, replay, pruning and storage remain library mechanisms.

## 4. Target cache contracts

### 4.1 One declaration and runtime, distinct lifetimes

Extend existing resource/native/table declarations rather than introduce an independent settings registry. Generate configuration validation, effective session settings, status fields and witnesses from those declarations.

| Surface | Owner and native mechanism | Reuse key / lifetime |
|---|---|---|
| Parquet metadata | QueryRuntime; concrete `DefaultCache<Path, CachedFileMetadataEntry>` injected through `CacheManagerConfig` | Canonical physical file identity; immutable-file validation; process-local, byte-evicted |
| Live/exact Delta state | Shared namespace registry reached through DeltaStore; delta-rs incremental snapshots; `DefaultCache` resident values | Namespace incarnation, table identity, version and load class; mutable head refresh is explicit |
| Verified contracts | Same runtime/namespace owner; typed `DefaultCache` | Contract registry incarnation, contract identity and exact semantic/compilation witness; positives only |
| Immutable provider ingredients | QueryRuntime typed `DefaultCache`, sharing accounted snapshot values | Exact table/version/cohort/contract/scan configuration/session witness; no lease or operation handle |
| Operation materialization | Existing OperationCacheFactory and owned spill | One admitted operation binding; discarded after last physical consumer |
| Decoder predicate cache | Native Parquet reader | Reader/row-group lifetime and explicit reader-policy cap; never a cross-query result cache |
| Persisted provider descriptor | Retained command/replay relation and existing artifact ownership | Versioned restricted binary envelope plus exact dependencies; durable descriptor, not persisted runtime cache |

No statistics/listing cache consumer is currently established. Disable those managers, not Delta log statistics or Parquet pruning. A future real ListingTable consumer must explicitly reinstate the relevant family with its validity and qualification contract.

### 4.2 Resource policy and initial values

Preserve workstation defaults: **32 GiB native pool, 64 GiB spill, 2 GiB metadata cache, 16 partitions, 16 compute workers and 16 blocking workers per owned lane**. Maintain distinct parser, transport, writer and process limits. These are ceilings, not reservations at startup.

Initial **proposed**, configurable additions:

| Policy | Initial target | Interpretation |
|---|---|---|
| Shared snapshot residency | 4 GiB global; 512 MiB per entry | Head, exact-version and provider references share values; admission may bypass residency for an oversized otherwise authorized operation |
| Typed provider descriptors | Preserve 1 GiB family ceiling | Snapshot allocation is shared, not re-encoded or charged again as a second allocation |
| Verified contracts / immutable preparation | 64 MiB family ceiling | Positive-only, byte-evicted, no TTL |
| Parquet predicate cache | Explicit 100 MiB per row group/reader | Preserve the pinned default initially; qualify aggregate concurrent exposure and smaller/larger target options in CP12 |
| Checkpoint interval | Control 10; contracts/evidence/results/HTTP/history initially 100 | One table-class declaration; extend a high-churn class only with workload evidence |
| Native metadata cache | Preserve 2 GiB | Report accounted occupancy and separate external/live memory; do not describe it as measured RSS |

Reserve shared snapshot/contract/provider allocations against the native budget exactly once for their lifetime. Evicted-but-live values retain reservations. Cache residency limits are not additional promises that these bytes can all be allocated beyond the pool. Metadata, decoder and kernel allocations need the CP09 external-allocation contract; the existing `DefaultCache` limit alone does not enforce it. Large-workstation defaults stay performance-oriented; small pressure fixtures never overwrite normal defaults.

Checkpointing bounds restart/replay work but is not log deletion authorization. Keep protected version/CDF/transaction-marker horizons independently enforced. Per-entry/total limits, arithmetic overflow, field counts, decode depth and in-flight requests are validated before unbounded work. Admission failure can decline an optional cache insertion; it cannot silently turn the requested computation into an empty result or run an unaccounted fallback.

### 4.3 Snapshot registry: value, freshness and publication

**Ownership and identity**

- Construct one registry for the shared runtime and canonical namespace incarnation. Repeated `DeltaStore::new` calls and clones for the same namespace share it. A table name alone is never a process-wide key.
- Namespace identity includes canonical root, the owned store/runtime binding and state/root generation. Loaded state includes Delta table ID, exact version, protocol/metadata identity and load class. Root removal/recreation invalidates even if the pathname is reused.
- Values retain immutable Delta state and their accounting owner. Pass `Arc<Snapshot>`/`Arc<EagerSnapshot>` through the public `TableProviderBuilder` snapshot methods, which use the native wrapper internally; do not duplicate the active-file inventory. The head is an index into committed state, not a second publication authority.
- Native policy/retention plans decide what may be read or reclaimed. Rust coordinates bounded lookups, synchronization, Arc lifetimes and library calls. Do not implement log replay or retention policy in a cache map.

**Operations**

1. `current` captures the current committed head under the namespace/maintenance fence. Cold miss loads once; resident refresh uses `DeltaTable::update_incremental`. Coalesce overlapping refreshes against the same observed generation. A later request still checks for foreign commits unless exclusive-writer freshness has actually been established.
2. `at(version)` reuses an exact admitted resident value, or performs an owned exact load. It never downgrades the current head. Keep only bounded recently used versions; durable history remains Delta's responsibility.
3. Perform updates on a candidate cloned handle. Publish only a fully successful, identity-validated snapshot. An error leaves the prior immutable value intact but does not make it an acceptable answer to a failed `current` request.
4. Every successful create/append/constraint/metadata/DML/OPTIMIZE/maintenance write publishes the returned committed state. Route native control, contracts, evidence, results, HTTP cache, diagnostic history and maintenance through this owner. Handle committed version and post-commit maintenance result separately.
5. Publish monotonically within one incarnation. An older writer result arriving after a newer one cannot move the head backwards. Same-version checkpoint adoption is a new internal snapshot value without a semantic table-version change.
6. Known conflict marks the head dirty and forces refresh/recomputation of native command preconditions. Do not destroy still-valid exact historical entries merely because a head conflicted. Unknown commit/lost acknowledgement requires durable transaction/command reconciliation before retry or cache publication.
7. Acquire exact retention enrollment before exposing a provider. Metadata needed to determine that enrollment may be captured under the bounded bootstrap/maintenance fence; recheck and retry if its generation changed. Never call public enrollment recursively from the registry or contract bootstrap.
8. Failed fills, cancellation and abandoned waiters have explicit state. Owned work retains physical tokens until actual exit; no cached failure-as-absence and no unbounded retry. No mutex over unrelated table I/O; no wait on a cache slot while holding a native permit required by its filler.
9. The bounded coordination entry must remain coherent while a fill/refresh is active even if its resident value is evicted. Separate short-lived in-flight coordination from native LRU values; remove coordination when no participant/physical task owns it.

The registry and contract cache must not introduce Arc cycles back through QueryRuntime/DeltaStore/session/log-store ownership. Drain physical tasks, then release resident values during shutdown; cache eviction alone is not physical completion.

### 4.4 Contract and provider reuse

**Contract cache:** key positive verification by registry root/incarnation/table ID, immutable contract identity, semantic mapping and compiler/definition witness. Cache the fully verified contract and immutable derived preparation. Validate full semantics before insertion, including the F0 comparison path. Negative, duplicate, corrupt, unavailable and mismatched results are never cached as absence. Publish a positive registration only after acknowledged durable commit or successful reconciliation. Invalidate on registry replacement/removal or witness changes, not on unrelated append-only registrations.

**Provider cache:** key exact root/table/version/cohort/semantic contract/scan configuration/effective session and transform/function witnesses. Store typed snapshot ingredients and schema/scan configuration; borrow the snapshot's owned allocation from the registry. Rebuild a fresh read-only provider bound to current runtime/store and current read protection on every use. No cached mutable provider, lease, effect grant, task context, operation ID or physical plan.

Precompute the session witness after all options, application extensions and function revisions are bound. Canonical typed hashing must preserve every equality discriminator; keep the underlying canonical value for equality/diagnostics rather than letting a short hash be sole authority. An operation-specific option change gets a new witness. Do not sort/rebuild the entire configuration vector on each binding check.

Cache correctness comes from exact identity and admission. Replace the current provider cache's 300-second residency TTL with byte eviction and explicit invalidation, avoiding unnecessary expiration of immutable values. The current TTL is not itself the existing identity/authority check. TTL support remains available in DataFusion for a future genuinely time-bounded consumer, with deterministic `TimeProvider` tests if adopted.

### 4.5 Shared file metadata and CDF

The local profile has one root object store at `file:///`. Standardize CDF reads on that same store and canonical absolute file paths already used by next scans, including add/remove/CDC and DV-related reader calls. Preserve native path/URI normalization; do not concatenate filesystem strings or weaken containment. Derive one stable `ObjectMeta` convention for a physical immutable file so reuse is valid across scan routes; if two routes cannot supply equivalent metadata, accept a miss rather than fabricate a match.

Create the CDF reader factory inside `build_with_metrics` from its supplied session, then thread that factory through all helpers. Remove its private cache field/allocation on this route. Assert the configured shared runtime and retained planner/function policy; no silent fallback session. Any future nonlocal store requires a namespace-qualified cache adapter or separate store-scoped entries because upstream `FileMetadataCache` keys omit the object-store URL.

Use `DefaultCache` for eviction and concrete `memory_used()` for accounted occupancy. Avoid `list_entries()` in routine status: it clones keys and values into a HashMap. Retention supplies exact physically deleted paths to `remove`; `drop_table_entries` does not work for upstream plain-Path metadata keys. Coordinate deletion with active readers and maintenance generation so a concurrent refill cannot republish a removed file. Failed deletion remains a failure and never reports freed bytes.

The upstream metadata validator compares **size and last_modified** at this pin, not table identity, object-store identity, ETag or content digest. The target's immutable-file and root-incarnation contracts therefore matter. Cache-enabled reads must preserve missing/corrupt-file errors and DV/full-scan/COUNT correctness.

### 4.6 Persisted descriptors and narrow vendor seams

Separate the two costs: in-process reuse is typed and never serializes; CP08 still implements the required real persisted immutable-provider descriptor/replay consumer. Do not remove replay scope merely because the fast path no longer needs a codec.

- Keep the restricted `DeltaLogicalCodec` provider hook, or a narrow versioned binary extension of it, as the library-owned serialization boundary. Exclude TODO generic logical hooks, DeltaPhysicalCodec, commands, operation materialization and runtime handles before invocation.
- Persist one current binary format with raw Arrow IPC payloads, explicit lengths, codec/pin/semantic identities and bounded decode. Do not wrap the current JSON number-array snapshot in a binary field and call it a binary pivot.
- Use native Snapshot serialization/reconstruction and identity checks; do not copy kernel log replay. The exact binary serializer must pass an isolated compatibility probe against the pinned Snapshot visitor, protocol/metadata types and Arrow IPC, including empty/malformed/oversized cases, before a dependency is selected. The current lock has no general binary Serde codec; an unverified serializer choice is not an API fact.
- Once selected, use one format and delete the predecessor decoder; no runtime negotiation or compatibility fallback. Persist descriptors only for actual retained command/replay dependencies. A daemon-wide persisted warm-snapshot cache is separately deferred (§9).
- Rebind root/store/runtime, semantic contract and exact retention under current authority before execution. Revoked scope, missing retained history and wrong identity refuse even when bytes decode successfully.

Expected narrow vendor work: CDF reader/cache injection and path alignment; bounded typed snapshot size/access seam if no public route supplies it; restricted binary provider encoding/decoding. Record exact source hashes, pin provenance, upstream rationale and a revisit trigger. Existing successful seams remain until replaced; no broad vendor cleanup or copying private algorithms. Preparing an upstream patch is useful; publishing it is a separate authorized action and upstream acceptance does not block the local pivot.

## 5. Integrated execution packages

CP identifiers are work owners, not new product gates. The ordering below reuses working foundations and makes caching part of the target architecture before completing its consumers.

| Order | Work | Depends on |
|---|---|---|
| 1 | CP00 inventories/decision updates; CP01 declaration and witness interfaces | Current checkpoint |
| 2 | CP02-A F0; CP02-B cache/table policy and metadata owner | CP01 interfaces |
| 3 | CP02-C registry; CP02-D contracts; CP02-E typed providers | CP02-A/B, CP03/CP07 existing ownership interfaces |
| 4 | CP03 writer/control/effect consumers and CP04 remaining materialization work | CP01/CP02; CP07 fence contract declared, not all reclamation implemented |
| 5 | CP05 producer/acquisition and CP06 research/delivery | CP01–CP04 |
| 6 | CP07 full retention/removal; CP08 CDF/replay and persisted codec | CP02–CP06; implement CP07 enrollment/fence API before CP08 consumers |
| Throughout | CP09 accounting/diagnostics and deletion as each replacement lands | Changed owner interfaces |
| 7 | CP10 source/package/enforcement deletion; all target harnesses authored | CP00–CP09 implementation |
| 8 | CP11 matching build and verified installed/state/client retirement | CP10 |
| 9 | CP12 complete final qualification and physical choices | **CP11 barrier closed** |
| 10 | CP13 fresh activation, smoke and completion | CP12 current candidate |

CP02 registry/protection interface work does not wait for all CP07 reclamation implementation. CP03 establishes exact effects before CP05 adds all producer consumers. CP09 closes each allocation/lifetime at the owner being changed, not as a late instrumentation retrofit.

### CP00 — Authority, route inventory and decisions

1. Carry forward every field/operation/provider/effect/grant/retention/replay/telemetry owner and all public/internal/maintenance routes. Add the cache surfaces in §4.1 and a consumer matrix for every `DeltaStore` construction, load, provider, writer and CDF call.
2. Record chosen snapshot/contract/metadata lifetimes and revisit triggers. Amend **proposed** ADR-0053 for typed in-process values and binary persisted descriptors; preserve its exact rebinding/authority guarantees. Allocate a new cache-governance ADR from the live index for the new shared owner, policy and status contract. Review the revised authority/lifetime/wire boundaries before accepting those decisions, using primary-source verification and the applicable ADR workflow. Preserve ADR-0052/0054 semantics. Accepted records require a successor rather than rewriting their argument.
3. Update living DESIGN §8.4 and affected schema/publication/resource sections during implementation. Distinguish a cached committed value from publication/read authority and from cross-operation query results. Complete source/pin/feature/configuration/qualification capability joins.
4. Reconcile all vendor manifests and existing ADR-0047–0051 obligations. Keep operator-tooling ADR-0046 distinct from the ty producer requirement. Prepare the protected-enforcement patch against current files; actual installation is operator-applied. An outstanding patch does not prevent independent architecture work.

**Exit:** complete owner/route/deletion inventory and recorded decisions before changing the affected binding contracts. Check documents, ADR links and provenance; no product journey.

### CP01 — Complete native declarations, typed values and witnesses

1. Finish all control/result/retention/replay/grant/acquisition/diagnostic field, domain, collection and reference declarations: scoped key pairs, variant guards, cardinality/order/duplicates and NULL versus empty. Add cache keys/status/policy/load classes to the same mechanism.
2. Complete remaining binary IDs/digests, typed clocks/units, coordinates including LSP UTF-16, canonical independent vectors and exact widths. Retain external identifiers in their actual source format.
3. Finish complete value/byte/version/cohort/contract/function/transform/policy/producer/build/environment/codec dependencies for cursors, process grants, CDF, replay and caches. Add namespace incarnation, load capability and bound-session witness. Equal physical schema or a build digest alone never proves reuse.
4. Finish depth/field/metadata/vocabulary bounds and segment-valued paths; refuse duplicate fields, unknown extensions and literal-dot/case ambiguity before recursive work.
5. Complete exact generated encoders/decoders for worker, retained results, resources, export and recovery: full-range numeric strings, clocks, binary bytes, union presence and escaped UTF-8 allocation. Remove remaining semantic Arrow→JSON→Serde bridges and duplicated vocabulary/field owners.

**Exit:** one real declaration/variant/reference change reaches all generated consumers; canonical/bounds/presence/witness units pass. Regenerate current schemas/DTOs together. SC01/02/05–08 terminal forms remain CP12. **Deletion:** L06/L09/L13/L16–L24 as applicable.

### CP02 — Semantic planning, providers and cache foundations

**A. Restore the complete contract check.** Repair F0 without bypassing `changes` or relaxing invariants. Add independent equal/mismatched semantic-manifest units, including same physical layout with different meaning. Author register→reopen→provider→pin regressions for CP12; do not run a full storage journey as a “unit.” Repair tests must reach the warmed verification path before cache insertion.

**B. Bind library policy once.** Construct the concrete shared metadata cache, disable unused statistics/listing caches and declare per-family budgets, predicate limit and table-class checkpoint interval. Bind all effective options through NativePolicy and the session witness. Add bounded snapshot/contract/provider value/accounting types and load classes. Keep composed planners, shared store and explicit session-fallback refusal.

**C. Install the snapshot registry.** Implement §4.3 including incremental refresh, exact versions, bounded misses, monotone publication, foreign-commit freshness, same-version checkpoints, root generations, cancellation and ownership. Centralize all cold-load entry points. Cache neutral engine/log-store handles only where store operation IDs do not alter the backend; otherwise obtain the correct operation-scoped handle. Do not retain operation context in a cached engine.

**D. Install positive contract reuse.** Implement §4.4 with single-flight, durable registration, duplicate/corrupt refusal, immutable preparation and correct registry-incarnation invalidation. The contract catalog's own bootstrap must not recurse into its cache/registry verification or retention enrollment. Native manifest diff remains authoritative on misses.

**E. Replace hot provider serialization.** Typed descriptors share the accounted snapshot; current runtime/store/protection are freshly bound on every hit. Delete JSON encode/decode and proportional decoded-copy reservation from the hit path when replaced. Do not add a second active-file inventory or a cache of live authorized providers.

**F. Finish inherited provider/planning scope.** Complete JOIN/UNION/CASE/IN/cast/aggregate/nested/output validation before coercion and at optimized/physical boundaries; valid outer-join widening; parent/variant requiredness; List/Map/uniqueness/references; trusted constraints/statistics only after admission. Complete allowed/refused public Delta mutation/metadata/feature routes, interrupted constraint installation and builder session/store/transaction/resource ownership. Refuse unsafe raw provider INSERT. Finish immutable inventory, qualified references, defaults, filter exactness and extension replacement/boundedness/order/partition/statistics/metrics. Resolve paired kernel schema/expression edits through an actual supported consumer, not private code copies.

**Exit:** unit truth tables, pure admission/plan checks and DC01–DC05/DC08/DC10–DC12 isolated forms. Author real mutation, metadata, refresh and concurrency tests for CP12. **Deletion:** L03/L14/L17/L19/L20/L26–L32.

### CP03 — Control, writer publication, exact effects and physical owners

1. Route every control/contracts/evidence/result/HTTP/history write through returned-state publication. Refresh mutable heads before command admission; preserve common-predecessor transaction keys and recompute commands after conflict. Handle post-commit maintenance failure separately from a definitely uncommitted write.
2. Complete native request/specification/interest/shutdown/claim/eligibility decisions and all scoped references. No new cache map becomes a policy engine or an alternative durable job/claim journal.
3. Bind authorization to argv, environment/image, input bytes/modes/inventory, outputs, network and budgets together. Qualification cannot become a caller-controlled bypass. Matching a supplied digest alone does not establish scope.
4. Finish derived environments, per-request warm-LSP grants, methods/documents/positions, revocation and physical exit. Reconcile lost acknowledgement, duplicate requests, expiry and boot/start identity before reuse.
5. Keep owned native execution, paid permits and actual task lifetime through registry fill, contract verification, cache materialization, writers and external/uninstrumented children. Terminal job state is not cleanup. Register candidate/stage/result obligations before bytes can become unreachable.
6. Remove direct-driver/ownership-JSON fixtures and bypasses; preserve only process-group/sandbox/framing/lock mechanics needed by target drivers.

**Exit:** isolated native decisions/exact authorization and ownership/cancellation units, including capacity-one cache misses and last waiter drop. Independent-process races, crash and execution-profile proof remain Q03/Q05/Q09/Q11 in CP12.

### CP04 — Complete existing operation materialization

1. Preserve the four current families, pure CacheFactory preparation, shared owned initialization, native spill/readers, terminal error/cancellation behavior and planner composition. Do not reimplement completed foundations.
2. Replace repeated option-vector construction with CP01's immutable session witness. Preserve exact operation pointer, input plan/version/cohort/contracts and transform/function/policy identity, including views and mutable/volatile source refusal.
3. Close rewrite/property and physical lifetime edges: consumer filter/projection/limit cannot poison the base; exact metadata/order/partition/statistics claims remain truthful; failed fills never become ready. Reservations, leases, deadline and paid permits survive actual physical users.
4. Complete four consumer count/page/summary fixtures. Native aggregates use the same handle; no count side channel or hidden second materialization. Keep native spill as the complete route. A reservation-backed Arrow memory tier is a measured CP12 physical option using the same contract, not a new global cache.

**Exit:** remaining CF01–CF08 isolated cases and workload fixtures authored; integrated CF06/09/10 remain CP12. **Deletion:** L25 and all residual alternate materialization consumers/fixtures.

### CP05 — Acquisition, producers and environment consumers

1. Finish typed source/registry/revision/freshness/receipt relations and native preference/reachability/fallback/archive/header policy. HTTP cache tables consume the snapshot owner, but cached bytes are not semantic freshness authority.
2. Apply exact environment/effect contracts to hosted/fallback rustdoc, static workers, Cargo/uv, rust-analyzer, ty and runtime verification. Bind lock/installed closure and producer/operation through preparation, readiness, grants and result usability.
3. Complete raw LSP/runtime-object facts and native scope/diagnostic/position/outcome/artifact lowering. Remove procedural semantic JSON transcript interpretation and ad hoc observation sorting/merging.
4. Finish independent Rust/Python definition/binding/visibility/reexport/inheritance/overload/base/external-target/source-span/epistemic fidelity, including `.py` and `.pyi`. Keep format-61 facts and aligned renderer. Complete ordered callable/qualifier List<Struct> aggregation and preserve source-rendered signatures without display-text inference.
5. Complete cycle/depth/work/parser/IPC/large-input bounds, malformed/truncated/worker-exit outcomes, partial coverage and physically owned staging cleanup with accounting before large allocations.

**Exit:** fact/normalization/callable/bounds units and scoped worker lint/types. Real workers→Delta→MCP, restart and export remain CP12. **Deletion:** L04–L07/L21/L23.

### CP06 — Native research, complete delivery and MCP consumers

1. Finish overview/inspection/ambiguity/aspect/evidence-window/recovery plans, exact rank/explanation factors and deterministic ties/kind/scope. Finish typed before/after callable/field differences, environment conflicts and exact coverage without coupled arrays, first-winner semantics or JSON equality.
2. Reuse CP04 handles and bind every cursor/section/request to exact input, contract, policy/transform and codec witnesses. Caches do not decide evidence availability or turn an empty result into a capability-absence claim.
3. Complete handler page/result/encoded-fit selection using native plans over full retained results, preserving the already implemented exact tool/resource/stdio framing profile and native byte UDF. Move `ops/artifact.rs::read_blocking` prefix choice into a native selection path with a narrow format/UTF-8 kernel where needed. Preserve forward progress without materializing every prefix into quadratic-size candidate values.
4. Finish artifact receipt/section/window/reference admission, export/recovery closure and result validation before terminal success. No arbitrary paths, prefix authority, synthetic recovery objects or semantic trimming. Native plans choose the result; sinks mechanically encode it.
5. Finish all nine tools and five resources with strict generated wire, actual installed SDK serialization and current product guidance. Keep local transport/protocol failures distinct from native evidence-budget claims. Update cache status schema from the same native declaration; no Python policy copy.

**Exit:** selection/difference/count/window/byte-bound units and schema/adapter static checks. Q06/SC07/SC09 actual-route qualification remains CP12. **Deletion:** L08–L11/L13/L21–L23/L25/L30.

### CP07 — Exact retention, maintenance and cache invalidation

1. Finish exact enrollment for definition/result/artifact/export/read-only/CDF/replay/query/cache/warm owners before provider/data opening, including optimized-away scans, failed fills and last readers. Resolve read-only bundles with a service-owned external lease authority or equivalent explicit immutable-root protection contract; do not mutate an exported bundle's contents/digest. A global root lock alone does not fulfill the exact-version target.
2. Complete data/log/checkpoint/CDF/replay/transaction-marker horizons and persist effective maintenance witnesses. Feed exact protected versions into native keep-version/checkpoint/log-compaction/VACUUM operations. Checkpoint interval policy cannot relax any horizon.
3. Implement explicit publication/result root removal and complete native dependency closure, orphan/unselected cohort/stage/export-candidate selection and storage-release obligations. Never count failed deletion as freed bytes or free a live physical owner.
4. Fence new reads with maintenance/root generation. Invalidate registry heads/versions/provider ingredients/contracts when their authority is removed; remove file metadata only for actually released canonical paths. Reconcile an interrupted deletion/invalidation sequence on restart, including stale cache entries and unknown commits.
5. Preserve admitted old readers through their exact leases; neither a current head update nor cache eviction ends them. Refuse newly requested below-horizon/removed versions even if a full snapshot remains in RAM. Cached state cannot mask missing CDF/log reconstruction.
6. Finish reader/writer/replay/CDF fencing and unknown physical-task reconciliation. Replace operator cleanup/reset inventories with native-selected ownership. Keep ongoing reclamation separate from CP11 one-time legacy retirement.
7. Author retention/fault/storage durability harnesses. Required filesystem power-loss proof is not replaced with a process-kill test.

**Exit:** native closure/fence/invalidation selection and isolated ownership units. Actual retention/VACUUM/export/crash/power-loss journeys remain Q07–Q11/DC07/DC09 in CP12. **Deletion:** L01/L02/L04/L12/L26/L31.

### CP08 — CDF, durable descriptors and fresh-process replay

1. Wire CDF to CP02's shared metadata owner through §4.5; preserve explicit inclusive version bounds, timestamps, required JSON/history checks, unsupported mapping/feature refusal, DV handling and native row filters. Cache reuse cannot change these semantics.
2. Finish native rebuild/incremental eligibility, pre/postimages, mutation/cohort vectors, unpublished candidates and no-op OPTIMIZE. Select output/checkpoint offsets atomically; missing history/schema/contract means rebuild or refusal, never empty success.
3. Implement the actual persisted descriptor format and restricted DeltaLogicalCodec consumer (§4.6). Complete size/depth/identity/codec/admission negatives and one binary format. Cache deserialization is bounded external input, not trusted runtime restoration.
4. Complete fresh-process typed-command replay: exact dependencies/output descriptors, current plans and newly bound runtime/store/lease/effect handles. Expired app transaction markers do not remove durable command/result reconciliation. Revoked effects never revive from receipts.
5. Finish output/control failure obligations, unknown/duplicate commits and contract-change-driven CDF/replay/provider invalidation for domain/variant/requiredness/mapping/canonical/wire/callable/function/policy changes, including equal physical shapes.

**Exit:** pure descriptor/codec/witness units and reader-factory construction checks. Actual CDF, persisted-descriptor reload and fresh-process service journeys remain Q07/Q08/Q09/DC06/DC09 in CP12. **Deletion:** L09/L12/L14/L27–L29.

### CP09 — Accounting, observations and lifecycle closure

1. Generate typed observations for cold loads, incremental refresh/no-change, shared waiters, positive contract/provider hits, validation failures, invalidation reason, occupancy and live reservations. Extend existing operation materialization and builder/kernel lineage by attempt/grant/job/publication/maintenance identity.
2. Distinguish resident accounted bytes, evicted-but-live owned bytes, in-flight decode/replay/writer allocations and measured external memory. Publish no false exactness for kernel estimates or cache value-size methods. Account shared snapshot allocations once; account cloned metadata and keys where they actually live.
3. Complete preallocation/external bounds for parsers, IPC, canonical kernels, log replay/materialization, metadata/page indexes, predicate caches, writer buffers/uploads, transport and child processes. Bound decode before accepting a persistent descriptor. Do not retain the current encoded-bytes-times-eight heuristic as proof of full external accounting.
4. Keep status and component/recovery policy native and bounded. Routine status does not enumerate/copy the cache. Do not emit a cache-history write that recursively emits another cache-history event. Atomics remain mechanical counters/sequencing, not durable truth.
5. Finish queue saturation/fault/drop/conflict/unknown append/restart/concurrent-close paths, explicit loss and joined physical/release/diagnostic drain. Cache clears and zero counters must correspond to actual memory/task ownership, not just removed map entries.
6. Author full process/LSP/writer/cache pressure and shutdown cases while running only isolated lifecycle units before CP11. Observe metadata/predicate/kernel allocations separately from Arrow pool totals.

**Exit:** bounded typed observation/accounting/saturation/lifetime units. Full Q05/Q09/Q11, CF05/09 and DC10–DC12 remain CP12. **Deletion:** L10/L12/L14/L32/L33.

### CP10 — Complete source/package/enforcement deletion

1. Close L01–L33 across normal/error/recovery code, tests/fixtures, imports/dependencies/features, generated output, configuration, scripts, packages and product skill. Replace stale typed fixtures instead of restoring old APIs or suppressing checks.
2. Add focused structural rules for direct Delta cold loads outside the registry, contract bypass, hot-provider serialization, local private CDF cache construction, missing session binding and non-native result/materialization alternatives. Rules need positive/negative fixtures and a declared fixture/serialization-boundary scope; broad bans on every `DefaultCache::new` or `serde_json` would reject legitimate owners and transport code.
3. Reconcile DESIGN/ADRs/dispositions/plan/status/setup/launch docs and native ownership inventory. Apply the protected enforcement update through the permitted operator boundary and record actual installed disposition; a prepared patch alone is not enforcement.
4. Finish target harnesses and Q/SC/CF/DC selectors. Confirm no unexplained semantic owner, alternate cache route, legacy package or executable compatibility copy remains.

**Exit:** CP00–CP09 source complete; L source/package closure; affected unit/static/schema/compile checks and target harness compilation pass. Installed closure remains CP11; terminal acceptance remains CP12/CP13.

### CP11 — Matching candidate and the deletion barrier

1. Implement the missing Q recipes listed in §6 with explicit unit versus integration selectors and source-bound receipt capture. Build locked matching daemon, native worker/executor and non-editable Python package with consistent source/config/schema identity. Inspect package contents without service journeys.
2. Inventory actual retired service-owned processes, installations, state, artifacts, caches and client registrations. Revalidate exact paths/ownership/dependencies; old plan paths are leads, never blind deletion commands. Preserve unrelated repositories, shared Cargo/uv caches and frozen/review evidence.
3. Stop/drain/reconcile legacy processes and apply only the verified retirement manifest. Remove retired state, packages, launch/registration aliases and service-owned legacy caches. No executable backup or old-state importer. Record failures precisely and continue independent work where possible.
4. Record the barrier receipt: CP00–CP10 implementation complete; source/package/enforcement removal evidenced; matching candidate built; actual retired installations/state/registrations removed; no old processes. A dry run, path list, unapplied required patch or compile pass cannot satisfy this barrier.

**Exit:** barrier closed. This is build/deletion evidence, not integrated candidate qualification. Only now may CP12 run.

### CP12 — Final qualification and physical choices

1. Run complete Q01–Q12, SC01–SC10, CF01–CF10 and DC01–DC12 against current source with independent expected results. Reproduce and close the documented integration failures using the matching worker. Repair target code, regenerate/rebuild and repeat the affected final-source oracles.
2. Qualify all tools/resources, Rust/Python static and contained execution, valid/invalid snippets, offline/cold comparison, retained results, export/readback/restart, Codex/Claude and required Linux profiles from unrelated directories on fresh dedicated state. Include C20 and real-XDG leak checks. Missing prerequisites are named blocks, never mocked passes.
3. Run independent-process writers/readers, foreign commits, unknown acknowledgements, retention/VACUUM races, conditional-create/commit/file-directory synchronization and required filesystem power-loss proof. A cache hit never substitutes for durable reload in that oracle.
4. Measure complete journeys with source/workload/configuration identities: cold/warm control capture, evidence/provider reuse, CDF publication, restart, export, research and execution. Record p50/p95/p99, log/commit/file counts, checkpoint state, metadata requests/bytes, cache loads/refreshes/hits, pool/live/external peaks and spill/growth/cleanup. No speedup claim without a comparable baseline; review debug-fixture ratios are not product performance.
5. Qualify initial snapshot/contract/provider/predicate limits and checkpoint intervals. Compare supported target settings, including cache disabled versus enabled where possible; never restore legacy production code. No requirement that cache hits be independent of file count until all native validation work is measured.
6. Finish inherited file/row-group/partition/statistics/compaction/clustering/Z-order/dictionary/view/filter/IPC choices. Measure Delta log stats, DF stats, kernel pruning and Parquet I/O separately, including dotted statistics, heavy-column projection, strict missing files and DV/COUNT equivalence. Evaluate the optional owned-memory materialization tier under the existing lifecycle.
7. Publish adopted and unused dispositions and negative measurements. Regenerate qualification for final repaired source and recheck deletions when packages/registration change.

**Exit:** mandatory preactivation evidence qualifies one current target candidate. Full performance evaluation is terminal work, not an architecture-implementation loop.

### CP13 — Fresh activation and completion

1. Activate only the qualified matching target with fresh paired state and generated current registrations. No old state or rollback executable returns.
2. Verify deployed source/build/config/process/client identities, actual smoke and no unexplained jobs/children/leases/reservations; recheck installation removal scope.
3. Run Q13 aggregate, source-bound acceptance generation/check and final G1–G7 design review. Update STATUS, this ledger/index, capability dispositions and setup guidance from evidence.
4. Close only when implementation, deletion, required Q/SC/CF/DC, durability, actual clients/profiles and activation are complete. Required `failed`, `blocked` or `not_run` work stays open; capability or compile evidence cannot close it.

## 6. Verification and complete traceability

### 6.1 Existing terminal oracles retained

The full definitions in [Plan 18 §5–§6](18-schema-governed-runtime-and-cache-completion.md#5-cache-specific-regression-obligations) and [Plan 17 §6.1](17-schema-governed-unified-runtime-hard-pivot.md#61-schema-oracles-within-the-existing-acceptance-gates) remain binding. New DC oracles below extend their owners, not the product acceptance-ID registry. All complete final-target matrices remain open; this planning pass ran none.

| Q recipe | Complete obligation, including this plan's changes |
|---|---|
| Q01 `native-contracts-check` | One real declaration/field/aspect/reference/policy addition through all consumers; corrected warm contract check, generated cache/load/status contracts and planner composition |
| Q02 `delta-mutation-check` | Actual nested/tag/CHECK/null, feature/metadata/global-key/merge routes; effect-free refusal, semantic operators/extensions and correct post-commit cache state |
| Q03 `delta-control-check` | Independent-process claim/head/interest/renewal/reservation races, duplicate/stale ownership, unknown acknowledgements and monotone registry publication |
| Q04 `native-evidence-check` | Complete Arrow/value/identity/deep producer fidelity, partial/bounded facts and exact dependency witnesses |
| Q05 `native-effect-check` | Pure planning, exact grant/dispatch/revocation, physical child/writer/warm-resource lifetime and owned operation/registry fills |
| Q06 `native-research-check` | Factors/scope/aspect/callables/counts/pages/windows/results, exact bytes/references and retained completion through actual MCP |
| Q07 `delta-incremental-check` | Full/incremental mutation/cohort/schema/rule/history equivalence, CDF cache route, unpublished candidates/OPTIMIZE and atomic output/offset faults |
| Q08 `native-replay-check` | Fresh-process typed commands and restricted binary provider descriptors, current authority, changed witnesses and forbidden codec/node negatives |
| Q09 `delta-retention-check` | All query/result/cache/CDF/replay/read-only/export protection, enrollment/maintenance races, marker expiry, reconstruction, explicit root removal and real reclamation |
| Q10 `delta-storage-check` | Backend identity/conditional create, commit/storage faults, file/directory synchronization and filesystem power-loss proof |
| Q11 `native-resources-check` | Native/external pressure and physical ownership, strict files/DV/statistics, bounded telemetry and complete journey cache/layout measurements |
| Q12 `native-removal-check` | L01–L33 source/package/fixtures/config/rules and actual installation/process/state/registration closure |
| Q13 `unified-runtime-qualify` | Final-source quality, Q01–Q12, actual nine tools/resources/clients/profiles/offline/export and deployed smoke |

Schema sub-oracles remain: **SC01** one declaration; **SC02** exact values/identities; **SC03** real mutation requiredness; **SC04** semantic planning before coercion and through real sessions; **SC05** exact nested fields/no silent filling; **SC06** scoped references/collections; **SC07** actual generated MCP value/frame fidelity; **SC08** each semantic witness independently invalidates; **SC09** real callable observations; **SC10** separately measured layouts/pruning/projection. No direct rule call replaces SC04 ordering, predicate unit replaces SC03 builder rejection, or generated schema replaces SC07 actual transport.

Existing cache sub-oracles remain: **CF01** admission/session; **CF02** preparation purity; **CF03** shared fill; **CF04** transformations/properties; **CF05** physical lifetime; **CF06** four consumers; **CF07** dependency invalidation; **CF08** cache/replay boundaries; **CF09** bounded observations/whole cost; **CF10** deletion/composition. Update CF07/08 for typed provider values and separate persisted descriptors, retaining every previous authority/refusal case.

### 6.2 Additional cache oracles

Each oracle has an isolated pre-barrier part where feasible and a terminal application part. Author all parts while implementing; do not use the word “unit” to run the deferred journey early.

| ID | Required result | Pre-barrier selection | Terminal owner |
|---|---|---|---|
| DC01 contract correctness | Equal manifests pass; semantic-only differences fail; register/reopen/provider/control pin cannot hide F0 behind a cached positive | Pure Arrow diff/violation-shape fixtures | CP02; Q01/Q02 |
| DC02 resident reuse | Concurrent misses coalesce, active slot eviction cannot duplicate owned fill, current/exact/provider users share snapshot allocation; one cold load per resident key/generation | Coordination and Arc/accounting units; isolated native snapshot API characterization | CP02/03; Q03/Q11 |
| DC03 freshness/publication | Foreign commits observed; delayed old writer never regresses head; conflict refreshes/recomputes; unknown acknowledgement reconciles; failed update leaves old value but fails current request | State-transition/owned-task units | CP02/03; Q03/Q05 |
| DC04 Delta equivalence | Incremental versus cold exact load matches version, protocol, metadata, full active-file/stat/DV inventory and selected rows; same-version checkpoint and lazy/full separation correct | Isolated library fixtures where no service journey; load-class negatives | CP02/07; Q02/Q07/Q09 |
| DC05 positive contracts | One verification per resident full key; simultaneous misses coalesce; negatives are not cached; failed/unacknowledged registration not published; recreation and compiler change invalidate | In-memory cache/identity/manifest tests | CP02; Q01/Q03/SC08 |
| DC06 actual CDF sharing | Add/remove/CDC and DV routes use bound shared reader; CDF then normal scan reuses a compatible physical-file entry; same relative filename in two roots cannot alias | Reader/path construction and synthetic key/metadata tests | CP08; Q07/Q11 |
| DC07 physical invalidation | Exact canonical deleted paths disappear; live admitted readers stay protected; concurrent refill cannot resurrect revoked generation; failed deletion does not report release | Invalidation/fence/metadata-cache units | CP07; Q09/Q10 |
| DC08 typed provider path | Hit performs no serialization/replay/copy of active-file inventory; fresh authority/store attached; wrong root/table/version/cohort/contract/config denied; eviction preserves live allocation | Provider/key/schema/ownership units | CP02/08; Q08/CF07/CF08 |
| DC09 durable boundary | Binary descriptor exact round trip and bounds; fresh process rebinds current authority; missing history/revoked scope/old codec/unsupported node refuse | Pure binary codec and malformed-input units | CP08; Q08/Q09 |
| DC10 resource/observation truth | Disabled families absent; counters bounded; accounted versus live/external memory distinguished; shared allocation not double charged; pressure/resize/clear and active-reader eviction safe | DefaultCache/accounting/status/schema units, deterministic time source if needed | CP09; Q11/CF05/CF09 |
| DC11 read/write policy | Declared checkpoint property applied by class; write path remains full-file capable; predicate cap reaches actual readers; cache-enabled/disabled results, pruning and missing-file behavior equivalent | Config/session/provider construction units | CP02/07/12; Q02/Q10/Q11/SC10 |
| DC12 ownership/setup reuse | Session witness built from final immutable options; changes miss correctly; engine reuse preserves each call's operation/effect/subscriber/physical tracker; capacity-one and shutdown do not deadlock or leak | Scope/witness/executor units | CP03/04/09; Q05/Q11 |

Record commands, exact source/build/pins, test selection, profile, limits, inputs/log digests and truthful `passed`, `failed`, `blocked` or `not_run` for every receipt. Timing-only comparisons are not semantic equivalence proof. A cache hit must have an independently correct result, not only a counter increment.

### 6.3 Required qualification scenarios

- Mutable control head: cold load, many protected reads, local commits, foreign commits, same-version checkpoint, conflict and unknown commit; report commit/checkpoint count alongside latency.
- Exact evidence: current and old version under protection, two cohorts/contracts at the same physical shape, eviction with a reader, explicit root removal/recreation and below-floor requests.
- CDF: add/remove/CDC/DV paths, inclusive boundary, unavailable log, ordinary-scan warm reuse, two roots with equal relative names and matching size/time, corrupt/missing file with warm metadata.
- Restart/replay: cold process with checkpointed tables, persisted descriptor read, wrong incarnation/witness/codec, revoked effect and interrupted output/offset acknowledgement.
- Resource load: concurrent cached/uncached operations, small fault budgets, workstation budgets, queue saturation, cancellation/shutdown with native and external children still active, no recursive telemetry.

These scenarios supplement all inherited producer/research/storage/client functionality; they do not replace it.

## 7. Deletion ledger

Each row includes production/error/recovery paths, fixtures, imports/features, generated/package/config outputs and applicable installed consumers. Existing deletions are credited; rows identify remaining closure rather than claiming the old code still exists.

| ID | Remaining obligation | Owner |
|---|---|---|
| L01 | Sole native publication/retention/log/maintenance/cleanup authority | CP03/07/10 |
| L02 | Residual caller/claim/physical-owner policy and ownership-JSON fixtures/readers | CP03/10 |
| L03 | Native mutation/admission/resource routes; only bounded writer mechanics remain | CP02/05/09 |
| L04 | Acquisition/environment/result merge and dependency/orphan/publication closure | CP03/05/06/07 |
| L05 | Producer/LSP native lowering/fidelity; no historical internal-format adapter | CP05/10 |
| L06 | Remaining semantic JSON/generic-row bridges and duplicated inventories | CP01/05 |
| L07 | Complete native source/registry/revision/freshness/environment authority | CP05 |
| L08 | Handler score/kind/scope/token duplication and actual tool semantics | CP06/12 |
| L09 | Residual identity/digest/dependency policy; no semantic JSON hashes | CP01/06/08 |
| L10 | Exact effects/readiness/handler/physical policy under one native owner | CP03/05/06/09 |
| L11 | Handler page/result/recovery composition and semantic trimming | CP06/10 |
| L12 | Duplicate protection/replay/maintenance/telemetry/resource authority | CP07/08/09 |
| L13 | Generated wire/fixtures/packages/config/skill and actual client fidelity | CP01/06/10–13 |
| L14 | Unused imports/features/dependencies/scripts/rules and alternate executables | CP02/08/10/11 |
| L15 | Verified stop/drain and retired install/state/registration removal before integration | CP11/13 |
| L16 | All-family tagged/presence declaration coverage and real extension oracle | CP01/12 |
| L17 | Duplicated vocabulary/reference/field policy and presentation-role dispatch | CP01/02/10 |
| L18 | Remaining binary ID/digest/unit inventory; retain true external text | CP01/12 |
| L19 | Semantic coercion/NULL-fill bypasses in mutation/reopen/operators | CP02/12 |
| L20 | Control/result/retention scoped references and wrong-scope cases | CP01/02/03/07 |
| L21 | Coupled arrays/zips/known flags; correct order and absence | CP01/05/06 |
| L22 | Mechanical row boundary for every resource/export/MCP route | CP01/06 |
| L23 | Callable before/after semantics and independent real-tool fidelity | CP05/06/12 |
| L24 | Current epoch/fixture/rule/package/install audit; no legacy runtime backup | CP10/11/13 |
| L25 | Residual eager operation index/count channel/custom IPC and alternate consumers | CP04/06/10 |
| L26 — new | Unowned per-consumer cold Delta load/replay and discarded writer return state | CP02/03/07/10 |
| L27 — new | Per-provider contract replay/lookup and duplicated immutable semantic preparation | CP02/10 |
| L28 — new | In-process provider JSON encode/decode, duplicate file inventory and TTL-driven immutable reuse | CP02/08/10 |
| L29 — new | Private CDF metadata cache, relative shared-cache keys and unbound reader construction | CP02/08/10 |
| L30 — new | Unconsumed statistics/listing cache config/status claims and generated variants | CP02/06/09/10 |
| L31 — new | Cache validity based only on path/name/TTL without incarnation/retention/physical deletion closure | CP02/07/08 |
| L32 — new | Repeated effective-option sorting and operation-neutral engine construction at each consumer | CP01/02/04/09 |
| L33 — new | Old persisted provider codec, duplicate cache policy/settings, unbounded cache-status enumeration and obsolete cache fixtures | CP08/09/10/11 |

Physical-option comparisons use supported target configurations and recorded evidence, never retained executable legacy code. No broad automated cleanup of the dirty tree is authorized.

## 8. Requirement and route coverage

### 8.1 Inherited packages and findings

| Original owner | Remaining owner in this plan |
|---|---|
| FP00 authority; FP15 design/deletion | CP00/CP10, installed removal CP11 |
| FP01 declarations; FP02 typed identity/storage; FP04 wire | CP01/CP02/CP06/CP08 |
| FP03 semantic plans; FP05 provider/mutation | CP02/CP04, full mutation proof CP12 |
| FP06 control; FP08 effects | CP03/CP05/CP07 |
| FP07 producers | CP05 |
| FP09 research; FP10 results/MCP | CP04/CP06 |
| FP11 CDF/replay; FP12 retention/durability | CP07/CP08/CP12 |
| FP13 resources; FP14 physical choices | CP09/CP12 |
| FP16 candidate/retirement/qualification/activation | CP11/CP12/CP13, in that order |

Preserve Plan 18 §9's complete **F01–F14, DFU-01–DFU-08, S01–S12, E01–E09 and prior CacheFactory CF-F1–CF-F6** mappings. The new review's F0–F8 map in §3.1, and every additional CX01–CX10 has an owner and oracle in §3.3/§6.2. No new cache route excuses missing producer, semantic-schema, result, replay or retention scope.

Preserve all [Plan 17 §7.2 integration dispositions](17-schema-governed-unified-runtime-hard-pivot.md#72-complete-integration-dispositions): public Delta builders supply private mapping/validation/merge/metric nodes; no private algorithm copies. Unity/remote/experimental stores stay unconfigured without a real deployment. The qualified local conditional/synchronizing store remains mandatory; ConditionalPutShim remains excluded. No cross-process DataFusion FFI, general schema DSL, Variant/Union persistence or historical migration layer is introduced.

### 8.2 Public functionality

| Route | Completion owner and cache effect |
|---|---|
| `resolve_library` | CP03/05/06; source/HTTP/control tables use snapshot owner, native freshness/effects/results preserved |
| `library_overview` | CP04/06; shared namespaces/children and exact provider reuse |
| `search_evidence` | CP04/06; shared score/key relation and native count/page/explanations |
| `inspect_symbol` | CP05/06; exact/ambiguous/aspect/window and execution evidence, freshly protected providers |
| `compare_releases` | CP04/05/06; typed differences, side coverage and shared changed-key relation |
| `verify_usage` | CP03/05/06; exact environment/action grants and native outcome, never cached effect authorization |
| `read_artifact` | CP06/07; native receipt/section/window and exact bytes/cursors/caps |
| `job_control` | CP03/06/09; native interests/cancel/recovery and physical cleanup independent of cached head |
| `service_status` | CP03/06/09; native readiness/policy/history plus truthful bounded cache families |
| Five resources and export CLI | CP06/07/08; identical protected publication/result/codecs, portable closure and immutable bundle contract |

Caching adds no new MCP tool, raw SQL interface or caller-controlled cache administration endpoint.

## 9. Capability evidence and deliberately unselected options

### 9.1 Pinned source basis, inspected 2026-09-17

Use the [DataFusion skill](../../.claude/skills/datafusion/SKILL.md), [DeltaLake skill](../../.claude/skills/deltalake/SKILL.md) and exact sources. Context7 was not used for DataFusion, Arrow or Delta. The FastMCP boundary is retained from Plan 18; further clarification uses its pinned skill.

Pins remain DataFusion **55.1.0**, Arrow/Parquet **59.3.0**, object_store **0.13.2**, delta-rs **58f07cd62bfbce3649a7e1c87c696288068ae184**, buoyant kernel **8ba063f8f84fec222000f66d40d70911d7c79675**, Rust **1.98.1**, FastMCP **4.0.3**, MCP **2.2.0**, PyArrow **25.0.1**, rustdoc-types **0.61.0** and the recorded patched public-api **0.52.2** renderer. Reverify lock/vendor provenance when building the candidate; no upstream repin is part of this plan.

| Evidence | What it establishes and what it does not |
|---|---|
| [`datafusion_execution::cache::default_cache::DefaultCache`](../../.claude/skills/datafusion/content/api/datafusion_execution.cache.default_cache.md), [`Cache`](../../.claude/skills/datafusion/content/api/datafusion_execution.cache.md) | Native byte eviction, optional TTL, `memory_used`, removal/resizing and `TimeProvider`. `put` is not a compute-if-absent/single-flight primitive; `list_entries` clones a map; size accounting is not allocator proof |
| [`datafusion_execution::cache::cache_manager`](../../.claude/skills/datafusion/content/api/datafusion_execution.cache.cache_manager.md), [`CachedParquetFileReaderFactory`](../../.claude/skills/datafusion/content/api/datafusion_datasource_parquet.reader.md) | Custom concrete cache injection and shared readers; FileMetadataCache key is Path. Registry source `datafusion-execution-55.1.0/src/cache/cache_manager.rs:284` checks size/time only |
| [`DeltaTable::update_incremental` / `load_version`](../../vendor/delta-rs/crates/core/src/table/mod.rs), [`Snapshot::update`](../../vendor/delta-rs/crates/core/src/kernel/snapshot/mod.rs) | Forward-only update, explicit older-version load, seeded materialization and same-version checkpoint refresh. Does not supply application latest-head freshness or cache coordination |
| [`TableProviderBuilder` snapshot methods](../../vendor/delta-rs/crates/core/src/delta_datafusion/table_provider.rs) and internal [`SnapshotWrapper` / `DeltaScan::new` / `rebind_immutable`](../../vendor/delta-rs/crates/core/src/delta_datafusion/table_provider/next/mod.rs) | Public typed Arc snapshot inputs and current scan checks; no need to JSON-decode a provider on every in-process hit; the wrapper itself is not publicly importable |
| [`CdfLoadBuilder`](../../vendor/delta-rs/crates/core/src/operations/load_cdf.rs), [next scan](../../vendor/delta-rs/crates/core/src/delta_datafusion/table_provider/next/scan/mod.rs) | Private 1 MiB factory at construction, Session at build, relative CDF paths versus absolute next-scan paths; reader must be threaded through helper paths |
| [Snapshot serde](../../vendor/delta-rs/crates/core/src/kernel/snapshot/serde.rs) | Native materialized batches are already Arrow IPC; current Serde representation carries byte vectors. Does not prove any chosen binary serializer is compatible or bounded |
| [Post-commit transaction hooks](../../vendor/delta-rs/crates/core/src/kernel/transaction/mod.rs), [Delta table builder](../../vendor/delta-rs/crates/core/src/table/builder.rs) | Native checkpoint cadence and `without_files`/stats configuration; lazy writes skip checkpoint hook |
| [`QueryRuntime`](../../crates/enrichment-store/src/runtime.rs), [`DeltaStore`](../../crates/enrichment-store/src/native_delta.rs), [`ProviderCache`](../../crates/enrichment-store/src/provider_cache.rs), [`OwnedLogStore`](../../crates/enrichment-store/src/kernel_runtime.rs) | Current shared runtime/factory, fresh load per call, contract shape mismatch, serialized provider hot path and per-call engine construction |
| [Pinned config catalog](../../.claude/skills/datafusion/content/catalogs/config-options.md); Cargo registry `datafusion-datasource-parquet-55.1.0/src/opener/mod.rs:1490`, `parquet-59.3.0/src/arrow/arrow_reader/mod.rs:188,424` | Option forwarding and actual 100 MiB row-group predicate-cache default. Source-checked, not an executed pressure result |
| Kernel checkout `8ba063f/kernel/src/snapshot/mod.rs:317–339` under the pinned Cargo git source; Delta [snapshot accessors](../../vendor/delta-rs/crates/core/src/kernel/snapshot/mod.rs) | Best-effort heap estimate excludes shared values; Delta wrapper exposes neither all private batches nor the kernel inner directly. A narrow size seam may be needed |

The review's A1–A4/C/D observations were measured on a tiny debug/local fixture; E/B were refused by F0. This planning pass inspected those receipts and sources and ran **no new runtime probe or product test**. New differential, namespace-collision, pressure and replay claims remain proposed oracles, not measured guarantees.

### 9.2 Conditional and unused capabilities

| Capability / simpler alternative | Disposition and concrete revisit trigger |
|---|---|
| Checkpoints alone, continued replay-per-call | Insufficient as final design: improves cold start but retains repeated replay/verification and no shared owner. Checkpoints are adopted alongside the registry |
| Persisted full Snapshot cache for daemon warm start | Defer. Required persisted command/provider descriptors still ship. Revisit when checkpointed cold-open p95 breaches the operation budget or representative tables exceed 100,000 active files |
| Custom data-byte/range cache or remote read coalescer | Do not add for local NVMe; first use shared metadata, retained snapshots and the OS page cache. Revisit when a configured remote-store workload shows range latency/bytes dominate |
| `delta-cache` / foyer | Do not enable dormant feature scaffolding. Revisit when the selected upstream pin has a real supported scan/log consumer and qualification evidence |
| Generic DataFusion statistics/listing caches | Disable for current Delta consumers. Revisit with an actual ListingTable/non-Delta source; keep native log statistics/pruning active |
| Wholesale DeltaSessionContext adoption | Not selected; existing template has the required custom analyzers/planners/UDFs/factory/runtime. Revisit only if it can demonstrably preserve the complete composition without duplication |
| Stats-free full snapshots | Do not use as the query/write default; qualify only an explicit metadata/maintenance consumer that does not require stats. `without_files` metadata-only reads are adopted separately |
| Predicate/page/Bloom/filter and footer-prefetch tuning | Keep native capabilities, explicit predicate limit and existing policy. Measure `metadata_size_hint`, page-index completeness and concurrent decoder effects at CP12 before changing physical defaults |
| Reservation-backed memory tier for operation cache | Evaluate at CP12 under the same owned lifecycle. Adopt only with representative benefit and full equivalence/pressure evidence; native spill remains the implemented route |
| Generic logical/physical plan serialization | Exclude unsupported logical hooks and DeltaPhysicalCodec. Persist typed commands and restricted immutable read descriptors; rebuild physical plans under current authority |
| Automatic cache prewarming or lease batching | Not selected: no evidence of benefit sufficient to add speculative I/O or change per-read durability. Revisit only after CP12 isolates a remaining bottleneck and proves retention/freshness equivalence |

CP00 records these dispositions and triggers in the living decision register; CP12 records final physical choices. None defers the mandatory registry, positive contract reuse, typed provider path, shared CDF cache, exact retention, binary replay consumer or full inherited pivot.

## 10. Completion ledger and immediate implementation start

**2026-09-17: planning complete; target changes not executed by this pass.** CP00–CP13 remain open. Use Plan 18 §10 for preserved implementation credit and this document for remaining work. At execution start capture a fresh task-relevant baseline, inspect concurrent changes and record exact source identities before editing.

First implementation slice:

1. CP00 decision/owner amendments and CP01 cache/load/session declarations.
2. CP02-A native contract shape fix with a decisive pure Arrow regression.
3. CP02-B shared metadata/policy construction, then CP02-C snapshot owner and CP03 post-commit consumers.
4. CP02-D positive contracts and CP02-E typed provider reuse; remove the replaced hot paths immediately.
5. Continue CP04–CP09 and deletions in dependency order, then CP10/CP11. Do not pause that architectural sequence for repeated integration runs.

Completion means implemented target architecture, actual legacy deletion, current-source and installed qualification, fresh activation and updated guidance. A compile pass, library capability map, cache-hit counter or completed document is not implementation completion.

## Outcome (recorded after implementation)

### What was built

Not yet recorded. This document is the combined implementation plan; the existing source checkpoint is §2.

### A mistake made and corrected

Implementation outcome pending. Planning corrections to the supplied review are recorded in §3.2 rather than silently propagated into the target.

### Deviations from the plan, deliberate

None executed in this planning pass. Record implementation changes with their decision and evidence links here when they occur.
