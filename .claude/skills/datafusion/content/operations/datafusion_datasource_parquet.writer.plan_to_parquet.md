# `datafusion_datasource_parquet::writer::plan_to_parquet`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.writer.plan_to_parquet.json).

<a id="op-5b7d7c07898a485239db893e"></a>
## plan_to_parquet

`function` · `datafusion_datasource_parquet::writer::plan_to_parquet` · datafusion-datasource-parquet 55.1.0

```rust
async fn plan_to_parquet(task_ctx: std::sync::Arc<datafusion_execution::TaskContext>, plan: std::sync::Arc<dyn ExecutionPlan>, path: impl AsRef<str>, writer_properties: Option<parquet::file::properties::WriterProperties>) -> datafusion_common::Result<()>
```

Source: `src/writer.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Executes a query and writes the results to a partitioned Parquet file.
