# DataFusion and Delta caching: best-in-class utilization review

## 1. Decision and scope

**Proposal:** assess how the service should leverage every caching capability in pinned
DataFusion 55.1.0 and delta-rs `58f07cd6` (with buoyant kernel `8ba063f8`), against the current
tree and the Plan 18 target, and say what best-in-class looks like.

**Status:** current code **Interface-checked** and, where stated, **Measured** on a local fixture;
proposed changes **Proposed**. **Reviewer:** Claude (Fable 5.1), 2026-09-17, at the user's request.
**Affected revisions:** state 11, snapshot 10.0, wire 5.0; Plan 18 §3.1 and CP02/CP07/CP08/CP09;
ADR-0052/ADR-0053 (proposed).

**Observable outcome.** The overview document's three planes — DataFusion `RuntimeEnv` caches,
`CacheFactory`, and Delta snapshot materialization — are all present in the tree, and two of them
are in good shape. The plane that is **not** governed is the one that dominates cost today: Delta
**log replay**. Every consumer opens a fresh `DeltaTable` and replays the log; a protected control
read replays the control table three times, the contracts registry three times, runs three
DataFusion lookups and commits once. Best-in-class here is not a bigger LRU. It is a service-owned
**live snapshot registry** advanced with delta-rs's incremental snapshot update, a **contract
registry cache** for append-only content-addressed rows, and routing CDF through the shared
metadata cache. The operation `CacheFactory` and the shared `FileMetadataCache` should be kept
as they are; the immutable-provider descriptor cache should stop serializing to JSON inside one
process.

**Baseline.** One `RuntimeEnv` per daemon with a 32 GiB `FairSpillPool`, 64 GiB spill, a 2 GiB
`FileMetadataCache`, DataFusion-default 20 MiB statistics and 1 MiB listing caches
(`runtime.rs:778-786`); one `OperationCacheFactory` in the session template (`runtime.rs:847`);
a 1 GiB `DefaultCache` of JSON-encoded `DeltaScanNext` descriptors with a 300 s TTL
(`provider_cache.rs:76-78`); `DeltaStore::load` building a new `DeltaTable` on every call
(`native_delta.rs:482-498`).

**Supported scope and non-goals.** In scope: every cache-shaped mechanism in the two libraries
and in the service. Out of scope: re-reviewing the `CacheFactory` lifecycle (done 2026-09-17),
FastMCP, producers, and blob storage. Policy and rules were deliberately **not** treated as
constraints; where best design needs a rule or ADR change, §10 says so.

**Constraints and uncertainty.** Local NVMe object store only; tables are small (tens of files)
today but control and evidence tables receive one commit per protected read or publication, so
commit counts, not file counts, drive replay cost. Numbers below come from a debug build and a
one-column fixture; treat ratios as evidence and milliseconds as indicative.

### Method and coverage

Read: `runtime.rs` (construction, counters, sessions), `operation_index.rs` and `spill.rs`,
`provider_cache.rs`, `native_delta.rs` (load, open, contracts, provider, CDF), `control.rs`
(load, pin, capture, commit, maintenance), `retention.rs` (`enroll_control`, `require_tables`),
`leases.rs`, `native_discovery.rs`, `delta_evidence.rs`, `kernel_runtime.rs`, `native_policy.rs`,
`telemetry_history.rs`, `config.rs`, `config/service.workstation.toml`. Vendored delta-rs:
`delta_datafusion/session.rs`, `mod.rs` (codec), `table_provider.rs`, `table_provider/next/
mod.rs` and `scan/mod.rs`, `operations/load_cdf.rs`, `kernel/snapshot/{mod,serde}.rs`,
`table/{mod,state,builder,config}.rs`, `kernel/transaction/mod.rs`, `logstore/mod.rs`,
`datafile/properties.rs`. Kernel `8ba063f8`: `snapshot/mod.rs`, `incremental_scan/mod.rs`,
`default-engine/src/parquet.rs`. Skills: DataFusion `sessions-and-runtime`, `parquet`,
`datafusion_execution.cache*`, `datafusion_datasource_parquet.{reader,metadata}`, config
catalog; deltalake `snapshot-and-log`, `table-identity`, `datafusion`, symbol index. Prior
reviews: CacheFactory (2026-09-17) and its P01–P14; Delta follow-up (2026-09-15).

Executed: a six-part probe on the pinned stack, receipt in
[`evidence/datafusion-deltalake-caching-2026-09-17/`](evidence/datafusion-deltalake-caching-2026-09-17/README.md)
(**A1–A4, C, D** ran; **E/B refused** by a current-tree defect, §7 F0). Context7 was not used.

