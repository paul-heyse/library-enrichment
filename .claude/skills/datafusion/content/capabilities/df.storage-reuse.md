# Separate source reuse, plan reuse, materialization and writes

Provider reuse, logical views, cached result batches and file-metadata caches reuse different things. File writers and provider DML also have different capability and failure boundaries.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| view / cloned logical plan | Reuse query definition | Does not by itself freeze data or cache results. |
| DataFrame::cache | Reuse materialized results | Consumes memory and can become stale relative to changing sources. |
| RuntimeEnv metadata cache/store registry | Reuse source metadata or storage handles | Invalidation/lifetime differs from query-result caching. |
| write_parquet / COPY / provider DML | Persist output or mutate a table | Choose the actual sink contract, write mode and error handling. |

## Contract

**configuration.** SessionContext clones share state, while RuntimeEnv resources may be shared across contexts. Choose isolation based on catalogs, configuration and workload ownership rather than one-context-per-service folklore.
Claim `df.storage-reuse.configuration`; upstream_contract_interpretation; evidence: upstream.

**writes.** DataFrame writers execute query output into format/sink paths. write_table/INSERT depend on provider insertion support. A readable provider need not implement DML; COPY to files is not evidence it does.
Claim `df.storage-reuse.writes`; upstream_contract_interpretation; evidence: upstream.

**failure.** Inspect append/overwrite/partitioning and sink-specific partial-write behavior. Neither a successful plan nor Arrow batch validity establishes transactional or atomic persistence.
Claim `df.storage-reuse.failure`; upstream_contract_interpretation; evidence: upstream.

## Implementation

- Specify exactly which computation/data/metadata is reused and when it becomes stale.
- Verify writer and source ownership, registry lifetime and output schema.
- Round-trip a small written file; separately qualify atomicity or cloud failure guarantees if required.

## Limits and unknowns

- Cloud stores, transactional semantics and cache invalidation under external mutation are not established by local fixtures.

## Exact contracts

- [`datafusion::dataframe::DataFrame::cache`](../operations/datafusion.dataframe.DataFrame.md#op-46de08d8309fcb4f0782d08e) — `async fn cache(self) -> Result<DataFrame>`
- [`datafusion::dataframe::DataFrame::into_view`](../operations/datafusion.dataframe.DataFrame.md#op-ce2618e84e6ecfd888a3b153) — `fn into_view(self) -> Arc<dyn TableProvider>`
- [`datafusion::dataframe::DataFrame::write_parquet`](../operations/datafusion.dataframe.DataFrame.md#op-3c9ca113f3c5829bfb6ac6f0) — `async fn write_parquet(self, path: &str, options: DataFrameWriteOptions, writer_options: Option<TableParquetOptions>) -> Result<Vec<RecordBatch>, DataFusionError>`
- [`datafusion::dataframe::DataFrame::write_table`](../operations/datafusion.dataframe.DataFrame.md#op-03b06a16dfccd7287df86167) — `async fn write_table(self, table_name: &str, write_options: DataFrameWriteOptions) -> Result<Vec<RecordBatch>, DataFusionError>`
- [`datafusion_execution::runtime_env::RuntimeEnv`](../operations/datafusion_execution.runtime_env.RuntimeEnv.md#op-c598f4df4cf51824ae4b9a67) — `struct RuntimeEnv`
- [`datafusion_session::table::TableProvider::insert_into`](../operations/datafusion_session.table.TableProvider.md#op-f6f850e68c1fe795e5e5fac4) — `async fn insert_into(&self, _state: &dyn Session, _input: Arc<dyn ExecutionPlan>, _insert_op: InsertOp) -> Result<Arc<dyn ExecutionPlan>>`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [probe](../../skill_improvement/evidence/implementation/probe-results.json): Only the named assertions in the recorded Cargo profile.
  Tests: stream_collect_cache_and_sort_resource_error, parquet_listing_source_executes_query
