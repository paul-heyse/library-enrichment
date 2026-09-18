# `arrow_ipc::writer::StreamWriter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.writer.StreamWriter.json).

<a id="op-b808ea0aac7879e19d25b939"></a>
## StreamWriter

`struct` · `arrow_ipc::writer::StreamWriter` · arrow-ipc 59.3.0

```rust
struct StreamWriter<W>
```

Source: `src/writer.rs:2015`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Arrow Stream Writer

Writes Arrow [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es to bytes using the [IPC Streaming Format].

# See Also

* [`FileWriter`](../operations/arrow_ipc.writer.FileWriter.md#op-de36feaee6f6c9a3aec5da12) for writing IPC Files

# Example - Basic usage
```
# use arrow_array::record_batch;
# use arrow_ipc::writer::StreamWriter;
# let mut stream = vec![]; // mimic a stream for the example
let batch = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
// create a new writer, the schema must be known in advance
let mut writer = StreamWriter::try_new(&mut stream, &batch.schema()).unwrap();
// write each batch to the underlying stream
writer.write(&batch).unwrap();
// When all batches are written, call finish to flush all buffers
writer.finish().unwrap();
```
# Example - Efficient delta dictionaries
```
# use arrow_array::record_batch;
# use arrow_ipc::writer::{StreamWriter, IpcWriteOptions};
# use arrow_ipc::writer::DictionaryHandling;
# use arrow_schema::{DataType, Field, Schema, SchemaRef};
# use arrow_array::{
#    builder::StringDictionaryBuilder, types::Int32Type, Array, ArrayRef, DictionaryArray,
#    RecordBatch, StringArray,
# };
# use std::sync::Arc;

let schema = Arc::new(Schema::new(vec![Field::new(
   "col1",
   DataType::Dictionary(Box::from(DataType::Int32), Box::from(DataType::Utf8)),
   true,
)]));

let mut builder = StringDictionaryBuilder::<arrow_array::types::Int32Type>::new();

// `finish_preserve_values` will keep the dictionary values along with their
// key assignments so that they can be re-used in the next batch.
builder.append("a").unwrap();
builder.append("b").unwrap();
let array1 = builder.finish_preserve_values();
let batch1 = RecordBatch::try_new(schema.clone(), vec![Arc::new(array1) as ArrayRef]).unwrap();

// In this batch, 'a' will have the same dictionary key as 'a' in the previous batch,
// and 'd' will take the next available key.
builder.append("a").unwrap();
builder.append("d").unwrap();
let array2 = builder.finish_preserve_values();
let batch2 = RecordBatch::try_new(schema.clone(), vec![Arc::new(array2) as ArrayRef]).unwrap();

let mut stream = vec![];
// You must set `.with_dictionary_handling(DictionaryHandling::Delta)` to
// enable delta dictionaries in the writer
let options = IpcWriteOptions::default().with_dictionary_handling(DictionaryHandling::Delta);
let mut writer = StreamWriter::try_new_with_options(&mut stream, &schema, options).unwrap();

// When writing the first batch, a dictionary message with 'a' and 'b' will be written
// prior to the record batch.
writer.write(&batch1).unwrap();
// With the second batch only a delta dictionary with 'd' will be written
// prior to the record batch. This is only possible with `finish_preserve_values`.
// Without it, 'a' and 'd' in this batch would have different keys than the
// first batch and so we'd have to send a replacement dictionary with new keys
// for both.
writer.write(&batch2).unwrap();
writer.finish().unwrap();
```
[IPC Streaming Format]: https://arrow.apache.org/docs/format/Columnar.html#ipc-streaming-format

<a id="op-3d644ef8b80a5b12179cb587"></a>
## close

`function` · `arrow_ipc::writer::StreamWriter::close` · arrow-ipc 59.3.0

```rust
fn close(self) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::StreamWriter", "path": "StreamWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2186, 1], "end": [2194, 2], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "arrow_array::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}, "trait_path": "arrow_array::record_batch::RecordBatchWriter"}`

Source: `src/writer.rs:2191`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfd74b35f68e3588da399f1f"></a>
## finish

`function` · `arrow_ipc::writer::StreamWriter::finish` · arrow-ipc 59.3.0

```rust
fn finish(&mut self) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::StreamWriter", "path": "StreamWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2039, 1], "end": [2184, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:2103`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Write continuation bytes, and mark the stream as done

<a id="op-34b59c4b7036cac83d9f6030"></a>
## flush

`function` · `arrow_ipc::writer::StreamWriter::flush` · arrow-ipc 59.3.0

```rust
fn flush(&mut self) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::StreamWriter", "path": "StreamWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2039, 1], "end": [2184, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:2135`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Flush the underlying writer.

Both the BufWriter and the underlying writer are flushed.

<a id="op-6475cc55b408a54258a79946"></a>
## get_mut

`function` · `arrow_ipc::writer::StreamWriter::get_mut` · arrow-ipc 59.3.0

```rust
fn get_mut(&mut self) -> &mut W
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::StreamWriter", "path": "StreamWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2039, 1], "end": [2184, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:2128`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Gets a mutable reference to the underlying writer.

It is inadvisable to directly write to the underlying writer.

<a id="op-646a2ab874f091eaa486a6bd"></a>
## get_ref

`function` · `arrow_ipc::writer::StreamWriter::get_ref` · arrow-ipc 59.3.0

```rust
fn get_ref(&self) -> &W
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::StreamWriter", "path": "StreamWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2039, 1], "end": [2184, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:2121`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Gets a reference to the underlying writer.

<a id="op-312891b0974b18d1552d982c"></a>
## into_inner

`function` · `arrow_ipc::writer::StreamWriter::into_inner` · arrow-ipc 59.3.0

```rust
fn into_inner(self) -> Result<W, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::StreamWriter", "path": "StreamWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2039, 1], "end": [2184, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:2177`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Unwraps the the underlying writer.

The writer is flushed and the StreamWriter is finished before returning.

# Errors

An ['Err'](Result::Err) may be returned if an error occurs while finishing the StreamWriter
or while flushing the writer.

# Example

```
# use arrow_ipc::writer::{StreamWriter, IpcWriteOptions};
# use arrow_ipc::MetadataVersion;
# use arrow_schema::{ArrowError, Schema};
# fn main() -> Result<(), ArrowError> {
// The result we expect from an empty schema
let expected = vec![
    255, 255, 255, 255,  48,   0,   0,   0,
     16,   0,   0,   0,   0,   0,  10,   0,
     12,   0,  10,   0,   9,   0,   4,   0,
     10,   0,   0,   0,  16,   0,   0,   0,
      0,   1,   4,   0,   8,   0,   8,   0,
      0,   0,   4,   0,   8,   0,   0,   0,
      4,   0,   0,   0,   0,   0,   0,   0,
    255, 255, 255, 255,   0,   0,   0,   0
];

let schema = Schema::empty();
let buffer: Vec<u8> = Vec::new();
let options = IpcWriteOptions::try_new(8, false, MetadataVersion::V5)?;
let stream_writer = StreamWriter::try_new_with_options(buffer, &schema, options)?;

assert_eq!(stream_writer.into_inner()?, expected);
# Ok(())
# }
```

Unresolved upstream links (retained, not inferred): `Result::Err`.

<a id="op-094105a98440871df40f3ee9"></a>
## try_new

`function` · `arrow_ipc::writer::StreamWriter::try_new` · arrow-ipc 59.3.0

```rust
fn try_new(writer: W, schema: &Schema) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::StreamWriter", "path": "StreamWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2039, 1], "end": [2184, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:2047`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Try to create a new writer, with the schema written as part of the header.

Note that there is no internal buffering. See also [`StreamWriter::try_new_buffered`](../operations/arrow_ipc.writer.StreamWriter.md#op-8cb740091b4aeaae99dcf288).

# Errors

An ['Err'](Result::Err) may be returned if writing the header to the writer fails.

Unresolved upstream links (retained, not inferred): `Result::Err`.

<a id="op-8cb740091b4aeaae99dcf288"></a>
## try_new_buffered

`function` · `arrow_ipc::writer::StreamWriter::try_new_buffered` · arrow-ipc 59.3.0

```rust
fn try_new_buffered(writer: W, schema: &Schema) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "alloc::io::buffered::bufwriter::BufWriter", "path": "std::io::BufWriter"}}}], "constraints": []}}, "id": "arrow_ipc::writer::StreamWriter", "path": "StreamWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2030, 1], "end": [2037, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:2034`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Try to create a new stream writer with the writer wrapped in a BufWriter.

See [`StreamWriter::try_new`](../operations/arrow_ipc.writer.StreamWriter.md#op-094105a98440871df40f3ee9) for an unbuffered version.

<a id="op-5bbed701c9c05c790b5dff22"></a>
## try_new_with_options

`function` · `arrow_ipc::writer::StreamWriter::try_new_with_options` · arrow-ipc 59.3.0

```rust
fn try_new_with_options(writer: W, schema: &Schema, write_options: IpcWriteOptions) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::StreamWriter", "path": "StreamWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2039, 1], "end": [2184, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:2057`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Try to create a new writer with [`IpcWriteOptions`](../operations/arrow_ipc.writer.IpcWriteOptions.md#op-7958df777a3d4faa0bb68d5a).

# Errors

An ['Err'](Result::Err) may be returned if writing the header to the writer fails.

Unresolved upstream links (retained, not inferred): `Result::Err`.

<a id="op-171528eecf9f0129ed9da6cd"></a>
## write

`function` · `arrow_ipc::writer::StreamWriter::write` · arrow-ipc 59.3.0

```rust
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::StreamWriter", "path": "StreamWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2039, 1], "end": [2184, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:2085`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Write a record batch to the stream

<a id="op-7919f0cc098153cdcb130334"></a>
## write

`function` · `arrow_ipc::writer::StreamWriter::write` · arrow-ipc 59.3.0

```rust
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::StreamWriter", "path": "StreamWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2186, 1], "end": [2194, 2], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "arrow_array::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}, "trait_path": "arrow_array::record_batch::RecordBatchWriter"}`

Source: `src/writer.rs:2187`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