Not inspected: blob store reads, HTTP negative-cache semantics, producer capsule caches, and the
`CacheFactory` route's own lifecycle (covered by the prior review). Not attacked: the correctness
of `FileMetadataCache` validation under file rewrite (Delta never rewrites a data file in place,
so it was accepted as sound), and memory-pressure behavior of any cache. Guarantees stated but
not tested here are labeled asserted.

## 2. Authority and lifecycle map

| Concept | Semantic type and identity | Authority / owner | Snapshot boundary | Permitted update path | Derived representations |
|---|---|---|---|---|---|
| Parquet footer/page metadata | `object_store::Path` → `CachedParquetMetaData`, validated by `ObjectMeta` (`cache_manager.md` `CachedFileMetadataEntry::is_valid_for`) | `RuntimeEnv.cache_manager` (one per daemon) | Immutable file bytes | Transparent fill by `CachedParquetFileReaderFactory` on scan (`scan/mod.rs:703-706`) | Status counters (`runtime.rs:1042-1046`) |
| File statistics / listing | `TableScopedPath` keyed; `SchemaFingerprint` validity | `RuntimeEnv.cache_manager` | n/a | **No consumer in this service** (no `ListingTable`; Delta does not use them) | Two status families that always read zero |
| Operation intermediate | Operation-unique `Binding` (id, family, optimized input, options) | The admitted operation (`operation_index.rs:78-92`) | One fill per binding; disposable | `DataFrame::cache()` → `OperationCacheFactory` → execution-owned spill fill | Native spill IPC; typed telemetry |
| Immutable provider descriptor | `Key{root, table_id, version, cohort, contract, definition, codec, all session options}` (`provider_cache.rs:25-35`) | `ProviderCache` (`DefaultCache`, 1 GiB, 300 s TTL) | Exact Delta version | Miss: `load` + codec encode; hit: JSON decode + `rebind_immutable` | Serialized `DeltaScanNext` bytes |
| **Delta table state (live snapshot)** | `table_root + version (+ checkpoint_version, protocol, metadata)` — the kernel's own `SnapshotIdentity` (`snapshot/mod.rs:100-113`) | **Nobody.** Rebuilt by every `DeltaStore::load` call | Committed version | Should be: forward-only `update_incremental`, exact-version pins served from retained state | `DeltaScanNext`, providers, CDF builders |
| Contract registry row | `contract_id` = content identity of the semantic schema; append-only | Contracts Delta table | Never changes once present | Should be: memoized presence/verified contract; today replay + query per lookup | `StorageContract` |
| CDF metadata | private 1 MiB `DefaultCache` per `CdfLoadBuilder` (`load_cdf.rs:49,93-96`) | delta-rs, per builder | per builder | Not configurable at the pin | none |
| HTTP observation cache | `http_responses` Delta table | `HttpCache` | per commit | `open_or_create` per fetch (`http_cache.rs:50-60`) | n/a |

**Deliberately opaque behavior.** Kernel log replay, checkpoint parsing and Parquet decoding
remain library mechanisms behind `LogStore::engine` and `DeltaScanNext`. The kernel's CRC-backed
domain-metadata cache (`snapshot/mod.rs:468`, "zero I/O" when the CRC is resolved) is used
implicitly and needs no service surface.

**Identity behavior.** A Delta version is immutable; serving a retained snapshot for an exact
version request is semantically identical to replaying the log to that version, as long as the
log has not been truncated below it. Retention fences (`retention.rs`, log floor) already govern
truncation; the registry must consult them.

## 3. Semantic contracts and invariants

