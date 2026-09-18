# `parquet::arrow::async_writer`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.async_writer.json).

<a id="op-7658fdf69eb303596f1af00d"></a>
## async_writer

`module` · `parquet::arrow::async_writer` · parquet 59.3.0

```rust
mod async_writer
```

Source: `src/arrow/async_writer/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

`async` API for writing [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es to Parquet files

See the [crate-level documentation](crate) for more details.

The `async` API for writing [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es is
similar to the [`sync` API](ArrowWriter), so please
read the documentation there before using this API.

Here is an example for using [`AsyncArrowWriter`](../operations/parquet.arrow.async_writer.AsyncArrowWriter.md#op-d9df4726e3133f2ae1d81ca9):

```
# #[tokio::main(flavor="current_thread")]
# async fn main() {
#
# use std::sync::Arc;
# use arrow_array::{ArrayRef, Int64Array, RecordBatch, RecordBatchReader};
# use bytes::Bytes;
# use parquet::arrow::{AsyncArrowWriter, arrow_reader::ParquetRecordBatchReaderBuilder};
#
let col = Arc::new(Int64Array::from_iter_values([1, 2, 3])) as ArrayRef;
let to_write = RecordBatch::try_from_iter([("col", col)]).unwrap();

let mut buffer = Vec::new();
let mut writer = AsyncArrowWriter::try_new(&mut buffer, to_write.schema(), None).unwrap();
writer.write(&to_write).await.unwrap();
writer.close().await.unwrap();

let buffer = Bytes::from(buffer);
let mut reader = ParquetRecordBatchReaderBuilder::try_new(buffer.clone())
    .unwrap()
    .build()
    .unwrap();
let read = reader.next().unwrap().unwrap();

assert_eq!(to_write, read);
# }
```

There is a blanket implementation of [`AsyncFileWriter`](../operations/parquet.arrow.async_writer.AsyncFileWriter.md#op-d1acaf94f7f04028351c03f9) for all types that
implement [`AsyncWrite`], so writers such as `tokio::fs::File`, or
`object_store::buffered::BufWriter` for writing to object storage, can be
passed to [`AsyncArrowWriter`](../operations/parquet.arrow.async_writer.AsyncArrowWriter.md#op-d9df4726e3133f2ae1d81ca9) directly.

Unresolved upstream links (retained, not inferred): ``AsyncWrite``.
