# `parquet::arrow::arrow_writer::ArrowWriter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_writer.ArrowWriter.json).

<a id="op-dc781972049f13083eb8f4f3"></a>
## ArrowWriter

`struct` · `parquet::arrow::arrow_writer::ArrowWriter` · parquet 59.3.0

```rust
struct ArrowWriter<W: Write>
```

Source: `src/arrow/arrow_writer/mod.rs:182`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Encodes [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) to parquet

Writes Arrow `RecordBatch`es to a Parquet writer. Multiple [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) will be encoded
to the same row group, up to `max_row_group_size` rows. Any remaining rows will be
flushed on close, leading the final row group in the output file to potentially
contain fewer than `max_row_group_size` rows

# Example: Writing `RecordBatch`es
```
# use std::sync::Arc;
# use bytes::Bytes;
# use arrow_array::{ArrayRef, Int64Array};
# use arrow_array::RecordBatch;
# use parquet::arrow::arrow_writer::ArrowWriter;
# use parquet::arrow::arrow_reader::ParquetRecordBatchReader;
let col = Arc::new(Int64Array::from_iter_values([1, 2, 3])) as ArrayRef;
let to_write = RecordBatch::try_from_iter([("col", col)]).unwrap();

let mut buffer = Vec::new();
let mut writer = ArrowWriter::try_new(&mut buffer, to_write.schema(), None).unwrap();
writer.write(&to_write).unwrap();
writer.close().unwrap();

let mut reader = ParquetRecordBatchReader::try_new(Bytes::from(buffer), 1024).unwrap();
let read = reader.next().unwrap().unwrap();

assert_eq!(to_write, read);
```

# Memory Usage and Limiting

The nature of Parquet requires buffering of an entire row group before it can
be flushed to the underlying writer. Data is mostly buffered in its encoded
form, reducing memory usage. However, some data such as dictionary keys,
large strings or very nested data may still result in non-trivial memory
usage.

