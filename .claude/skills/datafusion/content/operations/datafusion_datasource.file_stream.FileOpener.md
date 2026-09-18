# `datafusion_datasource::file_stream::FileOpener`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_stream.FileOpener.json).

<a id="op-de9e825a0c7367b18e03d6f1"></a>
## FileOpener

`trait` · `datafusion_datasource::file_stream::FileOpener` · datafusion-datasource 55.1.0

```rust
trait FileOpener: Unpin + Send + Sync
```

Source: `src/file_stream/mod.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Generic API for opening a file using an [`ObjectStore`] and resolving to a
stream of [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)

[`ObjectStore`]: object_store::ObjectStore

<a id="op-15f0f8428e55d4bd45f27910"></a>
## open

`function` · `datafusion_datasource::file_stream::FileOpener::open` · datafusion-datasource 55.1.0

```rust
fn open(&self, partitioned_file: PartitionedFile) -> Result<FileOpenFuture>
```

Source: `src/file_stream/mod.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Asynchronously open the specified file and return a stream
of [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)
