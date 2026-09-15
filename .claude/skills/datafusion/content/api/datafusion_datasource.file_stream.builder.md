# `datafusion_datasource::file_stream::builder`

Crate `datafusion-datasource` · 1 public items · structured records in [`model/datafusion_datasource.file_stream.builder.json`](../model/datafusion_datasource.file_stream.builder.json)

## FileStreamBuilder

`struct` · `datafusion_datasource::file_stream::builder::FileStreamBuilder`

Also reachable as `datafusion::datasource::physical_plan::FileStreamBuilder`, `datafusion_datasource::file_stream::FileStreamBuilder`

```rust
struct FileStreamBuilder<'a>
```

**Methods** (7)

```rust
fn build(self) -> Result<FileStream>
fn new(config: &'a FileScanConfig) -> Self
fn with_file_opener(self, file_opener: Arc<dyn FileOpener>) -> Self
fn with_metrics(self, metrics: &'a ExecutionPlanMetricsSet) -> Self
fn with_morselizer(self, morselizer: Box<dyn Morselizer>) -> Self
fn with_on_error(self, on_error: OnError) -> Self
fn with_partition(self, partition: usize) -> Self
```

Builder for constructing a [`FileStream`].

---
