# Plan 13 native operation measurements — 2026-09-15

The W11 execution choices are complete: retain operation-local native indexes and the measured
endpoint semi join, with no additional partition/batch tuning. The early development sections
below preserve their historical measurement limits. The later installed-release section supplies
the actual MCP one/eight-client, high-fanout and comparison measurements that were still pending
at that earlier checkpoint. Final product acceptance and deployment are recorded separately in
[the final qualification report](plan13-final-qualification-2026-09-15.md).

## Decision and scope

Search, overview and comparison now reuse an operation-local native relation. DataFusion 55.1.0 executes
selection/ranking once, its disk manager owns quota-accounted temporary files, Arrow 59.3.0
streams IPC, and DataFusion `StreamingTable` supplies relational consumers. Search retains
keys/scores/aliases and hydrates long text/signatures after page selection. Overview retains
its selected namespace set and chosen children for their independent consumers. Comparison retains reconciled changed keys
for count and page consumers; before/after values stay in their admitted source relations until
their independent pages are selected. No persistent computation cache is added.

These are development experiments, not final-source acceptance or an MCP/client throughput
claim. The manual `measure_research_operations_one_and_eight_clients` fixture acquired real
fixture evidence, stopped its upstream, and checked equal result-data digests under retained
reads. One/eight means simultaneous native service-dispatch callers, not external client apps.
Three rounds give 3 samples per single-caller workload and 24 observations per eight-caller
workload; concurrent samples share load and are not independent tail-latency trials. Reported
min/median/max are descriptive. A p95 estimate would not establish reliable tail behavior here.
The relationship workload uses the small fixture, not a qualified large-library fanout case.

## Observed wall times

Seconds include reconstructing complete results through the public artifact operation when
needed. First response can be a usable artifact descriptor; it is not labeled first useful
semantic content. Native time sums child-query durations, which can overlap under concurrency.

| Callers | Task | Samples | Min | Median | Max | First-response median | Native median |
|---|---|---|---|---|---|---|---|
| 1 | known_api | 3 | 0.292 | 0.309 | 0.475 | 0.309 | 0.284 |
| 1 | relationships | 3 | 0.170 | 0.195 | 0.201 | 0.195 | 0.173 |
| 1 | search | 3 | 0.587 | 0.641 | 0.682 | 0.641 | 0.603 |
| 1 | overview | 3 | 0.756 | 0.766 | 0.808 | 0.737 | 0.678 |
| 8 | known_api | 24 | 0.321 | 0.332 | 0.339 | 0.332 | 0.305 |
| 8 | relationships | 24 | 0.178 | 0.187 | 0.198 | 0.187 | 0.166 |
| 8 | search | 24 | 0.737 | 0.754 | 0.798 | 0.754 | 0.704 |
| 8 | overview | 24 | 0.925 | 0.948 | 0.972 | 0.911 | 0.834 |

The workload uses `config/service.workstation.toml`: 32 GiB shared Arrow memory, 64 GiB
shared spill, 16 partitions and 16 query admission slots. The measurement test has eight Tokio
workers. No pool, partition, batch or endpoint-join tuning was applied from these samples.

## Repeated work and delivery

- Before search reuse, two cold/offline native sums were 0.919/0.931 s; after the first index
  implementation they were 0.632/0.677 s with equal fixture answers. Each search ran one ranking
  index and two scans. The retained key index was about 17–20 KiB in these fixture runs.
- Before overview reuse, the three single-caller complete-result times in the partial experiment
  were 0.927/0.917/0.930 s. With reuse they were 0.756/0.766/0.808 s. Its 15,432-byte index was
  read three times. The baseline and new runs are sequential development observations, not a
  randomized controlled benchmark or a portable speedup guarantee.
