# `parquet::arrow::async_reader::ParquetRecordBatchStreamBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.async_reader.ParquetRecordBatchStreamBuilder.json).

<a id="op-36119f71505f76bb82a22932"></a>
## ParquetRecordBatchStreamBuilder

`type_alias` · `parquet::arrow::async_reader::ParquetRecordBatchStreamBuilder` · parquet 59.3.0

```rust
type ParquetRecordBatchStreamBuilder<T> = arrow::arrow_reader::ArrowReaderBuilder<AsyncReader<T>>
```

Source: `src/arrow/async_reader/mod.rs:302`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A builder for reading parquet files from an `async` source as  [`ParquetRecordBatchStream`](../operations/parquet.arrow.async_reader.ParquetRecordBatchStream.md#op-5aaf529c72b40fe254f58220)

This can be used to decode a Parquet file in streaming fashion (without
downloading the whole file at once) from a remote source, such as an object store.

This builder handles reading the parquet file metadata, allowing consumers
to use this information to select what specific columns, row groups, etc.
they wish to be read by the resulting stream.

See examples on [`ParquetRecordBatchStreamBuilder::new`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-a1314ec698521a426652f8d3), including how to
issue multiple I/O requests in parallel using multiple streams.

# See also:
* [`ParquetPushDecoderBuilder`](../operations/parquet.arrow.push_decoder.ParquetPushDecoderBuilder.md#op-be5e1b449f8e3584201abd08) for lower level control over buffering and
  decoding.
* [`ParquetRecordBatchStream::next_row_group`](../operations/parquet.arrow.async_reader.ParquetRecordBatchStream.md#op-6a7b74ccf9b4191804415952) for I/O prefetching


See [`ArrowReaderBuilder`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-2cb4803ec228a018e60491ee) for additional member functions
