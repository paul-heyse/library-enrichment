# `parquet::arrow::arrow_writer`

Crate `parquet` · 8 public items · structured records in [`model/parquet.arrow.arrow_writer.json`](../model/parquet.arrow.arrow_writer.json)

## compute_leaves

`function` · `parquet::arrow::arrow_writer::compute_leaves`

```rust
fn compute_leaves(field: &arrow_schema::Field, array: &arrow_array::ArrayRef) -> errors::Result<Vec<ArrowLeafColumn>>
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_writer.compute_leaves.md).


Computes the [`ArrowLeafColumn`] for a potentially nested [`ArrayRef`]

This function can be used along with [`get_column_writers`] to encode
individual columns in parallel. See example on [`ArrowColumnWriter`]

---

## get_column_writers

`function` · `parquet::arrow::arrow_writer::get_column_writers`

> **Deprecated** — since 57.0.0: Use `ArrowRowGroupWriterFactory` instead

```rust
fn get_column_writers(parquet: &schema::types::SchemaDescriptor, props: &file::properties::WriterPropertiesPtr, arrow: &arrow_schema::SchemaRef) -> errors::Result<Vec<ArrowColumnWriter>>
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_writer.get_column_writers.md).


Returns [`ArrowColumnWriter`]s for each column in a given schema

---

## ArrowColumnChunk

`struct` · `parquet::arrow::arrow_writer::ArrowColumnChunk`

```rust
struct ArrowColumnChunk
```

**Derives**: Debug

**Methods** (3)

```rust
fn append_to_row_group<W: Write + Send>(self, writer: &mut SerializedRowGroupWriter<'_, W>) -> Result<()>
fn close(&self) -> &ColumnCloseResult
fn close_mut(&mut self) -> &mut ColumnCloseResult
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_writer.ArrowColumnChunk.md).


The data for a single column chunk, see [`ArrowColumnWriter`]

---

## ArrowColumnWriter

`struct` · `parquet::arrow::arrow_writer::ArrowColumnWriter`

```rust
struct ArrowColumnWriter
```

**Derives**: Debug

**Methods** (4)

```rust
fn close(self) -> Result<ArrowColumnChunk>
fn get_estimated_total_bytes(&self) -> usize
fn memory_size(&self) -> usize
fn write(&mut self, col: &ArrowLeafColumn) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_writer.ArrowColumnWriter.md).


Encodes [`ArrowLeafColumn`] to [`ArrowColumnChunk`]

`ArrowColumnWriter` instances can be created using an [`ArrowRowGroupWriterFactory`];

Note: This is a low-level interface for applications that require
fine-grained control of encoding (e.g. encoding using multiple threads),
see [`ArrowWriter`] for a higher-level interface

# Example: Encoding two Arrow Array's in Parallel
```
// The arrow schema
# use std::sync::Arc;
# use arrow_array::*;
# use arrow_schema::*;
# use parquet::arrow::ArrowSchemaConverter;
# use parquet::arrow::arrow_writer::{compute_leaves, ArrowColumnChunk, ArrowLeafColumn, ArrowRowGroupWriterFactory};
# use parquet::file::properties::WriterProperties;
# use parquet::file::writer::{SerializedFileWriter, SerializedRowGroupWriter};
#
let schema = Arc::new(Schema::new(vec![
    Field::new("i32", DataType::Int32, false),
    Field::new("f32", DataType::Float32, false),
]));

// Compute the parquet schema
let props = Arc::new(WriterProperties::default());
let parquet_schema = ArrowSchemaConverter::new()
  .with_coerce_types(props.coerce_types())
  .convert(&schema)
  .unwrap();

// Create parquet writer
let root_schema = parquet_schema.root_schema_ptr();
// write to memory in the example, but this could be a File
let mut out = Vec::with_capacity(1024);
let mut writer = SerializedFileWriter::new(&mut out, root_schema, props.clone())
  .unwrap();

// Create a factory for building Arrow column writers
let row_group_factory = ArrowRowGroupWriterFactory::new(&writer, Arc::clone(&schema));
// Create column writers for the 0th row group
let col_writers = row_group_factory.create_column_writers(0).unwrap();

// Spawn a worker thread for each column
//
// Note: This is for demonstration purposes, a thread-pool e.g. rayon or tokio, would be better.
// The `map` produces an iterator of type `tuple of (thread handle, send channel)`.
let mut workers: Vec<_> = col_writers
    .into_iter()
    .map(|mut col_writer| {
        let (send, recv) = std::sync::mpsc::channel::<ArrowLeafColumn>();
        let handle = std::thread::spawn(move || {
            // receive Arrays to encode via the channel
            for col in recv {
                col_writer.write(&col)?;
            }
            // once the input is complete, close the writer
            // to return the newly created ArrowColumnChunk
            col_writer.close()
        });
        (handle, send)
    })
    .collect();

// Start row group
let mut row_group_writer: SerializedRowGroupWriter<'_, _> = writer
  .next_row_group()
  .unwrap();

// Create some example input columns to encode
let to_write = vec![
    Arc::new(Int32Array::from_iter_values([1, 2, 3])) as _,
    Arc::new(Float32Array::from_iter_values([1., 45., -1.])) as _,
];

// Send the input columns to the workers
let mut worker_iter = workers.iter_mut();
for (arr, field) in to_write.iter().zip(&schema.fields) {
    for leaves in compute_leaves(field, arr).unwrap() {
        worker_iter.next().unwrap().1.send(leaves).unwrap();
    }
}

// Wait for the workers to complete encoding, and append
// the resulting column chunks to the row group (and the file)
for (handle, send) in workers {
    drop(send); // Drop send side to signal termination
    // wait for the worker to send the completed chunk
    let chunk: ArrowColumnChunk = handle.join().unwrap().unwrap();
    chunk.append_to_row_group(&mut row_group_writer).unwrap();
}
// Close the row group which writes to the underlying file
row_group_writer.close().unwrap();

let metadata = writer.close().unwrap();
assert_eq!(metadata.file_metadata().num_rows(), 3);
```