- An overview reconstructs a 22,117-byte indexed document in 11 deliberately small 4096-byte
  artifact reads in this helper. That measures the small-cap path; clients may request a larger
  supported cap or a named section. The final sample observed roughly 31–33 ms for these reads
  with eight callers. No repeated whole-result parse occurs in the service.
- Operation histories are bounded to 32 completed owners. The first measurement incorrectly
  read this ring after artifact expansion, by which time 88 follow-up reads had displaced the
  eight overview records. Capture now occurs at each response's actual request identity;
  artifact read time/calls/bytes are recorded separately. This was a measurement defect, not
  evidence of leaked operation state.
- The real cold-comparison/restart fixture was measured once on each side of the comparison
  index change. Native child-query time changed from 3.129379 to 2.853049 s and owned job lifetime
  from 7.358347 to 6.927068 s. Query count changed from 143 to 144: one materialization replaces
  repeated reconciliation, then two reads consume its 4488-byte key index. The independent
  97-alternative traversal also passed. These paired development observations support this
  reuse boundary; they do not establish a general comparison speedup or live-library scale.

## Native ownership and exact-version constraints

Every index belongs to one call's isolated catalog, captured request/config identity and pinned
snapshot leases. No key is reusable across operations. Reader memory reservations precede IPC
decoding; writers reserve encoding workspace; the runtime bounds each scan batch. Native disk
quota remains authoritative. Dropping the last reader releases spill bytes and reservations;
empty results and failed quota admission exercise the same cleanup. These bounds are not a
claim to measure total process RSS.

The pinned default `DataFrame::cache()` collects into `MemTable`; the default `WindowAggStream`
for a global count window retains input batches. Neither offers this bounded reuse boundary.
`SpillManager` is a hidden doctest re-export in 55.1.0 and is not an implementation dependency.
Public disk-manager/SpillFile, Arrow stream and streaming-table APIs are used instead. The
55.1 OS spill writer erases its quota error into `io::ErrorKind::Other`; the index writer adds
an explicit counter-based preflight witness, while native atomic enforcement remains in place.
An ambiguous concurrent I/O failure is reported as I/O, never guessed from message text.

## Receipts and remaining measurements

Raw development logs (preserved including failures):

- `.dev-state/plan13-native-query-baseline.log`
- `.dev-state/plan13-search-index-measurement.log`
- `.dev-state/plan13-one-eight-operation-measurement-current.log` — failed measurement-ring assumption
- `.dev-state/plan13-owned-operation-and-delivery-measurement.log` — completed equal-result experiment
- `.dev-state/plan13-operation-index-lifecycle.log` — native successful/empty/quota cleanup
- `.dev-state/plan13-operation-index-typed-quota.log` — typed quota origin and cleanup
- `.dev-state/plan13-comparison-owned-before.log` — cold comparison and restart before reuse
- `.dev-state/plan13-comparison-owned-after.log` — same comparison/restart after reuse
- `.dev-state/plan13-reused-comparison-alternatives.log` — independent 97-alternative traversal

The completed multi-request experiment predates subsequent comparison-index and quota-diagnostic
refinements. Final-source receipts, live release-comparison measurements, large high-fanout input,
actual MCP/client calls, first useful named-section timing, process/peak memory and broader
cold acquisitions remain open W11/W12 work. No arbitrary performance gate is inferred.

## Measured endpoint choice on live 55.1.0 snapshots

The bounded native probe compared the current four-condition endpoint semi join with four
keyed joins, a union of relationship identities, and semi-join hydration. Each returned the
same ordered 33 identities and Arrow bytes for the same pinned symbol. Three rounds alternate
execution order; these are single-caller native queries, not end-to-end client latency.

| Selected symbol | Current semi join median | Keyed union median | Current planning median | Union planning median |
|---|---:|---:|---:|---:|
| `datafusion::prelude::SessionContext` | 43.41 ms | 102.08 ms | 10.42 ms | 23.63 ms |
| `datafusion::dataframe::DataFrame` | 41.39 ms | 100.56 ms | 10.28 ms | 23.78 ms |
| `datafusion_expr::Expr` | 72.37 ms | 159.27 ms | 10.74 ms | 23.79 ms |