| Contract or invariant | Representation | Enforcement boundary | Failure behavior | Evidence |
|---|---|---|---|---|
| Shared metadata cache is actually consumed by Delta scans | `state.runtime_env().cache_manager.get_file_metadata_cache()` in `scan/mod.rs:703-706` | Every next-scan | n/a | **Measured** probe D: 65 entries after first scan, 65 hits per later scan |
| Session Parquet options reach Delta scans | `ReaderProperties::to_table_parquet_options` copies `session.config().options().execution.parquet` (`datafile/properties.rs:17-25`); `DeltaScanConfig::new_from_session` maps `pushdown_filters` to `enable_parquet_pushdown` (`table_provider.rs:200-209`) | Scan planning | Silent if the session is not the bound one — mitigated by `SessionFallbackPolicy::RequireSessionState` everywhere (`native_delta.rs:574-575,1115-1116`, `control.rs:1349-1351`) | Interface-checked; `decoder_filter` defaults true (`config.rs:149`) so pushdown is on |
| A retained snapshot equals a replayed one | delta-rs `Snapshot::update` uses `KernelSnapshot::builder_from(existing)` and seeds materialization from the existing batches (`snapshot/mod.rs:291-345`; seeded branch calls `files_from_preserving_raw(existing_version, existing_data)`) | Library | `VersionDowngrade` refuses backward updates; same-version refresh re-checks checkpoints | **Measured** A2 advanced 60→65 in 11–14 ms vs 26–31 ms replay; equality of file sets **asserted** (not diffed here) |
| Descriptor cache membership conveys no authority | Key includes identity/version/cohort/contract/definition/options; `validate_identity` + `rebind_immutable` on hit (`provider_cache.rs:167-182`; `next/mod.rs:533-565`) | Every hit | Plan error | Interface-checked; provider_cache units (STATUS) |
| Contract lookup is correct after first registration | `contract_exists` → `native_contract::changes` → `require_empty` | `provider_preparation` | **Refuses every warm lookup** (`native_contract.rs:164` returns five columns; `runtime.rs:1262-1265` requires one) | **Tested (negative)** probe E/B refused; see §7 F0 |
| Unconsumed caches are not reported as capabilities | `NativeCacheFamily::{FileStatistics, FileListings}` populated from `get_file_statistic_cache()/get_list_files_cache()` (`runtime.rs:1050-1068`) | status | Always `entries: 0` | Interface-checked |

**Absence and uncertainty.** A cache miss is never evidence (§8.4 of `DESIGN.md` holds). A
registry cache must cache only *positive* presence; a negative lookup is unknown, not absent.
A snapshot registry entry below the retention log floor is *invalid*, not stale.

