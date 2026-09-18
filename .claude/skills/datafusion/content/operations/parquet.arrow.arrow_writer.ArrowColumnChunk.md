# `parquet::arrow::arrow_writer::ArrowColumnChunk`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_writer.ArrowColumnChunk.json).

<a id="op-efe7caf4e16e381ce78e4e21"></a>
## ArrowColumnChunk

`struct` · `parquet::arrow::arrow_writer::ArrowColumnChunk` · parquet 59.3.0

```rust
struct ArrowColumnChunk
```

Source: `src/arrow/arrow_writer/mod.rs:941`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The data for a single column chunk, see [`ArrowColumnWriter`](../operations/parquet.arrow.arrow_writer.ArrowColumnWriter.md#op-b2f79c40222ecdd56db8aa79)

<a id="op-feedab19ed6de3a9b7a45797"></a>
## append_to_row_group

`function` · `parquet::arrow::arrow_writer::ArrowColumnChunk::append_to_row_group` · parquet 59.3.0

```rust
fn append_to_row_group<W: Write + Send>(self, writer: &mut SerializedRowGroupWriter<'_, W>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowColumnChunk", "path": "ArrowColumnChunk"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [954, 1], "end": [991, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:977`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Splices this column's buffered pages into the row group, streaming them
back out of the [`PageStore`](../operations/parquet.column.page_store.PageStore.md#op-049980600cf52224c7af9631) one page at a time.

<a id="op-ca3843247f2066d97ced1f54"></a>
## close

`function` · `parquet::arrow::arrow_writer::ArrowColumnChunk::close` · parquet 59.3.0

```rust
fn close(&self) -> &ColumnCloseResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowColumnChunk", "path": "ArrowColumnChunk"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [954, 1], "end": [991, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:961`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the [`ColumnCloseResult`](../operations/parquet.column.writer.ColumnCloseResult.md#op-1f8b8cac3518e50577a9d9b2) produced when the chunk was closed.

Exposes encoding information, collected statistics, and the optional
[`ColumnIndexMetaData`](crate::file::page_index::column_index::ColumnIndexMetaData)
/ [`OffsetIndexMetaData`](crate::file::page_index::offset_index::OffsetIndexMetaData)
gathered for the column chunk.

<a id="op-c68954395da081b45f48f194"></a>
## close_mut

`function` · `parquet::arrow::arrow_writer::ArrowColumnChunk::close_mut` · parquet 59.3.0

```rust
fn close_mut(&mut self) -> &mut ColumnCloseResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowColumnChunk", "path": "ArrowColumnChunk"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [954, 1], "end": [991, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_writer/mod.rs:971`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a mutable reference to the [`ColumnCloseResult`](../operations/parquet.column.writer.ColumnCloseResult.md#op-1f8b8cac3518e50577a9d9b2).

This allows callers to mutate the close result before the chunk is
appended to a row group — for example, clearing `column_index` or
`bloom_filter` based on a dynamic rule that inspects the encodings and
collected page statistics.

<a id="op-b4156a91316ae29c5b0474ff"></a>
## fmt

`function` · `parquet::arrow::arrow_writer::ArrowColumnChunk::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_writer::ArrowColumnChunk", "path": "ArrowColumnChunk"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [946, 1], "end": [952, 2], "filename": "src/arrow/arrow_writer/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/arrow_writer/mod.rs:947`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