**Choice:** retain the current native endpoint semi join. The observed physical plan is a
`NestedLoopJoinExec` with a one-symbol right side and bounded selected output. The union
alternative adds hash joins, deduplication and repeated source work. Its algorithm name does
not establish a win for this input. The complete selected plans and 18 timings are retained in
`.dev-state/plan13-endpoint-plan-current-measurement.log`; the native probe uses the exact
workspace DataFusion 55.1.0 libraries, not the installed DataFusion CLI 53.1.0.

This is an equal-result bounded-page choice, not a claim about unbounded endpoint traversal.
Revisit if measured production high-fanout page latency or scan work grows materially with
relationship count; query/runtime maintainers own that decision. No partition or batch tuning
is justified by these samples. The original overview timing predates selected-namespace
materialization and therefore does not measure that later correctness refinement.

## Installed release: one and eight actual MCP clients

The immutable installed release candidate completed **135 equal-result tasks** over eight real
stdio FastMCP adapters: three rounds of five tasks with one caller, then three rounds with eight
concurrent callers. All callers used the same retained DataFusion 55.1.0 snapshots. Result data
digests agreed within each task. Requests, frames, native counter deltas and timings are under
`.dev-state/p13-installed-live/measurement-124310*`; the receipt records the installed manifest.

| Callers | Task | Samples | Complete min / median / max (s) | First response median (s) | Median tool calls |
|---|---|---:|---|---:|---:|
| 1 | known_api | 3 | 0.119 / 0.123 / 0.141 | 0.123 | 1 |
| 1 | relationships | 3 | 0.089 / 0.093 / 0.109 | 0.075 | 5 |
| 1 | search | 3 | 0.183 / 0.202 / 0.213 | 0.202 | 1 |
| 1 | overview | 3 | 0.242 / 0.242 / 0.256 | 0.221 | 6 |
| 1 | comparison | 3 | 1.257 / 1.294 / 1.621 | 0.009 | 4 |
| 8 | known_api | 24 | 0.163 / 0.179 / 0.209 | 0.179 | 1 |
| 8 | relationships | 24 | 0.117 / 0.128 / 0.139 | 0.103 | 5 |
| 8 | search | 24 | 0.290 / 0.305 / 0.329 | 0.305 | 1 |
| 8 | overview | 24 | 0.342 / 0.360 / 0.368 | 0.318 | 6 |
| 8 | comparison | 24 | 1.284 / 1.340 / 1.380 | 0.044 | 4 |

Comparison returned `pending` and the measurement followed the job plus complete artifact reads.
This records pending completion on retained inputs, not new release acquisition. The separate
installed focused campaign recorded cold acquisition/comparison and traversed all **117**
SessionContext relationship identities across bounded pages without duplicates. The eight-client
relationship task above measures its first bounded page, not all 117 edges on each request.

Sampled daemon RSS peaked at **486 MiB**; the sum across the explicitly launched process trees
peaked at **684 MiB**, sampled every 50 ms. These are observed resident-memory samples, not
complete allocation accounting or proof of a true process-tree peak. Native counter deltas include
query preparation/execution across each barrier; queue, scan and hydration time were not separately
exposed through the public MCP counter response and are not invented here. The earlier native
operation experiment retains its more detailed attribution. The installed files stayed byte
identical. The workspace fingerprint changed during this experiment because the client acceptance
parser changed; this is installed-artifact performance evidence, not a final workspace receipt.

No further batch/partition tuning is supported by these samples. Broader throughput, independently
measured tail latency and larger live fanouts are deferred to the query/runtime maintainer until
production latency or retained memory grows beyond these bounded workloads. This does not defer
any observed nontermination, output-budget breach or repeated full-result service parsing.
