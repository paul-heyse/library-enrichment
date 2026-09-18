# `parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.ParquetRecordBatchReaderBuilder.json).

<a id="op-1638827d919dceda05d4f2dd"></a>
## ParquetRecordBatchReaderBuilder

`type_alias` · `parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder` · parquet 59.3.0

```rust
type ParquetRecordBatchReaderBuilder<T> = ArrowReaderBuilder<SyncReader<T>>
```

Source: `src/arrow/arrow_reader/mod.rs:1073`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates [`ParquetRecordBatchReader`](../operations/parquet.arrow.arrow_reader.ParquetRecordBatchReader.md#op-d346feb61c9116e17f0cd9c3) for reading Parquet files into Arrow [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es

# See Also
* [`crate::arrow::async_reader::ParquetRecordBatchStreamBuilder`](../operations/parquet.arrow.async_reader.ParquetRecordBatchStreamBuilder.md#op-36119f71505f76bb82a22932) for an async API
* [`crate::arrow::push_decoder::ParquetPushDecoderBuilder`](../operations/parquet.arrow.push_decoder.ParquetPushDecoderBuilder.md#op-be5e1b449f8e3584201abd08) for a SansIO decoder API
* [`ArrowReaderBuilder`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-2cb4803ec228a018e60491ee) for additional member functions
