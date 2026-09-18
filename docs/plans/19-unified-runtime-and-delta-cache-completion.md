---
title: Complete the unified runtime with governed DataFusion and Delta caching
status: in-progress
date: 2026-09-17
adrs: [ADR-0041, ADR-0042, ADR-0044, ADR-0045, ADR-0047, ADR-0048, ADR-0049, ADR-0050, ADR-0051, ADR-0052, ADR-0053, ADR-0054, ADR-0055, ADR-0056]
phase: 6
---

# Complete the unified runtime with governed DataFusion and Delta caching

## 1. Purpose, scope and execution discipline

This plan combines:

1. The [DataFusion/Delta caching review](../design_review/reviews/design_review_datafusion-deltalake-caching_2026-09-17.md), including F0–F8, its recommendations, limitations and [recorded probes](../design_review/reviews/evidence/datafusion-deltalake-caching-2026-09-17/README.md).
2. Additional opportunities and necessary corrections established from the current implementation and pinned library sources in this planning pass (§3–§4).
3. **Every remaining obligation in [Plan 18](18-schema-governed-runtime-and-cache-completion.md)**, reconciled against its [execution ledger](18-schema-governed-runtime-and-cache-completion.md#10-execution-ledger-and-completion-rule).

**Use Plan 19 as the combined remaining sequence when implementation resumes.** CP00–CP13 retain their identifiers and complete inherited responsibilities. This plan changes their cache implementation, prerequisites and regression coverage; it does not create a separate cache project or discard the unfinished runtime pivot. Plan 18 preserves dated implementation receipts. Plan 17's complete FP00–FP16, Q01–Q13, SC01–SC10 and integration dispositions remain incorporated through Plan 18. L01–L25 and CF01–CF10 remain binding, with additions below. No requirement is waived by a shorter summary here.

**Implementation authorized 2026-09-17.** New architecture remains Proposed until implemented and verified; review timings remain historical fixture measurements. The execution baseline is `.dev-state/plan19/execution/baseline.json`. Work proceeds in the dependency order below, preserving the CP11 integration barrier.

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

The execution-start source identities were **state 11, snapshot 10.0, wire 5.0, native-json/3**. Current source uses **state 22, snapshot 12.0, wire 8.0, native-json/3** and the new `delta-immutable-cbor/1` persistence boundary. These are source identities, not evidence of installation. Incompatible state is refused; fresh activation has not run. Plan 18's opening state-10/wire-4 baseline is historical; its §10 is the current checkpoint.

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

**All CP packages remain open.** Plan 18's focused units, serializer probes and production compile/Clippy receipts retain their exact source and scope. They do not close complete Q/SC/CF matrices. The stale typed-ID, request-default and removed-resource-API fixtures have been ported and affected all-target harnesses compile. Their service journeys remain deferred; compilation does not qualify them.

Keep the unresolved evidence in Plan 17 §2.3 visible: navigation List metadata, execution/export artifact Struct metadata, Decimal diagnostics, nullable document kinds, result clock/window admission, diagnostic pressure/shutdown, and fixtures expecting retired ownership JSON. Their final matching-worker oracles belong to CP12. Do not claim that these old failures were reproduced in this planning pass.

**Resolved source prerequisite:** F0 now projects a declared single violation identity from `native_contract::changes` before the `QueryRuntime::require_empty` boundary. Focused equal/mismatched manifest units passed before positive-cache admission was wired. Actual register/reopen/provider/pin qualification remains CP12 (§5, CP02-A).

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

## 10. Completion ledger and next implementation boundary

**2026-09-18: implementation active; CP00–CP13 remain open.** This ledger credits source owners
and specific isolated receipts without closing the broader package or its final application oracle.
The task baseline is `.dev-state/plan19/execution/baseline.json`; shared pre-existing changes are
preserved. Plan 18 §10 retains its earlier receipts.

| Package | Implemented source in this execution | Remaining boundary |
|---|---|---|
| CP00 | Proposed ADR-0055; revised proposed ADR-0053; DESIGN and exact Ciborium compatibility receipts | Complete inherited owner/field/route/deletion inventory and reviewed authority changes; protected patch is prepared/check-only, actual installation remains open |
| CP01 | Shared cache declaration, full session witness, physical namespace identity, native Binary descriptor bytes and typed binary SHA-256 digests, shared bounded cursor codec and complete Arrow-container schema traversal, one intrinsic field compiler with independent vocabulary/contextual rules, encoded-container value/reference admission, set uniqueness and generated wire projection, schema-derived retained-result snapshot closure, metadata-preserving list expansion, explicit UTF-8/UTF-16 protocol coordinates, binary Job/Interest/Attempt identities and full-field parameters/collections, typed job/qualification clocks, shared native diagnostics; wire 8.0/state 22/snapshot 12.0/executor 7 | Remaining identity/clock/coordinate families, declarations/bounds and complete semantic/codec witnesses |
| CP02 | F0 native violation identity; concrete shared metadata cache; disabled unused families; explicit predicate/checkpoint policy; accounted incremental/exact snapshot registry with namespace-specific fill fences and same-version refresh adoption; positive contracts; typed provider ingredients; neutral engine reuse; pool-admitted oversized snapshots bypass both residency caches | Concrete collection/lambda/aggregate field admission, one shared DuckDB dialect, before/after analyzer checks and shared optimized/physical planner admission are implemented; the full operator/provider/mutation/feature matrix, complete physical-expression/resource scenarios and registry/constraint interruptions remain |
| CP03 | Control publication policy and complete-row unknown-acknowledgement reconciliation are shared; control/contracts/evidence/results/HTTP/history writer returns feed registry; control and evidence compaction share owned maintenance; staged document bytes receive native obligations before creation; one typed launch declaration supplies target environment, network, image and budgets to native admission and the physical launcher; warm semantic conversations derive documents/methods/positions from the current command and exact snapshot and retain effect dependencies; PreparedCapsule unifies resolved environment/lock/installed inventory, native command/claim/snapshot binding and finite overlays; runtime program/transport digests are verified; pending LSP I/O monitors fresh authority; static Python workers use retained native effects, exact input inventories, active revocation and owned child/output cleanup | Preparation/acquisition lineage and complete finite-producer/physical qualification, warm-LSP physical/revocation and unknown-commit/crash closure |
| CP04 | Four actual native materialization consumers have pure fixtures; sealed logical input, shared immutable paid ingress and session options/function witnesses | Remaining integrated property/lifetime and pressure matrix; no alternate materializer restored |
| CP05 | Document extraction runs on owned blocking lane; IPC input owner survives native readers and schedules durable cleanup; startup recovery selects native obligations after process reconciliation; fresh and retained semantic consumer construction, LSP response decoding and protocol positions are native UDFs; native source routing compares captured digest/size and bounds retained document closure; one source-capture plan owns file/document/media/role selection; native static-worker launch derives requests and exact job scope; Rust decoding has durable private input/fact ownership, a retained native decoder effect and generated request/receipt contract; document navigation selection and windows use native plans | Native execution-fact lowering/admission and prepared-inventory dependency selection are also implemented. Shared native producer-run composition/admission, Cargo source/identity/manifest/preparation verdicts and Python distribution/metadata selection now replace their procedural owners. Remaining acquisition/source lineage and fidelity, exact installed environment/effect consumers and physical worker qualification remain |
| CP06 | Native page count/sentinel/continuation; typed requested coverage and catalog-declared comparison scopes; native confounders/disposition/change warnings; exact Arrow JSON-size kernel and cumulative alternative-prefix selection; native artifact reachability; cursor runtime/policy witnesses; existing artifact UTF-8/escaped-byte prefix fitting; component readiness derives from declared requirements and the actual native execution-route decisions | Native aspect events, pinned full-text recovery, retained execution selection/ambiguity, inspection publication and initial/settled probe presentation now have shared owners. Native coverage request selection now enforces exact requested execution artifacts and producing bindings. Remaining route/consumer qualification and result/recovery branches, complete tool/resource and replay qualification remain |
| CP07 | Durable live-source and sealed immutable-bundle read vectors replace blanket Root protection; every immutable-provider entry checks scope; namespace/cache invalidation; durable private export-stage deletion; atomic native root-removal closure/cleanup obligations, catalog visibility and rooted read admission; operator preview/apply; shared OPTIMIZE and protected-version VACUUM/log cleanup; generic native row-key DELETE for cohorts/results/definitions, partial dependency settlement, pending-writer recovery, native artifact reclamation and standalone receipt roots; bounded incomplete-table removal; durable exact maintenance selections and historical-floor admission; shared private source/input/worker ownership with native multi-process cleanup refusal | Complete remaining private/orphan recovery and physical row/artifact reclamation qualification; historical admission/transaction horizons, complete immutable-root lifetime and maintenance/restart fences; all physical reclamation journeys |
| CP08 | Shared CDF reader/cache with canonical paths; binary CBOR/raw-IPC provider codec with framing/footer bounds; retained search descriptors, native bijection/digest admission, metadata-only current-history check and shared-state restoration | Complete fresh-process command/replay, CDF/unsupported-feature/unknown-commit and semantic invalidation matrices |
| CP09 | Shared reservations survive snapshot/provider eviction; bounded cache observations; owned descriptor encode/decode and native input cleanup; pre-admission release slots for leases/inputs/quarantines and refusal to close with live owners; snapshot reservations distinguish in-flight, live and resident values and survive native eviction through the last physical owner; selected research and complete RPC byte reservations through socket delivery; physically retained source readers and worker input/output ownership; parser address envelopes reserved before spawn through reap; constant-size file-metadata lookup observations | Full external allocation/reader/builder/predicate accounting, remaining per-family observations, fault/close and physical process matrix |
| CP10 | JSON provider hit path/old decoder/private CDF cache/unused families removed; old snapshot-path/staging recovery API and fixture deleted; stale typed fixtures ported; focused load-owner/typed-cache/required-session/shared-CDF-cache rules and fixtures; duplicate launch flags/environment and opaque ProcessBinding tuple removed; semantic dispatch no longer exposes arbitrary method/params | Remaining L01–L33 source/package/enforcement inventory, target harness coverage and actual runtime/install retirement |
| CP11 | Not entered | CP00–CP10 architecture complete, matching candidate, actual obsolete installations/state/client registrations removed and required enforcement installed |
| CP12 | Only isolated development checks/probes executed | Full Q/SC/CF/DC, storage/publication/CDF/export, real workers/clients, pressure/durability and installed qualification **not_run** |
| CP13 | No activation attempted | Fresh target state, deployed smoke and aggregate closure **not_run** |

### 10.1 Development receipts

All receipts are dated 2026-09-17 under `.dev-state/plan19/execution/`. A changed source after a
receipt requires the affected check before that receipt is promoted. No receipt below is a CP11
barrier or terminal application pass.

- `plan19-units.log`: 10 isolated store checks passed (2.31 s): metadata owner, coordination
  cancellation, session COW witness, positive contract identity, restricted upstream binary codec,
  captured exact read vector/root, F0 semantic manifest, native artifact prefix fitting, and native
  descriptor bijection/digest/portable-route admission and input-file refusal. This predates the
  additional pure recovery-selection test.
- `core-plan19-units.log`: native Binary Arrow/JSON/generated-schema exact-value unit passed.
- `all-target-check.log`: `cargo check --locked --workspace --tests` passed after fixture updates
  and document-input ownership changes. Test harnesses compiled; service journeys did not execute.
- `schemas-generate.log`: schema/DTO generation and conformance passed: 4 valid fixtures and 8
  rejected negatives. State/format/wire declarations are current; installed state is not updated.
- `upstream-verification.md`, `binary-codec-verification.md`: independent exact source/API
  verification and isolated serializer behavior; the latter separates 0.2.2 from newer examples.
- `delta-capability-scan.log`, `datafusion-capability-scan.log`: skill structural hints inspected.
  Collect/default-session/default-vacuum hits are library/test fixtures; production extension nodes
  implement the current `statistics_from_inputs` contract, which the older pattern does not match.
- `clippy.log`: production workspace Clippy passed with application warnings denied; vendor
  deprecations remain visible. `adr-lint.log`: 55 records/index/52 register rows passed.

### 10.2 Next execution sequence

Finish remaining CP01–CP09 consumers and exact removal/replay/lifetime contracts, with focused units
and compile/static checks. In particular, finish incomplete-writer/orphan/artifact/definition reclamation, the operator loop, historical
admission and restart fences; per-request environment/LSP/effect witnesses; residual handler result
composition; complete external-resource ownership; and command replay/unknown-ack reconciliation.
Then close L01–L33 source/package/enforcement and authored target harnesses in CP10. Build and
perform actual CP11 retirement before entering any integration, client, installed or full CI run.

Completion means implemented target architecture, actual legacy deletion, current-source and
installed qualification, fresh activation and updated guidance. A compile pass, library capability
map, cache-hit counter or completed document is not implementation completion.

## Outcome — implementation in progress

### What was built

The implementation above installs the shared native cache/snapshot owners, binary service replay,
exact protection, common compaction and native document-input cleanup, while retaining the full
unfinished runtime pivot. No complete package or final application qualification is claimed.

### A mistake made and corrected

A serialized provider embeds an absolute table root and physical incarnation. Putting it into a
portable export would invalidate it after the export's atomic rename or a later copy. Export
checkpoints now explicitly select exact native Delta reconstruction and forbid those descriptors;
service checkpoints require the full binary descriptor set. Native admission tests both routes.

The old startup cleanup still recognized deleted Parquet snapshot artifacts despite the document
producer writing Arrow IPC. It was replaced by native obligation selection and physically retained
input ownership; obsolete path APIs and that cleanup fixture were deleted.

### Deviations from the plan, deliberate

Portable exports use current Delta logs plus exact bindings rather than location-bound binary
provider payloads. This is an explicit `export_rebuild` contract, never a decoder fallback or
compatibility route. [ADR-0053](../adr/0053-immutable-delta-provider-rebinding.md) records why.
No final matrix, inherited scope or deletion obligation was waived.

### Additional source checkpoint — 2026-09-17

Publication/result removal now derives a bounded reverse dependency closure with native recursive
queries. Tombstones and cleanup obligations share one control commit. Newly captured catalog views
apply removal policy; a rooted enrollment checks current root authority and exact dependencies under
the same predecessor. Existing leases remain valid. `retire-roots ROOT... [--plan|--apply]` is an
operator-only route, with preview as its default; it has not been run against service state.

Retained results derive snapshot references from Arrow extension declarations, including nested
optional/list fields. Their native reference walker shares the full-field list/map projections; no
JSON traversal or new semantic-analyzer exception was introduced. Root-removal and schema-reference
units exposed metadata/coercion issues that were corrected in the native expression routes.

Shared maintenance now selects completed, unselected cohorts, executes native DELETE, VACUUM with
retained versions, checkpoint/log work, and acknowledges only completed dependencies. Other pending
resources remain owned. Physical file counts come only from acknowledged native deletion results.
The current claim conservatively retains its captured versions; release of completed cohort
dependencies can permit reclamation on a subsequent maintenance claim, subject to native horizons.
This source work does not close CP07, CP11 or any full storage/retention journey.


### Native row, writer, artifact and release ownership checkpoint — 2026-09-17

- Replaced the cohort-only dependency member and separate definition dependency with one optional
  `RowKey` on the exact `TableVersion`. The same native candidate selection, DELETE, retained-version
  VACUUM and dependency settlement path covers cohorts, result rows and definition rows. A different
  or whole-table live selector conservatively protects overlapping rows. No old dependency decoder.
- Evidence/search/result/definition writers declare `PendingRow` targets before writing. Startup and
  operator recovery resolve physically exited writers against current Delta identity/contract. Native
  set operations update only eligible obligations. Missing tables additionally require a bounded
  empty-directory observation; unknown abandoned files remain refused and owned.
- Standalone receipt retention creates roots in the receipt commit. Artifact visibility follows live
  root dependencies. The `reclaim-artifacts --apply` operator uses native candidate selection, an
  artifact maintenance generation and exclusive root ownership through physical deletion and fsync.
  Native settlement shares the same partial-dependency machinery as row cleanup. Neither operator
  route has been executed against service state before CP11.
- Lease, input-directory and output-quarantine owners reserve bounded release slots before admission.
  Saturation refuses new owners rather than losing already owed releases. Close joins descendants
  and refuses surviving owners before closing diagnostics/caches. Release failures remain durable.
- Shared schema-only IPC decoding checks framing, shape/depth/field/metadata bounds before Arrow's
  allocating conversion. Registry discovery and contract verification have one decoder; the
  encoded-size-times-two estimate has been removed. External allocation qualification remains open.
- Added structural rules for native Delta loads, typed cache contents, required sessions and shared
  CDF cache construction, with inline and separate-file fixture scope explicitly declared. These
  syntax checks complement API/consumer tests and do not claim import or type resolution.

Intermediate pure pending-writer fixtures exposed a field-metadata loss, an optimizer projection
ambiguity and unsupported nested scalar array comparison. Native declared record construction,
explicit projections and `array_except`/`arrays_overlap` resolved them; no analyzer weakening or
row-wise dependency evaluator was added. Consult current receipts in `.dev-state/plan19/execution/`.
All CP11–CP13 restrictions and the remaining full architecture scope above continue to apply.

### Exact launch and semantic conversation checkpoint — 2026-09-17

- Executor protocol 5 has one native `Launch` containing the immutable image, containment identity,
  complete target environment, network mode, resource limits, output bound and deadline. Native
  admission compares actual values with independently selected policy values. Container arguments
  consume that retained declaration; the helper clears inherited environment and installs its exact
  environment. Qualification compares the same launch contract and still admits only fixed probes.
- Native process-effect roots now depend on the exact process-operation definition row. The old
  image/containment-only `ProcessBinding` type/key and separate launch environment list are deleted.
- A private prepared conversation joins the live command to an exact snapshot/context/environment
  and the inspection-selected definition. An immutable DataFusion UDF derives source-language text,
  the exact anchor and requested finite methods. Retained-result lookup uses that same constructor.
  A second UDF produces explicitly typed UTF-8 bytes or UTF-16 code units; outbound JSON framing
  consumes the retained coordinates. The daemon's former consumer builder/coordinate encoder is gone.
- Conversation definitions retain the snapshot table vector and process-effect dependency. Physical
  document owners hold the exact lease. Every semantic method checks current ownership and native
  membership; the generic request function is private. Qualification cannot issue a conversation.
  Request IDs use checked arithmetic, and server-request replies recheck current authority.
- `plan19-units-retention-current.log`: 22 isolated store checks passed (2.60 s).
  `exact-launch-unit.log`: actual-value policy mutation unit passed (2.27 s).
  `semantic-scope-unit.log`: pure Arrow command/snapshot/symbol/method/position/budget selection passed
  (6.59 s). `semantic-position-unit.log`: typed encoding/character-boundary unit passed (0.03 s).
  `exact-process-identity-unit.log`: process preimage mutation unit passed (0.01 s).
  `semantic-launch-clippy.log`: production workspace libraries/binaries passed with warnings denied
  (33.80 s); the two pinned vendor Parquet deprecations remain visible.
- The launch probe initially hit the pinned `map_entries` metadata-loss panic. It now consumes the
  existing metadata-preserving native map projection plus DataFusion set comparison. The corrected
  unit passes. Skill scan hints apply only to the bounded, in-memory coordinate unit fixture.

This is not full process/effect closure: derived environment and lock/installed closure binding,
finite producer-program/output admission, raw LSP answer lowering, warm physical/revocation/unknown
acknowledgement cases and installed worker qualification remain open. No integration journey ran.

### Snapshot allocation and residency checkpoint — 2026-09-17

The registry's native reservation now travels in one shared snapshot owner through writer/replay
capture, full and metadata-only loads, provider construction and optimized-away scan protection.
Native cache eviction drops a separate residency token; it cannot zero the live allocation or
release a reader's reservation. Constant-size observations expose in-flight, live-value and
resident-value accounted bytes. Other cache families report this measurement as absent.

The per-entry limit is now a residency threshold. Oversized captured values must fit the native
memory pool and remain uncached; both snapshot and provider-ingredient caches bypass them. Head
identity/version tracking remains monotone, and head-generation exhaustion refuses instead of
wrapping. This does not establish a preallocation bound for every kernel/parser allocation; the
kernel heap figure remains a best-effort estimate, and these counters make no RSS claim.

`plan19-owned-runtime-units.log`: **25 isolated store checks passed** (10.84 s), including actual
DataFusion cache eviction, pool accounting through the last reader, uncached larger reservations,
failed expansion and the retained native scope/removal/replay fixtures. The source compiles through
schema generation; `schemas-owned-runtime.log` records regenerated DTO/schema conformance with
4 valid and 8 rejected negative fixtures. No complete Q/SC/CF/DC or service journey is promoted.

### Native producer and recovery checkpoint — 2026-09-17

Source owners now include:

- `job_recovery` joins durable command, claim/publication/transition, exact snapshot, attempt,
  result roles and static/producer-input closure. Command admission compares complete native
  argument preimages. Resolution, inspection, verification and comparison recovery share this
  native path; former JSON receipt recovery and daemon-side set/equality checks are deleted.
- Recovery and final delivery hold `CapturedResult` physical protection through immutable byte
  reads. Native canonical-value comparison checks the decoded retained record. A decoded cache
  value alone does not grant read authority.
- Verification captures typed process observations and input artifacts once. `probe_plan` selects
  outcome/evidence class and lowers the retained UsageProbe; recovery uses the same native payload
  constructor. Runtime-object inspection has one bounded external-format decoder UDF, followed by
  native outcome and exact selector admission. Diagnostic JSON transcripts remain raw evidence.
- `native_catalog::input/work` owns finite operation registration. Sixty-eight direct registration
  calls in nineteen store files were replaced, plus two unit catalog fixtures. Duplicate operation
  inputs and bound-catalog shadowing now refuse through one owner. Architecture wire-version checks
  read the authoritative Rust declaration rather than duplicating a version literal.
- New recovery enforcement refuses JSON decoders in recovery/publication functions; fixture tests
  passed (four allowed, three rejected). The complete architecture check passed after registration
  replacement. Neither result is terminal CP11 acceptance.
- `environment_plan` shares native consumer-target admission with retained child selection.
  Paired static digest/locator equivalence uses symmetric DataFusion EXCEPT; the former two-array
  projection/zip and daemon environment comparison are deleted.
- Executor protocol **6** retains a finite producer invocation. DataFusion derives argv, mode,
  registry access, required files and returned file/directory roots for preparation, probes,
  runtime inspection, language servers and rustdoc. Drivers supply typed parameters rather than
  arbitrary argv/output policy. Admission reconstructs and compares full operation values and
  joins the durable command; probe admission checks requested mode/ecosystem and actual snippet
  digest. Rustdoc admission checks opt-in, target, freshness and requested features/defaults.
  Qualification retains a separate private, exact fixed-probe scope; it cannot grant target work.

The higher-order producer SQL uses the pinned library's DuckDB parser dialect within the same
bound SessionState: GenericDialect parses `->` as JSON access. Struct-valued lambda parameters
use explicit `get_field`; predicates compare hex-encoded SHA-256 with inventory digest text.
These are narrow exact-pin corrections, not a separate session or a copied array engine.

Receipts: `plan19-native-pivot-units.log` records **31 pure store units passed** (15.01 s),
`native-registration-clippy.log` records production Clippy passed (5.93 s), and
`native-environment-clippy.log` passed (22.78 s). `plan19-finite-producer-units.log` records **34 pure store units passed** (15.61 s), and
`finite-producer-clippy.log` records production Clippy passed (11.09 s).
No integration, installed qualification, legacy installation retirement or activation ran.

Remaining: exact derived environment/lock/installed closure in process grants, raw acquisition/LSP
lowering, remaining handler composition, external allocation and physical-lifetime coverage,
complete inherited inventories/deletions, CP11 actual retirement and CP12/13 qualification/activation.
All packages remain open; these source changes do not erase their broader acceptance obligations.

### Native LSP response checkpoint — 2026-09-17

`native_lsp` now owns bounded external LSP response decoding as a typed immutable DataFusion UDF.
Protocol positions carry explicit UTF-8 or UTF-16 variants; a coordinate UDF validates and converts
retained source spans. It handles hover markup, diagnostics, Location/LocationLink, integer/range
bounds, null/unsupported replies and malformed partial results. `semantic_response_plan` owns
results/empty/unresolved/unsupported/incomplete policy, indexing and structural-scope limitations,
method position selection and evidence class. Daemon code retains physical source reads and protocol
transport. The old normalizer, Location parser and `lsp/document.rs` implementation are deleted.
Producer implementation identity uses the complete build-derived source closure plus semantic scope;
the manually selected source-file fingerprint list is deleted. Producer identity revision is 5.

`native-semantic-response-unit.log`: one pure native response/coordinate/outcome unit passed (2.30 s)
with the final finite response-kind refinement (current receipt: 2.29 s).
`native-lsp-clippy.log`: production Clippy passed (5.38 s). `native-semantic-response-rule.log`
records three allowed/three rejected structural fixtures. The complete architecture check passed
in `native-producer-response-architecture.log`; the focused capability scan returned no findings.
No service, container or integration journey ran. Source-closure selection/bounds, exact environment
and installed-input binding, all remaining CP inventories/harnesses and CP11–CP13 remain open.


### Prepared process, source closure and qualification harness checkpoint — 2026-09-17

At this prepared-process checkpoint, source was **state 13 / snapshot 11.0 / wire 6.0 / executor 7**. The new control epoch
reflects the shared PreparedCapsule declaration inside retained capsule records; executor 7 retains
that full declaration alongside the finite invocation. Neither source identity is an installed-state claim.

- PreparedCapsule owns the parent release/environment/context, resolved environment, dependency lock
  and installed inventory. Retained selection, execution and warm semantic evidence consume it.
  Native rules check identity preimages, pinned toolchain/target/image, feature/default/lock scope,
  captured lock bytes, exact current claim/request/snapshot context and actual installed input closure.
  Finite per-command overlays replace unrestricted post-preparation changes.
- The runtime-object helper moved to core. A native external-format encoder supplies the selection
  file and expected admission bytes. Native rules reject changed helper, selection or output limit.
- Native file-URI parsing and source routing select consumer/installed/external/refused locations.
  Captured source must match admitted inventory size/digest and native source closure bounds before
  it becomes an artifact span. Physical source reads now use the owned blocking lane. Relational
  aggregate/join plans replaced higher-order traversal of evidence records, preserving semantic-field
  enforcement instead of adding an analyzer bypass.
- LSP exchanges, notifications and readiness waits monitor fresh command authority during pending
  I/O. Responses are checked before acceptance; server-reply writes are checked before dispatch.
  Revocation drops protocol work and the manager awaits container cleanup. A failed graceful shutdown
  goes directly to physical cleanup. Warm sessions derive environment/lock from the retained process
  descriptor instead of accepting a separate caller-supplied scope.
- Real containment oracles moved to `execution/qualification_tests` behind a private test-only
  exact-contract constructor. They remain ignored **integration** tests. Assertions now read native
  active ownership and reservation state; orphan fixtures reserve native ownership before creation.
  The JSON ownership-file/capsule-reservation directory fixtures are deleted. Operator recipes and
  the R09 nextest identity select these names. The finite-producer API rule refuses a raw argv API.
- Native grant checks return a boxed future at their public boundary, keeping planner implementation
  types out of every process/LSP state machine without relaxing compiler limits or authorization.

Receipts in `.dev-state/plan19/execution/`: `prepared-capsule-units.log` covers the in-memory
prepared-input, runtime transport and command scope matrix (three passed, 3.38 s); `prepared-capsule-reuse-unit.log` passed
one pure retained-inventory test (2.43 s); `semantic-source-unit.log` passed one native source-route/
closure test (5.20 s). Production Clippy passed in `prepared-capsule-clippy.log` (41.01 s), before the
source-route addition. `prepared-source-architecture.log` and `qualification-recipe-ruff.log` passed;
`prepared-source-capability-scan.log` was empty. Later changes require affected follow-up checks.

No service, publication, container, installed or integration journey ran. All CP00–CP13 remain open;
these owners do not close acquisition lineage, complete family/route inventories, handler composition,
external allocation/lifetime accounting, full fault matrices, protected enforcement or CP11 retirement.


### Native research policy and continuation checkpoint — 2026-09-17

Source additions and deletions:

- `page_plan` owns page count, sentinel, detail and continuation decisions. Search, comparison and
  snapshot facets consume its boundary. Rust truncation and parallel selected-key arrays are removed.
- `coverage` decodes declared ScopeAssessment records directly. Native groups/aggregates derive
  completeness, indexed/missing kinds and limitations across every requested subject/producer.
  `Coverage::complete`, `refresh_kinds`, the parallel kind/state/witness/subject decoder and its
  obsolete QueryFamily variant are deleted. Acquisition, search, manifest, inspection and comparison
  use the native projection. Unknown is distinct from a declared gap; partial scopes cannot be hidden
  by another successful subject of the same evidence kind.
- `comparison_scopes` declares the scope/evidence mapping once in the immutable native catalog.
  `comparison_policy` owns default/selected scope sets, package compatibility, coverage, normalization
  and artifact-variant confounders, API completeness and partial disposition. Native change composition
  adds incomplete-scope warnings. Handler match tables, set unions and interpretation edits are deleted.
- `comparison_page` selects one ordered alternative prefix with native row/count/window aggregates.
  `native_transport::value_bytes` measures borrowed Arrow in the exact final JSON codec without
  materializing a JSON tree. Native plans choose inline/artifact mode and cumulative artifact budget;
  the physical sink encodes selected values and checks the exact byte witness. Rust row-by-row budget,
  size-class and truncation policy is deleted. A first unfit value refuses with capacity diagnostics;
  past-end continuation is invalid input. Artifact handles come from native reachability/deduplication.
- Selection identities now include runtime and service-policy witnesses for every cursor family.
  One retained typed runtime preimage includes effective session options, the compiled source/function/
  codec closure, native/cache policy and resource bounds. Runtime clones share the captured witness.
  Input snapshot pairs/artifact identities and exact request selection remain separately bound.
- Native rule owners can supply a structured refusal cause. A capacity/input refusal keeps actionable
  diagnostics; query/physical failures preserve their actual origin. No text matching or handler-side
  inference selects the cause.

Focused evidence (all pure native/Arrow units, no service storage journey):
`native-coverage-unit.log` passed (0.26 s), `native-comparison-policy-unit.log` passed (0.50 s),
`native-cursor-witness-unit.log` passed (0.16 s), `native-change-projection-unit.log` passed (0.72 s),
and `native-alternative-page-unit.log` passed (0.64 s). `native-page-unit.log` records the current
page unit follow-up. `native-research-clippy.log` passed (45.37 s), before the final page cause change;
`native-research-all-targets.log` records subsequent compile-only fixture/package coverage.
`native-research-architecture.log` passed; `native-research-capability-scan.log` returned no findings.

The coverage probe exposed DataFusion 55.1's Boolean coalesce simplification changing projected
nullability. Completeness now uses exact filtered counts, preserving the result contract without
weakening native schema checks or adding a custom Boolean kernel. All CP packages remain open;
these receipts do not establish actual storage, producer, installed, MCP or deletion-barrier closure.


### Typed comparison and native search checkpoint — 2026-09-17

Current source is **state 14 / snapshot 11.0 / wire 7.0 / executor 7**. This is source identity;
CP11 deletion, installed qualification and activation have not run.

- Closed `ComparisonValue` variants replace generic inline JSON. API values reuse `ApiPayload`
  and its typed callable declarations; source stays independently qualified. Direct native record
  construction replaces intermediate metadata-losing built-in structs; both sides share one axis
  definition. The permissive recursive comparison-value whitelist and JSON inline decoder are gone.
- `ComparisonFieldPath` identifies declared fields and sequence ordinals. A bounded Arrow UDF
  lowers native values into exact canonical field facts; DataFusion performs set difference and
  ordering. Parent facts preserve alternative correlation and full sequence ordering. Earlier
  per-field joins and wide expression alternatives exceeded the fixture deadline and were replaced.
  The UDF declares count/byte/depth limits and temporary pool admission; this does not close CP09's
  complete output/external-allocation lifetime accounting.
- Inline output uses the generated native union codec; artifacts stream the same declared wire
  format (`service:comparison-value/4`). Opposite-side detail presence is selected by a native
  aggregate; before/after values no longer rely on positional Vec/pop assembly.
- One immutable search-family declaration supplies default/selected families, fragments and
  evidence coverage. Native request admission handles normalization, unknown/source/empty families
  and page bounds. Handler family lists, mapping branches and duplicate cursor-count policy are gone.
- Search hit/citation/key projection, Unicode excerpts and citation identity are native. Ranking
  and score factors now use their declared core types; handwritten search-row and score-schema
  modules are deleted. Count consumers decode a declared native aggregate. Token-count admission
  and the shared cursor/final-delivery byte cap use native plans.
- ADR-0056 remains proposed with a scoped design review. Schemas/DTOs/guidance regenerated;
  `typed-research-schemas.log` reports 4 valid and 8 rejected negative fixtures.

`typed-comparison-fields-unit.log`: two pure Arrow comparison units passed (1.55 s).
`native-search-units.log`: two pure search policy/projection units passed (0.90 s), before the
subsequent unknown-hit negative case. `typed-research-clippy.log` passed production workspace
libraries/binaries (34.60 s), before the shared byte-cap addition. Follow-up receipts distinguish
those later edits. No service storage, publication, artifact journey or MCP test ran.

All CP packages remain open. The remaining boundary still includes complete consumer/operator
matrices, other handler composition, external allocation lifetimes, orphan/historical admission,
replay/unknown-ack/process qualification, source/package/enforcement/install retirement and final
actual-client qualification. This checkpoint does not promote focused evidence to those gates.


### Borrowed delivery and shared research presentation checkpoint — 2026-09-17

Source identities remain state 14 / snapshot 11.0 / wire 7.0 / executor 7. No installation,
retirement or terminal qualification was performed.

- `native_transport::DeliveryBytes` borrows array and scalar Struct inputs directly. It no longer
  expands scalar results, decodes a complete `ResultRecord`, or reserves an input-size multiplier.
  Its output allocation is checked at the fixed UInt64 width. A bounded-input schema and the native
  JSON sink still govern recursive formatting; this is not a claim of complete external accounting.
- One envelope field declaration generates its public shape and native field projection. Private
  status/job/error fields keep their checked constructors. One streaming MCP formatter supplies
  both native candidate measurement and selected RPC projection. Nested error previews and resource
  text stream through an escaping writer, without candidate-sized intermediate strings. Unicode
  bytes remain valid even when writes split a code point. At this checkpoint the selected RPC still created its final
  JSON value; the following retention/transport checkpoint replaces that allocation path.
- `research_citations` owns qualified identity, Unicode excerpt selection and ordered sampling for
  search, overview, inspection and acquisition. Duplicate facts stay distinct inputs; presentation
  bounds do not change citation identity. Zero character width yields an empty excerpt. Handler
  `truncate`/`evidence_from_fragment` helpers and direct citation construction are removed.
- `research_overview` selects area/item bounds, catalog-derived coverage kinds, native counts,
  summary and limitations. Handler kind/count/sampling policy is deleted.
- Fragment pages now decode one declared `SelectedFragment`; the parallel Boolean-array pairing
  is gone. Native records attach the full recovery template only to incomplete text and select
  truncation. Recovery cursor construction remains mechanical in the handlers. Native outcome
  plans distinguish indexed-empty from unqualified empty results and preserve explicit failures.
- Two scoped structural rules, with positive/negative fixtures, reject reintroduced owned delivery
  decoding and retired handler citation/truncation authority. The existing physical page/provider
  checks remain: projection happens after the family schema check and native page selection.

Executed receipts in `.dev-state/plan19/execution/`:

| Receipt | Result | Scope |
|---|---|---|
| `typed-research-comparison-final.log` | 2 passed, 1.50 s | Typed value/field comparison |
| `native-search-final-units.log` | 2 passed, 1.32 s | Family/request/byte policy and original search projection |
| `native-presence-final-units.log` | 1 passed, 0.66 s | Alternative prefix and opposite-side presence |
| `native-artifact-reach-final-units.log` | 1 passed, 1.49 s | Change and artifact identity projection |
| `typed-research-all-targets-final.log` | passed, 23.64 s | All-target compilation before this presentation checkpoint |
| `typed-research-final-clippy.log` | passed, 39.26 s | Production Clippy before this presentation checkpoint |
| `typed-research-schemas-final.log` | passed, 4 valid / 8 rejected negative fixtures | Prior generated contract checkpoint |
| `borrowed-delivery-unit.log` | 5 passed, 1.02 s | All outcomes, terminal job errors, retained links, tool/resource byte parity and escaping |
| `borrowed-delivery-allocation-unit.log` | 1 passed, 1.45 s | Large borrowed array/scalar candidates under a 128-byte output fixture pool; not a workstation setting or RSS proof |
| `native-citations-unit.log` | 1 passed, 2.40 s | Identity/provenance, order, duplicates, zero and Unicode excerpt bounds |
| `native-overview-unit.log` | 1 passed, 2.33 s | Request bounds, empty/nonempty counts and catalog coverage kinds |
| `native-shared-citation-search-unit.log` | 1 passed, 1.85 s | Shared citation/hit/ranking consistency |
| `borrowed-delivery-selection-unit.log` | 1 passed, 29.03 s | Pure Arrow native byte predicate at exact tool/resource fit boundaries |
| `native-fragment-delivery-verified.log` | 1 passed, 2.29 s | Typed pairing, recovery presence, continuation/truncation |
| `native-empty-outcome-unit.log` | 1 passed, 3.60 s | Qualified absence and explicit failure preservation |
| `native-presentation-clippy.log` | passed, 40.17 s | Production workspace libraries/binaries |
| `native-presentation-rules.log` | 2 rules passed | Scoped positive/negative structural fixtures |

Two fixture-only compile mistakes (a SubjectRef variant name and an ambiguous `.into()`) were
corrected before the final fragment/outcome receipts. Final schema/all-target/static receipts are
recorded with the `native-presentation-` prefix. The acceptance source digest was stale after these
edits; regeneration joins recorded acceptance receipts, never promotes these isolated units to
terminal gates. CP00–CP13 remain open, including complete declaration/operator/consumer matrices,
remaining acquisition/aspect/replay policy, external accounting, orphan/historical/unknown-ack
physical lifetimes and the CP10–CP13 deletion, qualification and activation sequence.


### Durable maintenance and transport ownership checkpoint — 2026-09-17

At this checkpoint source was **state 15 / snapshot 11.0 / wire 7.0 / executor 7**. State 15 added the immutable
maintenance selection; obsolete state is refused without a migration or historical decoder.
Installation, retirement and full qualification remain unrun; CP00–CP13 remain open.

- Completed pending writers now enter a native maintenance claim that rejects every retained root,
  live lease, live writer and completed exact-table dependency in the same scope. All selected
  pending rows settle only after acknowledged physical cleanup. A dropped or indeterminate task
  leaves its durable fence claimed for physical reconciliation.
- `owned_tree` supplies one bounded, no-symlink inventory and exact durable file deletion path for
  private export tables and incomplete writer directories. Delta's pinned
  `deltalake_core::logstore::LogStore::is_delta_table_location` parses commit/checkpoint/compaction
  paths before incomplete removal; `NotATable` by itself never grants deletion. Namespace and exact
  metadata invalidation surround physical release. This is source behavior, not a power-loss claim.
- `MaintenanceSelection` records the table incarnation, protected versions, log floor, reclamation
  intent, observed clock and log cutoff before physical maintenance. Native invariants refuse scope
  changes and replacement of a committed selection. New exact-table/CDF enrollment rejects versions
  below that incarnation's recorded reclamation floor, including failed/interrupted runs. Existing
  readers retain their previously enrolled protection. The control table refreshes after recording
  its own witness so maintenance does not knowingly start from a superseded head.
- `JsonOutput` uses Serde JSON 1.0.151 `raw_value` to retain the selected UTF-8 JSON buffer directly.
  It validates raw JSON without constructing a Value tree and charges its buffer to the shared
  DataFusion pool before allocation. The complete outgoing RPC frame has a second explicit owner;
  both reservations survive socket writes. Measurement uses a counting sink. Allocator overcapacity
  and any boxing overlap are charged; this does not claim allocator/RSS or parser-stack exactness.
- `research_inspection` moves qualified execution limitations, source absence, cfg/locator disagreement
  and missing-evidence deduplication from the handler into a native relation.
- Removed the empty-directory-only recovery check, duplicate export file walker, final research
  encode/Value-decode/encode path, metrics byte copy, and stale nextest binary overrides. A scoped
  structural rule guards encoded RPC ownership. Real containment fixtures stay ignored behind CP11.

Exact API evidence: pinned skill catalogs plus `vendor/delta-rs/crates/core/src/logstore/mod.rs`
(commit/checkpoint path parsing), `crates/core/src/operations/vacuum/mod.rs` (native protected versions),
and the installed `serde_json-1.0.151/src/raw.rs:186` (validated String-buffer reuse). Context7 was
used only for Serde discovery; no Context7 DataFusion/Delta/Arrow/FastMCP claims were used.

Receipts are recorded under `.dev-state/plan19/execution/` with `incomplete-writer-`, `owned-json-`,
`retention-history-` and `retention-transport-` prefixes. The first nextest attempt found stale deleted
binary overrides before running tests; these were replaced with the target private module selector.
Pure fixture compilation also caught a Coverage set-versus-vector mismatch, corrected in source.
Remaining work includes all external metadata/predicate/kernel/writer/parser lifetimes, complete
mutation/CDF/replay/unknown-ack matrices, remaining orphan owners, full source/enforcement/package
closure, and CP11–CP13 retirement, integrated qualification and activation.


### Native source selection and durable private owners — 2026-09-17

At this checkpoint, source was **state 16 / snapshot 11.0 / wire 7.0 / executor 7**. State 16 replaces the
private input-owner grammar and layout; no previous owner-name decoder or state migration is kept.
This is source identity, not installation. **All CP packages remain open.**

Implemented across CP01/03/05/06/07/09/10:

- `research_inspection::ancillary` computes consensus using DataFusion COUNT DISTINCT and filtered
  FIRST_VALUE. Missing observations cannot mask a unanimous non-NULL value; independent conflicts
  still produce no selected locator/configuration. Source-artifact consensus runs before decoding
  one deterministically ordered descriptor. The unbounded handler role lookup and first/hash loop
  are deleted.
- `research_source` selects ecosystem-qualified source coordinates, artifact kind/role and the
  admitted line window. Native array/range expressions produce bounded safe archive suffixes;
  regular-file observations feed an ordered native member selection. The handler no longer owns
  locator interpretation or first-existing-path selection. The physical line parser preserves
  CR/CRLF/LF, final empty lines, UTF-8 and byte limits; it refuses invalid commands rather than
  silently changing their windows. SourceExcerpt declares member-path and one-based range rules.
- Blocking extraction, inventory, source capture and inspection reads use QueryRuntime. Exact
  snapshot protection and the private directory survive cancelled waiters through physical reads.
- `private_directory` replaces `input_directory` and the source/revision temporary cleanup drivers.
  Document IPC, Rust package/Python distribution/revision trees and worker Arrow output enroll
  durable obligations before bytes. The last owner reserves and joins release work. Native cleanup
  requires exact declared directory identity and an anti-join against every surviving owner.
- Before a worker receives its source-bearing request, both source and output enroll the direct
  child's observed machine/boot/PID-namespace/PID/start identity. Child witnesses survive parent
  death and lost acknowledgement; creator exit alone cannot authorize directory removal.
  Generic producer grant/worker-process supervision and its final physical qualification remain open.
- TAR and ZIP extraction count implicit parent directories before filesystem expansion. Cleanup
  consumes the same archive entry bound, refuses symbolic/hard links, verifies directory/device/inode
  and file identity, and uses the existing durable store for file deletion. Unknown outcomes retain
  obligations. Export staging outside the service namespace remains a separate unfinished owner.
- Deleted the unowned staging-provider variant, old extraction TempDir/Scratch paths, the unpacked
  cache accessor, and bare source-handler blocking tasks. Ported worker/physical-stream fixtures to
  durable owners; they are compiled, **not executed**, until CP12. The scoped
  `native-source-directory-owner` rule guards those deletions.

Pinned capability evidence: DataFusion 55.1.0 `datafusion_functions_aggregate::first_last::first_value`
([skill API](../../.claude/skills/datafusion/content/api/datafusion_functions_aggregate.first_last.md));
aggregate FILTER and native array_slice/array_to_string/generate_series/array_has_any documented in
that skill's SQL guides and executed by pure Arrow fixtures. Existing native object-store deletion,
Delta control transactions and process-observation rules remain the authority boundaries.

Receipts under `.dev-state/plan19/execution/`: `source-consensus-units.log` (3 passed),
`source-routing-units.log` (1 passed); `private-source-units.log` records the combined focused
selection. `private-source-all-targets.log`, `private-source-clippy.log`, `private-source-rules.log`,
`private-source-rule-scan.log`, `private-source-architecture.log` and the DataFusion/Delta skill
scans record affected compilation/static checks. A removed SourceTree type first exposed the
acquisition consumers; they were ported to the same durable owner. Clippy also found and prompted
removal of a redundant borrowed destination. These were implementation failures, not gate results.

The preceding maintenance/transport checkpoint completed 13 focused checks in 2.714 s and production
Clippy in 37.28 s (`retention-transport-native-units.log`, `retention-transport-clippy.log`); all-target
compilation passed in 5.92 s, and architecture/scoped rules/skill scans passed at that checkpoint.
Those receipts do not prove the later source changes or close integrated acceptance.

No service/Delta publication/CDF/export/restart/client journey, installed qualification, full CI,
legacy installation retirement or activation ran. Complete private/export/replay/transaction horizons,
producer supervision, external allocation accounting, remaining acquisition/aspect/recovery policy,
CP10/CP11 deletions and CP12/CP13 qualification remain mandatory.

### Native capture policy and static-worker effects — 2026-09-17

All CP packages remain open. This checkpoint extends CP03/05/09/10; it does not cross CP11.

- `source_capture` is the single native classification/selection plan for Rust package, revision
  and Python source captures. The immutable document filename roster supplies Rust document kinds;
  native predicates supply Python file scope, media types, example/document kinds and source roles.
  Duplicate paths and excessive inventory bytes refuse. Deleted `source::text_files` and repeated
  handler classification/role construction. Physical walkers and bounded artifact sinks remain.
- Core `execution::static_worker::{Launch,Effect}` declares the request and exact local extractor
  effect. Store `static_worker::lower/prepare` derives protocol, argv, environment, cwd, output,
  memory/CPU/output limits and the live Python-resolution scope. The immutable Delta definition
  binds the command/grant/policy, source artifact, full file inventory and compiled implementation.
  Artifact IDs use the declared digest relation. A retained definition alone grants no dispatch.
- Dispatch requires source and output PrivateDirectory owners and verifies every directory component.
  The complete inventory captures paths, modes, lengths and content digests using the shared bounded
  physical inventory reader. Native anti-join checks every selected source file; symmetric native
  map-entry differences refuse changes immediately before spawn. The child is enrolled before stdin.
- The worker receives an absolute installed interpreter, fixed isolated argv and the complete native
  environment; daemon PATH inheritance and handler-built WorkerRequest/Command are removed. Request
  serialization is measured before allocation and retains its native pool reservation through write.
  Python reads the request-byte ceiling from its generated Arrow metadata.
- Fresh durable authority checks occur at dispatch, during parsing and before returning admitted
  facts. The child owner reserves cleanup capacity before spawn and retains the producer permit and
  source/output owners through reaping after cancellation. An unresolved wait retains ownership.
  Output creation runs on the owned blocking lane; pending Tokio file I/O keeps its directory alive
  until flush joins it. The Rust rustdoc worker also uses the shared blocking lane instead of a bare
  blocking task, retaining admission until its cancellation-aware supervisor physically exits.
- `native-static-worker-launch` guards the deleted handler command path. Its fixtures and production
  scan pass. The source-directory guard remains in force.

Pinned capability evidence: DataFusion 55.1.0 native `array_except`/UNNEST and
`datafusion_functions_nested::map::map` supply inventory differences, membership and environment
construction. The latter is verified in the installed 55.1.0 crate's `src/map.rs:46`; the DataFusion
skill's native function index and SQL guides supply discovery. Immutable effects use the existing
Delta returned-state/contract/definition owner; no parallel effect journal or cache authority.

Receipts in `.dev-state/plan19/execution/`:

- `source-capture-units.log`: four focused units passed in 3.661 s; all-target compilation passed
  in 22.43 s. The preceding private-source group passed 13 units in 2.722 s, production Clippy in
  15.24 s and all-target compilation in 15.61 s.
- `static-worker-units-final.log`: three isolated launch/scope/reaping units passed in 2.355 s.
  `static-worker-owned-units.log`: those plus Rust decoder cancellation passed, four in 2.319 s.
  They use native Arrow inputs or isolated OS children, without a service/Delta/worker journey.
- `static-worker-owned-clippy.log`: production workspace Clippy passed in 22.87 s.
  `static-worker-owned-all-targets.log`: all targets compiled in 30.09 s. Schema/static follow-ups
  use the same prefix. The first worker invariant fixture exposed a multi-column witness projection;
  it was fixed. Clippy's nested-if finding was fixed without suppression.

Still required: acquisition-to-effect/result lineage, matching installed interpreter/package/producer
closure, complete static worker fidelity and allocation bounds, raw Rust worker staging/ownership,
remaining source/environment/recovery policy, the full physical failure/revocation/unknown-ack and
shutdown matrices, CP10/11 deletion/install barrier, and CP12/13 qualification/activation. No full
integration, real installed worker, publication/CDF/export/client journey or activation ran.


### Durable Rust parsing and primary export staging — 2026-09-17

Rust decoder capture and its seven Arrow fact outputs now use `PrivateDirectory::Rustdoc`.
The former TempDir provider, long-lived root lock, global blocking parser mutex and separate
stdout/stderr threads are removed. Capture/receipt filesystem work uses the shared blocking lane;
subprocess I/O is async and shares `OwnedChild` reaping with Python. Parser admission is separately
declared, defaults to 16, and remains held through physical child exit. Capacity-one units prove
query admission stays available while parser slots are occupied. State 17 introduced that policy.

State **18** adds the declared external-directory dependency. Export sibling staging is durably
enrolled in the **primary** control store before files are created. Its location includes the
canonical parent, device/inode witness and generated child identity. The existing native cleanup
relation admits only the exact owner/location with all process witnesses released; conflicting
location records and parent/link replacement refuse deletion. The same private-directory recovery
entry point discovers unfinished exports after primary process reconciliation. Target-local writer
obligations remain useful within the bundle, but no longer carry sole orphan-discovery authority.

`InputContext` mechanically retains the private owner across DataFusion async/blocking descendants
and final release writes. A canceled caller cannot release a still-running blocking child. Mutable
target consumers leave scope before a submitted-release barrier; only then is the final control
identity captured and the manifest sealed. The final synchronization/rename closure retains the
owner. Post-rename cleanup addresses only the empty staging parent, never the published destination.
The old sibling TempDir cleanup is deleted and a scoped structural rule prevents its reintroduction.

Evidence under `.dev-state/plan19/execution/`:

- `rustdoc-owner-units.log`: seven isolated units passed (2.396 s). Production Clippy/all-target
  compilation and generated schema conformance passed; no matching installed decoder was executed.
- `export-owner-units.log`: five isolated native-policy, parent identity, canceled-blocking-child
  and release-barrier units passed (2.351 s), 162 tests skipped.
- `export-owner-rules.log`, `export-owner-architecture.log`, `export-owner-adr-lint.log`: passed.
- `export-owner-clippy.log`, `export-owner-all-targets.log`, `export-owner-schemas.log`: follow-up
  receipts; their final exit/result controls evidence, not their existence.
- `typed-comparison-provenance.log`: all four seals passed. Proposed ADR-0056 records the successor
  wire-7 guide seal and hash-verified wire-5 predecessor relocation; no old manifest/map was edited.

CP05/07/09 remain open for full lineage, installed workers, external allocation accounting,
historical/transaction horizons and physical restart/reclamation/unknown-ack matrices. CP11 is
still open; no service export/publication/restart journey or final integration suite ran here.


### Native retained resolution and root-removal admission — 2026-09-17

`research_resolution` now projects complete retained metadata consensus, distinct gap values and
hosted artifact selection from protected native relations. It compares typed canonical values
without collapsing Map contents, NULL/empty distinctions or ordered repeated headers. Conflicting
qualified values refuse; the handler no longer decodes all alternatives to compare them. The unused
`SnapshotReader::coverage` and `release_metadata` hydration APIs are deleted.

Rust, Python, revision and current-registry/retained-source resolution share a native freshness
projection. Registry timestamps come from acquisition receipts, not rendering-time `now()`. Index
presence, ecosystem, explicit-version scope and revision mode govern latest-check reporting. The
MAX clock aggregate uses the explicit `clock_instant`/`acquisition_time` conversion after a focused
fixture exposed semantic metadata loss. A shared native array projection replaces only its own
retained/reselected limitation; it no longer pops an unrelated last note. Python/revision artifact
handles are complete, with byte-fit handled by native delivery instead of `.take(10)`.

Whole-root maintenance now reconciles native processes/writers/private directories and requires
no live lease, cleanup owner or claimed maintenance run. An unsettled private directory prevents
removing its primary recovery record. The filesystem inventory targets current `private` data;
retired `document-inputs`, `unpacked` and `workers` payload names are removed. A separately executing
export retains the primary physical root-removal fence through its final cleanup settlement; it
supplies no table/version read authority. Operator deletion/restart/race qualification stays CP12.

Receipts under `.dev-state/plan19/execution/`:

- `resolution-projection-units.log`: metadata/gap fixtures plus the existing pure native recovery
  selector passed, three tests in 7.184 s. No Delta journey ran.
- `resolution-freshness-unit.log`: the initial clock-metadata mismatch is preserved.
- `resolution-freshness-final-units.log`: receipt-based freshness and idempotent owned-note
  replacement passed, two tests in 2.316 s.
- `resolution-owner-clippy.log`, `resolution-owner-all-targets.log`, `root-quiescence-unit.log`:
  final follow-up receipts; only completed results establish their individual claims.
- DataFusion/Delta skill scans on the affected consumers are empty; architecture check passed.

This advances CP01/05/06/07/10. Remaining acquisition lineage, inventory/document navigation policy,
all external memory lifetimes, full route/operator/mutation/replay/fault matrices and the CP11–13
installation/removal/qualification boundary remain open. No package or terminal gate is closed.


### Native decoder, navigation, component and cache observations — 2026-09-17

Source state **19** removes the implicit PATH-selected Python interpreter configuration. The
interpreter is optional until an installed launch configuration supplies an absolute path; absent
or relative paths cannot dispatch. Existing launch generation already supplies the installed path.
No installation or state initialization was executed at this checkpoint.

The trusted Rust decoder consumes core `execution::rustdoc_decoder` declarations (protocol **3**)
for its request, exact effect and complete stream receipt. `rustdoc_decoder_plan` lowers artifact
identity and bounds, retains the effect under the live Rust resolve claim, validates complete
stream cardinality/identity and checks the current producer revision. Dispatch uses the declared
executable/argv/environment/cwd and observes claim revocation while the child runs. The old
handwritten Request/Extraction/Receipt/Report definitions are deleted. The worker/storage fixture
now establishes a real claim; it remains deferred until CP12.

Both parser drivers reserve their declared address-space ceiling against the shared DataFusion
pool before spawning. The envelope and parser permit remain with the OS child/reaper through
physical exit. This is reserved external capacity, not measured RSS or proof of every allocation.
Kernel async stream tasks also retain the private-input context across their physical lifetime.
The main 32-GiB pool, 16 parser slots and other workstation defaults remain unchanged.

`DocumentFact` is one native declaration, replacing the handwritten field and vocabulary encoder.
Its physical document staging is a generated flattening. `documentation_plan` selects three
**distinct** navigation pages in deterministic order before the fetch bound, retains complete
accepted inventory entries and fetched artifacts, and applies the 8000-character Unicode window
and inventory-version classification through native expressions. Handler `.take(3)` and
`.chars().take(8000)` policy is removed.

`status_components` declares the finite roster and requirements once. `status_plan::components`
joins captured store/worker facts with the same per-ecosystem/profile decisions used for effect
admission. A qualified image with a disabled route is unavailable; an independently admitted
Python route remains usable when Rust is unavailable. The daemon's per-component mutation loops
and obsolete phase-based availability constructors are deleted. A configuration-only CLI view
reports uniform store absence using the same roster.

The file metadata owner uses a narrow Cache observer that delegates all residency/TTL/eviction
to DataFusion DefaultCache. Lookup hit/miss and invalidation counters are constant-size. A lookup
hit is explicitly **before reader validation**, and zero resident bytes does not imply zero live
Parquet allocation. Status never calls `list_entries()`. No claim of complete CP09 external
accounting is made: the upstream metadata reader can clone the inner Parquet Arc beyond the
cache entry's lifetime.

Development receipts under `.dev-state/plan19/execution/`:

- `decoder-contract-units.log`: metadata-manager/counter and canceled-parser reservation units
  passed; the two initial decoder policy failures are preserved. They exposed a renamed sibling
  identity reference and the invariant query's single-column requirement, both repaired.
- `decoder-document-units.log`: two native decoder admission/receipt cases and distinct-page/
  Unicode document projection passed (3 tests, 2.411 s). No worker/storage journey ran.
- `decoder-effect-check.log`: all-target store/daemon compilation passed (36.60 s) before the later
  component/configuration additions. Later checks must cover those additions.
- `decoder-document-clippy.log` and `native-components-clippy.log`: initial all-target failures
  remain recorded. Redundant test borrows, helper placement, an async-returning-async fixture and
  stale imports were corrected; the final follow-up receipt determines current check status.
- `decoder-rule-test.log`: one scoped rule fixture passed; production scan is empty. DataFusion
  and Delta capability scans on the affected owners returned no findings.

CP01/03/05/06/09/10 advance, but no CP package closes. Remaining producer/acquisition lineage,
all kernel/metadata/predicate/writer allocation lifetimes, route/operator/mutation/replay/fault
matrices, physical layouts, full deletion inventory and CP11–13 remain open. Integration,
installed qualification, publication/CDF/export/restart journeys and actual clients remain
**not_run** until the CP11 barrier is satisfied.


### Native inspection, execution provenance and prepared inputs — 2026-09-18

The shared tree remains at HEAD `efc646c` plus preserved uncommitted work. This checkpoint advances
CP01/02/03/05/06/07/09/10; **no CP package or terminal qualification is closed**.

- The runtime selects DataFusion's DuckDB SQL dialect before capturing its session witness.
  The per-producer parser helper is deleted. Concrete `ArrayAnyMatch`, `ArrayFilter`, `MakeArray`
  and `ArrayAgg` admission checks full native child fields; lambdas validate their bound variables
  and bodies. Cross-domain comparisons and scalar metadata erasure still refuse. The pinned
  library has no `ArrayAllMatch`; no such API was retained. Concrete `GetFieldFunc`,
  `NamedStructFunc` and `CoalesceFunc` checks replace name-based function allowances.
- `research_catalog` owns fragment routes. `research_inspection` owns required observations,
  ordered aspect outcomes and deterministic multi-stage failure composition. It rejects duplicate,
  missing and out-of-scope observations. Full-text recovery removes the preview width, preserves
  the exact symbol/snapshot and retained intent, and rebinds the original cursor boundary to the
  complete witness. Cursor encoding uses a bounded sink before hex expansion.
- `inspection_execution_plan` owns producer method/selector completeness, gaps, inspection data,
  qualified coverage and artifact closure. Native anti/semi-joins replace handler fact/receipt
  filtering. All acquisition receipts survive; output handles use the full canonical receipt key
  for deterministic ties. Default semantic methods have one declaration. Native retained reuse
  distinguishes unsupported evidence from failed/incomplete observations, preserves ambiguity,
  and creates sorted exact-snapshot recovery requests. Native execution admission pins the actual
  symbol/definition and requires the explicit profile and exact Python runtime import binding.
- `probe_plan` supplies both initial and physically settled state, outcome, summary, coverage and
  limitations from the original capture. The physical supervisor alone reports settled cleanup;
  the original process observations remain unchanged. Handler limitation rewrites and the unused
  `ProducedProbe` state/duplicate lowering are deleted.
- `execution_fact_plan` is shared by verification, inspection and execution ingestion. It binds
  the exact producer run/environment/image/containment, checks the retained canonical payload,
  validates evidence class/profile, applies native intrinsic and cross-field scope/outcome rules,
  and derives ordered observation IDs. Production publication no longer calls per-row
  `ExecutionObservation::new` or computes producer bindings in handlers. Bounded public codec
  validation remains; full family/route validation coverage is still CP01/02/05.
- `capsule_input_plan` selects archive and manifest captures from the complete `PreparedCapsule`
  inventory. Exact paths, digests, lengths and per-file/closure limits are native decisions.
  The old dependency walker, extension dispatch, sort and digest-first acquisition map are
  deleted. The bounded physical reader verifies captured bytes and retains a pool reservation
  through blob insertion. Input readers retain the original execution permit and locks until
  actual blocking exit, including waiter cancellation; cleanup cannot delete their inputs early.
- Semantic input projection deduplicates identical meaning after removing acquisition clocks,
  while preserving independent source URIs. Execution input conflict grouping includes producer,
  role, digest and source URI. Raw receipts remain independently retained.
- The unused `ProducerSpec`/`ProducerPlan` scaffold is deleted; `producer/run.rs` keeps actual
  generated run provenance and native identity boundaries. `status::service_status` and its
  silent default-configuration fallback are deleted. Structural rules guard inspection policy
  and execution-fact ownership; fixture-only constructors remain available for independent oracles.

Focused development receipts in `.dev-state/plan19/execution/`:

- `inspection-native-units.log`: 3 pure units passed (2.342 s): semantic lambda domains,
  aspect observations and full-text recovery.
- `inspection-aggregate-units.log`: metadata-erasure negative unit passed; the old render query
  exposed DataFusion's duplicate-mark optimizer failure. A native selected-ID union/semi-join
  replaced that query. Each `IS DISTINCT FROM` operand is explicitly parenthesized for the
  selected dialect. No optimizer rule was disabled.
- `inspection-reuse-final-units.log`: 4 pure inspection completeness, scope, reuse, ambiguity,
  recovery, artifact and nonempty coverage units passed (3.582 s). A malformed runtime-results
  fixture was corrected to include observed result content; the initial failure remains logged.
- `probe-policy-units.log` and `inspection-reuse-units.log`: the initial/settled probe unit passed,
  including unchanged process evidence and consistent final coverage/limitations.
- `native-facts-history-units.log`: native fact/provenance and immutable maintenance-selection
  units passed (2 tests, 12.168 s). Failed maintenance cannot erase or replace its recorded floor;
  wrong table scope, future cutoff/floor and unauthorized reclamation refuse.
- `capsule-acquisition-units.log`: exact prepared-input selection and cancelled blocking-reader
  ownership units passed (2 tests, 2.328 s); no service, container or storage journey ran.
- `execution-fact-rule-tests.log`: both scoped rule groups passed; their production scan is empty.
  `native-execution-capability-scan.log`: pinned skill source scan returned no findings.
- Final affected Clippy, schema/static and acquisition-provenance checks are recorded in the
  following checkpoint receipts; do not promote an earlier receipt across later source changes.

Remaining architecture includes complete declaration/operator/provider/mutation and consumer
matrices, acquisition/preparation/run provenance, all external metadata/predicate/kernel/writer
lifetimes, orphan/history/transaction and unknown-ack reconciliation, fresh-process command/CDF
replay, complete L01–L33 source/package/enforcement closure, and CP11 actual install/state/client
retirement. CP12 integrated/installed qualification and CP13 activation remain **not_run**.


Follow-up receipts, **2026-09-18**, at the same source checkpoint:

- `native-producer-closure-final-clippy.log`: core/store/daemon all-target Clippy passed with
  application warnings denied (20.81 s). Earlier unused-import/mutability failures were repaired;
  the two pinned Delta Parquet deprecation warnings remain visible.
- `acquisition-provenance-unit.log`: qualified input projection passed (1 unit, 1.748 s), retaining
  separate source URIs for equal bytes while excluding repeated acquisition clocks from semantic IDs.
- `native-execution-schemas.log`: current generated JSON/Python/Arrow contracts and conformance
  passed (4 admitted fixtures, 8 rejected negatives). Generator format warnings remain visible.
- `native-execution-architecture.log`: architecture check passed. `native-execution-rule-tests.log`:
  3 focused rule groups passed; the production scan is empty. `git diff --check` passed.
- `doctor-20260918.log`: all hard tool prerequisites present; Rust 1.98.1 matches the pin.
- Restricted replay additionally compares the decoded Delta protocol with exact durable protocol,
  alongside version and metadata. The source-backed API is `EagerSnapshot::protocol` at the locked
  Delta revision; `replay-protocol-clippy.log` records compilation. The physical replay/feature
  mutation matrix remains deferred to CP12.
- `native-execution-acceptance-report.log` / `native-execution-acceptance-check.log`: the freshly
  generated registry report is **0 passed / 0 failed / 0 blocked / 48 not_run**. Plain development
  logs are not source-bound terminal acceptance receipts. No installed retirement or activation ran.


### Shared planning, producer provenance and preparation — 2026-09-18

**All CP00–CP13 remain open.** These are source and isolated development receipts. No service
storage/publication/CDF/export/restart/MCP journey, installed retirement or activation ran.

**Shared contracts and planning (CP01/02/03/05).**

- `producer/run.rs` declares common attempt/run fields once. `producer_run_plan` owns input
  roles, canonical ordered maps, run composition, clocks/log requirements and acquisition closure.
  Resolve, local rustdoc, inspection and verification use that owner; ingest, attempt and control
  admission share the same rules. Exact operational receipt identity excludes receipts without
  excluding a legitimate acquisition of equal bytes from another origin. Multiple qualified log
  acquisitions survive. Producer input IDs/digests are still text; this does not close L18.
- Native `RustdocBuildConfiguration` replaces the ad hoc semantic JSON configuration preimage.
  Its keyed semantic identity remains separate from the digest of its stored native bytes.
- `SemanticAfterAnalysis` complements the pre-coercion check. The common `NativePlanner` validates
  optimized logical plans, including Delta/direct DataFrame consumers, then traverses every
  physical child for executable invariants and valid full-field annotations. A hidden invalid
  child cannot pass through a valid root projection. Literal NULL remains legitimate absence;
  domain-erasing value casts remain refused. This is not a complete physical-expression matrix.

**Preparation and source policy (CP05/09/10).**

- `preparation_plan` uses `cargo_toml_entries_v1`, a bounded syntax-only UDF with segment paths,
  to admit dependency/workspace sources. SQL distinguishes actual Cargo source sections from
  arbitrary package metadata. Native plans select configuration removals, exact package/version
  and library name, feature/default/target options, and the generated consumer manifest.
  Local rustdoc no longer guesses a library name by splitting an archive name on hyphens.
- `python_distribution` owns metadata-file authority, complete source/stub/native/typed-marker
  inventory, import roots, optional headers and entry points. `python_metadata_headers_v1`
  decodes bounded external syntax. Native PEP 440 functions, set difference and array operations
  corroborate metadata/filename/dist-info identities, interpreter requirements and expanded
  WHEEL tags. Acquisition and execution dependency preparation share it. Equal version spellings
  such as 1.0 and 1.0.0 compare by native parsed precedence; duplicate metadata authorities refuse.
- `producer_plan::preparation_result` recomputes the exact finite command and selects readiness
  from the physical exit, captured identity and cleanup facts. The daemon mechanically dispatches
  the selected verdict. Python/toolchain identities now share core declarations. Native commands
  and current grants remain separate from this outcome check; no effect is granted by a verdict.
- `OwnedBytes` reserves the actual file-buffer bytes before allocation and keeps the reservation
  with the immutable buffer. Descriptor reads reject non-regular files, final-component symlinks,
  excess size, truncation/growth and artifact hash/length disagreement. Rustdoc archive/lock/output,
  capsule source and selected distribution metadata use owned reads. Blocking extraction/removal
  retains cleanup and storage owners through physical exit. Captured metadata text has a separate
  live reservation. This does not claim allocator RSS, parser expansion or all wire/worker allocations.
- Deleted: `BlobStore::read`, daemon `BuildSpec`/recursive Cargo source policy, procedural Python
  archive inventory/validator, duplicate metadata chooser, ad hoc producer-run constructors and
  content-only operational receipt filtering. Dormant service fixtures now use exact receipt reads;
  they were compiled, not executed. Focused structural rules guard the new preparation and run owners.

**Checks recorded under `.dev-state/plan19/execution/`.**

- `producer-configuration-clippy.log`: core/store/daemon all-target Clippy passed (36.28 s).
- `native-planner-final-clippy.log`: the shared planner change passed all-target Clippy (26.95 s).
- `planning-stage-final-units.log`: 13 pure planning/provenance units passed (4.136 s).
  `planning-consumer-units.log`: 14 isolated consumers passed (15.017 s);
  `shared-planner-consumer-units.log`: 11 shared-planner consumers passed (2.500 s).
- `native-preparation-closure-units.log`: 14 native/Arrow/physical-buffer units passed (7.011 s).
  `preparation-outcome-repaired-unit.log`: exact command/identity/cleanup unit passed (2.952 s).
- `preparation-outcome-final-clippy.log`: all-target application Clippy passed (49.98 s), before
  the subsequent SQL-only whitespace repair. Two pinned Delta Parquet deprecations and the
  proc-macro-error2 future-compatibility warning remain unsuppressed.
- `preparation-architecture.log`: native architecture checks passed. `preparation-owner-rules.log`:
  two focused rule groups passed; production scan empty. `preparation-capability-scan.log`: the
  DataFusion skill's project capability scan found no syntactic gaps in these owners.
- `preparation-acceptance-{report,check}.log`: generated/checked report, **48 not_run**, zero other
  states. `preparation-doctor.log`: hard prerequisites present, toolchain pin matches. Neither
  tool presence nor these plain development logs qualifies a service gate.

**Failures resolved without weakening admission.** `owned-rustdoc-units.log` records conservative
boolean nullability; the explicit Arrow representation boundary supplies the declared projection.
`native-preparation-units.log` also records an unsigned array subscript; native last-element access
replaces it. `native-distribution-units.log` records a physical lambda-variable mismatch in a
nested optimized query; equivalent native regex/path expressions remove that unsupported shape.
`distribution-owner-final-units.log` records DISTINCT-expression grouping under the selected
SQL dialect; explicit parentheses repair it. `preparation-outcome-unit.log` records SQL trim's
handling of the producer newline; native Unicode-whitespace trimming repairs it. These receipts
are evidence for these concrete routes, not a claim that every library operator is qualified.

**Next:** finish remaining schema/domain/operator and source-fidelity inventories; acquisition
lineage/effect revocation and unknown acknowledgement; exact bundle/orphan/history/transaction
retention and replay; full external allocation/close matrices; L01–L33 and CP11 actual retirement.
The new shared owners above are implemented foundations, not scope to re-create.


### Native bundle policy and immutable root protection — 2026-09-18

**Implemented source, CP02/06/07/09/10; every CP package remains open.**

- `bundle_plan` owns checksum syntax, path/digest rules, duplicate authority, exact file coverage,
  hash mismatch selection, canonical manifest ordering, declared bundle contract and captured
  catalog/context/snapshot agreement. A second native plan unifies semantic inputs and operational
  artifacts, rejects contradictory content lengths/identities, deduplicates copies and checks the
  512 MiB closure before physical copying. Rust captures bounded file/hash observations and drives
  the selected copies. The old procedural checksum map and destination-existence deduplication
  loops are removed. Manifest and descriptor input buffers use `OwnedBytes`.
- Live export uses the normal durable catalog and exact snapshot leases. Copy chunks enroll exact
  artifact dependencies with current source-root checks before opening bytes; the blocking driver
  retains the lease and primary staging owner through physical exit, including caller cancellation.
- Target writers/releases drain before an exclusive root fence creates the permanent immutable
  seal. The seal is included in the checksum inventory. Bundle contract is now
  **`delta-evidence-bundle/2`**; version 1 is refused, with no compatibility reader. Bundle opening
  takes an `ImmutableRoot` handle without changing bundle bytes. Control/evidence/result providers
  carry that explicit contract and exact version vector through the existing retention planner.
- Ordinary blob writes and Delta creation/constraints/append/maintenance refuse sealed roots,
  including previously opened handles. Delta append/constraint mutation additionally checks its
  captured physical namespace before dispatch; another store cannot pass it a foreign table.
  Namespace admission also rejects a writer result outside the owning root.
- Removed `EvidenceRepository::read_only`, `ControlStore::read_only` and the root-lock-only
  `ReadProtection::capture`. `ControlStore::inspect` is a control-record inspection surface;
  it cannot open dependent artifact/result inputs. Immutable bundles require an explicit sealed
  root; mutable evidence uses durable enrollment. Dormant bundle/provider fixtures use the target
  APIs. Their storage and export journeys were not run.

**Development evidence** in `.dev-state/plan19/execution/`:

- `bundle-selection-units.log`: **5 passed** (2.535 s): pure native checksum/copy/catalog policy,
  isolated seal mechanics and exact vector/root identity. Repaired earlier logs preserve the
  invariant-output shape failure and a missing helper during implementation.
- `bundle-selection-clippy.log`: core/store/daemon **all-target Clippy passed** (24.68 s).
  The later dormant foreign-namespace fixture addition is not included in that receipt.
- `bundle-owner-rule.log`: focused rule fixtures passed; production scan empty.
  `bundle-capability-scan.log`: pinned DataFusion project capability scan empty (syntactic scope).
  `bundle-architecture.log`: architecture check passed.
- Last independent acceptance audit: **2026-09-18 05:46:08 UTC**, source digest
  `3d1fa011e43a1e0cb99999b3b57634355bf0bbd8666d137ed73ed4da7d8e04a0`, **48 not_run** and no other states.
  It confirmed the 14 specified development logs were not promoted to gate evidence. Source has
  changed since that audit; it is not current qualification.

**Remaining:** full sealed-root/provider/optimized-plan/stream lifetime and mutation fault matrix;
actual export, readback, restart, root-removal races, unknown rename acknowledgement and durability
journeys; native orphan/transaction horizon closure and external allocation work. These source
changes do not close CP07 or authorize pre-CP11 integrations. CP11 retirement/installation and
CP12/13 qualification/activation have not started.


### Shared positive evidence proofs — 2026-09-18

The repository's `verified_blobs`, `verified_manifests` and `verified_attempts` maps and manual
256/4096-entry clear thresholds are removed. The existing shared positive-contract cache now
owns storage and evidence validation proofs under one native `DefaultCache` and the existing
64 MiB policy; no additional independently tuned cache family is introduced.

Blob keys carry the path, digest, expected bytes and physical file witness (including inode and
ctime). Snapshot keys carry every physical table incarnation, exact descriptor digest, runtime
meaning and validation limits. Attempt keys additionally carry the control incarnation and exact
catalog identity. Only successful validation inserts a proof. Warm attempts still recheck their
physical log/delivery bytes, and normal scope/admission/retention checks precede reuse. A proof
cannot issue a lease or resurrect a removed root. Authority invalidation conservatively clears
this bounded shared cache without enumerating it. A caller-held proof and its log vector retain
the native memory reservation after eviction until their final physical owner drops.

`shared-validation-units.log`: **2 pure units passed** (0.008 s), covering existing full schema/
registry proof identity plus file replacement, changed runtime/limits/catalog, invalidation and
an evicted-but-live proof. `shared-validation-clippy.log`: affected all-target Clippy passed
(21.93 s) before the subsequent validation-limit key and unit additions. These are source progress
for CP02/07/09/10. Full consumer, pressure, physical allocation and terminal matrices remain open.
The cache's reported occupancy is declared native accounting, not allocator/RSS measurement.


### Native retained validation and physical read ownership — 2026-09-18

**Implemented source for CP02/05/06/07/09/10; packages remain open.**

- `execution_documents` now joins canonical execution-payload digests to retained input receipts,
  selects document/snippet/location closure, rejects contradictory or oversized document inputs,
  and lowers positions/diagnostic/location ranges into one declared coordinate relation. The
  `semantic_utf8_range` UDF applies the existing bounded UTF-8 syntax kernel. Rust drives unique,
  16-document read chunks with exact artifact enrollment and paid file/text buffers. The old
  Rust payload-dispatch validator, local text cache and unowned blocking callback are deleted.
- `snapshot_validation` owns distinct content/byte admission and exact component/count/coverage
  agreement. It composes shared native semantic digest and coverage plans; the repository no longer
  compares these summaries in Rust or accumulates its validation byte policy in a row visitor.
  Artifact hashes acquire exact protection in bounded chunks and carry it into the physical callback.
  Warm positive proofs still recheck file witnesses under current read protection.
- Retained-result dependency selection rejects conflicting digest/size declarations and a requested
  root whose bytes disagree with the captured catalog. Its verification callback now owns the exact
  rooted artifact vector. The subsequent shared-closure checkpoint below replaces the repository's
  remaining delivery-union policy.
- Contract verification moves its loaded snapshot into owned native execution. Snapshot accounting
  and the namespace verification flight are attached to its provider. Both survive optimizer rewrites,
  execution streams and descendant work rather than depending only on the awaiting caller.
- `InputContext` now composes parent physical owners with native provider/plan/stream retention.
  Provider planning, execution and every stream poll establish the scope inherited by DataFusion's
  async/blocking task hooks. An aborted parent cannot release the root held by an active child reader.
- `DurableLocalStore` uses the shared pool for exact file-range buffers before allocation, and
  `Bytes::from_owner` retains each reservation through the last clone/slice. `GetResult` exposes one
  lazy owned range buffer, preserving ownership through its native `bytes()` collection; native
  metadata/preconditions/range normalization remain delegated to `LocalFileSystem`. Metadata lookup,
  range reads and file/directory synchronization run on the separate owned I/O lane. This avoids
  blocking-pool inversion when a synchronous caller occupies the sole compute blocking thread.

**Evidence and limits:** `native-document-units.log`: 2 passed (2.454 s); native payload/document
closure, bounds and UTF-8 coordinates. `snapshot-admission-units.log`: 2 passed (2.434 s); unique
content bounds and component/count/coverage agreement. `immutable-owner-unit.log`: 1 passed
(0.070 s); pruned/unpruned plan and stream ownership plus cancelled blocking reads.
`native-stream-owner-repaired-unit.log`: 1 passed (0.045 s); a blocking native child retains exact
root protection after its stream has been cancelled and dropped. `object-byte-owner-repaired-unit.log`:
1 passed (0.032 s); exact ranges, suffix/precondition behavior, preallocation refusal, last-clone/slice
accounting and one-thread lane independence. These are isolated units, not Delta or service journeys.

The object-buffer unit first exposed a compute-lane metadata deadlock; it was stopped and the
metadata route moved to the I/O lane. An initial child-lifetime fixture held its cancellation notice
outside the spawned future; the corrected fixture owns the whole input and proves parent drop.
Earlier logs also preserve a macro-token spacing failure, DuckDB `IS DISTINCT FROM`/`OR` precedence,
a single-range test lint and direct catalog registration in unit fixtures. Fixes preserve native
contracts and use the shared bound-catalog constructor, with no suppression or rule exemption.

Focused removal guards cover retained validation and owned object-byte routes.
`native-read-repaired-architecture.log`: architecture check passed. The pinned DataFusion capability
scan reports only existing bounded/dormant test collectors and one deliberately independent default
session fixture in `native_delta`; these findings do not identify a new production fallback.

The read-buffer change does **not** measure decoded Parquet metadata/predicate/kernel/writer heap,
transport copies or RSS. A full-file get now produces one pool-admitted buffer; selective Parquet
range reads remain selective. Final performance/pressure qualification remains CP12. Result-root
removal races, durable enrollment/write reconciliation and actual export/CDF/restart journeys remain
unrun. No installation, legacy state retirement, integration qualification or activation occurred.

### Shared result closure, native acquisition and overview — 2026-09-18

**Scope remains CP04–CP07/CP09 work in progress; CP00–CP13 are still open.**

- `result_catalog` now uses one bounded native dependency union for all selected job/comparison
  result roots. It refuses missing/cyclic dependencies, conflicting content descriptors and root
  identity mismatches before physical reads; depth 64, 1024 roots/artifacts and 512 MiB are explicit
  bounds. The repository and bundle export use this owner; procedural delivery-byte/set loops are
  deleted. The exact rooted dependency vector stays with the verification callback.
- `snapshot_validation` compares physical counts for every declared table, including execution
  tables, against the snapshot declaration. Semantic digest and coverage owners remain shared.
- `browse` assembles overview counts, namespaces and child samples in one native plan. Paired
  `{key,value}` records feed the kind-count map; ordered child structs preserve names, flags and
  truncation together. Empty namespaces/inputs have explicit typed empty values. The procedural
  `projection/browse.rs` owner and obsolete summary/count query families are deleted.
- The overview unit exposed a consumer optimizer removing the captured cache base's `Sort(fetch=2)`.
  A materialization now exposes no rewritable logical children. Its binding owns the exact admitted
  base and session; the extension planner prepares that base and exposes its physical child for
  admission. First execution still owns the shared fill. ADR-0052 remains proposed and records this
  refinement; no eager fill or undeclared family was introduced.
- `producer_run_plan` now shares semantic producer binding and qualified input-receipt consensus
  between execution/publication facts and Rustdoc capture. Conflicting URI, length or descriptor
  meaning cannot be resolved by receipt-vector order. A receipt clock is selected only after source
  consensus; known mismatched source versions remain mismatched.
- `research_resolution` owns hosted-build fallback policy and one acquisition presentation plan for
  Rust, Python and revision captures. Absent versus explicitly empty features, independent receipt
  clocks, exact receipt order, gaps and the local-nightly/stable-compiler distinction survive. The
  pending hosted normalization summary is explicit. Separate procedural summaries/coverage/handle
  loops and the old hosted build comparison helper are removed.
- HTTP bodies are now shared `bytes::Bytes` with native reservations through the final clone/slice.
  `OwnedBuffer` pays replacement overlap before growth and enforces the admitted bound. Cache reads
  enroll exact artifact protection before paid reads; Rustdoc input capture also carries an exact
  lease through its blocking callback. `read_artifact` uses the same paid range owner; its unowned
  range helper is deleted. Prefix-kernel copying starts after its existing workspace reservation.

**Executed development evidence:** all logs below are under `.dev-state/plan19/execution/` and
carry no terminal acceptance sidecars.

- `result-closure-repaired-unit.log`: shared native
  dependency-union unit passed (2.499 s); full bundle/restart journeys remain deferred.
- `retained-bound-fixture-units.log`: 4 native document/snapshot units passed (2.444 s), after
  correcting test catalog bindings to the declared admission kind.
- `native-consumer-sealed-cache-units.log`: 5 passed (2.844 s), pure cache planning/identity,
  native overview, qualified producer source and hosted fallback policy.
- `native-acquisition-presentation-unit.log`: 1 passed (2.473 s), ecosystem/mode/gap/receipt
  cases and malformed acquisition refusal.
- `acquisition-owned-buffer-units.log`: 3 passed (2.427 s), incremental preallocation/refusal,
  last shared/sliced buffer lifetime and updated qualified producer-input consensus.
- `native-consumer-composition-clippy.log`: affected application all-target Clippy passed
  (31.52 s). `acquisition-byte-owner-repaired-clippy.log`: affected all-target Clippy passed
  (8.18 s), before the subsequent Arrow ownership work.
- `overview-acquisition-rule.log`: focused structural rule group passed. New acquisition-input
  rule snapshots are authored; their non-updating check remains to be recorded.

Retained failures are explicit: initial overview fixture discarded its inner Result and was not
valid proof; fixing it exposed the captured-base rewrite refusal. The repair was tested with the
actual native consumer. The first Bytes conversion compile found two archive consumers still using
Vec-only access; both now borrow shared slices. Existing pinned Parquet deprecations and the
proc-macro-error2 future-compatibility notice remain unsuppressed.

### Native Arrow buffer ownership — 2026-09-18

The pinned Arrow `pool` and DataFusion execution `arrow_buffer_pool` features are explicitly enabled.
`owned_batch` prepays the native `ArrayData::claim` traversal and splits the reservation into Arrow's
shared buffers, which own it through the final array/batch clone or slice. Native result/fold and
operation-cache output boundaries use this owner. Failed prepayment does not replace an existing
buffer owner. Reclaiming the same buffer replaces its earlier Arrow reservation rather than adding
a second persistent charge; alias traversal is conservatively prepaid during transfer.

Exact local 59.3/55.1 source inspection established that `ArrayData::claim` visits buffers, validity
and children, and that the ordinary DataFusion Arrow adapter uses infallible `grow`. The prepaid
bridge preserves fallible admission before attaching reservations. This is **retained-buffer**
accounting, not proof that all native decoders allocate under a cap, nor allocator/RSS accounting.
Decoded metadata, predicate/kernel/writer workspace, external transport and complete pressure/drain
closure remain CP09/CP12. Native immutable Arrow buffer accounting can coexist conservatively with
an upstream workspace or snapshot envelope; those are distinct ownership categories.

Focused ownership/prefix units passed: `native-arrow-claim-repaired-units.log`, 3 passed (2.407 s).
`native-retained-acquisition-consumer-repaired-units.log`: 16 passed (3.396 s), native cache,
source metadata/commit, qualified fact-source, overview and retained validation consumers.
`native-acquisition-handoff-final-clippy.log`: affected all-target Clippy passed (45.86 s).
`native-python-import-root-units.log`: 1 passed (2.472 s), including the subsequent single-root
release binding. `typed-acquisition-owner-rules.log`: 2 structural rule groups passed.
`acquisition-arrow-architecture.log`: architecture passed. These are development receipts only.

The initial Clippy check found the separately gated DataFusion Arrow reservation implementation
and mandatory `available` method; both were wired. The initial cache-output fixture incorrectly
required a globally empty pool while diagnostics retained reservations; it now verifies release of
that output's charge. The independent buffer-owner unit still proves final-buffer release to zero.
No full integration, package completion, installation or activation follows from these checks.


### Native source metadata and control acknowledgement — 2026-09-18

PyPI versions/release metadata and revision commit/tree responses are typed external source
records. Rust handlers no longer build generic JSON trees or reconstruct release metadata policy.
Native plans select distribution SHA-256, canonical release identity, optional license/project URLs,
and exact commit/tree identity. Release import binding requires exactly one distinct top-level
import root; empty or multiple roots produce no singular binding. Unknown external fields remain
in the retained raw artifact; missing or malformed required known fields refuse.

Control publication replay, selection predecessor/conflict, live claim owner/fence/expiry and
transaction-key selection now share `control/admission.rs`. `PublicationFence` and `SelectionChange`
use native declarations. The corresponding procedural lookup/equality/eligibility branches were
removed from `commit_once`.

Both control append entry points use `control/reconciliation.rs`. An acknowledgement error
refreshes the same table incarnation and compares complete canonical semantic row fingerprints
with native occurrence counts in durable history, including overwritten transitions. Empty,
partial and wrong-value histories do not establish success. A failed refresh preserves an unknown
outcome and cannot masquerade as a retryable conflict. No write is reissued by reconciliation.
The proof does not depend on Delta's expirable application transaction markers. A CP12-only
actual post-commit fault fixture is authored in `tests/control.rs`; it has **not_run**.

Control native evidence: `native-control-policy-repaired-unit.log`, 1 passed (5.442 s), covers
selection predecessors, durable replay/content mismatch, absent/current/wrong/expired claims and
exact transaction keys. `native-control-union-reconciliation-unit.log`, 1 passed (2.388 s), covers
complete/partial/empty/changed/duplicate histories and the actual wide control union.
`native-control-final-clippy.log`: affected all-target Clippy passed (31.97 s), before the
subsequent research-consumer and captured-input changes. `native-control-owner-rule.log`: one rule
group passed; repaired production scan is empty and architecture passed. ADR index/lint passed.
These receipts do not execute the authored post-commit fault or fresh-process journey.

Early native fixtures exposed an unsupported
projected EXISTS shape and the pinned DataFusion EXCEPT ALL membership lowering, which does not
subtract duplicate multiplicities. Explicit native grouped occurrence counts replace both.
Pinned source: `LogicalPlanBuilder::intersect_or_except` in datafusion-expr 55.1 and Delta's
`PostCommit::into_future` at 58f07cd6, where a post-commit hook can return an error after durability.
No CP package or final qualification is closed by this checkpoint.


### Shared immutable input and actual cache consumers — 2026-09-18

The common `native_catalog::batch` boundary now uses `AdmittedProvider::from_batch` and prepays
retained Arrow buffer claims. Its prior bare MemTable exposed mutable provider semantics and was
correctly refused by operation-cache admission. Both actual comparison and search native consumers
exposed this gap; cache admission was not relaxed. The read-only captured provider grants no
uniqueness or foreign-key fact. DataFusion still owns execution, spill, cache preparation and eviction.

Comparison's native changed-key preparation is now one reusable internal function used by its real
page route; its redundant `delta_<axis>` view was deleted. A capacity-one pure Arrow fixture exercises
typed API reconciliation, duplicate alternatives, unchanged documentation, added/removed/changed
keys, narrowed first reads, native total/page consumers and one physical fill.
`native-research-cache-consumer-repaired-units.log` records that comparison pass; its concurrent
search fixture failed because it had not used the actual Delta nullable child layout.

`native-search-cache-consumer-final-unit.log`: 1 passed (24.460 s), actual folded/ranked search,
alias preservation, count/page/hydration, keyset continuation and empty search, with capacity one.
The fixture now uses the declared evidence child layout and a separate work session per request.
The intermediate work-table collision was a fixture reuse error; production work-name collision
refusal remains unchanged. Together with the actual overview fixture and generic materialization
units, all four cache families now have pure consumer evidence; physical storage/retention/pressure
and integrated CF/DC matrices remain CP12.

`native-captured-input-owner` guards the removed mutable ingress construction. The broader focused
consumer group passed: 14 units (5.927 s), `native-input-owner-consumer-units.log`. This checkpoint does not close
CP00–CP13 or permit crossing the CP11 deletion barrier.


### Binary replay values, bounded schemas and native coverage — 2026-09-18

CP01/CP02/CP06/CP08 source work continues; all package-level exits remain open.

- Persisted provider `ReadDescriptor.digest` and positive provider-cache witnesses now use
  `Sha256Digest`: FixedSizeBinary(32) with the registered SHA-256 extension, strict lowercase hex
  only at the wire boundary. DataFusion's SHA-256 expression and the checked binary projection
  supply descriptor construction, native digest admission and replay verification. The previous
  String cache witness and text digest comparison were removed. State generation is now **20**;
  there is no previous-state reader or migration and no installed activation.
- Declaration and derived-field admission share a bounded borrowed Arrow traversal before
  expression inference/schema cloning. It covers ListView/LargeListView, Union, Dictionary and
  RunEndEncoded children, rejects malformed Map/list/dictionary layouts, and applies the same
  depth/field/metadata budgets. Qualified joins may repeat top-level field names; nested records
  cannot. Contract manifests capture those containers' child semantics and Union tags. The
  manifest/intrinsic witnesses are now **2/4**; common physical shape does not hide changed meaning.
- Coverage requests use typed Arrow inputs and native deduplication/selection. Acquisition
  fallback is native; execution requests must all match retained observations before assessing
  their exact producer bindings. The old host-set selection, placeholder SQL and unused subject
  scalar adapter were deleted. Repeated assessments receive fresh work namespaces around the
  same immutable snapshot catalogs. The first fixture exposed both an unannotated conditional
  identity and DuckDB named-argument parsing of Boolean aggregates; typed identity columns plus
  a separate Boolean policy and parenthesized aggregate predicates repair those causes.
- Artifact, row, search, comparison and alternative cursors now share one bounded codec.
  Serialization stops at the actual escaped JSON byte budget before hex expansion; all decoders
  enforce route, width and lowercase representation. Per-family native checksums and exact
  request/snapshot witnesses remain declared by their cursor records. Duplicate unbounded
  encoders/decoders were removed and structural guards prevent their return.

Development receipts: `native-binary-digest-units.log` passed 2 units (2.401 s), including corrupt
checkpoint rejection. `native-schema-shape-units.log` passed 7 units (0.014 s), including hidden
extension rejection and distinct nested semantic contract identities. `native-coverage-cursor-units.log` passed 7 focused units (3.757 s), including missing-artifact
refusal, exact producer scope, cursor family bounds and qualified absence.
`native-values-coverage-final-clippy.log` passed affected all-target Clippy with application
warnings denied (43.00 s), before the following catalog work. These are isolated native checks, not
service, replay, installed or terminal qualification; the CP11 barrier remains in force.


### Declared catalog metadata and segment references — 2026-09-18

The bounded catalog inventory now uses `RelationInfo`, `FieldInfo`, `RuleInfo`, `RuleCondition`
and `InventoryInfo` native declarations. `projection/contracts.rs` and its separate hand-written
Arrow encoders were deleted. Generated metadata tables use captured immutable providers with
shared pool claims instead of exposing a mutable MemTable. Their declared and validated keys
remain separate; constructing metadata grants no data constraint.

Field, source-reference, target-reference and conditional-reference paths are segment sequences.
The dotted-string parser was removed. Literal `nested.value` and nested `nested`/`value` are
independent selections in metadata and native anti-join admission. Generic scoped reference access,
Delta projection and contextual range/artifact checks now use exact unqualified Column names at
root segments. Incoming bound provider schemas are checked before discovery traversal. Dictionary,
list-view, Union and run-end children are included within explicit discovery bounds.

`native-catalog-declaration-units.log` passed 4 isolated units (1.792 s), including exact literal-dot
reference selection, conditional refusal, immutable inventory and truncation. The subsequent
container traversal adjustments passed affected all-target Clippy with application warnings denied
(38.00 s, `native-catalog-path-final-clippy.log`). `native-catalog-values-architecture.log` passed;
new cursor/coverage guard fixtures passed and their production scan was empty. A targeted pinned
DataFusion capability scan was empty. These source checks do not close CP00–CP13.

The replay hash boundary now invokes DataFusion's pinned scalar SHA-256 kernel directly, avoiding
large-payload expression-simplifier cloning. Generation hashes under the existing owned encoder;
replay hashes on the owned native lane with its retained read owner and an admitted input-copy
reservation. Native descriptor admission still uses the optimizer-visible SHA-256 expression.
Pinned `SHAFunc`/`digest_process` source proves scalar input borrowing and a fixed 32-byte output;
this is not a claim that all external decoder/kernel/writer allocation accounting is complete.
`native-catalog-digest-final-units.log` passed 7 isolated checks (2.468 s).


### Field-owned references and complete store Arrow ingress — 2026-09-18

CP01/02/03/04/09/10 advance; CP00–CP13 remain open and the integration/deletion barrier is
unchanged. Source state is now **21**, with manifest/intrinsic witnesses **2/5**. No installed
state has been activated or migrated.

**One field reference owner.** `Rule::ForeignKey` declares a target table, a segment-valued field
and bounded paired source/target scope keys in the same bound catalog/schema. The existing
`Reference`/`ScopedReference` evidence domains and the new relation references expose one shared
descriptor. `field_admission` compiles them into native anti-joins for evidence and control;
requiredness and optional-parent/list presence remain schema properties. Unknown evidence domains
now refuse instead of silently being compared to the release. Evidence admission's release and
environment scope inputs now use native binary identities and typed literals.

Deleted `ReferenceRule`, `RelationContract::references`, the separate evidence/control reference
registry and the duplicate evidence admission loop. Public bindings now declare their definition
reference on the actual Arrow field. Context release/environment/parent, snapshot context, attempt
snapshot and job command references moved to their field declarations. Composite claim command,
projection input vector, selection snapshot/context, comparison before/after and job publication
snapshot/context/attempt checks also moved there. Nine redundant control SQL checks were removed,
including two vocabulary copies already enforced by native vocabulary declarations. Remaining
multi-row/historical/package rules are explicit native plans.

Catalog rule discovery now walks the same declared fields, including nested/scoped references,
and emits typed `ReferenceScope` records. It retains target evidence domains without inventing a
control namespace target for embedded evidence values. This replaces the prior `RuleCondition`
metadata field; there is no alternate reader. Rule metadata does not grant a foreign key or
uniqueness constraint to the optimizer. At the pinned release, DataFusion's
`datafusion_common::functional_dependencies::Constraint` exposes PrimaryKey and Unique; the
reference checks correctly remain native anti-join admission plans.

**One paid immutable Arrow ingress.** Replaced 169 direct single/multiple-batch ingress calls
throughout 62 store source/test files with the shared captured provider. The structural rewrite
receipt is `captured-input-rewrite.json` (166 AST matches); three calls inside assertion macro
bodies were replaced after the lexical audit. The mutable-input refusal unit constructs its
explicit adversarial MemTable directly. No `read_batch`/`read_batches` call remains in store
source. `AdmittedProvider` construction requires the runtime pool, checks identical full batch
schemas and attaches Arrow buffer payment; generated declaration tables use the same owner.
Captured batches reject a missing schema, grant no row constraints and remain independently
addressable native sources. There is no public mutation handle on the captured provider.

**Development evidence.** `native-field-reference-units.log`: 7 passed (2.466 s), including
nullable/list scoped references, wrong scope/missing target, typed release comparison, literal
field dots and rule discovery. All current control schemas validate; changing a foreign target
changes the native contract witness. `native-field-reference-final-clippy.log`: affected all-target
Clippy passed (1m 06s). After the ingress pivot, `immutable-ingress-final-clippy.log` passed (11.43 s).
`field-reference-ingress-architecture.log` passed; targeted DataFusion capability scan was empty;
reference/ingress guard fixtures passed and changed source has a clean whitespace check.

The initial ingress units passed 7 of 8; the lifetime test incorrectly retained its own DataFrame
while asserting last-reader release. It now drops that owner before checking the physical plan
and final output. `immutable-ingress-lifetime-repaired-unit.log` passed the focused rerun (1 unit, 0.009 s).
No integration, publication/CDF/restart,
installed qualification, activation, or terminal package exit is claimed.

### One intrinsic compiler, encoded values and paid contract inputs — 2026-09-18

CP01/02/09/10 advance. **All CP00–CP13 packages remain open.** Current source identities remain
state 21/snapshot 11.0/wire 7.0; manifest/intrinsic witnesses are now **2/7**. No compatibility
reader, migrated state, installed target or final qualification was introduced.

**Deleted the second intrinsic validator.** Evidence and control now share
`native_schema::intrinsic_witness`/`intrinsic_violations`. The recursive role dispatcher and
handwritten enum vocabulary registry were deleted from `evidence/arrow_model/checks.rs`.
Requiredness, tagged payload presence, sibling rules and all row witness identities flow through
the same native compiler. A literal dot in the witness column remains one exact field name.

The negative Python-origin fixture exposed a real declaration bug: a type's vocabulary metadata
overwrote the explicit contextual field rule. `native_vocabulary!` now emits
`enrichment.vocabulary`, independently of `enrichment.rule`. Both restrictions must pass. All
actual vocabulary columns use the type-owned metadata; the generated worker Arrow contract was
regenerated. There is no reader for the overwritten representation.

**Encoded values share admission.** Dictionary values, ListView/LargeListView values, run-end
values and Arrow union branches reach intrinsic/reference/identity traversal. Narrow full-field
projection UDFs delegate decoding and union selection to Arrow 59.3 kernels. DataFusion's native
union tag gates the active branch; null or inactive branches cannot create references. Run-end
offsets have no application value rules; such rules belong to the values Field. No encoded
container is silently accepted merely because the prior walker omitted it. Recognizing these
Arrow containers does not expand Delta's storage representations or the wire codec implicitly.

Exact source basis: the DataFusion skill's
`arrow_select::union_extract::union_extract_by_id` API and pinned Arrow cast source establish the
native kernels. Pinned DataFusion 55.1 `union_extract` reconstructs a Field without top-level
metadata, which is why the narrow wrapper retains the full child Field. Native `union_tag`
observes the selected branch without reinterpreting its identity domain.

**Collection contracts agree.** `Rule::Set` rejects repeated values, including repeated NULLs,
through native `array_distinct`/`array_length`; ordering remains immaterial. `Rule::Sequence`
preserves order and repeats. The generated wire schema projects `uniqueItems`, and the BTreeSet
wire decoder refuses duplicates instead of silently discarding them. Canonical identity
normalization does not substitute for row admission.

**Contract-cache input ownership.** Core manifest comparison now accepts supplied DataFrames.
Its Delta consumer and pure fixture construct those inputs through the paid immutable ingress.
The remaining production `read_batch` in core manifest verification was deleted; the scoped
ingress guard covers that module too. Core's protocol-position `read_batch` is only a unit fixture.

**Development evidence**, all under `.dev-state/plan19/execution/`:

- `single-field-admission-repaired-units.log`: 5 passed (2.343 s), including independent vocabulary
  and contextual rules, all invalid row IDs and nullable nested semantic requiredness.
- `container-set-admission-units.log`: 6 passed / 1 failed (2.497 s). Encoded-container value and
  reference admission passed. The remaining failure located the set wire decoder; its repair
  passed in `native-set-admission-repaired-unit.log` (1 unit, 1.886 s). Earlier fixture compile
  errors and a mistaken unbounded-count expectation for a bounded reference witness remain in
  their logs; they were repaired without weakening the value/reference negatives.
- `native-manifest-encoded-domain-units.log`: 2 passed (2.384 s): an active Union child retains
  its binary Snapshot domain, absent/decoy branches create no dependency, and paid equal/changed
  contract manifests produce the declared violation identity.
- `native-vocabulary-set-schemas.log`: generation and conformance passed; 4 independent fixtures
  validate and 8 negative cases refuse. Datamodel-codegen reports its existing unknown numeric
  format warnings; generated numeric constraints are independently checked.
- `native-admission-owner-repaired-clippy.log`: core/store/daemon all-target Clippy passed
  (27.27 s), after repairing one borrowed-contract call. Two pinned Delta Parquet deprecations
  and the proc-macro-error2 future-compatibility warning remain unsuppressed.
- `native-admission-owner-rules.log`: 3 focused rule groups passed. Production rule scan and
  targeted DataFusion capability scan are empty. Architecture check and whitespace check pass.
- `native-admission-doctor.log`: all hard prerequisites present; Rust 1.98.1 matches the pin.
- The authored adapter still referenced generated wire enum `field_6_0`; it now selects
  `field_7_0`. `native-admission-adapter-{ruff,types-repaired}.log` pass, and
  `native-admission-envelope-unit.log` passes the isolated pure envelope-builder check
  (1 unit, 0.15 s). The earlier explicit lint of `_generated` bypassed the repository's existing
  ADR-0007 lint scope; no generated output was hand-edited and no new exclusion was added.
- Final acceptance generation/check at **2026-09-18 09:56:37 UTC** reports **0 passed / 0 failed /
  0 blocked / 48 not_run** for source `65a22a0cd82764c6066a2ce5ff8560cdba0d68322b3ec2895a0d37d0ab9b5803`.
  Development receipts above are not promoted to complete acceptance gates.
  Independent acceptance re-audit confirmed the current digest, exact registry/report agreement,
  correctly excluded stale historical receipts and no false passes or required downgrades.

The remaining boundary is still the full CP01 identity/clock/bounds inventory, CP02
operator/provider/mutation matrix, CP03 effects/revocation/physical-exit closure, CP05/06 producer
and consumer fidelity, CP07/08 historical/orphan/replay fences, CP09 external metadata/predicate/
kernel/writer accounting, and CP10/11 source/package/enforcement/install/state/client retirement.
No full integration, service publication/CDF/export/restart, installed client qualification or
activation ran. CP12 begins only after the actual deletion barrier.


### Binary operational identities, full-field selections and one diagnostic owner — 2026-09-18

CP00/01/02/03/06/09/10 advance. **All CP00–CP13 remain open.** Current source identities are
**state 22 / snapshot 12.0 / wire 8.0 / native-json/3**. No old-state reader, wire alias,
compatibility path, installed retirement or fresh activation was introduced.

**Operational identities are native values.** One UUID declaration now owns `JobId`,
`InterestId` and `AttemptId`: FixedSizeBinary(16), distinct extension domains, full-field native
literals/SQL parameters, canonical lowercase prefixed wire values and exact generated patterns.
Identical bytes never establish equality between domains. Hash construction refuses a UUID
identity. Job state, claims, cancellation, control publication, retained resolution, producer
runs, ingest selection and exact attempt lookup consume those types. Job IDs are no longer
reused as producer attempt IDs. Path, transaction-key and diagnostic rendering are explicit
boundaries. `Resolution`, the attempt artifact map and publication scope share the declared
attempt identity. The remaining content/grant/policy/process/physical-owner inventory is open.

**Complete Fields survive native construction.** SQL `native_named_struct` retains argument
Fields, and the declared-record constructor uses the same owner. Native `array_agg` over one-field
Structs plus a zero-copy list-value projection preserves scalar identity metadata without a
replacement aggregate. Typed empty collection parameters preserve the same list contract.
Encoded-container compatibility now covers ListView/LargeListView, run-end values and Union
branch tags, including valid null widening and domain-erasure refusal. Native catalog field
inventory includes bounded complete metadata. HTTP cache digests are native SHA-256 values.

Pinned DataFusion 55.1 `CoalesceFunc::return_field_from_args` and scalar `Expr::Case` rebuild a
Field without its metadata. The shared expression helper carries the selected scalar through
one Struct child, delegates selection to native CASE and extracts that child afterwards.
Publication's job coalescing and transaction keys use this route. Mismatched job/attempt branches
refuse; no annotation is restored from a caller-supplied type guess.

**Derived selections are distinct from source declarations.** A typed result may project away
a source reference's sibling context. `check_record_selection` validates its bounded derived
Fields, the complete destination declaration and exact semantic domains; source `check_input`
still requires the reference scope. Physical result Fields remain checked before decoding.
The shared field-shape check is reused rather than maintaining two layout validators.

**One diagnostic contract replaces the JSON bridge.** Single and combined invariant checks now
share native witness projection, with bounds of eight and one respectively. Missing binary
identities remain NULL, rather than becoming prefix-only strings. Telemetry's independently
maintained `Failure` record and `recovery_payload` JSON conversion were deleted. Event history
retains the declared `Diagnostic`, including native tagged recovery actions and sequence-valued
witnesses. This also fixes the old history Set declaration rejecting repeated witnesses that the
wire Diagnostic correctly permits. Qualification receipts now carry `ObservationTime`; native
policy uses that timestamp and formats it only in its human-readable summary. Readiness recovery
actions now use the declared tagged alternative and typed empty collections.

**One wire epoch and protected update proposal.** `SCHEMA_VERSION` derives from the native
`SchemaVersion::Current` vocabulary; the adapter consumes its generated sole member. Retired
handwritten enum-variant references were removed. Schemas, Python DTOs, packaged contracts and
worker Arrow output were regenerated. Independent wire negatives cover wrong UUID domains,
uppercase IDs, noncanonical clock precision and the previous epoch. The active product guide
uses wire 8.0; the new provenance bundle preserves every older sealed byte. Accepted ADR-0047's
argument is unchanged; only an implementation history entry was appended.

The [protected-enforcement patch](../adr/evidence/plan19-enforcement-2026-09-18/README.md) covers
five current instruction files. Its base hashes, exact patch context and proposed post-image
hashes verify; `git apply --check` passes. **It is not applied.** Existing protected files,
hooks and frozen blueprint/handoff were untouched. CP10/11 still require operator installation
and the actual deletion barrier; independent implementation can continue.

**Development receipts**, under `.dev-state/plan19/execution/`:

- `binary-job-native-units.log`: 7 passed, including independent UUID bytes/domain refusal,
  encoded semantics, pure command planning, catalog metadata, storage representation and native
  typed job/interest consumers. This predates the subsequent attempt and diagnostic changes.
- `binary-job-route-repair-unit.log`: 1 passed (2.740 s); declared readiness actions and canonical
  qualification summary. The prior flat-action failure remains in `binary-job-wire-clock-units.log`.
- `typed-attempt-native-units.log`: 2 core units passed; 3 native consumers failed. These located
  plain-text invariant projection and transaction-key formatting, not a reason to erase domains.
  `typed-attempt-witness-repaired-units.log`: the 2 producer composition/admission units passed;
  the remaining 2 failures exposed scalar coalesce metadata and the duplicate diagnostic owner.
- `native-diagnostic-projection-repaired-units.log`: 3 passed (core 0.020 s, store 13.070 s):
  native diagnostic codec, bounded same/cross-domain witnesses/coalesce and pure publication
  predecessor/replay/claim policy. The earlier partial repairs remain in their logs.
- `native-selected-field-boundary-unit.log`: 1 passed; source reference scope still refuses,
  derived projection passes, and erased/wrong identity domains refuse.
- `native-identities-schema-generation.log`: regeneration/reproducibility passed; 4 independent
  fixtures and 11 negatives passed for both packaged and generated schemas. Existing
  datamodel-codegen numeric-format warnings remain; no generated file was hand-edited.
- `uuid-owner-rule-verify.log`: 1 group/12 cases passed; production scan empty.
  `native-diagnostic-rule-verify.log`: 1 group/4 cases passed. Architecture and scoped DataFusion
  capability scans pass; the capability scan is syntactic evidence only.
- `target-enforcement-patch-check.log` and `target-enforcement-manifest-check.log`: read-only
  patch and all five exact base/post-image checks passed, with no installed update.
- `native-identities-final-clippy.log`: core/store/daemon all-target Clippy passed (45.87 s).
  Two pinned Delta Parquet deprecations and proc-macro-error2 future-compatibility remain.
  `native-identities-python-{ruff,types}.log` pass for authored adapter code.
- `native-identities-acceptance-{report,check}.log`: report generation/check at
  **2026-09-18 14:49:25 UTC** passes with **0 passed / 0 failed / 0 blocked / 48 not_run**,
  source `8c8ddf801101371d2aa74652edb960f03f311c0135de4ae09f6dc0b678adc000`.
  Independent re-audit confirmed current-source and exact registry/report agreement, excluded
  stale historical receipts and no false passes or required corrections. Development receipts
  do not establish whole acceptance gates.
- `native-identities-provenance.log`: all five bundle generations verified.
  `native-identities-adr-lint.log`: 56 records, current index and 53 register rows pass.
  `native-identities-doctor.log`: all hard prerequisites present; Rust 1.98.1 matches the pin.

**Next unmet implementation:** complete the remaining typed identity/digest/clock/unit inventory
and full provider/operator/mutation matrix, exact effect/revocation/physical-exit scope,
producer/consumer fidelity, orphan/history/transaction/replay fences, external decoded cache/
kernel/predicate/writer allocation ownership, and L01–L33 source/package/harness retirement.
No full integration, service publication/CDF/export/restart, installed client qualification,
installation or activation ran. CP12 remains behind the actual CP11 barrier.


### Binary grants, generated resource contracts and fresh cached-read history — 2026-09-18

CP00/01/02/03/06/07/08/10 advance. **Plan 19 is not complete; all CP00–CP13 terminal exits
remain open.** Current source epochs: **state 23 / snapshot 13.0 / wire 9.0**, executor **8**,
Rust decoder **native-rustdoc-arrow/4**. There is no predecessor reader or compatibility alias.
The unchanged pool/cache/parallelism defaults remain the performance-oriented values in §4.2.

#### Implemented source scope

| Item | Concrete target implementation | Boundary still open |
|---|---|---|
| Canonical grant identity | `GrantId` is FixedSizeBinary(32), extension domain `EffectGrant`, exact `grant_` wire form. Native claim admission recomputes the key from job/job-key/policy/owner/attempt/fence. Claims, effect records, process authority, static/decoder/semantic consumers and SQL parameters retain this domain. | Other content/policy/process/physical-owner identities and complete effect scope/revocation/exit matrix |
| Full-field SQL selection | `native_selection::coalesce` admits complete Fields and lowers through existing DataFusion CASE/Struct/get-field operations. Literal NULL stays uncoerced until the complete selected Field is known. Durable request and operation policy use it for Context/Snapshot identities. | Complete CP02 operator/provider/mutation matrix |
| One resource catalog | Rust `request::resources` declares the workflow and four operation resources. Request schema projects parameter schemas from operations; native catalog exposes the typed relation; Python constructs FastMCP components from generated bindings. Five handwritten resource functions/decorators are deleted. | Actual SDK transport, all route bodies and installed nine-tool/five-resource qualification |
| Fresh exact-version reuse | Cached exact `DeltaStore::load` reconstructs metadata-only history before returning full cached state. Provider preparation does the same before typed ingredients or verified-descriptor hits. `MetadataTable::verify_table` owns version/metadata/protocol comparison. | Deferred real storage/history-removal/replay/maintenance races |
| Ephemeral read admission | `PreparedRead` is neither cloned nor cached; it binds the exact key, current metadata history and read protection. Provider construction consumes it. Descriptor encoding/decoding share its checks, and metadata-only state never enters the full-file registry. | Complete lifecycle/allocation and fresh-process replay matrices |
| Receipt definition | Source fingerprint now includes vendor, selector/configuration files and installed protected enforcement. Pure fixtures verify additions, mutations and removals change the digest. | Q/SC/CF/DC selector/recipe completeness and terminal source-bound receipts |

The replay/storage regression now warms the provider and descriptor, removes durable log files
while retaining the namespace and allocations, then requires cached exact load, provider and
replay refusal. **It was authored/compiled, not executed**: it is a CP12 service storage journey.
Cache state still owns no read authority. Metadata-only reconstruction is fresh native I/O, not a
claim that cache hits have constant cost or a replacement for complete CDF/file-history proof.

Pinned source basis: DataFusion 55.1 `CoalesceFunc::return_field_from_args`, its simplification
and `try_type_union_resolution` lose or cannot establish the required scalar Field contract.
The narrow UDF preserves it while native CASE performs selection. A unit exposed the
`NULL + FixedSizeBinary` coercion issue; retaining literal NULL until full-field lowering fixes it
without accepting unannotated non-null bytes. Delta's pinned `DeltaTable::load_version` with
`require_files=false` supplies separate fresh metadata history; no private replay algorithm is
copied. FastMCP 4.0.3 `ResourceTemplate`/`TextResource` and provider catalog APIs construct the
adapter directly from native declarations. Skills and exact installed sources were used.

Schemas, packaged schemas/DTOs, Arrow worker output and active guidance were regenerated.
The new `bundle-2026-09-18-native-grants-resources` seals wire-9 guidance and preserves all older
provenance bytes. ADR-0047 has an appended implementation history entry only. The existing
protected-enforcement proposal remains **unapplied**; no protected instructions/hooks changed.

#### Development evidence and repairs

All logs are under `.dev-state/plan19/execution/`. These are scoped development receipts, not
terminal acceptance or full integration:

- `grants-resources-native-units.log`: **5 passed** (12.931 s), canonical grant domain/key,
  Rust resource projection, and exact Python static/semantic grant consumers.
- `native-grants-coalesce-repaired-units.log`: **3 passed / 1 failed**. The failure identified
  native NULL coercion, not permission to weaken domain checks. Earlier fixture-session and
  subsequent cast/Field failures remain in their respective native-coalesce/selection logs.
- `native-selection-admission-repaired-units.log`: coalesce/alias/NULL/physical witness unit
  **passed (12.485 s)**; request fixture failed because it omitted the explicit execution aspect.
  The fixture now proves that omission refuses and supplies Semantics/Runtime explicitly for
  positive requests. `native-request-selection-repaired-unit.log`: **1 passed (12.612 s)**,
  also retaining independent invalid UTF-8 boundary, runtime attribute and result-reference cases.
- `native-resource-receipt-repaired-units.log`: **8 passed (0.87 s)**. Pure FastMCP component
  construction, source identity and receipt checks. Earlier failures found the wire union's flat
  tag payload and FastMCP's removal of unused `$defs`; the generated declaration remains owner.
- `native-grants-resources-schema.log`: generation/reproducibility and **4 independent fixtures /
  11 negative cases** passed. Datamodel-codegen's existing numeric-format warnings remain.
- `native-grants-history-clippy.log`: core/store/daemon **all-target Clippy passed (51.62 s)**.
  Two pinned Delta Parquet deprecations and the proc-macro-error2 future warning remain unsuppressed.
- Authored Python Ruff/ty pass in `native-resources-repaired-ruff.log` and
  `native-resources-types.log`. Generated component rule: **1 group / 5 cases passed**;
  production rule scan and targeted DataFusion/Delta/FastMCP capability scans are empty.
  Architecture check, whitespace, six provenance generations and ADR/index/register lint pass.
- `native-grants-doctor.log`: all hard prerequisites present; Rust 1.98.1 matches the pin.
- Acceptance report/check at **2026-09-18 19:21:09 UTC**: **0 passed / 0 failed / 0 blocked /
  48 not_run** for source `95538d61fbf64ec999f0942a6f96ff139a52e844503b26351354be7170334b63`.
  Independent re-audit confirmed live digest/registry agreement, all 19 protected enforcement
  files in the source identity, correctly excluded stale historical receipts and no discrepancies.
  This is honest report health, not a completed product gate.

#### Next unmet implementation

Continue the full remaining identity/digest/clock/unit/reference inventory (especially immutable
policy/process/effect/physical-owner definitions and typed retention row keys), the operator and
mutation truth tables, exact environment/effect/fidelity consumers, remaining research/recovery
routes, orphan/history/transaction/CDF/replay fences, and metadata/predicate/kernel/writer external
allocation ownership. Finish L01–L33 and Q/SC/CF/DC harnesses; then matching packages and actual
protected enforcement/install/state/client retirement. **CP11 is still open.** No full integration,
service publication/CDF/export/restart, real/installed client or producer qualification, installation
or activation ran in this checkpoint. CP12 remains behind the actual deletion barrier.


### Typed immutable definitions and retention selections — 2026-09-18

**The proposed eight-family definition/retention slice is implemented. Plan 19 remains
incomplete; all CP00–CP13 terminal exits remain open.** Source epochs are **state 24 /
snapshot 14.0 / wire 10.0**, executor **9**. The decoder remains **native-rustdoc-arrow/4**:
its retained effect changed, while its subprocess request/report and fact stream did not.
Resource limits, dependency pins and performance defaults remain unchanged.

#### Implemented architecture and deletions

- `identity/definitions.rs` is the sealed declaration of the eight families: operation policy,
  process operation, process effect, static-worker effect, Rustdoc-decoder effect, semantic
  conversation, registry capture and revision capture. Each binds its payload, domain/key,
  table, identity column and typed retention value. Every identity is FixedSizeBinary(32)
  with its own extension domain and canonical prefixed wire form. Key schemas resolve their
  payload through that declaration; unused procedural policy/process schema helpers are gone.
- `Definitions<D>` derives its contract from the declaration, captures one immutable Arrow
  record once, performs intrinsic admission, selects the typed native digest and returns its
  identity with the exact table/version/contract binding. Reads, canonical-key verification,
  duplicate refusal and dependencies remain native. Loose table/key/column constructors and
  the store's Utf8 identity column, String results and `&str` identity readers are deleted.
- Policies are a direct specialization of the shared owner. The separate policy wrapper is
  deleted; command construction retains the identity/binding pair together in its OnceCell.
  Policy, command/claim/grant, process authority, executor frames/observations, physical
  operation references, worker/decoder effects, conversations and capture consumers now retain
  the appropriate domain. Qualification's definition reference is also a ProcessOperationId,
  as it names the same executor operation. Request telemetry IDs remain separate identities;
  display/log/transaction-label formatting is an explicit text boundary.
- `RowKey.value` is a declared union of eight identity domains plus explicit text for existing
  text columns. One `retention::selection::Selection` admits the full semantic Field and is
  used by definition reads, pending enrollment/reconciliation and physical reclamation.
  Missing/optional columns, wrong domains, missing annotations and wrong widths refuse.
  Native field requiredness includes `enrichment.null`, rather than assuming the Arrow
  nullable bit alone represents the declaration. Column names are literal schema segments.
- Delta deletion lowers only an admitted value through `StorageContract` to its physical
  Binary/Utf8 representation and uses native DeleteBuilder equality expressions. The old
  Utf8-only recovery/deletion guards and old text RowKey decoder are gone. CDF cohort
  dependencies, result selections, fixtures and native settlement now use the tagged value.
  Exact history/incarnation checks, maintenance fences, physical exit and conservative live
  protection remain required. Recovery also verifies the freshly resolved contract identity.
- Full-field analyzer admission now recognizes concrete DataFusion ArrayEmpty, ArrayExcept,
  ArrayConcat and ArrayHasAny implementations. Composition verifies each input against the
  output, overlap verifies operand domains, and collection emptiness observes structure.
  A same-named arbitrary UDF is not trusted. Scalar identity annotations lost by native concat
  still refuse; the retained nested record Fields support the actual dependency arrays.

Pinned capability evidence: DataFusion 55.1 `ScalarAndMetadata`, `empty::ArrayEmpty`,
`except::ArrayExcept`, `concat::ArrayConcat` and `array_has::ArrayHasAny` were checked through
skill indexes and exact installed source. Delta's pinned `operations/delete.rs` contains
`test_delete_binary_equality_non_partition`; the service uses its native predicate interface.
Pure service-native fixtures prove semantic admission and physical binary equality, without
running a Delta storage journey or copying an upstream algorithm.

#### Development evidence

Logs are under `.dev-state/plan19/execution/`. These are scoped development results, not
terminal gate receipts:

- `typed-definitions-units.log`: 6 passed / 2 failed. Passing cases include all eight declared
  contracts, single-record capture, the nine-way row-value domain matrix, registry capture,
  policy/command key sensitivity and pending-removal selection. The failures exposed missing
  collection-operator admission after dependencies acquired nested identity domains.
- `typed-definitions-repaired-units.log`: 10 passed / 2 failed. Typed grant recomputation,
  worker/semantic scopes, CASE preservation and resource-policy refusal passed. ArrayEmpty
  admission was repaired; native array subtraction/concatenation then exposed the remaining
  analyzer omissions. Both failure logs remain preserved.
- `typed-retention-operators-units.log`: **3 passed (12.724 s)** after the full-field operator
  repair: typed selection/domain isolation/native binary lowering, pending-writer settlement,
  and cleanup/settlement across all eight definition families plus text cohort/result keys.
  Negative cases include byte-identical different domains, metadata erasure, wrong width,
  missing/optional fields, old text wire keys and unsupported scalar concat erasure.
  Together these runs provide **13 distinct passing scoped units**, not acceptance gates.
- `typed-definitions-final-clippy-repaired.log`: core/store/daemon **all-target Clippy passed**
  (32.44 s). Earlier Clippy failures were test-module placement and an unnecessary fixture
  clone; both are repaired. Two pinned Delta Parquet deprecations and the proc-macro-error2
  future-compatibility warning remain unsuppressed.
- `typed-definitions-schema.log`: generation/reproducibility, packaged schemas/DTOs/guidance
  and worker Arrow contract pass; **4 independent fixtures / 11 negative cases** pass.
  The obsolete-epoch negative now specifically refuses wire 9.0. Existing generator warnings
  about numeric format names remain.
- `typed-definitions-ruff.log` and `typed-definitions-scoped-ty.log`: changed authored Python
  plus production Python type checking pass. The earlier unrestricted `uv run ty check` in
  `typed-definitions-ty.log` failed with 4,571 diagnostics, predominantly reference corpora,
  plus existing client-fixture/error-preview issues. No ignore lists were changed. This is
  not a claim that a repository-wide Python type check passes.
- Architecture, ADR/index/register lint, whitespace and all seven provenance bundles pass.
  Targeted library scans found only pre-existing `native_delta.rs` test-fixture collect/default
  session/vacuum cases; no new production findings. Doctor reports all hard prerequisites
  present and Rust 1.98.1 matching the pin.
- Acceptance report/check at **2026-09-18 19:56:51 UTC**: **0 passed / 0 failed / 0 blocked /
  48 not_run**, source `388931601deea83f46f105d7c19ce3ed3a1af4f69948d166c31735f2e6943a4d`.
  This is report health and stale-receipt exclusion, not product qualification.
  Independent read-only audit confirms the live digest, 46 frozen/48 registered IDs and zero
  accepted stale executions/skips, with no discrepancies (`typed-delta-acceptance-audit.md`).

The exact-version policy regression is updated and compiled. The new
`typed_definition_unknown_ack_recovery_reclaims_only_unrooted_binary_key` service regression
is authored/compiled: it stages an unacknowledged definition write, records physical exit,
reopens/reconciles its typed pending key, reclaims that row and verifies the separately rooted
policy and its exact historical binding. **It was not executed**, as required by CP11/CP12.

Current guidance is sealed in `bundle-2026-09-18-typed-definitions`; every older seal and
accepted ADR argument is preserved. ADR-0047 has an appended implementation-history entry.
The protected enforcement proposal remains unapplied. No installation, activation, full
integration or real/installed client/producer qualification ran.

#### Next unmet implementation

Continue other content/digest/physical-owner/cohort/reference identities and clock/unit typing;
finish the broader operator/provider/mutation/feature matrix and exact environment/effect/fidelity
consumers. Complete remaining research/recovery routes, orphan/history/transaction/CDF/replay
fences, metadata/predicate/kernel/writer allocation ownership and lifecycle/shutdown matrices.
Finish L01–L33 and Q/SC/CF/DC source/package/enforcement/install/state/client retirement.
**CP11 remains open; CP12 storage, restart and client qualification remains deferred.**


### Typed physical ownership and native cleanup — 2026-09-18

The bounded ownership/retention slice implements six UUID identity domains, a typed inline
retention-policy identity and one native private-directory ownership contract. **Plan 19 remains
incomplete; CP00–CP13 terminal exits remain open.** Source epochs are **state 25 / snapshot 15.0 /
wire 11.0**, executor **9**, decoder **native-rustdoc-arrow/4**. Pins and performance defaults
are unchanged. There are no historical readers, migration paths or compatibility variants.

#### Implemented architecture and deletions

- `RetentionLeaseId`, `CleanupObligationId`, `MaintenanceRunId`, `PrivateDirectoryId`,
  `PhysicalOwnerId` and `StorageReservationId` are distinct FixedSizeBinary(16) domains with
  canonical prefixed wire forms. Native parameters/literals retain full-field metadata. The
  existing native retention-policy key now returns FixedSizeBinary(32) `RetentionPolicyId`;
  the exact policy remains inline in each maintenance run, with no new table or cache.
- The three retention lifecycle records use descriptive `label` fields. Native selection uses
  typed IDs, captured process witnesses, exact dependencies and generation fences. Shared native
  history admission refuses authority rebinding and reversal of physical-release, settlement,
  lease-release or terminal maintenance state. A repeated physical-release acknowledgement
  preserves any already-committed settlement.
- `PrivateDirectoryRef` declares local ID/purpose or export ID/observed parent identity.
  One `Dependency::PrivateDirectory` replaces `PhysicalOwner { name }` and `ExternalDirectory`.
  Cleanup uses UNNEST, full-field identity projection and joins/anti-joins to refuse live readers,
  conflicting scope and malformed cardinality. Native history prevents obligation retargeting
  and fully retired directory reuse while allowing independent parent/child acknowledgements.
  Labels, prefix/regex routing and positional dependency selection are removed as authorities.
- Private directory creation enrolls before bytes, child protection retains the same exact
  reference, and recovery reconstructs the mechanical path from the typed record. Export keeps
  parent device/inode, canonical path, no-link and declared-inventory checks. IDs group owners;
  exact scope remains mandatory. Heterogeneous root-removal blockers are rendered through the
  shared native diagnostic projection before UNION, preserving domain separation.
- Container supervision, physical-owner observations, creator fencing and cleanup retain
  PhysicalOwnerId. The `libenr-...` broker name is rendered only for broker arguments and
  operation filenames. StorageReservationId derives quarantine leaves; native inventory and
  exact-cache reservations select recovery. Redundant stored child/quarantine strings are gone;
  remove/fsync-before-release and capacity accounting remain.
- Generated wire schemas, packaged schemas/DTOs/worker Arrow/guidance and independent fixtures
  advance together. The predecessor wire 10.0 fixture is rejected. The expanded
  `native-uuid-identity-owner` guard rejects the retired text lifecycle keys. The active guide is
  sealed in `bundle-2026-09-18-typed-ownership`; all older seals and accepted ADR arguments remain.
  ADR-0047 gains implementation history only. Protected enforcement remains unapplied.

Pinned DataFusion 55.1/Arrow 59.3 contracts are supplied by the DataFusion skill and installed
sources; Delta remains `58f07cd6`/kernel `8ba063f8` through the Delta skill and existing native
storage contracts. This slice composes the existing full-field expressions and native providers;
no new engine, custom cleanup evaluator or dependency upgrade was introduced.

#### Development evidence

Logs and exact command inventory are in `.dev-state/plan19/execution/typed-ownership-*`.
These are scoped development results, not terminal gate receipts.

- `typed-ownership-focused-units.log`: **10 passed / 3 failed**. Identity bytes/domains,
  native lifecycle monotonicity, quarantine selection, removal diagnostics, row/pending cleanup,
  filesystem refusal and supervisor ownership passed. Three directory cases exposed SQL
  precedence around `IS DISTINCT FROM`; explicit comparison grouping repaired that issue.
- `typed-ownership-directory-repaired-units.log`: **2 passed / 1 failed**. The remaining history
  case exposed an optimizer-generated nested-expression column name at a join. Current/history
  owners now share one named typed directory projection; joins consume those columns.
- `typed-ownership-directory-final-units.log`: **3 passed (12.938 s)**, including conflicting
  scope, malformed dependency count, retargeting, partial three-owner settlement, retired reuse,
  and exact local/export cleanup. `typed-ownership-policy-units.log`: **3 passed (42.576 s)**,
  including binary identity/domain/transport, unsafe horizons and captured policy. Together with
  the first run this gives **16 distinct passing scoped units**, not acceptance gates.
- `typed-ownership-final-clippy.log`: core/store/daemon **all-target Clippy passed**. Two pinned
  Delta Parquet deprecations and the proc-macro-error2 future warning remain unsuppressed.
  Initial compile/Clippy failures from the typed conversion were repaired; logs are retained.
- `typed-ownership-schema.log`: reproducible Rust/generated/packaged schemas, DTOs, worker Arrow
  and guidance pass; **4 independent fixtures / 11 negative cases** pass. Existing code-generator
  format-name warnings remain. Scoped Ruff/ty, architecture, whitespace and ADR/index/register
  lint pass. UUID rule: **1 group / 18 cases passed**; targeted DataFusion/Delta scans are empty.
  All **eight provenance bundles** verify. Doctor reports the Rust 1.98.1 pin and all hard
  prerequisites present; uv is now 0.12.17.
- The first build ran out of disk before executing units. Task-created derived output was
  reclaimed; a subsequent build overlapped the operator's `cargo clean` and lost its output
  directory. Both were rerun successfully. Those logs are build failures, not test evidence.
- Acceptance report/check at **2026-09-18T20:37:18+00:00**: **0 passed / 0 failed / 0 blocked / 48 not_run**,
  source `22d634d824b9a474c89956e13cec1ddeaebd948597239efdd75359ab9e15a875`. This is honest report health and stale-receipt exclusion, not product
  qualification. Independent audit confirmed the live digest, all frozen/registered IDs and
  stale-receipt exclusion with no discrepancies (`typed-ownership-acceptance-audit.md`).

The converted physical-owner, storage-capacity, process reconciliation, export and restart
regressions are authored/compiled only where they require real service storage. No full
integration, storage/publication/CDF/export/restart journey, installed or real-client/producer
qualification, installation, activation or full CI ran. CP12 remains deferred behind CP11.

#### Next unmet implementation

Continue remaining content/digest/cohort/reference identities and clock/unit typing; the complete
operator/provider/mutation/feature matrix; exact effect/environment/fidelity consumers; remaining
research/recovery routes; orphan/history/transaction/CDF/replay fences; metadata/predicate/kernel/
writer allocation ownership and lifecycle/shutdown matrices. Finish L01–L33 and Q/SC/CF/DC
source/package/harness removal plus actual protected enforcement, matching packages and legacy
install/state/client retirement. **CP11 remains open; this slice does not claim whole-pivot closure.**


### Typed Delta references and shared read admission — 2026-09-18

This bounded slice replaces duplicate Delta references and string cohort/schema-contract
identity authority. **Plan 19 remains incomplete; CP00–CP13 terminal exits remain open.** Source
epochs are **state 26 / snapshot 16.0 / wire 12.0**. Executor 9, decoder native-rustdoc-arrow/4,
native-json/3 and delta-immutable-cbor/1 retain their encoding contracts. Dependency pins and
performance defaults are unchanged. No migration reader or compatibility alias is retained.

#### Implemented architecture and deletions

- `CohortId` is a full-field FixedSizeBinary(16) domain with canonical `cohort_` transport.
  `SchemaContractId` is FixedSizeBinary(32), using the existing native schema-manifest hash
  directly and canonical `schema_contract_` transport. Native fields, parameters, row keys,
  filters, cache keys and retained declarations keep typed bytes. Delta properties and path,
  transaction-label and wire boundaries render text. External Delta table IDs remain opaque
  source strings, including non-UUID values.
- One `DeltaTableRef` plus `DeltaVersionRef` supplies publication, definition, result, discovery,
  retention, maintenance and replay scope. `TableSelection` adds an optional typed row key;
  `CdfWindow` carries the same table reference and checked inclusive bounds. The old
  `DefinitionBinding` and `TableVersion` declarations and definition `Binding` alias are deleted.
  Pending writes retain only the available location, typed contract and row selection until
  returned Delta state supplies the committed reference. A shared capture function names the
  actual returned table/version. Candidate control snapshots have no captured table authority.
- The existing semantic-contract registry now has one native declared record with typed identity
  and bounded Arrow IPC bytes. Its raw bootstrap lookup admits the semantic selection before
  lowering to physical Binary, avoiding recursive registration. Positive proofs retain typed
  contract keys. Intrinsic byte-bound predicates explicitly convert length to UInt64 and count
  bytes before comparison; direct physical validation cannot rely on SQL's implicit coercion.
- One exact-read admission checks protected selection, namespace, semantic contract and fresh
  metadata history before provider-cache reuse or replay. Evidence, retained results and
  immutable definitions use it. Provider installation checks the admitted reference and retains
  protection; writer-owned returned-state comparisons remain within their existing obligation.
  Cached provider ingredients compose the shared exact reference and typed cohort with current
  namespace/session witnesses. The full-snapshot registry remains a separate lower-level cache.
- Native CDF dependency planning now operates through a shared function: unique matching
  relation vectors, exact ordered sources, typed cohort row selections and inclusive windows.
  CDF image-key selection uses native typed cohort predicates. Signed CDF bounds, retained end
  commit checks and refusal of clamped feeds remain. Retention, history and cleanup consume the
  shared nested scope; full-field coalescing retains schema-contract metadata. Root removal uses
  the same table-scope projection. Export still rebinds physical references without changing the
  semantic snapshot ID.
- The `native-delta-reference-owner` rule refuses retired text identity fields and flat binding
  declarations. Active transport fixtures advance to wire 12.0; predecessor wire 11.0 is a
  negative case. The guide is sealed in `bundle-2026-09-18-typed-delta-references`, preserving
  previous frozen documentation bytes. ADR-0047 gains implementation history only; protected
  enforcement and frozen blueprint/handoff remain untouched.

The updated DataFusion skill supplies exact 55.1/Arrow 59.3 schema, expression, relational and
coalescing contracts; the Delta skill supplies snapshot, registry, CDF and replay boundaries at
58f07cd6/kernel 8ba063f8. No Context7 or dependency upgrade was used. Metadata preservation was
verified through the affected optimized and physical plans, not inferred from batch construction.

#### Development evidence

Logs, baseline source archive and changed Rust inventory are under
`.dev-state/plan19/execution/typed-delta-*`. These are scoped development checks, not acceptance
receipts for terminal product journeys.

- `typed-delta-focused-units.log`: **3 passed / 12 failed**. Core reference transport, UUID
  domains and schema identity passed. Store units exposed Int32/UInt64 comparison in binary
  intrinsic admission through the newly declared registry and runtime diagnostics bootstrap.
- `typed-delta-repaired-units.log`: **13 passed (46.302 s)** after correcting the shared byte
  bound expression. This includes an added Binary/LargeBinary direct-validation regression.
  Across both runs **16 distinct units pass**: reference/wire/identity domains, full-field
  logical/optimized/physical join and union results, cohort CDF image selection, exact vectors
  and inclusive ranges, typed physical selection, protection refusal, retention/maintenance/
  history/pending-writer cleanup and persisted-descriptor declaration admission.
- `typed-delta-verified-clippy.log`: core/store/daemon **all-target Clippy passed**. The two
  pinned Delta Parquet deprecations and proc-macro-error2 future warning remain unsuppressed.
- Structural rule: **1 group / 13 cases passed**; its scoped source scan and rule review are
  empty. Architecture and all nine provenance bundles pass. Doctor/toolchain check confirms
  Rust 1.98.1 and all hard prerequisites. Repository-wide whitespace checking finds only
  preexisting trailing TSV fields in the separately edited Delta skill; those are preserved.
- `typed-delta-schema.log`: reproducible Rust/generated/packaged schemas, DTOs, worker Arrow
  and guidance pass; **4 independent fixtures / 11 negative cases** pass. Existing generator
  format-name warnings remain. An explicitly targeted Ruff run bypassed the established
  ADR-0007 generated-source exclusion and reported 183 generated style findings; generated
  files were not hand-edited. Maintained-source Ruff and ty both pass using the repository's
  existing scope (`typed-delta-scoped-{ruff,ty}.log`).
- Acceptance report/check at **2026-09-18T22:16:02+00:00**: **0 passed / 0 failed / 0 blocked /
  48 not_run**, source `32738daebe00f615729a05bd44d9eaf6460231316daa43f9389e2b20721972ae`. This is report health and stale-receipt exclusion, not product qualification.
  Independent read-only audit confirms the live digest, 46 frozen/48 registered IDs and zero
  accepted stale executions/skips, with no discrepancies (`typed-delta-acceptance-audit.md`).

Real storage/cache/replay, publication/CDF/export/restart and client/producer journeys are
updated and compiled where affected, but are not executed as terminal qualification. No
installation, activation or full CI ran. **CP12 remains deferred behind actual CP11 closure.**

#### Next unmet implementation

Continue other content/digest/reference identities and clock/unit typing; complete the broader
operator/provider/mutation/feature matrix and exact effect/environment/fidelity consumers.
Finish remaining research/recovery routes, orphan/history/transaction/CDF/replay fences,
metadata/predicate/kernel/writer external allocation ownership and queue/shutdown matrices.
Close L01–L33 and Q/SC/CF/DC source/package/harness removal, actual protected enforcement,
matching packages and legacy install/state/client retirement before terminal qualification.
Typed Delta references complete this bounded consumer conversion; they do not establish
whole-runtime cache/recovery correctness or close CP11.
