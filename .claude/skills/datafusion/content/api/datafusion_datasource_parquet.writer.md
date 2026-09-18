# `datafusion_datasource_parquet::writer`

Crate `datafusion-datasource-parquet` · 1 public items · structured records in [`model/datafusion_datasource_parquet.writer.json`](../model/datafusion_datasource_parquet.writer.json)

## plan_to_parquet

`function` · `datafusion_datasource_parquet::writer::plan_to_parquet`

Also reachable as `datafusion::datasource::physical_plan::parquet::plan_to_parquet`, `datafusion_datasource_parquet::plan_to_parquet`

```rust
async fn plan_to_parquet(task_ctx: std::sync::Arc<datafusion_execution::TaskContext>, plan: std::sync::Arc<dyn ExecutionPlan>, path: impl AsRef<str>, writer_properties: Option<parquet::file::properties::WriterProperties>) -> datafusion_common::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_parquet.writer.plan_to_parquet.md).


Executes a query and writes the results to a partitioned Parquet file.

---