---

## ArrowLeafColumn

`struct` · `parquet::arrow::arrow_writer::ArrowLeafColumn`

```rust
struct ArrowLeafColumn
```

**Derives**: Debug

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_writer.ArrowLeafColumn.md).


A leaf column that can be encoded by [`ArrowColumnWriter`]

---

## ArrowRowGroupWriterFactory

`struct` · `parquet::arrow::arrow_writer::ArrowRowGroupWriterFactory`

```rust
struct ArrowRowGroupWriterFactory
```

**Derives**: Debug

**Methods** (3)

```rust
fn create_column_writers(&self, row_group_index: usize) -> Result<Vec<ArrowColumnWriter>>
fn new<W: Write + Send>(file_writer: &SerializedFileWriter<W>, arrow_schema: SchemaRef) -> Self
fn with_page_store_factory(self, page_store_factory: Arc<dyn PageStoreFactory>) -> Self
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_writer.ArrowRowGroupWriterFactory.md).


Factory that creates new column writers for each row group in the Parquet file.

You can create this structure via an [`ArrowWriter::into_serialized_writer`].
See the example on [`ArrowColumnWriter`] for how to encode columns in parallel

---

## ArrowWriter

`struct` · `parquet::arrow::arrow_writer::ArrowWriter`

Also reachable as `parquet::arrow::ArrowWriter`

```rust
struct ArrowWriter<W: Write>
```

**Implements**: `arrow_array::record_batch::RecordBatchWriter`

**Derives**: Debug

**Methods** (20)

```rust
fn append_key_value_metadata(&mut self, kv_metadata: KeyValue)
fn append_row_group(&mut self, chunks: Vec<ArrowColumnChunk>) -> Result<()>
fn bytes_written(&self) -> usize
fn close(self) -> Result<ParquetMetaData>
fn finish(&mut self) -> Result<ParquetMetaData>
fn flush(&mut self) -> Result<()>
fn flushed_row_groups(&self) -> &[RowGroupMetaData]
fn get_column_writers(&mut self) -> Result<Vec<ArrowColumnWriter>>
fn in_progress_rows(&self) -> usize
fn in_progress_size(&self) -> usize
fn inner(&self) -> &W
fn inner_mut(&mut self) -> &mut W
fn into_inner(self) -> Result<W>
fn into_serialized_writer(self) -> Result<(SerializedFileWriter<W>, ArrowRowGroupWriterFactory)>
fn memory_size(&self) -> usize
fn sync(&mut self) -> std::io::Result<()>
fn try_new(writer: W, arrow_schema: SchemaRef, props: Option<WriterProperties>) -> Result<Self>
fn try_new_with_options(writer: W, arrow_schema: SchemaRef, options: ArrowWriterOptions) -> Result<Self>
fn write(&mut self, batch: &RecordBatch) -> Result<()>
fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()>
```

**via `arrow_array::record_batch::RecordBatchWriter`**

```rust
fn close(self) -> std::result::Result<(), ArrowError>
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_writer.ArrowWriter.md).


Encodes [`RecordBatch`] to parquet

Writes Arrow `RecordBatch`es to a Parquet writer. Multiple [`RecordBatch`] will be encoded
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
* [`ArrowWriter::memory_size`]: the current memory usage of the writer.
* [`ArrowWriter::in_progress_size`]: Estimated size of the buffered row group,

Call [`Self::flush`] to trigger an early flush of a row group based on a
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
The writer can write Arrow [`RecordBatch`]s that are logically equivalent. This means that for
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

---

## ArrowWriterOptions

`struct` · `parquet::arrow::arrow_writer::ArrowWriterOptions`

```rust
struct ArrowWriterOptions
```

**Derives**: Clone, Debug, Default

**Methods** (6)

```rust
fn new() -> Self
fn with_page_store_factory(self, page_store_factory: Arc<dyn PageStoreFactory>) -> Self
fn with_parquet_schema(self, schema_descr: SchemaDescriptor) -> Self
fn with_properties(self, properties: WriterProperties) -> Self
fn with_schema_root(self, schema_root: String) -> Self
fn with_skip_arrow_metadata(self, skip_arrow_metadata: bool) -> Self
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_writer.ArrowWriterOptions.md).


Arrow-specific configuration settings for writing parquet files.

See [`ArrowWriter`] for how to configure the writer.

---
