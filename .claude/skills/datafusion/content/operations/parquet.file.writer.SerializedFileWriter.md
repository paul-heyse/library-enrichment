# `parquet::file::writer::SerializedFileWriter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.writer.SerializedFileWriter.json).

<a id="op-0b9cc37c23222f2e8aef5d0f"></a>
## SerializedFileWriter

`struct` · `parquet::file::writer::SerializedFileWriter` · parquet 59.3.0

```rust
struct SerializedFileWriter<W: Write>
```

Source: `src/file/writer.rs:158`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parquet file writer API.

This is a low level API for writing Parquet files directly, and handles
tracking the location of file structures such as row groups and column
chunks, and writing the metadata and file footer.

Data is written to row groups using  [`SerializedRowGroupWriter`](../operations/parquet.file.writer.SerializedRowGroupWriter.md#op-5cef0074657f4dfb224c1d39) and
columns using [`SerializedColumnWriter`](../operations/parquet.file.writer.SerializedColumnWriter.md#op-9dfcc6fa1c68565d8942c8b8). The `SerializedFileWriter` tracks
where all the data is written, and assembles the final file metadata.

The main workflow should be as following:
- Create file writer, this will open a new file and potentially write some metadata.
- Request a new row group writer by calling `next_row_group`.
- Once finished writing row group, close row group writer by calling `close`
- Write subsequent row groups, if necessary.
- After all row groups have been written, close the file writer using `close` method.

<a id="op-8e7f27d59566af63b32d2fc8"></a>
## append_key_value_metadata

`function` · `parquet::file::writer::SerializedFileWriter::append_key_value_metadata` · parquet 59.3.0

```rust
fn append_key_value_metadata(&mut self, kv_metadata: KeyValue)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedFileWriter", "path": "SerializedFileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [462, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:390`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Add a [`KeyValue`](../operations/parquet.file.metadata.KeyValue.md#op-c5f599a409980a2c03049085) to the file writer's metadata

<a id="op-48e0faa3cbfdd26a908d4d9c"></a>
## bytes_written

`function` · `parquet::file::writer::SerializedFileWriter::bytes_written` · parquet 59.3.0

```rust
fn bytes_written(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedFileWriter", "path": "SerializedFileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [462, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:453`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the number of bytes written to this instance

<a id="op-c5d79851e9d826c16053ce51"></a>
## close

`function` · `parquet::file::writer::SerializedFileWriter::close` · parquet 59.3.0

```rust
fn close(self) -> Result<ParquetMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedFileWriter", "path": "SerializedFileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [462, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:309`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Closes and finalises file writer, returning the file metadata.

<a id="op-d1916a5937e8b477439cc864"></a>
## finish

`function` · `parquet::file::writer::SerializedFileWriter::finish` · parquet 59.3.0

```rust
fn finish(&mut self) -> Result<ParquetMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedFileWriter", "path": "SerializedFileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [462, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:301`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Close and finalize the underlying Parquet writer

Unlike [`Self::close`](../operations/parquet.file.writer.SerializedFileWriter.md#op-c5d79851e9d826c16053ce51) this does not consume self

Attempting to write after calling finish will result in an error

<a id="op-9ada7c0e539df92d6c0ac584"></a>
## flush

`function` · `parquet::file::writer::SerializedFileWriter::flush` · parquet 59.3.0

```rust
fn flush(&mut self) -> std::io::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedFileWriter", "path": "SerializedFileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [462, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:428`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Flushes underlying writer

<a id="op-8f3cdf6d53c42186ced07958"></a>
## flushed_row_groups

`function` · `parquet::file::writer::SerializedFileWriter::flushed_row_groups` · parquet 59.3.0

```rust
fn flushed_row_groups(&self) -> &[RowGroupMetaData]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedFileWriter", "path": "SerializedFileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [462, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:292`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns metadata for any flushed row groups

<a id="op-ae1e12dfb8df5ee81e876ccf"></a>
## fmt

`function` · `parquet::file::writer::SerializedFileWriter::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedFileWriter", "path": "SerializedFileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [184, 2], "filename": "src/file/writer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/writer.rs:175`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f5aaf8b3857f1c9e70416b8"></a>
## inner

`function` · `parquet::file::writer::SerializedFileWriter::inner` · parquet 59.3.0

```rust
fn inner(&self) -> &W
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedFileWriter", "path": "SerializedFileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [462, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:411`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a reference to the underlying writer.

<a id="op-ef642587e86978a8f5237e88"></a>
## inner_mut

`function` · `parquet::file::writer::SerializedFileWriter::inner_mut` · parquet 59.3.0

```rust
fn inner_mut(&mut self) -> &mut W
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedFileWriter", "path": "SerializedFileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [462, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:440`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a mutable reference to the underlying writer.

**Warning**: if you write directly to this writer, you will skip
the `TrackedWrite` buffering and byte‐counting layers, which can cause
the file footer’s recorded offsets and sizes to diverge from reality,
resulting in an unreadable or corrupted Parquet file.

If you want to write safely to the underlying writer, use [`Self::write_all`](../operations/parquet.file.writer.SerializedFileWriter.md#op-158c5941d316f6c529503275).

<a id="op-49810a16b7cee24843427deb"></a>
## into_inner

`function` · `parquet::file::writer::SerializedFileWriter::into_inner` · parquet 59.3.0

```rust
fn into_inner(self) -> Result<W>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedFileWriter", "path": "SerializedFileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [462, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:445`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Writes the file footer and returns the underlying writer.

<a id="op-ccb28e034e219ca05d6279fc"></a>
## new

`function` · `parquet::file::writer::SerializedFileWriter::new` · parquet 59.3.0

```rust
fn new(buf: W, schema: TypePtr, properties: WriterPropertiesPtr) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedFileWriter", "path": "SerializedFileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [462, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:188`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new file writer.

<a id="op-f49e6cb8b226384a3cf4f7bc"></a>
## next_row_group

`function` · `parquet::file::writer::SerializedFileWriter::next_row_group` · parquet 59.3.0

```rust
fn next_row_group(&mut self) -> Result<SerializedRowGroupWriter<'_, W>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedFileWriter", "path": "SerializedFileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [462, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:237`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new row group from this file writer.

Note: Parquet files are limited to at most 2^15 row groups in a file; and row groups must
be written sequentially.

Every time the next row group is requested, the previous row group must
be finalised and closed using the [`SerializedRowGroupWriter::close`](../operations/parquet.file.writer.SerializedRowGroupWriter.md#op-f0ccaef92234231a7ab199f0)
method or an error will be returned.

<a id="op-28cf35ad1d59f265528dad7d"></a>
## properties

`function` · `parquet::file::writer::SerializedFileWriter::properties` · parquet 59.3.0

```rust
fn properties(&self) -> &WriterPropertiesPtr
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedFileWriter", "path": "SerializedFileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [462, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:406`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a reference to the writer properties

<a id="op-22e98fc70d9464ce13b4825f"></a>
## schema_descr

`function` · `parquet::file::writer::SerializedFileWriter::schema_descr` · parquet 59.3.0

```rust
fn schema_descr(&self) -> &SchemaDescriptor
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedFileWriter", "path": "SerializedFileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [462, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:395`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a reference to schema descriptor.

<a id="op-158c5941d316f6c529503275"></a>
## write_all

`function` · `parquet::file::writer::SerializedFileWriter::write_all` · parquet 59.3.0

```rust
fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedFileWriter", "path": "SerializedFileWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [462, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:423`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Writes the given buf bytes to the internal buffer.

This can be used to write raw data to an in-progress Parquet file, for
example, custom index structures or other payloads. Other Parquet readers
will skip this data when reading the files.

It's safe to use this method to write data to the underlying writer,
because it will ensure that the buffering and byte‐counting layers are used.
