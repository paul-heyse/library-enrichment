# DataFusion and Delta caching probe — 2026-09-17

Evidence for [the caching design review](../../design_review_datafusion-deltalake-caching_2026-09-17.md).
This directory characterizes library and current-service behavior on a small local fixture.
It is not a product acceptance gate, a workload benchmark or a Plan 18 receipt.

## Method and provenance

- Source tree: HEAD `ccf5739a333d29e6fbabe6e054c942f1c5da8b0b` plus the shared dirty tree
  described in `STATUS.md` (Plan 18 execution baseline). Pins: DataFusion 55.1.0, Arrow/Parquet
  59.3.0, delta-rs `58f07cd6` (vendored under `vendor/delta-rs`), buoyant kernel `8ba063f8`.
- Compiler: stable `rustc 1.98.1 (48a229cea 2026-09-01)`, Linux x86_64, workstation local
  filesystem (`DurableLocalStore` registered at `file:///`), debug profile, existing build cache.
- [`cache_probe.rs`](cache_probe.rs) was compiled and run as an integration test of
  `enrichment-store` (`crates/enrichment-store/tests/cache_probe_2026_09_17.rs`), then moved
  here. It uses only public service APIs (`QueryRuntime`, `DeltaStore`, `StorageContract`) plus
  pinned delta-rs and DataFusion APIs. No service daemon, MCP client or producer ran.
- Command and complete output: [`probe-run.log`](probe-run.log) (exit 0, 1 passed).

```bash
cargo test --locked -p enrichment-store --test cache_probe_2026_09_17 -- --nocapture
```

Digests at capture time: `cache_probe.rs` `af5e6a05…03e8d248`, `probe-run.log` `9a227cce…09c454`.

## Fixture

One Delta table with a single non-null `Int64` column, created through `DeltaStore::create`,
then **60 appends** through `DeltaStore::append` (one Parquet file per commit, no checkpoint;
delta-rs auto-checkpoints every 100 commits by default). Five foreign commits are added during
A2, so later rows see **65 files / version 65**. Timings are wall-clock milliseconds, five
samples each, debug build, warm page cache. They are order-of-magnitude evidence, not p95s.

## Results

| ID | Experiment | Observation (final run) |
|---|---|---|
| A1 | `DeltaStore::load(name, None)` — the path every consumer uses today (fresh `DeltaTable`, full log replay, eager files with stats) | **26.2–30.8 ms** per load at 60 commits |
| A2 | Retained `DeltaTable`, `update_incremental(None)` after one new foreign commit each time | **11.1–13.7 ms** per refresh; version advanced 60 → 65 |
| A3 | `DeltaTableBuilder::…without_files().load()` — lazy protocol/metadata snapshot | **9.1–12.2 ms** (first sample 12.2 includes engine warm-up) |
| A4 | Full `DeltaStore::load` after `create_checkpoint` at version 65 | **6.6–10.5 ms** |
| C | `DeltaLogicalCodec::encode_immutable_provider` of a `DeltaScanNext` over 65 files | **68,716 bytes JSON (1,057 B/file)**; encode 3.1 ms; `try_decode_table_provider` 3.2 ms; the same add-actions inventory as Arrow IPC: **9,480 bytes** (≈7.2× smaller) |
| D | Three `SUM(id)` scans of the raw next provider bound to the shared session | metadata cache entries **0 → 65** after scan 1; hits **65** after scan 2, **130** after scan 3; scan time **31.9 → 23.4 → 24.4 ms**; limit 2,147,483,648 bytes |
| E | `DeltaStore::open_or_create` and `ControlStore::pin` timing | **Refused, not measured** — see below |

Earlier runs of the same probe (before E/B were removed) recorded identical shapes:
A1 25.2–29.4 ms, A2 10.3–16.9 ms, A3 8.8–19.4 ms, A4 6.2–8.4 ms, C 68,655–68,708 bytes,
D 65 entries / 65 and 130 hits.

## Refused sub-probes (a current-tree defect, reproduced three times)

`DeltaStore::open_or_create`, `DeltaStore::provider` and `ControlStore::pin` on an already
registered contract all fail with:

```
InvariantFailure { cause: Internal, rule: "invariant query must select one identity column",
                   stage: "provider_preparation" }
```

Cause, by source reading: `native_delta.rs::contract_exists` finds the stored contract, calls
`enrichment_core::native_contract::changes(...)`, which returns a five-column frame
(`projection, path, kind, before, after`; `native_contract.rs:164`), and hands it to
`QueryRuntime::require_empty`, which refuses any frame that is not exactly one column
(`runtime.rs:1262-1265`). The first registration passes because the row does not exist yet.
Every warm reopen of a registered contract therefore refuses. This is outside the caching scope
but blocks any end-to-end measurement of the registry and control paths.

## Limits

- Debug build, local NVMe, tiny files: absolute numbers understate production decode cost and
  overstate I/O cost. Ratios (full replay vs incremental vs lazy vs checkpointed; JSON vs IPC)
  are the evidence, not the milliseconds.
- D scans a one-column table; the metadata-cache benefit scales with footer/page-index size.
- No measurement of the operation `CacheFactory` route, CDF scans, spill, memory pressure,
  restart or any MCP journey. Those remain Plan 18 CF/Q obligations.
