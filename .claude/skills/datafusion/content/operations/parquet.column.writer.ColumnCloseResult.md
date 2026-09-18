# `parquet::column::writer::ColumnCloseResult`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.writer.ColumnCloseResult.json).

<a id="op-1f8b8cac3518e50577a9d9b2"></a>
## ColumnCloseResult

`struct` · `parquet::column::writer::ColumnCloseResult` · parquet 59.3.0

```rust
struct ColumnCloseResult
```

Source: `src/column/writer/mod.rs:197`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Metadata for a column chunk of a Parquet file.

Note this structure is returned by [`ColumnWriter::close`](../operations/parquet.column.writer.ColumnWriter.md#op-2ee4edafe71da5d822f26275).

<a id="op-501d5379a429ddf838a2e7c4"></a>
## bloom_filter

`struct_field` · `parquet::column::writer::ColumnCloseResult::bloom_filter` · parquet 59.3.0

```rust
bloom_filter: Option<bloom_filter::Sbbf>
```

Source: `src/column/writer/mod.rs:205`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Optional bloom filter for this column

<a id="op-6e34c4e863b43818bb179ced"></a>
## bytes_written

`struct_field` · `parquet::column::writer::ColumnCloseResult::bytes_written` · parquet 59.3.0

```rust
bytes_written: u64
```

Source: `src/column/writer/mod.rs:199`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The total number of bytes written

<a id="op-bdc5247b733850409863f6b1"></a>
## clone

`function` · `parquet::column::writer::ColumnCloseResult::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ColumnCloseResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::writer::ColumnCloseResult", "path": "ColumnCloseResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 17], "end": [196, 22], "filename": "src/column/writer/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/column/writer/mod.rs:196`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7ecb7cf1f1a643d1c41f0e9"></a>
## column_index

`struct_field` · `parquet::column::writer::ColumnCloseResult::column_index` · parquet 59.3.0

```rust
column_index: Option<file::page_index::column_index::ColumnIndexMetaData>
```

Source: `src/column/writer/mod.rs:207`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Optional column index, for filtering

<a id="op-ece69ce1ac61d9047905febe"></a>
## fmt

`function` · `parquet::column::writer::ColumnCloseResult::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::writer::ColumnCloseResult", "path": "ColumnCloseResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 10], "end": [196, 15], "filename": "src/column/writer/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/column/writer/mod.rs:196`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-768cf5a3d73bb1f0cd1699a2"></a>
## metadata

`struct_field` · `parquet::column::writer::ColumnCloseResult::metadata` · parquet 59.3.0

```rust
metadata: file::metadata::ColumnChunkMetaData
```

Source: `src/column/writer/mod.rs:203`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Metadata for this column chunk

<a id="op-dfc422fee155a6ab4055a030"></a>
## offset_index

`struct_field` · `parquet::column::writer::ColumnCloseResult::offset_index` · parquet 59.3.0

```rust
offset_index: Option<file::page_index::offset_index::OffsetIndexMetaData>
```

Source: `src/column/writer/mod.rs:209`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Optional offset index, identifying page locations

<a id="op-f286dd2e01e27558d92032c1"></a>
## rows_written

`struct_field` · `parquet::column::writer::ColumnCloseResult::rows_written` · parquet 59.3.0

```rust
rows_written: u64
```

Source: `src/column/writer/mod.rs:201`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The total number of rows written

<a id="op-af9a5a9ba057420e46e25915"></a>
## update_dictionary_location

`function` · `parquet::column::writer::ColumnCloseResult::update_dictionary_location` · parquet 59.3.0

```rust
fn update_dictionary_location(self, dictionary_len: usize) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::writer::ColumnCloseResult", "path": "ColumnCloseResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [212, 1], "end": [241, 2], "filename": "src/column/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/writer/mod.rs:223`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Rewrite the page offsets for a dictionary-first on-disk layout.

A writer that buffers the whole column chunk and splices it later (the
Arrow path) may accept the data pages *before* the dictionary page so the
data pages can stream straight through, then emit the dictionary page
first at splice. The offsets recorded during encoding therefore assume a
data-pages-first layout; call this with the serialized length of the
dictionary page to move it to offset 0 and shift every data page after
it. A `dictionary_len` of 0 (no dictionary page) leaves the result
unchanged.
