# Arrow/DataFusion measurement protocol

> 2026-09-14 priority update: the operator explicitly deferred tuning and enforcement of
> these arbitrary timing targets until correctness, failure robustness and target functionality
> are complete. Preserve the measured baselines and original thresholds for later performance
> work; a threshold miss is not a Plan 12 functional-completion blocker or a performance pass.

Established 2026-09-14 during Plan 10 T0, before target operation tuning. These are development
measurements and acceptance thresholds, not library evidence to migrate or phase certification.

## Fixed baseline

The existing `every_tool_answers_offline_from_the_snapshot_published_cold` daemon fixture ran
five times as a precompiled test executable. Every run creates new temporary service state,
acquires from the local fixture upstream, publishes, and executes the offline tool round.
All five executions passed. Host caches were not cleared, so this is **not cold host I/O**.

Wall times were 0.5454, 0.5599, 0.5762, 0.5626 and 0.5977 seconds; median **0.5626 seconds**.
Whole test-process maximum RSS ranged from 116,064 to 118,804 KiB. The build invocation is
recorded separately and its compiler RSS/time are excluded from these operation measurements.
The baseline does not expose per-operator managed memory or stage timings.

Exact command, executable hash, relevant source-file hashes and run measurements are in
`.dev-state/plan10-baseline/offline-retrieval.json`; individual `/usr/bin/time -v` logs are
`offline-run-1.log` through `offline-run-5.log` in that directory. Executable SHA-256:
`f9d94a70e4c5b4aa07a1bbfeab0dfc24164eca8b1c0224eb0227205fd18a77de`.
The executable is the exact baseline artifact; the source hash subset is not a claim that all
later source edits were present in it. Do not carry its temporary snapshots into the target.

## Budgets and regression thresholds

| Boundary | Initial fixed requirement |
|---|---|
| Same local fixture workflow | Five-run median at most 1.5 seconds; each run at most 3 seconds |
| Warm selective operation on a representative retained snapshot | At most 1 second including hydration/serialization; no whole-snapshot re-admission or DTO materialization |
| Broad operation on the selected real-library corpus | At most 5 seconds of query execution; bounded output and explicit resource failures |
| Cold snapshot admission | At most 30 seconds including permit wait; batch/row/file bounds apply independently |
| Query execution | Shared 128 MiB managed pool, 512 MiB spill, 16 MiB metadata cache; two partitions, four concurrent operations |
| Query results | At most 10,000 retained rows and 16 MiB Arrow result buffers; 30-second hard deadline |
| Producer-to-Parquet construction | At most 1 MiB serialized record, 1,024 rows and 16 MiB Arrow bytes per batch; 256 MiB per file, 1,024 row groups, one million rows per relation |
| Initial enrichment assembly/semantic-key validation | 64 MiB explicit intermediate bound; resource exhaustion is an error |
| Native Parquet cold validator | One process at a time; at most 1 GiB virtual address space and 30 CPU seconds; parent wall deadline/cancellation; post-decode batch/group limits; actual RSS measured separately |
| Initial artifact admission | 256 MiB per input, 512 MiB per distinct input closure; no unbounded hash read |

The fixture regression allowance accommodates mandatory provenance/admission and fsync work;
it is approximately 2.7 times the measured median, not a promised speedup. Do not move it after
a target failure. A proposed change needs a documented workload reason and explicit review.
Managed-pool limits are **not an RSS ceiling**: scan buffers, producer graphs, Arrow writers,
footer metadata, task queues and final DTOs need their own bounds and measurements.

### Operator resource priority, 2026-09-14

The operator clarified that prompt-serving latency for LLM coding agents takes precedence over
minimizing resource use. Initial use is intermittent and personal on a 16-core/32-thread,
192 GiB workstation. Live host inspection confirmed a Ryzen 9 9950X3D and 197,452,792 KiB usable
physical memory reported by Linux. The original small pools above remain a reference workload
configuration, **not the resource ceiling for the target personal deployment**.

`config/service.workstation.toml` starts with 32 GiB shared managed query memory, 64 GiB spill,
2 GiB metadata cache, 16 partitions, 16 concurrent queries and eight expensive producer jobs.
These limits are not eager reservations, and resource use is not a quantity to minimize at the
expense of response latency. Compare 8/16/32 native partitions and simultaneous operation
counts on the preselected real workloads, observing latency distributions and actual CPU/RSS.
Keep enough parallelism to use the host where it improves execution; retain admission and
plan reuse where repeated planning or validation dominates. No speedup is inferred merely
from increased limits or the potential to parallelize.

Keep workload/build/configuration identities in results. The old fixed fixture thresholds stay
useful regression checks; they are not a reason to constrain the workstation to 128 MiB or to
trade away useful parallelism. Whole-process measurements protect against runaway work and
identify costs outside the managed pool. Container CPU/memory settings and serial admission
must be included in the remaining performance audit, rather than assuming DataFusion's query
settings control every stage.

## Remaining measurements before T6/T7 completion

Select and hash real Rust and Python libraries plus deterministic skew/alias/large-text fixtures.
Measure acquisition, normalization, Arrow construction, admission, publication, cold/warm open,
planning, execution, hydration and serialization separately. Record rows/bytes scanned, output
bytes, batches, managed-pool peaks, process RSS, spill, file metadata and write amplification.
Run isolated and concurrent exact, prefix, broad substring, overview, inspect and every comparison
scope. Distinguish fresh service caches from the unchanged host cache. Do not drop host caches.

