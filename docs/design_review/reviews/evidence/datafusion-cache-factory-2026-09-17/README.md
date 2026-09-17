# DataFusion cache factory investigation — 2026-09-17

Evidence for [the design assessment](../../design_review_datafusion-cache-factory_2026-09-17.md).
This directory characterizes library behavior. It does not qualify the service or close Plan 17.

## Method and provenance

1. Context7 resolved `apache/datafusion` to `/apache/datafusion`. Three documentation queries
   covered factory registration/lifecycle, the custom factory example, and native runtime caches.
   It identified `SessionStateBuilder`, `DataFrame::cache()`, the example, and cache configuration.
   It supplied no version-specific 55.1 identifier and insufficient detail on the custom factory.
   Its CLI `statistics_cache()` / `list_files_cache()` examples are discovery leads; this review
   does **not** assume those table functions are installed in our service.
2. The DataFusion skill resolved the API owners, exact signatures, native cache types and pinned
   example. Local registry source and Cargo.lock establish DataFusion **55.1.0**, Arrow **59.3.0**
   and object_store **0.13.2**. The skill corpus was generated 2026-09-15. Its original example
   and all inspected source hashes are in [source-manifest.json](source-manifest.json).
3. [probe.rs](probe.rs) executes actual pinned DataFusion against small in-process Arrow inputs.
   [upstream_example.rs](upstream_example.rs) retains the skill example's factory, logical node,
   planner and map implementation. The unrelated dataset/main runner was removed; Arrow/HashMap
   imports use DataFusion re-exports, the include-file documentation is an ordinary comment, and
   Rust formatting was applied. The Apache license is preserved. This is a behavioral adaptation,
   not an unmodified copy or a new production cache implementation.
4. [run.py](run.py) compiles only the probe using compatible existing dependency artifacts selected
   through Cargo fingerprints. [receipt.json](receipt.json) records the compiler identity, command,
   direct dependency artifacts/hashes, source hashes, log hashes and exit codes. It does not rebuild
   the workspace. No upstream Git commit is inferred from registry packages lacking VCS metadata.

The recorded compiler is stable **rustc 1.98.1**, Linux x86_64. Both compile and probe execution
returned **0**. [compile.log](compile.log) is empty; [probe-run.log](probe-run.log) has all results.
The final formatted runner passed scoped Ruff checks. No full integration tests ran.

## Reproduce

From the repository root, with the matching dependencies already built:

```bash
uv run python docs/design_review/reviews/evidence/datafusion-cache-factory-2026-09-17/run.py
```

The runner writes the binary under `.dev-state/plan17/cache-factory-probe/` and replaces this
directory's execution receipt/logs. It deliberately does not install dependencies or compile the
service. If compatible cached artifacts are missing, build the required dependencies separately;
do not interpret a missing artifact as an unsupported DataFusion capability. The source manifest
is a dated inspection checkpoint and is not refreshed by this command.

## Probe matrix

All rows are **passed characterization assertions**, including assertions that expose unsuitable
behavior. They are not product acceptance gates or assertions that the example is safe to adopt.

| ID | Actual experiment | Recorded observation / limit |
|---|---|---|
| P01 | Default `.cache()` followed by two reads | Input evaluated at `.cache()` once; no reevaluation on either read |
| P02 | Factory returns original plan | Factory invoked once; two reads evaluate input twice; a factory is a hook, not storage |
| P03 | Skill example, then `create_physical_plan()` and reads | Input evaluated during physical planning; subsequent reads reuse materialization |
| P04 | Skill example, plain `EXPLAIN`, without ANALYZE | Input evaluated once despite inspection-only request |
| P05 | Factory emits example extension without its planner | Planning refuses before input evaluation; no silent fallback |
| P06 | `SessionStateBuilder::new_from_existing` | Factory remains installed and is invoked |
| P07 | Factory explicitly refuses eligibility | Plan error before input evaluation |
| P08 | Factory substitutes a different output schema | `.cache()` accepts Int64-to-Utf8 replacement; implementor must enforce equivalence |
| P09 | Default cache of preallocated Arrow input under a 1,024-byte pool | Retained array accounting reports 4,195,072 bytes while pool reservation is zero; tested field metadata survives |
| P10 | Two concurrent example cache misses, synchronized start and async delayed source | Input evaluated twice; the example has no shared initialization on a miss |
| P11 | Two independently created identical cache requests in the same example planner | One evaluation; entry remains after DataFrame handles drop |
| P12 | Default cache of a provider with a primary key and declared ordering | Materialized provider has zero key constraints; physical ordering annotation is lost |
| P13 | Native `DefaultCache` with tiny deterministic byte budget | LRU, hit counter, byte accounting, table invalidation and oversized replacement refusal work; held Arc value survives eviction |
| P14 | Native `SchemaFingerprint` with different top-level field domain metadata | Fingerprints equal; this file-statistics identity is insufficient for our semantic contract identity |

