# `datafusion_datasource::file_stream`

Crate `datafusion-datasource` · 4 public items · structured records in [`model/datafusion_datasource.file_stream.json`](../model/datafusion_datasource.file_stream.json)

## OnError

`enum` · `datafusion_datasource::file_stream::OnError`

Also reachable as `datafusion::datasource::physical_plan::OnError`

```rust
enum OnError
```

**Variants**: `Fail`, `Skip`

**Derives**: Default

Describes the behavior of the `FileStream` if file opening or scanning fails

---

## FileStream

`struct` · `datafusion_datasource::file_stream::FileStream`

Also reachable as `datafusion::datasource::physical_plan::FileStream`

```rust
struct FileStream
```

**Implements**: `datafusion_execution::stream::RecordBatchStream`, `futures_core::stream::Stream`

**Methods** (2)

```rust
fn new(config: &FileScanConfig, partition: usize, file_opener: Arc<dyn FileOpener>, metrics: &ExecutionPlanMetricsSet) -> Result<Self>
fn with_on_error(self, on_error: OnError) -> Self
```

**via `datafusion_execution::stream::RecordBatchStream`**

```rust
fn schema(&self) -> SchemaRef
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

A stream that iterates record batch by record batch, file over file.

---

## FileOpener

`trait` · `datafusion_datasource::file_stream::FileOpener`

Also reachable as `datafusion::datasource::physical_plan::FileOpener`

```rust
trait FileOpener: Unpin + Send + Sync
```

**Implementors** (4)

- `datafusion_datasource::projection::ProjectionOpener`
- `datafusion_datasource_arrow::source::ArrowOpener`
- `datafusion_datasource_csv::source::CsvOpener`
- `datafusion_datasource_json::source::JsonOpener`

**Methods** (1)

```rust
fn open(&self, partitioned_file: PartitionedFile) -> Result<FileOpenFuture>
```

Generic API for opening a file using an [`ObjectStore`] and resolving to a
stream of [`RecordBatch`]

[`ObjectStore`]: object_store::ObjectStore

---

## FileOpenFuture

`type_alias` · `datafusion_datasource::file_stream::FileOpenFuture`

Also reachable as `datafusion::datasource::physical_plan::FileOpenFuture`

```rust
type FileOpenFuture = futures::future::BoxFuture<'static, datafusion_common::Result<futures::stream::BoxStream<'static, datafusion_common::Result<arrow::record_batch::RecordBatch>>>>
```

A fallible future that resolves to a stream of [`RecordBatch`]

---
