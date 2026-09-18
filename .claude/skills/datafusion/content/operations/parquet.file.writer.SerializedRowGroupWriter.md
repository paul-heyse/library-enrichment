# `parquet::file::writer::SerializedRowGroupWriter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.writer.SerializedRowGroupWriter.json).

<a id="op-5cef0074657f4dfb224c1d39"></a>
## SerializedRowGroupWriter

`struct` · `parquet::file::writer::SerializedRowGroupWriter` · parquet 59.3.0

```rust
struct SerializedRowGroupWriter<'a, W: Write>
```

Source: `src/file/writer.rs:515`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parquet row group writer API.

Provides methods to access column writers in an iterator-like fashion, order is
guaranteed to match the order of schema leaves (column descriptors).

All columns should be written sequentially; the main workflow is:
- Request the next column using `next_column` method - this will return `None` if no
  more columns are available to write.
- Once done writing a column, close column writer with `close`
- Once all columns have been written, close row group writer with `close`
  method. The close method will return row group metadata and is no-op
  on already closed row group.

<a id="op-7408154d8e7f89424877aa43"></a>
## append_column

`function` · `parquet::file::writer::SerializedRowGroupWriter::append_column` · parquet 59.3.0

```rust
fn append_column<R: ChunkReader>(&mut self, reader: &R, close: ColumnCloseResult) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedRowGroupWriter", "path": "SerializedRowGroupWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [535, 1], "end": [960, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:686`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Append an encoded column chunk from `reader` directly to the underlying
writer.

This method can be used for efficiently concatenating or projecting
Parquet data, or encoding Parquet data to temporary in-memory buffers.

Arguments:
- `reader`: a [`ChunkReader`](../operations/parquet.file.reader.ChunkReader.md#op-8eae425b3b94361396eb9d94) containing the encoded column data
- `close`: the [`ColumnCloseResult`](../operations/parquet.column.writer.ColumnCloseResult.md#op-1f8b8cac3518e50577a9d9b2) metadata returned from closing
  the column writer that wrote the data in `reader`.

See Also:
1. [`get_column_writer`](../operations/parquet.column.writer.get_column_writer.md#op-e0f41296e3e94d003a567aec)  for creating writers that can encode data.
2. [`Self::next_column`](../operations/parquet.file.writer.SerializedRowGroupWriter.md#op-490035e9e13384b58116961f) for writing data that isn't already encoded

<a id="op-f0ccaef92234231a7ab199f0"></a>
## close

`function` · `parquet::file::writer::SerializedRowGroupWriter::close` · parquet 59.3.0

```rust
fn close(self) -> Result<RowGroupMetaDataPtr>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedRowGroupWriter", "path": "SerializedRowGroupWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [535, 1], "end": [960, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:852`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Closes this row group writer and returns row group metadata.

<a id="op-1f4193b7b471923aee07a5d0"></a>
## new

`function` · `parquet::file::writer::SerializedRowGroupWriter::new` · parquet 59.3.0

```rust
fn new(schema_descr: SchemaDescPtr, properties: WriterPropertiesPtr, buf: &'a mut TrackedWrite<W>, row_group_index: i16, on_close: Option<OnCloseRowGroup<'a, W>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedRowGroupWriter", "path": "SerializedRowGroupWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [535, 1], "end": [960, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:544`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a new `SerializedRowGroupWriter` with:

- `schema_descr` - the schema to write
- `properties` - writer properties
- `buf` - the buffer to write data to
- `row_group_index` - row group index in this parquet file.
- `file_offset` - file offset of this row group in this parquet file.
- `on_close` - an optional callback that will invoked on [`Self::close`](../operations/parquet.file.writer.SerializedRowGroupWriter.md#op-f0ccaef92234231a7ab199f0)

<a id="op-490035e9e13384b58116961f"></a>
## next_column

`function` · `parquet::file::writer::SerializedRowGroupWriter::next_column` · parquet 59.3.0

```rust
fn next_column(&mut self) -> Result<Option<SerializedColumnWriter<'_>>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedRowGroupWriter", "path": "SerializedRowGroupWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [535, 1], "end": [960, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:665`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the next column writer, if available; otherwise returns `None`.
In case of any IO error or Thrift error, or if row group writer has already been
closed returns `Err`.
