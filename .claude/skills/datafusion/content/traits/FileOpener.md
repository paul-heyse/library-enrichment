# FileOpener

`datafusion_datasource::file_stream::FileOpener`

```rust
trait FileOpener: Unpin + Send + Sync
```

Also reachable as `datafusion::datasource::physical_plan::FileOpener`

Prose: [`api/datafusion_datasource.file_stream.md`](../api/datafusion_datasource.file_stream.md#fileopener) · records: [`model/datafusion_datasource.file_stream.json`](../model/datafusion_datasource.file_stream.json)

## Required

Every implementation must supply these.

```rust
fn open(&self, partitioned_file: PartitionedFile) -> Result<FileOpenFuture>
```

## Implementors (4)

Read one before writing your own.

- `datafusion_datasource::projection::ProjectionOpener`
- `datafusion_datasource_arrow::source::ArrowOpener`
- `datafusion_datasource_csv::source::CsvOpener`
- `datafusion_datasource_json::source::JsonOpener`

## Documentation

Generic API for opening a file using an [`ObjectStore`] and resolving to a
stream of [`RecordBatch`]

[`ObjectStore`]: object_store::ObjectStore
