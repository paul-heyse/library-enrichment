# `arrow_ipc::writer::FileWriter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.writer.FileWriter.json).

<a id="op-de36feaee6f6c9a3aec5da12"></a>
## FileWriter

`struct` · `arrow_ipc::writer::FileWriter` · arrow-ipc 59.3.0

```rust
struct FileWriter<W>
```

Source: `src/writer.rs:1593`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Arrow File Writer

Writes Arrow [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es in the [IPC File Format].

# See Also

* [`StreamWriter`](../operations/arrow_ipc.writer.StreamWriter.md#op-b808ea0aac7879e19d25b939) for writing IPC Streams

# Example
```
# use arrow_array::record_batch;
# use arrow_ipc::writer::FileWriter;
# let mut file = vec![]; // mimic a file for the example
let batch = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
// create a new writer, the schema must be known in advance
let mut writer = FileWriter::try_new(&mut file, &batch.schema()).unwrap();
// write each batch to the underlying writer
writer.write(&batch).unwrap();
// When all batches are written, call finish to flush all buffers
writer.finish().unwrap();
```
[IPC File Format]: https://arrow.apache.org/docs/format/Columnar.html#ipc-file-format

<a id="op-9f1d4d6427aa700efa8076a1"></a>
## close

`function` · `arrow_ipc::writer::FileWriter::close` · arrow-ipc 59.3.0

```rust
fn close(self) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::FileWriter", "path": "FileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1815, 1], "end": [1823, 2], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "arrow_array::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}, "trait_path": "arrow_array::record_batch::RecordBatchWriter"}`

Source: `src/writer.rs:1820`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-597b4e16495eb47582f02c1d"></a>
## finish

`function` · `arrow_ipc::writer::FileWriter::finish` · arrow-ipc 59.3.0

```rust
fn finish(&mut self) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::FileWriter", "path": "FileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1627, 1], "end": [1813, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1726`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Write footer and closing tag, then mark the writer as done

<a id="op-48451201c8020492c6be7d54"></a>
## flush

`function` · `arrow_ipc::writer::FileWriter::flush` · arrow-ipc 59.3.0

```rust
fn flush(&mut self) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::FileWriter", "path": "FileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1627, 1], "end": [1813, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1793`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Flush the underlying writer.

Both the BufWriter and the underlying writer are flushed.

<a id="op-cce46bb06d288480dd2fae98"></a>
## get_mut

`function` · `arrow_ipc::writer::FileWriter::get_mut` · arrow-ipc 59.3.0

```rust
fn get_mut(&mut self) -> &mut W
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::FileWriter", "path": "FileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1627, 1], "end": [1813, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1786`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Gets a mutable reference to the underlying writer.

It is inadvisable to directly write to the underlying writer.

<a id="op-3bf97064a8b41adba7db8732"></a>
## get_ref

`function` · `arrow_ipc::writer::FileWriter::get_ref` · arrow-ipc 59.3.0

```rust
fn get_ref(&self) -> &W
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::FileWriter", "path": "FileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1627, 1], "end": [1813, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1779`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Gets a reference to the underlying writer.

<a id="op-ebd2014f47336476a2bf1a0a"></a>
## into_inner

`function` · `arrow_ipc::writer::FileWriter::into_inner` · arrow-ipc 59.3.0

```rust
fn into_inner(self) -> Result<W, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::FileWriter", "path": "FileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1627, 1], "end": [1813, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1806`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Unwraps the underlying writer.

The writer is flushed and the FileWriter is finished before returning.

# Errors

An ['Err'](Result::Err) may be returned if an error occurs while finishing the StreamWriter
or while flushing the writer.

Unresolved upstream links (retained, not inferred): `Result::Err`.

<a id="op-3eb4206071c99c9afa09b47e"></a>
## schema

`function` · `arrow_ipc::writer::FileWriter::schema` · arrow-ipc 59.3.0

```rust
fn schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::FileWriter", "path": "FileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1627, 1], "end": [1813, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1774`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Returns the arrow [`SchemaRef`](../operations/arrow_schema.schema.SchemaRef.md#op-e48a1bb89d62307cfab4093a) for this arrow file.

<a id="op-cc607eacc4093edad57fbc22"></a>
## try_new

`function` · `arrow_ipc::writer::FileWriter::try_new` · arrow-ipc 59.3.0

```rust
fn try_new(writer: W, schema: &Schema) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::FileWriter", "path": "FileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1627, 1], "end": [1813, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1635`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Try to create a new writer, with the schema written as part of the header

Note the created writer is not buffered. See [`FileWriter::try_new_buffered`](../operations/arrow_ipc.writer.FileWriter.md#op-77820dc8b4c0a8610e5c1d08) for details.

# Errors

An ['Err'](Result::Err) may be returned if writing the header to the writer fails.

Unresolved upstream links (retained, not inferred): `Result::Err`.

<a id="op-77820dc8b4c0a8610e5c1d08"></a>
## try_new_buffered

`function` · `arrow_ipc::writer::FileWriter::try_new_buffered` · arrow-ipc 59.3.0

```rust
fn try_new_buffered(writer: W, schema: &Schema) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "alloc::io::buffered::bufwriter::BufWriter", "path": "std::io::BufWriter"}}}], "constraints": []}}, "id": "arrow_ipc::writer::FileWriter", "path": "FileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1618, 1], "end": [1625, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1622`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Try to create a new file writer with the writer wrapped in a BufWriter.

See [`FileWriter::try_new`](../operations/arrow_ipc.writer.FileWriter.md#op-cc607eacc4093edad57fbc22) for an unbuffered version.

<a id="op-a46c07bb585e81b387e788dd"></a>
## try_new_with_options

`function` · `arrow_ipc::writer::FileWriter::try_new_with_options` · arrow-ipc 59.3.0

```rust
fn try_new_with_options(writer: W, schema: &Schema, write_options: IpcWriteOptions) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::FileWriter", "path": "FileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1627, 1], "end": [1813, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1647`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Try to create a new writer with IpcWriteOptions

Note the created writer is not buffered. See [`FileWriter::try_new_buffered`](../operations/arrow_ipc.writer.FileWriter.md#op-77820dc8b4c0a8610e5c1d08) for details.

# Errors

An ['Err'](Result::Err) may be returned if writing the header to the writer fails.

Unresolved upstream links (retained, not inferred): `Result::Err`.

<a id="op-3b67f978fb50ade08aecb74f"></a>
## write

`function` · `arrow_ipc::writer::FileWriter::write` · arrow-ipc 59.3.0

```rust
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::FileWriter", "path": "FileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1627, 1], "end": [1813, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1689`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Write a record batch to the file

<a id="op-e24542c79441b94465683c43"></a>
## write

`function` · `arrow_ipc::writer::FileWriter::write` · arrow-ipc 59.3.0

```rust
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::FileWriter", "path": "FileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1815, 1], "end": [1823, 2], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "arrow_array::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}, "trait_path": "arrow_array::record_batch::RecordBatchWriter"}`

Source: `src/writer.rs:1816`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8b7d8a22f7ae238c2a5311f"></a>
## write_metadata

`function` · `arrow_ipc::writer::FileWriter::write_metadata` · arrow-ipc 59.3.0

```rust
fn write_metadata(&mut self, key: impl Into<String>, value: impl Into<String>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "arrow_ipc::writer::FileWriter", "path": "FileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1627, 1], "end": [1813, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:1684`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Adds a key-value pair to the [FileWriter](../operations/arrow_ipc.writer.FileWriter.md#op-de36feaee6f6c9a3aec5da12)'s custom metadata
