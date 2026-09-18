# `parquet::record::reader::RowIter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.record.reader.RowIter.json).

<a id="op-b0386c6715ec7d27cee7594a"></a>
## RowIter

`struct` · `parquet::record::reader::RowIter` · parquet 59.3.0

```rust
struct RowIter<'a>
```

Source: `src/record/reader.rs:667`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Access parquet data as an iterator of [`Row`](../operations/parquet.record.api.Row.md#op-8446812bf7ce8e0f4f9bf863)

# Caveats

Parquet stores data in a columnar fashion using [Dremel] encoding, and is therefore highly
optimised for reading data by column, not row. As a consequence applications concerned with
performance should prefer the columnar arrow or [ColumnReader] APIs.

Additionally the current implementation does not correctly handle repeated fields ([#2394]),
and workloads looking to handle such schema should use the other APIs.

[#2394]: https://github.com/apache/arrow-rs/issues/2394
[ColumnReader]: crate::file::reader::RowGroupReader::get_column_reader
[Dremel]: https://research.google/pubs/pub36632/

<a id="op-e62acd09fbde88c36edd05e9"></a>
## Item

`assoc_type` · `parquet::record::reader::RowIter::Item` · parquet 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "parquet::record::reader::RowIter", "path": "RowIter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [784, 1], "end": [819, 2], "filename": "src/record/reader.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/record/reader.rs:785`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6bf1b330604999b9038e2c3"></a>
## from_file

`function` · `parquet::record::reader::RowIter::from_file` · parquet 59.3.0

```rust
fn from_file(proj: Option<Type>, reader: &'a dyn FileReader) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::record::reader::RowIter", "path": "RowIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [676, 1], "end": [782, 2], "filename": "src/record/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/reader.rs:701`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates iterator of [`Row`](../operations/parquet.record.api.Row.md#op-8446812bf7ce8e0f4f9bf863)s for all row groups in a
file.

<a id="op-29fc2d2010e8e5d79430f54c"></a>
## from_file_into

`function` · `parquet::record::reader::RowIter::from_file_into` · parquet 59.3.0

```rust
fn from_file_into(reader: Box<dyn FileReader>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::record::reader::RowIter", "path": "RowIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [676, 1], "end": [782, 2], "filename": "src/record/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/reader.rs:721`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a iterator of [`Row`](../operations/parquet.record.api.Row.md#op-8446812bf7ce8e0f4f9bf863)s from a [`FileReader`](../operations/parquet.file.reader.FileReader.md#op-9b66de5d3c7d389f75983bc1) using the full file schema.

<a id="op-7d5bd23f0c743cf6363e2dad"></a>
## from_row_group

`function` · `parquet::record::reader::RowIter::from_row_group` · parquet 59.3.0

```rust
fn from_row_group(proj: Option<Type>, reader: &'a dyn RowGroupReader) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::record::reader::RowIter", "path": "RowIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [676, 1], "end": [782, 2], "filename": "src/record/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/reader.rs:710`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates iterator of [`Row`](../operations/parquet.record.api.Row.md#op-8446812bf7ce8e0f4f9bf863)s for a specific row group.

<a id="op-21a2c1cf0d749d5b0f6cb4eb"></a>
## next

`function` · `parquet::record::reader::RowIter::next` · parquet 59.3.0

```rust
fn next(&mut self) -> Option<Result<Row>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "parquet::record::reader::RowIter", "path": "RowIter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [784, 1], "end": [819, 2], "filename": "src/record/reader.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/record/reader.rs:787`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8539eded45a762d7d837516a"></a>
## project

`function` · `parquet::record::reader::RowIter::project` · parquet 59.3.0

```rust
fn project(self, proj: Option<Type>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::record::reader::RowIter", "path": "RowIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [676, 1], "end": [782, 2], "filename": "src/record/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/reader.rs:737`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Tries to create a iterator of [`Row`](../operations/parquet.record.api.Row.md#op-8446812bf7ce8e0f4f9bf863)s using projections.
Returns a error if a file reader is not the source of this iterator.

The Projected schema can be a subset of or equal to the file schema,
when it is None, full file schema is assumed.

<a id="op-6f7c0face5d9dbfcf5cc0360"></a>
## with_batch_size

`function` · `parquet::record::reader::RowIter::with_batch_size` · parquet 59.3.0

```rust
fn with_batch_size(self, batch_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::record::reader::RowIter", "path": "RowIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [676, 1], "end": [782, 2], "filename": "src/record/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/reader.rs:771`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets batch size for this row iter.
