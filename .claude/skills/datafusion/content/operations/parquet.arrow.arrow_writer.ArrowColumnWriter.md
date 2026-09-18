# `parquet::arrow::arrow_writer::ArrowColumnWriter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_writer.ArrowColumnWriter.json).

<a id="op-b2f79c40222ecdd56db8aa79"></a>
## ArrowColumnWriter

`struct` · `parquet::arrow::arrow_writer::ArrowColumnWriter` · parquet 59.3.0

```rust
struct ArrowColumnWriter
```

Source: `src/arrow/arrow_writer/mod.rs:1090`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Encodes [`ArrowLeafColumn`](../operations/parquet.arrow.arrow_writer.ArrowLeafColumn.md#op-3606dfdb8cca5efdff1e83c8) to [`ArrowColumnChunk`](../operations/parquet.arrow.arrow_writer.ArrowColumnChunk.md#op-efe7caf4e16e381ce78e4e21)

`ArrowColumnWriter` instances can be created using an [`ArrowRowGroupWriterFactory`](../operations/parquet.arrow.arrow_writer.ArrowRowGroupWriterFactory.md#op-94cdff34b1d4b2324c1c0d54);

Note: This is a low-level interface for applications that require
fine-grained control of encoding (e.g. encoding using multiple threads),
see [`ArrowWriter`](../operations/parquet.arrow.arrow_writer.ArrowWriter.md#op-dc781972049f13083eb8f4f3) for a higher-level interface

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

<a id="op-c5db0fea588caa48fce60430"></a>
## close

`function` · `parquet::arrow::arrow_writer::ArrowColumnWriter::close` · parquet 59.3.0

```rust
fn close(self) -> Result<ArrowColumnChunk>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowColumnWriter", "path": "ArrowColumnWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1106, 1], "end": [1202, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:1162`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Close this column returning the written [`ArrowColumnChunk`](../operations/parquet.arrow.arrow_writer.ArrowColumnChunk.md#op-efe7caf4e16e381ce78e4e21)

<a id="op-76dfc499c0591c5f62c3342e"></a>
## fmt

`function` · `parquet::arrow::arrow_writer::ArrowColumnWriter::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowColumnWriter", "path": "ArrowColumnWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1095, 1], "end": [1099, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/arrow_writer/mod.rs:1096`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea8a3380edb269c669c04508"></a>
## get_estimated_total_bytes

`function` · `parquet::arrow::arrow_writer::ArrowColumnWriter::get_estimated_total_bytes` · parquet 59.3.0

```rust
fn get_estimated_total_bytes(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowColumnWriter", "path": "ArrowColumnWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1106, 1], "end": [1202, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:1196`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the estimated total encoded bytes for this column writer.

This includes:
1. Data buffered in encoded form
2. An estimate of how large the data buffered in un-encoded form would be once encoded

This value should be less than or equal to [`Self::memory_size`](../operations/parquet.arrow.arrow_writer.ArrowColumnWriter.md#op-7bf58191e4f4ed1395186064)

<a id="op-7bf58191e4f4ed1395186064"></a>
## memory_size

`function` · `parquet::arrow::arrow_writer::ArrowColumnWriter::memory_size` · parquet 59.3.0

```rust
fn memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowColumnWriter", "path": "ArrowColumnWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1106, 1], "end": [1202, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:1182`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the estimated total memory usage by the writer.

This  [`Self::get_estimated_total_bytes`](../operations/parquet.arrow.arrow_writer.ArrowColumnWriter.md#op-ea8a3380edb269c669c04508) this is an estimate
of the current memory usage and not it's anticipated encoded size.

This includes:
1. Data buffered in encoded form
2. Data buffered in un-encoded form (e.g. `usize` dictionary keys)

This value should be greater than or equal to [`Self::get_estimated_total_bytes`](../operations/parquet.arrow.arrow_writer.ArrowColumnWriter.md#op-ea8a3380edb269c669c04508)

<a id="op-0a60be408183d374631bf36b"></a>
## write

`function` · `parquet::arrow::arrow_writer::ArrowColumnWriter::write` · parquet 59.3.0

```rust
fn write(&mut self, col: &ArrowLeafColumn) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowColumnWriter", "path": "ArrowColumnWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1106, 1], "end": [1202, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:1108`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Write an [`ArrowLeafColumn`](../operations/parquet.arrow.arrow_writer.ArrowLeafColumn.md#op-3606dfdb8cca5efdff1e83c8)