See Also:
* [`ArrowWriter::memory_size`](../operations/parquet.arrow.arrow_writer.ArrowWriter.md#op-a9f0f6c5417870b05921e32c): the current memory usage of the writer.
* [`ArrowWriter::in_progress_size`](../operations/parquet.arrow.arrow_writer.ArrowWriter.md#op-e5bc300f0e45d170e7ac1bfd): Estimated size of the buffered row group,

Call [`Self::flush`](../operations/parquet.arrow.arrow_writer.ArrowWriter.md#op-c06abc1686305492cd99b210) to trigger an early flush of a row group based on a
memory threshold and/or global memory pressure. However,  smaller row groups
result in higher metadata overheads, and thus may worsen compression ratios
and query performance.

```no_run
# use std::io::Write;
# use arrow_array::RecordBatch;
# use parquet::arrow::ArrowWriter;
# let mut writer: ArrowWriter<Vec<u8>> = todo!();
# let batch: RecordBatch = todo!();
writer.write(&batch).unwrap();
// Trigger an early flush if anticipated size exceeds 1_000_000
if writer.in_progress_size() > 1_000_000 {
    writer.flush().unwrap();
}
```

## Type Support

The writer supports writing all Arrow [`DataType`]s that have a direct mapping to
Parquet types including  [`StructArray`] and [`ListArray`].

The following are not supported:

* [`IntervalMonthDayNanoArray`]: Parquet does not [support nanosecond intervals].

[`DataType`]: https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html
[`StructArray`]: https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html
[`ListArray`]: https://docs.rs/arrow/latest/arrow/array/type.ListArray.html
[`IntervalMonthDayNanoArray`]: https://docs.rs/arrow/latest/arrow/array/type.IntervalMonthDayNanoArray.html
[support nanosecond intervals]: https://github.com/apache/parquet-format/blob/master/LogicalTypes.md#interval

## Type Compatibility
The writer can write Arrow [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)s that are logically equivalent. This means that for
a  given column, the writer can accept multiple Arrow [`DataType`]s that contain the same
value type.

For example, the following [`DataType`]s are all logically equivalent and can be written
to the same column:
* String, LargeString, StringView
* Binary, LargeBinary, BinaryView

The writer can will also accept both native and dictionary encoded arrays if the dictionaries
contain compatible values.
```
# use std::sync::Arc;
# use arrow_array::{DictionaryArray, LargeStringArray, RecordBatch, StringArray, UInt8Array};
# use arrow_schema::{DataType, Field, Schema};
# use parquet::arrow::arrow_writer::ArrowWriter;
let record_batch1 = RecordBatch::try_new(
   Arc::new(Schema::new(vec![Field::new("col", DataType::LargeUtf8, false)])),
   vec![Arc::new(LargeStringArray::from_iter_values(vec!["a", "b"]))]
 )
.unwrap();

let mut buffer = Vec::new();
let mut writer = ArrowWriter::try_new(&mut buffer, record_batch1.schema(), None).unwrap();
writer.write(&record_batch1).unwrap();

let record_batch2 = RecordBatch::try_new(
    Arc::new(Schema::new(vec![Field::new(
        "col",
        DataType::Dictionary(Box::new(DataType::UInt8), Box::new(DataType::Utf8)),
         false,
    )])),
    vec![Arc::new(DictionaryArray::new(
         UInt8Array::from_iter_values(vec![0, 1]),
         Arc::new(StringArray::from_iter_values(vec!["b", "c"])),
     ))],
 )
 .unwrap();
 writer.write(&record_batch2).unwrap();
 writer.close();
```

<a id="op-fbf50c26d753cc4de4779022"></a>
## append_key_value_metadata

`function` · `parquet::arrow::arrow_writer::ArrowWriter::append_key_value_metadata` · parquet 59.3.0

```rust
fn append_key_value_metadata(&mut self, kv_metadata: KeyValue)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:467`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Additional [`KeyValue`](../operations/parquet.file.metadata.KeyValue.md#op-c5f599a409980a2c03049085) metadata to be written in addition to those from [`WriterProperties`](../operations/parquet.file.properties.WriterProperties.md#op-1a8b0462c7a132d1d5004af2)

This method provide a way to append kv_metadata after write RecordBatch

<a id="op-665df59d128b89ebf3584148"></a>
## append_row_group

`function` · `parquet::arrow::arrow_writer::ArrowWriter::append_row_group` · parquet 59.3.0

```rust
fn append_row_group(&mut self, chunks: Vec<ArrowColumnChunk>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:527`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Append the given column chunks to the file as a new row group.

<a id="op-da99d85d4f466db2b344f0d6"></a>
## bytes_written

`function` · `parquet::arrow::arrow_writer::ArrowWriter::bytes_written` · parquet 59.3.0

```rust
fn bytes_written(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:343`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the number of bytes written by this instance

<a id="op-942c23b9c24bb46c35faf1c3"></a>
## close

`function` · `parquet::arrow::arrow_writer::ArrowWriter::close` · parquet 59.3.0

```rust
fn close(self) -> Result<ParquetMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:505`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Close and finalize the underlying Parquet writer

<a id="op-e37828f200d6f337e59bb822"></a>
## close

`function` · `parquet::arrow::arrow_writer::ArrowWriter::close` · parquet 59.3.0

```rust
fn close(self) -> std::result::Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [550, 1], "end": [559, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": {"args": null, "id": "arrow_array::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}, "trait_path": "arrow_array::record_batch::RecordBatchWriter"}`

Source: `src/arrow/arrow_writer/mod.rs:555`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3ffe6df4f328bbea3f62e5a"></a>
## finish

`function` · `parquet::arrow::arrow_writer::ArrowWriter::finish` · parquet 59.3.0

```rust
fn finish(&mut self) -> Result<ParquetMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:499`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Close and finalize the underlying Parquet writer

Unlike [`Self::close`](../operations/parquet.arrow.arrow_writer.ArrowWriter.md#op-942c23b9c24bb46c35faf1c3) this does not consume self

Attempting to write after calling finish will result in an error

<a id="op-c06abc1686305492cd99b210"></a>
## flush

`function` · `parquet::arrow::arrow_writer::ArrowWriter::flush` · parquet 59.3.0

```rust
fn flush(&mut self) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:450`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Flushes all buffered rows into a new row group

Note the underlying writer is not flushed with this call.
If this is a desired behavior, please call [`ArrowWriter::sync`](../operations/parquet.arrow.arrow_writer.ArrowWriter.md#op-bae112dc69b4f4b452201a5d).

<a id="op-dbc631961177743fe3fde6f6"></a>
## flushed_row_groups

`function` · `parquet::arrow::arrow_writer::ArrowWriter::flushed_row_groups` · parquet 59.3.0

```rust
fn flushed_row_groups(&self) -> &[RowGroupMetaData]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:302`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns metadata for any flushed row groups

<a id="op-001078c30aaff866ba152c1d"></a>
## fmt

`function` · `parquet::arrow::arrow_writer::ArrowWriter::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [207, 1], "end": [219, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/arrow_writer/mod.rs:208`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11c269cc106cf6fdb4385189"></a>
## get_column_writers

`function` · `parquet::arrow::arrow_writer::ArrowWriter::get_column_writers` · parquet 59.3.0

```rust
fn get_column_writers(&mut self) -> Result<Vec<ArrowColumnWriter>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:514`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new row group writer and return its column writers.

<a id="op-6073a5d9bc2cbdea9490fb8d"></a>
## in_progress_rows

`function` · `parquet::arrow::arrow_writer::ArrowWriter::in_progress_rows` · parquet 59.3.0

```rust
fn in_progress_rows(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:335`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the number of rows buffered in the in progress row group

<a id="op-e5bc300f0e45d170e7ac1bfd"></a>
## in_progress_size

`function` · `parquet::arrow::arrow_writer::ArrowWriter::in_progress_size` · parquet 59.3.0

```rust
fn in_progress_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:323`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Anticipated encoded size of the in progress row group.

This estimate the row group size after being completely encoded is,
formed by summing the values of
[`ArrowColumnWriter::get_estimated_total_bytes`](../operations/parquet.arrow.arrow_writer.ArrowColumnWriter.md#op-ea8a3380edb269c669c04508) for all in progress
columns.

<a id="op-edc3366b89b6bf4e2442529a"></a>
## inner

`function` · `parquet::arrow::arrow_writer::ArrowWriter::inner` · parquet 59.3.0

```rust
fn inner(&self) -> &W
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:472`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a reference to the underlying writer.

<a id="op-5b7923eab4a891091ab455d2"></a>
## inner_mut

`function` · `parquet::arrow::arrow_writer::ArrowWriter::inner_mut` · parquet 59.3.0

```rust
fn inner_mut(&mut self) -> &mut W
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:484`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a mutable reference to the underlying writer.

**Warning**: if you write directly to this writer, you will skip
the `TrackedWrite` buffering and byte‐counting layers. That’ll cause
the file footer’s recorded offsets and sizes to diverge from reality,
resulting in an unreadable or corrupted Parquet file.

If you want to write safely to the underlying writer, use [`Self::write_all`](../operations/parquet.arrow.arrow_writer.ArrowWriter.md#op-76feae78e2a66be382a7c260).

<a id="op-40b9d7ee4cfd8cde1e8c6f8b"></a>
## into_inner

`function` · `parquet::arrow::arrow_writer::ArrowWriter::into_inner` · parquet 59.3.0

```rust
fn into_inner(self) -> Result<W>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:489`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Flushes any outstanding data and returns the underlying writer.

<a id="op-6087fb00683dd3af12093f58"></a>
## into_serialized_writer

`function` · `parquet::arrow::arrow_writer::ArrowWriter::into_serialized_writer` · parquet 59.3.0

```rust
fn into_serialized_writer(self) -> Result<(SerializedFileWriter<W>, ArrowRowGroupWriterFactory)>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:542`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts this writer into a lower-level [`SerializedFileWriter`](../operations/parquet.file.writer.SerializedFileWriter.md#op-0b9cc37c23222f2e8aef5d0f) and [`ArrowRowGroupWriterFactory`](../operations/parquet.arrow.arrow_writer.ArrowRowGroupWriterFactory.md#op-94cdff34b1d4b2324c1c0d54).

Flushes any outstanding data before returning.

This can be useful to provide more control over how files are written, for example
to write columns in parallel. See the example on [`ArrowColumnWriter`](../operations/parquet.arrow.arrow_writer.ArrowColumnWriter.md#op-b2f79c40222ecdd56db8aa79).

<a id="op-a9f0f6c5417870b05921e32c"></a>
## memory_size

`function` · `parquet::arrow::arrow_writer::ArrowWriter::memory_size` · parquet 59.3.0

```rust
fn memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:310`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Estimated memory usage, in bytes, of this `ArrowWriter`

This estimate is formed bu summing the values of
[`ArrowColumnWriter::memory_size`](../operations/parquet.arrow.arrow_writer.ArrowColumnWriter.md#op-7bf58191e4f4ed1395186064) all in progress columns.

<a id="op-bae112dc69b4f4b452201a5d"></a>
## sync

`function` · `parquet::arrow::arrow_writer::ArrowWriter::sync` · parquet 59.3.0

```rust
fn sync(&mut self) -> std::io::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:442`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Flushes underlying writer

<a id="op-b1a094922dc9e224e14c2823"></a>
## try_new

`function` · `parquet::arrow::arrow_writer::ArrowWriter::try_new` · parquet 59.3.0

```rust
fn try_new(writer: W, arrow_schema: SchemaRef, props: Option<WriterProperties>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:227`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to create a new Arrow writer

The writer will fail if:
 * a `SerializedFileWriter` cannot be created from the ParquetWriter
 * the Arrow schema contains unsupported datatypes such as Unions

<a id="op-d7bcdaf15cee96f668d89f38"></a>
## try_new_with_options

`function` · `parquet::arrow::arrow_writer::ArrowWriter::try_new_with_options` · parquet 59.3.0

```rust
fn try_new_with_options(writer: W, arrow_schema: SchemaRef, options: ArrowWriterOptions) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:241`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to create a new Arrow writer with [`ArrowWriterOptions`](../operations/parquet.arrow.arrow_writer.ArrowWriterOptions.md#op-74a6b6dbd93b8f0178eea49c).

The writer will fail if:
 * a `SerializedFileWriter` cannot be created from the ParquetWriter
 * the Arrow schema contains unsupported datatypes such as Unions

<a id="op-4ac1f80718d1c2a5b981b4c5"></a>
## write

`function` · `parquet::arrow::arrow_writer::ArrowWriter::write` · parquet 59.3.0

```rust
fn write(&mut self, batch: &RecordBatch) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:358`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Encodes the provided [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)

If this would cause the current row group to exceed [`WriterProperties::max_row_group_row_count`](../operations/parquet.file.properties.WriterProperties.md#op-59c83d225f83bcf8d0c3dbd2)
rows or [`WriterProperties::max_row_group_bytes`](../operations/parquet.file.properties.WriterProperties.md#op-48d7f3ca1609a19a044b460d) bytes, the contents of `batch` will be
written to one or more row groups such that limits are respected.

If both limits are `None`, all data is written to a single row group.
If one limit is set, that limit is respected.
If both limits are set, the lower bound (whichever triggers first) is respected.

This will fail if the `batch`'s schema does not match the writer's schema.

<a id="op-8913d2632255af6ca0cdfcc8"></a>
## write

`function` · `parquet::arrow::arrow_writer::ArrowWriter::write` · parquet 59.3.0

```rust
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [550, 1], "end": [559, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": {"args": null, "id": "arrow_array::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}, "trait_path": "arrow_array::record_batch::RecordBatchWriter"}`

Source: `src/arrow/arrow_writer/mod.rs:551`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76feae78e2a66be382a7c260"></a>
## write_all

`function` · `parquet::arrow::arrow_writer::ArrowWriter::write_all` · parquet 59.3.0

```rust
fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::arrow::arrow_writer::ArrowWriter", "path": "ArrowWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [548, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:437`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Writes the given buf bytes to the internal buffer.

It's safe to use this method to write data to the underlying writer,
because it will ensure that the buffering and byte‐counting layers are used.