**Equivalence requirements.** Snapshot registry: exact version equality, protocol and metadata
equality (the kernel's `SnapshotIdentity::is_for`). Descriptor cache: byte-exact semantic
schema and scan config, already enforced by `rebind_immutable`.

## 4. Derivation and execution design

What the pinned libraries offer, what the service uses, and the gap:

| Capability (canonical path) | Pinned behavior | Service today | Best-in-class use |
|---|---|---|---|
| `RuntimeEnvBuilder::with_metadata_cache_limit` / `CacheManagerConfig::with_file_metadata_cache` | 50 MiB default; custom `Arc<dyn Cache>` injectable | 2 GiB limit; no concrete handle, so `occupied_bytes: None` | Build the metadata cache yourself with `DefaultCache::new(limit).with_name(..)`, inject via `with_cache_manager`, keep the concrete `Arc` for exact `memory_used()` and `remove(path)` on physical deletion |
| `with_file_statistics_cache_limit`, `with_object_list_cache_limit/_ttl` | 20 MiB / 1 MiB defaults, no TTL | Allocated, unconsumed, reported | Set both limits to **0** and drop the two wire families, or keep with a recorded "no consumer" disposition |
| `CachedParquetFileReaderFactory` (`datafusion_datasource_parquet::reader`) | Uses the runtime metadata cache | Used by Delta next scan (E10) | Unchanged. Also wire it into the CDF builder (vendor seam; `load_cdf.rs:93-96` hard-codes 1 MiB) |
| `datafusion.execution.parquet.{pruning, enable_page_index, bloom_filter_on_read, metadata_size_hint, pushdown_filters, reorder_filters}` | Defaults true/true/true/512 KiB/false/false | `pushdown_filters`, `reorder_filters` bound from policy; others default | Unchanged; record `metadata_size_hint` relative to our 256 MiB target files at CP12 (a footer with page index can exceed 512 KiB → two fetches) |
| `datafusion.execution.parquet.max_predicate_cache_size` | NULL (unbounded predicate cache when pushdown on) | Not set | Bound it explicitly from policy; it is the one Parquet-level cache not under a service limit |
| `SessionStateBuilder::with_cache_factory` | Hook only (P01–P14) | Operation-owned factory, execution-time fill, spill backing | Unchanged (ADR-0052). Minor: precompute the option digest instead of re-listing/sorting every config entry in `options()` on each `require` (`operation_index.rs:95-104,111-118`) |
| `DeltaSessionContext::with_runtime_env`, `DeltaSessionConfig`, `DeltaRuntimeEnvBuilder` (`session.rs:274-380`) | Delta defaults + `DeltaPlanner`; builder wraps only spill/temp | Not used; the service composes its own template with `NativePlanner` and injects `SessionFallbackPolicy::RequireSessionState` | Correct as is. Do not adopt: the service already carries the two settings that matter (`schema_force_view_types` per contract; ident normalization irrelevant to DataFrame plans) and its composed planner |
| `DeltaTable::update_incremental` / `update_state` (`table/mod.rs:199-231`) → `Snapshot::update` (incremental log segment + seeded file materialization) | Forward-only; same-version checkpoint refresh | **Unused**; every `load` rebuilds | **Adopt** as the refresh path of a per-table live snapshot registry |
| `DeltaTableBuilder::without_files()` / `with_skip_stats` / `with_log_buffer_size` (`builder.rs:41-63,141-166`) | Lazy P&M snapshot; skip stats; parallel log reads | Unused (`DeltaTableConfig::default()`) | `without_files()` for identity/contract checks and discovery capture that never scans; never write through it (post-commit checkpoint is skipped for `require_files=false`, `transaction/mod.rs:1152-1156`) |
| `Snapshot` serde with Arrow IPC batches and identity validation (`snapshot/serde.rs:198-295`) | Cross-process warm start of materialized files | Unused | Not now (A4 shows a checkpoint makes cold open 6–10 ms); revisit trigger in §10 |
| `delta.checkpointInterval` (default 100, `table/config.rs:266-268`; post-commit hook `transaction/mod.rs:1159-1161`) | Auto checkpoint every 100 commits | Not set; explicit checkpoints only in control OPTIMIZE (`control.rs:1362`) | Set per table class from the retention policy (control: ~10; evidence/results: default) — A1 vs A4 is a 4× cold-open difference at 60 commits |
| Kernel `Snapshot::builder_from`, `incremental_scan_builder`, `estimated_owned_heap_size_bytes` (`snapshot/mod.rs:121,784,332`) | Incremental listing and size estimate | Reached only through delta-rs | Use the heap estimate plus materialized batch sizes to bound the registry; no direct kernel calls needed |
| `DeltaLogicalCodec::encode_immutable_provider` (vendor seam, `mod.rs:499-511`) | `serde_json` of `DeltaScanNext` including IPC bytes as a JSON number array | Every hit decodes JSON | Keep the codec **only** for persisted descriptors (CP08 replay). In-process, cache the typed `Arc<Snapshot>` + `DeltaScanConfig`; **Measured** C: JSON is 7.2× the IPC inventory and costs a 3 ms parse per 65 files |
| `Cache::list_entries / drop_table_entries / update_cache_limit / update_cache_ttl` | Inspection, table-scoped invalidation, live resizing | `len`/`cache_limit` only | Use `list_entries` sizes for CP09 occupancy; `remove(path)` after VACUUM/physical release (metadata keys are plain paths, not table-scoped) |
| `delta-cache` feature (`foyer`) | Declared, no source references at the pin | Not enabled | Do not enable (dormant scaffolding) |
| Custom `AsyncFileReader` byte/range cache | Recommended by DF for remote stores | Not present | Do not add: local NVMe, page cache already serves bytes (DM-58) |

**Provider selection and limitations.** delta-rs at the pin has three sharp edges the design
must route around: the CDF builder's private cache, `DeltaLogicalCodec`'s unimplemented logical
hooks (`mod.rs:520-525`), and the checkpoint hook skipping lazy tables. The kernel default engine
has **no Parquet footer cache** (`default-engine/src/parquet.rs:401-427` reads the footer per call),
so every full replay re-reads checkpoint footers; a retained snapshot removes that cost entirely.

**Boundary contracts.** A registry entry is a typed value (`DeltaTable`/`Arc<Snapshot>`), never
bytes, never a lease. Read authority stays where it is: `ReadProtection` and retention anti-joins
(`leases.rs:122-158`, `retention.rs:134-146`) are attached *after* the snapshot is obtained.

**Coherent publication.** Writers (`append`, `commit_native`, `add_constraint`, OPTIMIZE) receive
the post-commit table from delta-rs; the registry stores that value (the pattern
`telemetry_history.rs:394-408` already uses) so the next read is current without I/O. A
`transaction_conflict` discards the entry and reloads; that is the only backward path.

## 5. Representative journeys

**Protected control read today** (`control.rs:1018-1035` → `retention.rs:516-560` →
`control.rs:1038-1042,998-1012` → `native_delta.rs:313-345,698-716`): `pin()` → `enroll_control`
→ `capture()` → `load()` → `open_or_create` → `register_contract` (contracts replay + lookup) +
`load("control")` (replay) → `pin_table` → `provider()` → `require_contract` (contracts replay +
lookup) → lease **commit** → `load("control", Some(v))` (replay) → `pin_table` → `require_contract`
(replay + lookup). Three control replays, three contracts replays, three DataFusion lookups,
one commit. With `commit_native` also reloading before every commit (`control.rs:1230-1250`) and
never retaining the returned table, the control log grows by one commit per read and each read
gets slower until the next checkpoint (at most 100 commits, or the next OPTIMIZE).

**Same read with the proposed registry:** `pin()` → registry `current("control")` (0 ms if a
writer already stored the post-commit table; ~12 ms `update_incremental` otherwise) →
contract cache hit (0 ms) → lease commit (unchanged; the returned table is stored) →
registry `at("control", v)` (the value just stored) → provider. One incremental step, zero
replays, zero registry lookups.

**Evidence read (`delta_evidence.rs:52-90`):** unchanged on hit (descriptor cache), except the
JSON decode disappears when the cache holds the typed snapshot. On miss, `require_contract`'s
replay disappears with the contract cache; the version pin comes from the registry when the
version is current.

**Publication with CDF (`native_delta.rs:733-780`):** `changes()` loads the table at `end`
(registry), then `scan_cdf()` builds a `CdfLoadBuilder` whose Parquet reads use a private
1 MiB metadata cache regardless of the 2 GiB configured limit. After the vendor seam change,
CDF footers land in the shared cache and are already warm when the ordinary scan of the same
version follows.

**Interruption:** a registry entry dropped mid-`update_incremental` leaves the old value in
place (the update returns a new `Arc`); a failed update is an error, never a partial snapshot.
Restart: the registry is empty; first reads replay once per table (A4 cost with checkpoints).
Log truncation below a pinned version: the registry refuses to serve it and the caller's
retention check fails as today.

## 6. Acceptance gates

| Gate | Verdict | Evidence or scope rationale | Required action |
|---|---|---|---|
| G1 — Authority | **Pass** for the three existing planes; **unresolved** for Delta table state | Owners are explicit for metadata cache, operation cache and descriptors (§2). Delta snapshots have no owner: rebuilt per call | Give `DeltaStore` a live snapshot registry; record it as a fourth lifetime in Plan 18 §3.1 |
| G2 — Semantic fidelity | **Pass** | Descriptor hits revalidate identity/contract; session Parquet options propagate to Delta scans; retained snapshots are version-exact by protocol | Add a differential unit: file set of `update_incremental` equals a fresh replay at the same version |
| G3 — Validity | **Fail** (current tree) | Warm contract lookup refuses on a shape mismatch between `changes()` and `require_empty` (F0) | Fix before any registry cache lands; add the negative to `native_contract` units |
| G4 — Hidden behavior | **Fail** | (a) Each protected read performs a hidden Delta commit plus three replays; (b) CDF reads silently bypass the configured metadata-cache limit through a private cache; (c) two status cache families report capability that has no consumer | Registry + contract cache remove (a); vendor seam removes (b); disable the two caches or mark them (c) |
| G5 — Consistency and recovery | **Pass**, scoped | Eviction never releases live readers (P13, provider units); conflicts reload; no persisted operation state | Registry design must specify conflict/truncation handling (§9) |
| G6 — Transformation and reuse | **Pass** with one observation | Reuse is keyed to exact semantic dependencies. In-process JSON encode/decode of descriptors is a lossless but pointless transformation (C) | Cache typed snapshots in-process; keep bytes for persisted descriptors only |
| G7 — Truthful capability claims | **Pass**, scoped | Claims here are Interface-checked or Measured with receipts; status counters overstate cache surface (see G4c) | Disable unconsumed caches; report `occupied_bytes` from a concrete cache handle |

## 7. Principle findings

| Finding | Principle IDs | Concrete evidence or gap | Consequence | Proposed correction | Verification |
|---|---|---|---|---|---|
| **F0 Blocking (out of caching scope): warm contract lookups refuse** | DM-07, DM-53 | `native_contract.rs:164` returns `projection, path, kind, before, after`; `runtime.rs:1262-1265` requires exactly one column; `native_delta.rs:637-641` feeds one to the other. Probe E/B reproduced it three times | `open_or_create`, `provider()`, `pin()` and `require_contract` fail for every already-registered contract; nothing that reopens a table works | Project the diff to one identity column (or give `require_empty` a declared identity selector) | Unit: register, reopen, expect `Ok`; **none exists today** — add to `native_contract` units and to Q01 |
| **F1 High: Delta log replay has no owner and is repeated per call** | DM-02, DM-26, DM-32, DM-36 | `native_delta.rs:482-498` builds a new `DeltaTable` per load; `control.rs:998-1042,1230-1250` reload three times per pin and once per commit; `http_cache.rs:50-60` reopens per fetch; no production use of `update_incremental` (grep) | Per-read latency grows linearly with commits since checkpoint × 3; **Measured** 26–31 ms per replay at 60 commits vs 11–14 ms incremental and ~0 ms when the writer stored its result | Live snapshot registry in `DeltaStore`: one retained table per name, `update_incremental` for "current", exact-version pins served from retained or recently seen versions, writers store post-commit tables, conflicts discard | `ast-grep` rule `rules/delta-load-only-through-registry.yml`: refuse `DeltaTable::new(`/`.load_version(`/`.load()` outside the registry module; unit counting kernel snapshot-load events per N pins |
| **F2 High: contract registry is replayed and queried on every provider** | DM-32, DM-33, DM-56 | `require_contract` (`native_delta.rs:698-704`) and `register_contract` (`:646-680`) each do a full `load(CONTRACT_TABLE)` plus a DataFusion filter; called from `provider()`, `open_or_create`, `pin_table`, `immutable_provider` miss | A second replay per read; three per pin | `DefaultCache<ContractId, VerifiedContract>` with no TTL; positive entries only (rows are append-only and content-addressed); misses take the current path | Unit: two lookups → one replay; negative never cached; `ast-grep` rule refusing `load(CONTRACT_TABLE` outside the cache module |
| **F3 Medium: CDF bypasses the shared metadata cache** | DM-19, DM-43, DM-31 | `load_cdf.rs:49,93-96`: private `DefaultCache::new(1 MiB)` per builder; the service's `changes()` supplies a session but the builder ignores its runtime | Publication/replay CDF reads re-fetch footers and never warm the cache the following scan uses; the configured 2 GiB limit does not govern CDF | Vendor seam: `CdfLoadBuilder::with_session_state` sets `parquet_metadata_cache` from `state.runtime_env().cache_manager.get_file_metadata_cache()`; upstream PR | Unit: after `scan_cdf` the runtime metadata cache has entries; `ast-grep` rule refusing `DefaultCache::new(` outside `runtime.rs`/registry modules in **our** crates |
| **F4 Medium: in-process descriptor cache serializes to JSON and back** | DM-37, DM-41, DM-58 | `provider_cache.rs:167-182,195-201`: hit path JSON-decodes then rebinds; `serde.rs:198-229` puts IPC bytes inside JSON; **Measured** C: 1,057 B/file, 7.2× the IPC inventory, 3 ms decode per 65 files | Linear parse cost per hit; 1 GiB holds ~1M file entries of JSON where IPC would hold ~7M; the reservation budget (`bytes × 8`) is sized to the inflated form | Cache `Arc<Snapshot>` + `DeltaScanConfig` in-process (rebind via `DeltaScan::new` + `rebind_immutable`'s checks); keep `encode_immutable_provider` for CP08 persisted descriptors, but make the vendor codec binary (IPC-native) there | Provider units: hit path performs no `serde_json` call (rule: refuse `serde_json::from_slice` in `provider_cache.rs`); C-style size assertion in units |
| **F5 Low: checkpoint cadence is not a policy** | DM-16, DM-36, DM-39 | No `delta.checkpointInterval` set (grep); delta-rs default 100 (`config.rs:266-268`); explicit checkpoint only in OPTIMIZE (`control.rs:1362`); **Measured** A1 vs A4: 26–31 → 6.6–10.5 ms | Control table cold opens degrade for up to 99 commits; every restart pays it per table | Set `delta.checkpointInterval` per table class at create (control ~10, results/evidence default) from `RetentionPolicy`; keep OPTIMIZE-time checkpoints | Unit: create → interval property present; CP12 measures cold open vs interval |
| **F6 Low: unconsumed caches are reported as capabilities** | DM-08, DM-59 | `runtime.rs:1050-1068` exposes `FileStatistics`/`FileListings`; no `ListingTable` in the service; Delta does not use them | `service_status` shows two caches that cannot ever fill; readers infer a capability | `with_file_statistics_cache_limit(0)`, `with_object_list_cache_limit(0)`; remove the two `NativeCacheFamily` variants (wire change) or annotate "no consumer" | Schema conformance after regeneration; unit asserting the manager returns `None` for both |
| **F7 Low: metadata cache occupancy and invalidation are unreachable** | DM-50, DM-33 | `occupied_bytes: None` for the metadata family; no `remove` after physical deletion; `FileMetadataCache` keys are plain `Path`s so `drop_table_entries` is inapplicable | CP09 cannot report real occupancy; footers of vacuumed files linger until LRU | Construct the metadata `DefaultCache` in `QueryRuntime`, inject through `CacheManagerConfig::with_file_metadata_cache`, keep the concrete handle for `memory_used()` and `remove(path)` in the CP07 reclamation path | Unit: after physical release the cache no longer contains the path; status shows `occupied_bytes: Some` |
| **F8 Observation: kernel engine per load, options re-listed per require** | DM-58 | `kernel_runtime.rs:187-195` builds a `DefaultEngine` per `engine()` call; `operation_index.rs:95-104` materializes and sorts every config entry on each `require` and in the descriptor key | Small constant costs; both shrink with F1 | Retain one `Arc<dyn Engine>` per `OwnedLogStore`; hash options once per session | None needed; measure at CP12 |

**Applicability.** Authority/identity (1, 3), reuse/invalidation (7), lowering/interchange (5,
9), physical layout (8), provenance/observation (10) and verification (11, 12) bear directly.
Domain modeling (2), Python conversion, MCP wire generation and product discovery did not bear
on this scope and were not re-reviewed; their exclusion is not assurance.

## 8. Alternatives and architectural leverage

| Alternative | Semantic duplication / locality | Correctness and operational risk | Cost | Performance evidence | Decision |
|---|---|---|---|---|---|
| Current baseline: replay per call, JSON descriptors, private CDF cache | One replay implementation (delta-rs) invoked everywhere | Correct; hidden commit-per-read growth; CDF outside policy | none | A1, C, D | Baseline |
| **Proposed: live snapshot registry + contract cache + CDF seam + typed in-process descriptors** | One owner for table state; reuses delta-rs incremental update and DataFusion `DefaultCache`; no new eviction algorithm | Must handle conflict, truncation below log floor, and lazy/eager handle discipline | Moderate: `DeltaStore` (registry), `native_delta` (contracts), `provider_cache` (typed values), vendor seam (CDF), config (checkpoint interval) | A2/A3/A4 ratios; D shows the shared cache works | **Recommended** |
| Simpler: keep replay per call, only raise `delta.checkpointInterval` and fix F0 | No new state | Lowest risk | Trivial | A4: replay drops to 6.6–10.5 ms but still ×3 per pin | Do first regardless; insufficient alone because the commit-per-read growth remains |
| Persisted `Snapshot` serde warm start | Adds a durable cache with its own validity | Identity-checked by delta-rs; still a second copy of log state | Moderate | No evidence of cold-open pain on NVMe with checkpoints | Reject now; §10 trigger |
| `AsyncFileReader` byte cache / `foyer` | Duplicates the OS page cache locally | Low | Moderate | None | Reject (DM-58) |

**Abstractions justified by current needs.** The registry is a bounded map of typed values
per table name with two operations (`current`, `at(version)`) and two mutation paths
(writer store, conflict discard). It is not a cache platform. The contract cache is a
`DefaultCache` with typed keys and no TTL.

**What remains ordinary code.** Kernel replay, incremental scans, Parquet decoding, spill IPC.

## 9. Verification and measurement plan

| Claim or risk | Label now | Oracle | Conditions and expected result | Gap |
|---|---|---|---|---|
| Incremental update equals fresh replay | Measured (timing only) | Differential unit: compare `get_file_uris()` sets and version after `update_incremental` vs `load_version` | Equal sets and versions across 5 foreign commits and one checkpoint | Not yet written |
| One replay per table per process for N pins | Proposed | Kernel metrics: count snapshot-load events (`kernel_metrics.rs` dispatch) during N `pin()` calls | ≤1 full load plus ≤N incremental steps | Blocked by F0 |
| Contract cache never grants absence | Proposed | Unit: lookup of an unregistered id twice → two registry queries; after registration → one | Negative never cached | Not yet written |
| CDF uses the shared cache | Proposed | Unit: `scan_cdf` then inspect `get_file_metadata_cache().len()` | > 0 entries, hits on the following ordinary scan | Requires the vendor seam |
| Descriptor hit path performs no serialization | Proposed | Rule + unit | No `serde_json` in `provider_cache.rs`; hit cost independent of file count | Requires F4 |
| Registry refuses versions below the log floor | Proposed | Unit with truncated log | Explicit error, no partial snapshot | Not yet written |
| End-to-end cost | Not run | CP12 journeys: control pin p50/p95, evidence read, publication with CDF, cold open per table after restart | Report absolute numbers with commit counts and checkpoint state | Post-barrier |

**Cost accounting.** Registry memory: kernel `estimated_owned_heap_size_bytes()` plus
materialized batch `get_array_memory_size()`, bounded per table and in total; charge to the
native pool like descriptor reservations. Contract cache: bytes of the semantic schema per
entry. Metadata cache: exact `memory_used()` once the handle is concrete.

## 10. Exceptions and unresolved decisions

Rules and decisions that should change to enable the design (the user asked for these to be
named rather than avoided):

- **ADR-0053 (proposed) amendment.** In-process descriptor values become typed `Arc<Snapshot>` +
  `DeltaScanConfig`; the restricted codec is reserved for persisted descriptors (CP08). Keys keep
  the same exact vector but hash the session options.
- **Plan 18 §3.1 "three cache lifetimes" → four.** Add "Live Delta table state: forward-only
  incremental, service-owned, version-exact pins, conflict/truncation invalidation; owner
  `DeltaStore`". CP07 gains the registry's log-floor consultation and `remove(path)` after
  physical release; CP09 gains registry counters; CP02 gains `delta.checkpointInterval` at create.
- **`DESIGN.md` §8.4 wording.** "No cross-operation result cache" stays true; add the sentence
  that committed Delta versions and content-addressed contract rows are immutable state, so
  retaining them across operations is reuse of authority, not a result cache.
- **Vendor seam policy.** A second narrow patch (CDF cache injection) under the same recorded
  vendor-seam discipline as `rebind_immutable`, with an upstream PR link as its revisit trigger.
- **Wire.** Dropping `FileStatistics`/`FileListings` from `NativeCacheFamily` is a wire change;
  if wire 5.0 must stay, keep the variants and set limits to 0 with a documented disposition.
- **Unused-capability register rows** (owner: Plan 18 CP12; revisit triggers): persisted
  `Snapshot` serde — trigger: cold-open p95 above the request budget on a checkpointed table, or
  tables above 10⁵ files; `AsyncFileReader` byte cache — trigger: a remote object store profile;
  `delta-cache`/`foyer` — trigger: upstream wires it into the scan path.

No MUST-level exception is requested. F0 is a defect, not an exception.

## 11. Decision and implementation changes

**Decision: Revise.** The DataFusion and operation-cache planes are used well and should not be
touched beyond telemetry. The Delta plane is under-governed: fix F0, then give table state an
owner. Adopt the following in Plan 18 order.

| Priority | Change | Principle IDs | Acceptance evidence | Regression protection |
|---|---|---|---|---|
| 1 — now | Fix the `changes()`/`require_empty` shape mismatch (F0) | DM-07, DM-53 | Register, reopen, provider, pin all succeed in a unit; probe E/B rerun and recorded | New negative in `native_contract` units; Q01 |
| 2 — CP02 | Set `delta.checkpointInterval` per table class at create (F5) | DM-16, DM-36 | Property present; cold open measured at CP12 | Unit on create |
| 3 — CP02/CP03 | Live snapshot registry in `DeltaStore`; writers store post-commit tables; `pin`/`capture`/`commit_native`/`HttpCache` consume it (F1) | DM-02, DM-26, DM-32, DM-35 | Differential and event-count units; CP12 pin latency | `rules/delta-load-only-through-registry.yml` |
| 4 — CP02 | Contract registry cache with positive-only entries (F2) | DM-32, DM-33 | Two lookups → one replay | `rules/no-contract-table-load-outside-cache.yml` |
| 5 — CP08 | Typed in-process descriptor values; binary codec for persisted descriptors; hashed option key (F4) | DM-37, DM-41 | No serialization on hit; size assertion | Rule refusing `serde_json` in `provider_cache.rs` |
| 6 — CP08 | Vendor seam: CDF builder consumes the session's metadata cache (F3) | DM-19, DM-43 | Cache entries after `scan_cdf` | Rule refusing `DefaultCache::new(` outside `runtime.rs`/registry modules in our crates |
| 7 — CP09 | Concrete metadata-cache handle: occupancy, `remove(path)` on physical release; disable statistics/listing caches or mark no-consumer (F6, F7) | DM-08, DM-50 | Status shows exact occupancy; schema conformance | Unit on manager configuration |
| 8 — CP12 | Measure: pin p50/p95 by commits-since-checkpoint, cold open per table, CDF publication, descriptor hit cost; record unused dispositions | DM-39, DM-59 | Absolute numbers with conditions | Acceptance report |

**Final check.** Claims above are labeled at the strength of their evidence: library behavior
Interface-checked against vendored source; A1–A4/C/D Measured on a debug fixture with receipts;
the registry and cache designs Proposed. The scope excludes remote stores and persisted warm
start on purpose, and each exclusion has a trigger.
