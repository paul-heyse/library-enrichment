# Plan 14 physical strategy decisions — 2026-09-15

**P8 decisions are complete; final product qualification is separate.** The selected implementation
uses the shared immutable policy and production codec/provider/runtime paths. DataFusion 55.1.0,
Arrow/Parquet 59.3.0 remain pinned. See the [machine receipt](plan14-physical-strategies-2026-09-15.json)
for sample counts, ranges, source archive, executable hash and raw-log hashes.

## Method and limits

The retained manual harnesses are `physical_measurements.rs`, `index_measurements.rs` and the
consumer-pool case in `query_diagnostics.rs`. They are excluded from ordinary regression timing.
The scan experiment encodes real API-observation contracts with Rust/Python-shaped signatures,
nullable documentation and nested source/payload fields. Every file passes the production isolated
native admission decoder before its exact provider exposes row statistics. These are consumer
experiments, not a claim about extraction completeness or end-to-end library latency.

The final storage comparison used production ZSTD level 3 and native page defaults: 2,304
observations across 32/4,096 rows, eight layouts, decoder off/on, one/four clients, selected ID,
absent ID and unselective aggregate queries, first/warm reads and three alternating-order rounds
per payload shape. All matched outputs were equal. OS caches were not cleared. The reported
first read is a fresh runtime/provider metadata read, not a physically cold disk claim.

The index comparison used 0/3/8,192 rows, three alternating-order rounds, four page consumers,
NULL labels and ties. All pages matched. The pool comparison used four concurrent reservation
owners and 40,000 grow/shrink pairs per sample, followed by real failed allocation. Index/pool measurements were rerun in isolation with their own source and executable receipt.
Build profile was unoptimized: these results select conservative mechanisms, not production throughput targets.
RSS is recorded separately as a process sample/high-water observation; managed-pool peaks are
neither decoder allocation totals nor RSS. Final product journeys use the installed release build.

An exploratory scan run used uncompressed files and small pages. The production-default follow-up
supersedes it for selection. An early footer counter read the writer after `finish` drained its
metadata; the final experiment reads actual footer row groups. Earlier zero counts are excluded.

## Decisions and actual consumers

Storage times below are medians of six matched warm, single-client samples across the two payload
shapes. Index and pool times use three rounds per strategy. Ranges and concurrent/cold samples
are in the machine receipt; no p95 is inferred.

| Choice | Decision and evidence | Production integration / revisit trigger |
|---|---|---|
| Decoder filtering / filter reordering | **Adopt both.** At 4,096 rows, selected-ID median fell 22.16→14.49 ms and absent-ID 22.65→13.46 ms; unselective 17.69→16.23 ms. Tiny selected IDs changed 11.55→11.87 ms, a small fixed-cost tradeoff. | `NativeQueryConfig` defaults; read-only `NativePolicy`; actual `ParquetFormat`/source consumer. Residual filtering remains Inexact. Revisit if a measured query family shows substantial predicate/cache overhead. |
| Row-group and file sizing | **Keep 1,024-row groups; select an independent 4,096-row catalog file target.** 256-row groups increased file bytes 261,254→406,580 and write median 53.1→74.4 ms. 4,096-row groups saved bytes (229,417) but selected-ID decoding with filtering was slower (15.40 vs 14.49 ms). Sixteen small files cost 721,698 bytes and 709.7 ms admission vs 261,254 and 600.2 ms for one file. | Shared writer factory, independent `WriteLimits` layout, reservation-backed catalog compaction, explicit byte/footer caps. Transaction limit remains 1,024 rows; it no longer determines file admission or compaction layout. Revisit for materially different payload width or sustained catalog size. |
| Page indexes / key Bloom filters | **Keep native page defaults; adopt Bloom only on `api_observations.observation_id`.** Fine pages increased bytes to 362,088 with no selective benefit. With decoder filtering, the selected-key Bloom reduced ID 14.49→13.30 ms and absent-ID 13.46→11.69 ms. Bytes increased 1.6%; write medians 53.09→53.43 ms. | One writer factory chooses the declared observation key; native Bloom/page readers remain enabled. No other column gets a speculative Bloom filter. Revisit another key only with an actual selective consumer and matched write/read evidence. |
| File grouping / scan parallelism | **Adopt whole-file groups capped by exact file count and runtime partitions.** For sixteen small files, decoder-filtered selected-ID median fell 35.30→18.29 ms and unselective 28.43→13.99 ms with four groups. One-file inputs stay one group. | `ExactParquet::scan` supplies bounded native `FileGroup`s. DataFusion's `FileGroupPartitioner` partitions byte ranges and its preserving mode does not divide a multi-file group, so it is not misused as a whole-file grouping API. No new tasks or custom execution operator. Revisit only for measured file-size skew. |
| Retained-index ordering | **Keep unsorted materialization plus native consumer sort.** At 8,192 rows, sort-once construction rose 23.24→39.07 ms, saving only 22.48→18.50 ms across four pages. At zero/three rows it also cost more overall. | Completed indexes still expose any ordering actually established by the completed native plan. No order is invented for an unsorted source. Revisit if actual operation-local consumers amortize the measured extra construction cost. |
| Small memory index / IPC | **Keep one IPC strategy.** Reserved memory saved roughly 2.4–2.8 ms construction at three/8,192 rows, while retaining 440/331,264 bytes. The IPC path retains zero managed payload bytes and already handles large indexes, quotas and ownership uniformly. This bounded saving does not justify a second adaptive storage branch. | Production `operation_index::materialize` remains the only strategy. Revisit when measured index construction dominates whole-operation latency and a threshold provides material end-to-end benefit within the same resource contract. |
| Richer statistics | **Do not add column-statistic storage.** Source inspection finds production total-count consumers use `COUNT(*)`; nullable-count/min/max reductions are not a required product total. Native Parquet metadata already handles value pruning. The experiment's nullable aggregate exercises real scans, not a fabricated exact-column shortcut. Current isolated admission returns exact rows, not certified per-column aggregates. | Physical `FileScanConfig` consumes exact admitted rows; every other unknown fact stays Absent. Revisit for a named aggregate/join consumer whose benefit warrants extending the existing admission reply, without a second footer pipeline. |
| TrackConsumersPool | **Decline.** It adds useful retained-owner text and preserves `ResourcesExhausted`, but the concurrent reservation loop cost over three times the plain pool (35.61 vs 9.82 ms median). Existing failures identify the failed consumer and operation, and peak recording already meets the required diagnostic consumer. | Keep `PeakRecordingPool(FairSpillPool)`, with shared-runtime scope and no resets. Revisit for a production capacity diagnosis that cannot be resolved from current operation/consumer evidence. |
| Dictionary-preserving scoring | **Not applicable to current scoring inputs.** Canonical queried codecs declare string/list/struct fields, no Dictionary fields; Parquet dictionary page encoding is not an Arrow Dictionary scoring input. The actual analyzed-UDF test proves Utf8/LargeUtf8/Utf8View arguments remain uncast and results/metadata match, including slices and nullable optional text. | Keep correct batch-boundary dictionary fallback in `TextColumn`; do not claim zero-copy dictionary scoring. Revisit only when a real scored source reaches the UDF as an Arrow Dictionary. |

The executable equality checks and native property/resource tests remain in the tree. Rejected
alternatives exist only in the explicitly manual test harness, not as runtime flags or fallback
providers. The final native/Python/MCP qualification report determines P9 and plan completion.