The probe counter is a deliberately Volatile scalar UDF to prevent constant folding from masking
evaluation. It is instrumentation, not a recommendation to cache arbitrary volatile functions.
P10 uses two Tokio workers, an async delayed partition and a barrier to expose overlapping misses;
it is one reproducible schedule, not an exhaustive concurrency proof.

P09's source buffers are allocated before registration. Array accounting includes capacity and
may count shared buffers; it is **not RSS**. The claim is that default cache retention is not charged
to the configured pool, not that all DataFusion operators bypass reservations. The 1 KiB setting
is an adversarial fixture, not a proposed workstation configuration.

## Source reference key

Registry-relative paths below start at the local Cargo registry source directory recorded in
`source-manifest.json`. Line numbers describe this inspected source, not future upstream versions.

| Key | Primary source | What it establishes |
|---|---|---|
| E01 | `datafusion-55.1.0/src/execution/session_state.rs:2363`, `:1510`, `:1182` | `CacheFactory::create`, builder registration and carry-forward from existing state |
| E02 | `datafusion-55.1.0/src/dataframe/mod.rs:2412` | Custom branch delegates to factory; default branch collects partitions and creates MemTable |
| E03 | Local [upstream_example.rs](upstream_example.rs), `CacheNodePlanner::plan_extension` | Input collection during planning; split lookup/fill/store; private LogicalPlan-keyed map |
| E04 | `datafusion-execution-55.1.0/src/cache/cache_manager.rs:32`, `:314`, `:373`, `:442` | Native cache defaults, construction and injection surfaces |
| E05 | `datafusion-execution-55.1.0/src/cache/default_cache.rs:71`, `:97`, `:175`, `:209` | Native LRU, TTL, injected clock, byte budget, invalidation, inspection and value ownership |
| E06 | `datafusion-execution-55.1.0/src/cache/mod.rs:169` | File-statistics SchemaFingerprint ignores top-level field/schema metadata |
| E07 | `crates/enrichment-store/src/operation_index.rs:166`, `:188` | Existing operation-scoped spill materialization and exact row-count side channel |
| E08 | `crates/enrichment-store/src/runtime.rs:730`, `:792`, `:896`, `:1100` | Shared RuntimeEnv, session template, isolated catalogs and preparation boundaries |
| E09 | `crates/enrichment-store/src/arrow_contract.rs:199` | Existing composed contract, retention, command and Delta planners |
| E10 | `vendor/delta-rs/crates/core/src/delta_datafusion/table_provider/next/scan/mod.rs:703` | Modern Delta scan uses RuntimeEnv file metadata cache through CachedParquetFileReaderFactory |
| E11 | `crates/enrichment-core/src/config.rs:105` | Workstation defaults: 32 GiB native pool, 64 GiB spill, 2 GiB metadata cache |

## Not tested

No service cache factory was implemented. Cancellation, physical-exit ownership, spill exhaustion,
optimizer rewrites, retention release, whole-journey latency, process restart and MCP serving with
the proposed design remain unqualified. TTL behavior was source-inspected, not separately probed.
No product gate result or plan completion status is changed by this evidence.
