# `datafusion::test_util::parquet`

Crate `datafusion` · 2 public items · structured records in [`model/datafusion.test_util.parquet.json`](../model/datafusion.test_util.parquet.json)

## ParquetScanOptions

`struct` · `datafusion::test_util::parquet::ParquetScanOptions`

```rust
struct ParquetScanOptions
```

**Fields**: `pushdown_filters`, `reorder_filters`, `enable_page_index`

**Derives**: Clone, Copy, Debug

**Methods** (1)

```rust
fn config(&self) -> SessionConfig
```

Options for how to create the parquet scan

---

## TestParquetFile

`struct` · `datafusion::test_util::parquet::TestParquetFile`

```rust
struct TestParquetFile
```

**Methods** (5)

```rust
fn create_scan(&self, ctx: &SessionContext, maybe_filter: Option<Expr>) -> Result<Arc<dyn ExecutionPlan>>
fn parquet_metrics(plan: &Arc<dyn ExecutionPlan>) -> Option<MetricsSet>
fn path(&self) -> &std::path::Path
fn schema(&self) -> SchemaRef
fn try_new(path: PathBuf, props: WriterProperties, batches: impl IntoIterator<Item = RecordBatch>) -> Result<Self>
```

a ParquetFile that has been created for testing.

---
