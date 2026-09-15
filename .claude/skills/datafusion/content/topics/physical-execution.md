# Physical execution

An `ExecutionPlan` produces a `SendableRecordBatchStream` per partition. Its `PlanProperties` — partitioning, equivalence, ordering, boundedness — are what the optimizer reasons about, so a node that reports them badly gets planned badly. Execution is pull-based and streaming by default; materialization is something a specific operator does, not the norm.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `datafusion_physical_plan::execution_plan::ExecutionPlan` | trait | 35 | [prose](../api/datafusion_physical_plan.execution_plan.md#executionplan) | [records](../model/datafusion_physical_plan.execution_plan.json) |
| `datafusion_execution::task::TaskContext` | struct | 28 | [prose](../api/datafusion_execution.task.md#taskcontext) | [records](../model/datafusion_execution.task.json) |
| `datafusion_common::stats::Statistics` | struct | 17 | [prose](../api/datafusion_common.stats.md#statistics) | [records](../model/datafusion_common.stats.json) |
| `datafusion_physical_plan::sorts::sort::SortExec` | struct | 35 | [prose](../api/datafusion_physical_plan.sorts.sort.md#sortexec) | [records](../model/datafusion_physical_plan.sorts.sort.json) |
| `datafusion_physical_plan::aggregates::AggregateExec` | struct | 43 | [prose](../api/datafusion_physical_plan.aggregates.md#aggregateexec) | [records](../model/datafusion_physical_plan.aggregates.json) |
| `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec` | struct | 42 | [prose](../api/datafusion_physical_plan.joins.hash_join.exec.md#hashjoinexec) | [records](../model/datafusion_physical_plan.joins.hash_join.exec.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `datafusion_physical_plan::execution_plan::ExecutionPlan` | 6 | 29 | 47 | [ExecutionPlan](../traits/ExecutionPlan.md) |
| `datafusion_datasource::sink::DataSink` | 2 | 2 | 4 | [DataSink](../traits/DataSink.md) |

## Settings (6)

Full table with Rust setters in [`../catalogs/config-options.md`](../catalogs/config-options.md).

| Setting | Default |
|---|---|
| `datafusion.execution.coalesce_batches` | true |
| `datafusion.execution.enforce_batch_size_in_joins` | false |
| `datafusion.execution.sort_in_place_threshold_bytes` | 1048576 |
| `datafusion.execution.sort_pushdown_buffer_capacity` | 1073741824 |
| `datafusion.execution.sort_spill_reservation_bytes` | 10485760 |
| `datafusion.execution.target_partitions` | 0 |

## Runnable examples (13)

- [`corpus/examples/execution_monitoring/main.rs`](../corpus/examples/execution_monitoring/main.rs)
- [`corpus/examples/execution_monitoring/memory_pool_execution_plan.rs`](../corpus/examples/execution_monitoring/memory_pool_execution_plan.rs)
- [`corpus/examples/execution_monitoring/memory_pool_tracking.rs`](../corpus/examples/execution_monitoring/memory_pool_tracking.rs)
- [`corpus/examples/execution_monitoring/tracing.rs`](../corpus/examples/execution_monitoring/tracing.rs)
- [`corpus/examples/custom_data_source/adapter_serialization.rs`](../corpus/examples/custom_data_source/adapter_serialization.rs)
- [`corpus/examples/custom_data_source/csv_json_opener.rs`](../corpus/examples/custom_data_source/csv_json_opener.rs)
- [`corpus/examples/custom_data_source/csv_sql_streaming.rs`](../corpus/examples/custom_data_source/csv_sql_streaming.rs)
- [`corpus/examples/custom_data_source/custom_datasource.rs`](../corpus/examples/custom_data_source/custom_datasource.rs)
- [`corpus/examples/custom_data_source/custom_file_casts.rs`](../corpus/examples/custom_data_source/custom_file_casts.rs)
- [`corpus/examples/custom_data_source/custom_file_format.rs`](../corpus/examples/custom_data_source/custom_file_format.rs)
- [`corpus/examples/custom_data_source/default_column_values.rs`](../corpus/examples/custom_data_source/default_column_values.rs)
- [`corpus/examples/custom_data_source/file_stream_provider.rs`](../corpus/examples/custom_data_source/file_stream_provider.rs)
- [`corpus/examples/custom_data_source/main.rs`](../corpus/examples/custom_data_source/main.rs)

## Upstream guides

- [`corpus/guides/user-guide/metrics.md`](../corpus/guides/user-guide/metrics.md)
- [`corpus/guides/library-user-guide/extending-operators.md`](../corpus/guides/library-user-guide/extending-operators.md)
- [`corpus/guides/user-guide/explain-usage.md`](../corpus/guides/user-guide/explain-usage.md)

## Decision rules

- A wrapper operator must forward `statistics`, ordering and filter-pushdown participation, or it becomes an optimization barrier.
- `execute_stream_partitioned` preserves parallelism; `execute_stream` coalesces to one partition first.
- Metrics are opt-in per operator and are what `EXPLAIN ANALYZE` reports.

## Anti-patterns

- Implementing `ExecutionPlan` without `partition_statistics`; the optimizer then plans that subtree blind.
- Buffering a whole partition inside an operator that could yield batches.

## Agent checklist

- Are `PlanProperties` accurate, especially output ordering and partitioning?
- Does the operator participate in filter pushdown, or silently block it?
- Are metrics recorded so `EXPLAIN ANALYZE` is useful?
