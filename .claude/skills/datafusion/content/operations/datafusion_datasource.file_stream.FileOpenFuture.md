# `datafusion_datasource::file_stream::FileOpenFuture`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_stream.FileOpenFuture.json).

<a id="op-bfc3ccad5270df581bae26d3"></a>
## FileOpenFuture

`type_alias` · `datafusion_datasource::file_stream::FileOpenFuture` · datafusion-datasource 55.1.0

```rust
type FileOpenFuture = futures::future::BoxFuture<'static, datafusion_common::Result<futures::stream::BoxStream<'static, datafusion_common::Result<arrow::record_batch::RecordBatch>>>>
```

Source: `src/file_stream/mod.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A fallible future that resolves to a stream of [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)
