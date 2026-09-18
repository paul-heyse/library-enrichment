# `parquet::arrow::async_writer::AsyncArrowWriter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.async_writer.AsyncArrowWriter.json).

<a id="op-d9df4726e3133f2ae1d81ca9"></a>
## AsyncArrowWriter

`struct` · `parquet::arrow::async_writer::AsyncArrowWriter` · parquet 59.3.0

```rust
struct AsyncArrowWriter<W>
```

Source: `src/arrow/async_writer/mod.rs:157`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Encodes [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) to parquet, outputting to an [`AsyncFileWriter`](../operations/parquet.arrow.async_writer.AsyncFileWriter.md#op-d1acaf94f7f04028351c03f9)

## Memory Usage

This writer eagerly writes data as soon as possible to the underlying [`AsyncFileWriter`](../operations/parquet.arrow.async_writer.AsyncFileWriter.md#op-d1acaf94f7f04028351c03f9),
permitting fine-grained control over buffering and I/O scheduling. However, the columnar
nature of parquet forces data for an entire row group to be buffered in memory, before
it can be flushed. Depending on the data and the configured row group size, this buffering
may be substantial.

Memory usage can be limited by calling [`Self::flush`](../operations/parquet.arrow.async_writer.AsyncArrowWriter.md#op-e33e535332a5dcdf85001c53) to flush the in progress row group,
although this will likely increase overall file size and reduce query performance.
See [ArrowWriter](../operations/parquet.arrow.arrow_writer.ArrowWriter.md#op-dc781972049f13083eb8f4f3) for more information.

```no_run
# use tokio::fs::File;
# use arrow_array::RecordBatch;
# use parquet::arrow::AsyncArrowWriter;
# async fn test() {
let mut writer: AsyncArrowWriter<File> = todo!();
let batch: RecordBatch = todo!();
writer.write(&batch).await.unwrap();
// Trigger an early flush if buffered size exceeds 1_000_000
if writer.in_progress_size() > 1_000_000 {
    writer.flush().await.unwrap()
}
# }
```

<a id="op-29bc5b3e6ff95a70f3361089"></a>
## append_key_value_metadata

`function` · `parquet::arrow::async_writer::AsyncArrowWriter::append_key_value_metadata` · parquet 59.3.0

```rust
fn append_key_value_metadata(&mut self, kv_metadata: KeyValue)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::async_writer::AsyncArrowWriter", "path": "AsyncArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_writer::AsyncFileWriter", "path": "AsyncFileWriter"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [297, 2], "filename": "src/arrow/async_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_writer/mod.rs:243`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Append [`KeyValue`](../operations/parquet.file.metadata.KeyValue.md#op-c5f599a409980a2c03049085) metadata in addition to those in [`WriterProperties`](../operations/parquet.file.properties.WriterProperties.md#op-1a8b0462c7a132d1d5004af2)

This method allows to append metadata after [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es are written.

<a id="op-abc42fd1981f1aae8594bc34"></a>
## bytes_written

`function` · `parquet::arrow::async_writer::AsyncArrowWriter::bytes_written` · parquet 59.3.0

```rust
fn bytes_written(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::async_writer::AsyncArrowWriter", "path": "AsyncArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_writer::AsyncFileWriter", "path": "AsyncFileWriter"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [297, 2], "filename": "src/arrow/async_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_writer/mod.rs:215`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the number of bytes written by this instance

<a id="op-5c282b7320072d6623b69960"></a>
## close

`function` · `parquet::arrow::async_writer::AsyncArrowWriter::close` · parquet 59.3.0

```rust
async fn close(self) -> Result<ParquetMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::async_writer::AsyncArrowWriter", "path": "AsyncArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_writer::AsyncFileWriter", "path": "AsyncFileWriter"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [297, 2], "filename": "src/arrow/async_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_writer/mod.rs:267`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Close and finalize the writer.

All the data in the inner buffer will be force flushed.

<a id="op-a8b620f7190f319356ab73a2"></a>
## finish

`function` · `parquet::arrow::async_writer::AsyncArrowWriter::finish` · parquet 59.3.0

```rust
async fn finish(&mut self) -> Result<ParquetMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::async_writer::AsyncArrowWriter", "path": "AsyncArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_writer::AsyncFileWriter", "path": "AsyncFileWriter"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [297, 2], "filename": "src/arrow/async_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_writer/mod.rs:254`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Close and finalize the writer.

All the data in the inner buffer will be force flushed.

Unlike [`Self::close`](../operations/parquet.arrow.async_writer.AsyncArrowWriter.md#op-5c282b7320072d6623b69960) this does not consume self

Attempting to write after calling finish will result in an error

<a id="op-e33e535332a5dcdf85001c53"></a>
## flush

`function` · `parquet::arrow::async_writer::AsyncArrowWriter::flush` · parquet 59.3.0

```rust
async fn flush(&mut self) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::async_writer::AsyncArrowWriter", "path": "AsyncArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_writer::AsyncFileWriter", "path": "AsyncFileWriter"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [297, 2], "filename": "src/arrow/async_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_writer/mod.rs:233`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Flushes all buffered rows into a new row group

<a id="op-5c25a2377bd885af0d6ba5e4"></a>
## flushed_row_groups

`function` · `parquet::arrow::async_writer::AsyncArrowWriter::flushed_row_groups` · parquet 59.3.0

```rust
fn flushed_row_groups(&self) -> &[RowGroupMetaData]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::async_writer::AsyncArrowWriter", "path": "AsyncArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_writer::AsyncFileWriter", "path": "AsyncFileWriter"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [297, 2], "filename": "src/arrow/async_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_writer/mod.rs:191`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns metadata for any flushed row groups

<a id="op-7bfc3a9ee50bcc5bb38610bf"></a>
## in_progress_rows

`function` · `parquet::arrow::async_writer::AsyncArrowWriter::in_progress_rows` · parquet 59.3.0

```rust
fn in_progress_rows(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::async_writer::AsyncArrowWriter", "path": "AsyncArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_writer::AsyncFileWriter", "path": "AsyncFileWriter"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [297, 2], "filename": "src/arrow/async_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_writer/mod.rs:210`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the number of rows buffered in the in progress row group

<a id="op-26a2013e7be3a95588a9e0dc"></a>
## in_progress_size

`function` · `parquet::arrow::async_writer::AsyncArrowWriter::in_progress_size` · parquet 59.3.0

```rust
fn in_progress_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::async_writer::AsyncArrowWriter", "path": "AsyncArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_writer::AsyncFileWriter", "path": "AsyncFileWriter"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [297, 2], "filename": "src/arrow/async_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_writer/mod.rs:205`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Anticipated encoded size of the in progress row group.

See [ArrowWriter::memory_size](../operations/parquet.arrow.arrow_writer.ArrowWriter.md#op-a9f0f6c5417870b05921e32c) for more information.

<a id="op-0ca48a047df2b3f0b24fb11c"></a>
## into_inner

`function` · `parquet::arrow::async_writer::AsyncArrowWriter::into_inner` · parquet 59.3.0

```rust
fn into_inner(self) -> W
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::async_writer::AsyncArrowWriter", "path": "AsyncArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_writer::AsyncFileWriter", "path": "AsyncFileWriter"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [297, 2], "filename": "src/arrow/async_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_writer/mod.rs:277`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Consumes the [`AsyncArrowWriter`](../operations/parquet.arrow.async_writer.AsyncArrowWriter.md#op-d9df4726e3133f2ae1d81ca9) and returns the underlying [`AsyncFileWriter`](../operations/parquet.arrow.async_writer.AsyncFileWriter.md#op-d1acaf94f7f04028351c03f9)

# Notes

This method does **not** flush or finalize the writer, so buffered data
will be lost if you have not called [`Self::finish`](../operations/parquet.arrow.async_writer.AsyncArrowWriter.md#op-a8b620f7190f319356ab73a2).

<a id="op-c1ee28d160f6af47c36ec925"></a>
## memory_size

`function` · `parquet::arrow::async_writer::AsyncArrowWriter::memory_size` · parquet 59.3.0

```rust
fn memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::async_writer::AsyncArrowWriter", "path": "AsyncArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_writer::AsyncFileWriter", "path": "AsyncFileWriter"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [297, 2], "filename": "src/arrow/async_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_writer/mod.rs:198`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Estimated memory usage, in bytes, of this `ArrowWriter`

See [ArrowWriter::memory_size](../operations/parquet.arrow.arrow_writer.ArrowWriter.md#op-a9f0f6c5417870b05921e32c) for more information.

<a id="op-fd7548dcb462a9bacb3890b9"></a>
## try_new

`function` · `parquet::arrow::async_writer::AsyncArrowWriter::try_new` · parquet 59.3.0

```rust
fn try_new(writer: W, arrow_schema: SchemaRef, props: Option<WriterProperties>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::async_writer::AsyncArrowWriter", "path": "AsyncArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_writer::AsyncFileWriter", "path": "AsyncFileWriter"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [297, 2], "filename": "src/arrow/async_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_writer/mod.rs:167`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to create a new Async Arrow Writer

<a id="op-a7345afa30be1128dbd9eb09"></a>
## try_new_with_options

`function` · `parquet::arrow::async_writer::AsyncArrowWriter::try_new_with_options` · parquet 59.3.0

```rust
fn try_new_with_options(writer: W, arrow_schema: SchemaRef, options: ArrowWriterOptions) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::async_writer::AsyncArrowWriter", "path": "AsyncArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_writer::AsyncFileWriter", "path": "AsyncFileWriter"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [297, 2], "filename": "src/arrow/async_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_writer/mod.rs:177`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to create a new Async Arrow Writer with [`ArrowWriterOptions`](../operations/parquet.arrow.arrow_writer.ArrowWriterOptions.md#op-74a6b6dbd93b8f0178eea49c)

<a id="op-f09eb8c2ec767c2748d61c7d"></a>
## write

`function` · `parquet::arrow::async_writer::AsyncArrowWriter::write` · parquet 59.3.0

```rust
async fn write(&mut self, batch: &RecordBatch) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::async_writer::AsyncArrowWriter", "path": "AsyncArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_writer::AsyncFileWriter", "path": "AsyncFileWriter"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [297, 2], "filename": "src/arrow/async_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_writer/mod.rs:223`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Enqueues the provided `RecordBatch` to be written

After every sync write by the inner [ArrowWriter](../operations/parquet.arrow.arrow_writer.ArrowWriter.md#op-dc781972049f13083eb8f4f3), the inner buffer will be
checked and flush if at least half full
