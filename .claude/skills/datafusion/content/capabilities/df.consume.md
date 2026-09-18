# Choose stream, materialization, cache and resource boundaries

Streaming avoids retaining every output batch in the terminal API; it does not bound the query working set. A blocking sort can still fail for memory even when output is streamed.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| collect | All output is intentionally retained | Returns Vec<RecordBatch>; memory includes result retention. |
| execute_stream | Incremental single-stream consumer | Planning and stream polling can fail separately; consumer retention still matters. |
| execute_stream_partitioned | Caller can manage output partition concurrency | Partition streams do not imply global ordering. |
| cache | Reuse materialized results | Executes and stores results in memory; a cloned plan/view is not the same guarantee. |

## Contract

**shape.** Terminal methods consume DataFrame. execute_stream asynchronously creates a physical plan and returns a fallible RecordBatch stream; collect returns a batch vector. Single-stream output is not an implicit sort.
Claim `df.consume.shape`; upstream_contract_interpretation; evidence: upstream.

**memory.** Operator state, prefetch, caches and consumer batches contribute memory. Default pool is unbounded; disk manager defaults to OS temporary storage. with_memory_limit explicitly does not account for every allocation.
Claim `df.consume.memory`; upstream_contract_interpretation; evidence: upstream, source.

**spill.** Configure a participating pool and operator-specific spill behavior, directory/capacity and concurrency. A pool budget is not an RSS bound. Inspect EXPLAIN/metrics and execute the workload.
Claim `df.consume.spill`; upstream_contract_interpretation; evidence: upstream.

**lifecycle.** Upstream documents dropping the output stream aborts its execution and frees associated resources. This is not a guarantee for unrelated application background tasks or retained batch buffers.
Claim `df.consume.lifecycle`; upstream_contract_interpretation; evidence: upstream.

## Implementation

- Choose terminal API from ownership and consumption requirements.
- Inspect blocking operators and configure their resource policy before execution.
- Handle failures both creating and polling streams; test cancellation if operational correctness depends on it.

## Limits and unknowns

- The probe proves a 1-byte pool sort fails and stream/collect values agree. It does not measure RSS, spill throughput, cancellation latency or all operator accounting.

## Exact contracts

- [`datafusion::dataframe::DataFrame::collect`](../operations/datafusion.dataframe.DataFrame.md#op-c2a2c9a0147637ad4b86374e) — `async fn collect(self) -> Result<Vec<RecordBatch>>`
- [`datafusion::dataframe::DataFrame::execute_stream`](../operations/datafusion.dataframe.DataFrame.md#op-acdcda8e3349a26da03fd615) — `async fn execute_stream(self) -> Result<SendableRecordBatchStream>`
- [`datafusion::dataframe::DataFrame::execute_stream_partitioned`](../operations/datafusion.dataframe.DataFrame.md#op-73ff2f187bcd18e356825de9) — `async fn execute_stream_partitioned(self) -> Result<Vec<SendableRecordBatchStream>>`
- [`datafusion::dataframe::DataFrame::cache`](../operations/datafusion.dataframe.DataFrame.md#op-46de08d8309fcb4f0782d08e) — `async fn cache(self) -> Result<DataFrame>`
- [`datafusion_execution::runtime_env::RuntimeEnvBuilder::with_memory_limit`](../operations/datafusion_execution.runtime_env.RuntimeEnvBuilder.md#op-67b941dfc4573dd8aea8cf06) — `fn with_memory_limit(self, max_memory: usize, memory_fraction: f64) -> Self`
- [`datafusion_execution::disk_manager::DiskManagerBuilder`](../operations/datafusion_execution.disk_manager.DiskManagerBuilder.md#op-3c6231e792a03a3bc83aef37) — `struct DiskManagerBuilder`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [probe](../../skill_improvement/evidence/implementation/probe-results.json): Only the named assertions in the recorded Cargo profile.
  Tests: stream_collect_cache_and_sort_resource_error
- [source](../../skill_improvement/evidence/sources/datafusion-execution-55.1.0/src/disk_manager.rs): Selected exact published source; source-manifest.json retains provenance.
  Tests: 