Prove exact semantics before interpreting speed. Inspect native plans for selective projection,
filters, aggregates, joins, windows and keyset bounds. Reject physical tuning without measured
benefit. Same-context enrichment now uses native DataFusion set unions and streamed Parquet writing.
Measure its scans, semantic-key validation, write amplification and catalog file-list growth;
these remain potential scaling costs despite removal of the Rust corpus materializer.
This baseline and focused module tests do not close those obligations.

## Plan 11 workload selection (2026-09-14)

### First current-target fixture series: failed, 2026-09-14

`.dev-state/plan11-fixture-series-1/measurements.json` records five actual runs of the
precompiled debug fixture: **7.2923, 7.4587, 10.1222, 7.9617 and 7.5085 seconds**; median
**7.5085 seconds**. Correctness passed in each run, but both performance thresholds failed.
No competing build/test ran during this series and host caches were not cleared.
The source digest was `1c6c3d47c8b7823810372897aa2ee155f05e84dbd1c66de68ab09cece86fab10`;
fixture executable SHA-256 `ada1cf647f56b5443b11f0ab76b0e5628bfe097cbe4c5a0018c08cb302f5f82d`;
native decoder SHA-256 `68c204f7b0c57ea4787da1a30d06b5df433086baa7f117966bad4701586e3933`.
Per-run `/usr/bin/time` maxima are not the sum of concurrent process-tree RSS peaks.

Separate test-only query diagnostics show repeated planning as a material cost: warm overview
executed 20 queries with approximately 663 ms summed planning time; search executed 12 with
375 ms; inspect executed 18 with 300 ms. These are cumulative executed-query times, not
end-to-end stage walls. Logical-view inlining is under qualification: an initial correctness
fixture took 6.59 seconds, but an expanded ownership test exposed exhausted-stream lease loss.
That candidate was not accepted before ownership was fixed. Budgets are unchanged.

The final ownership placement and pristine session-template reuse passed the repository,
typed Arrow, isolation and projection-pruning tests. Debug series 2 at
`.dev-state/plan11-fixture-series-2/measurements.json` improved to median **6.5740 seconds**,
but still failed. Keep this comparable debug result visible.

The optimized **release** artifact was then measured separately, with the existing Cargo
release profile (thin LTO, one codegen unit). Release series 1 measured median 1.5160 seconds
with Python timeout polling; its `/usr/bin/time` median was 1.48 seconds. Timeout-based Python
wait adds up to 50 ms of polling delay. The tracked `scripts/measure_arrow_fixture.py` now uses
blocking wait plus a separate 60-second process-group watchdog. It leaves thresholds unchanged.
Release series 2 passed: **1.4558, 1.4301, 1.4409, 1.4415 and 1.4397 seconds**, median
**1.4409 seconds**, all correctness checks passed. Exact current-source/executable/worker
identities are in `.dev-state/plan11-fixture-release-series-2/measurements.json`. No builds,
other tests, source edits or host-cache clearing occurred during that series.

This qualifies the fixture budget for that optimized artifact; it does not establish a speedup
against the old debug baseline, qualify debug performance, or close real-library/RSS/concurrent
resource budgets. Reproduce with the tracked measurement command and explicit precompiled
executable path; record the build profile and native worker in every subsequent receipt.

### Selected real inputs

Selected before the target benchmark series; neither acquisition nor performance is certified.
The tracked table pins the source archives. During acquisition also record the selected hosted
rustdoc/worker inputs, actual environment IDs, helper digests and containment qualification.
Do not silently substitute smaller packages after a failed run.

| Ecosystem/package | Releases | Source SHA-256 | Declared scope |
|---|---|---|---|
| rust/datafusion | 55.0.0 | `96f76f0167ed0842b29a3d1e41be3c034c0a46409a3a703cc4cc84ee8c24abf4` | x86_64-unknown-linux-gnu; stable 1.98.1; defaults enabled; no additional features |
| rust/datafusion | 55.1.0 | `14313430439ef858ed5f52ca9c840e9eea9844da34d3716d05a476449634da5d` | x86_64-unknown-linux-gnu; stable 1.98.1; defaults enabled; no additional features |
| python/pydantic | 2.13.4 | `c40756b57adaa8b1efeeced5c196f3f3b7c435f90e84ea7f443901bec8099ef6` | import root pydantic; no extras; Python 3.14.7; linux-x86_64 |
| python/pydantic | 2.13.5 | `51a9c5f7b2f8e636f04c6cada605d9b6a3bf1348fdf945a3d8869b19bba0ee08` | import root pydantic; no extras; Python 3.14.7; linux-x86_64 |

Rust archive hashes were computed from the exact Cargo registry archives. Python 2.13.5 is
pinned by uv.lock; 2.13.4 was selected from the exact [PyPI release metadata](https://pypi.org/pypi/pydantic/2.13.4/json).
Both Python archives must be hashed on acquisition before using their results. Source URLs and
the initial selection receipt are in `.dev-state/plan11-workloads.json`. These workload choices
do not change dependency pins or authorize execution outside existing policy.

The generated streaming case has 3,000 distinct fragments with 16,389-byte text, a 256 KiB batch
limit and independently checked complete IDs. The inspection pruning case has four 600,000-byte
deterministically generated documents. Add cross-operation alias/qualified-observation skew to
the W7 series; these focused regressions alone do not establish the performance budgets above.

The native Parquet reader's row count does not cap hostile footer, decompression or dictionary
allocations. ADR-0031 introduces a bounded native process and group-isolated decoding; the earlier
eight-row heuristic is deleted. Include worker executable mappings, peak RSS, startup/hash time
and queued cancellation in the W7 series. Focused hostile-footer/supervision tests are correctness
evidence, not a completed peak-RSS or latency benchmark.
