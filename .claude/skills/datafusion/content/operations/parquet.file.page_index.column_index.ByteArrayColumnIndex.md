# `parquet::file::page_index::column_index::ByteArrayColumnIndex`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.page_index.column_index.ByteArrayColumnIndex.json).

<a id="op-7cdb18ce07cc3255bf5fe730"></a>
## ByteArrayColumnIndex

`struct` · `parquet::file::page_index::column_index::ByteArrayColumnIndex` · parquet 59.3.0

```rust
struct ByteArrayColumnIndex
```

Source: `src/file/page_index/column_index.rs:297`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Column index for byte arrays (fixed length and variable)

<a id="op-325376c593a58fa15e51a305"></a>
## Target

`assoc_type` · `parquet::file::page_index::column_index::ByteArrayColumnIndex::Target` · parquet 59.3.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ByteArrayColumnIndex", "path": "ByteArrayColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 1], "end": [455, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/file/page_index/column_index.rs:450`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bec36c6bfb09867304cef3ed"></a>
## clone

`function` · `parquet::file::page_index::column_index::ByteArrayColumnIndex::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ByteArrayColumnIndex
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ByteArrayColumnIndex", "path": "ByteArrayColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 17], "end": [296, 22], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/page_index/column_index.rs:296`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45d0367b85cc960e573410a8"></a>
## deref

`function` · `parquet::file::page_index::column_index::ByteArrayColumnIndex::deref` · parquet 59.3.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ByteArrayColumnIndex", "path": "ByteArrayColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 1], "end": [455, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/file/page_index/column_index.rs:452`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ed1da2ca1fb422a2baabdc3"></a>
## eq

`function` · `parquet::file::page_index::column_index::ByteArrayColumnIndex::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &ByteArrayColumnIndex) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ByteArrayColumnIndex", "path": "ByteArrayColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 24], "end": [296, 33], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/page_index/column_index.rs:296`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0473755d7be175408eb9a1f"></a>
## fmt

`function` · `parquet::file::page_index::column_index::ByteArrayColumnIndex::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ByteArrayColumnIndex", "path": "ByteArrayColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 10], "end": [296, 15], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/page_index/column_index.rs:296`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf5e3300588f2e58f28aca09"></a>
## max_value

`function` · `parquet::file::page_index::column_index::ByteArrayColumnIndex::max_value` · parquet 59.3.0

```rust
fn max_value(&self, idx: usize) -> Option<&[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ByteArrayColumnIndex", "path": "ByteArrayColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 1], "end": [447, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:424`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the max value for the page indexed by `idx`

It is `None` when all values are null

<a id="op-1a53af41643f2046640b8df4"></a>
## max_values_iter

`function` · `parquet::file::page_index::column_index::ByteArrayColumnIndex::max_values_iter` · parquet 59.3.0

```rust
fn max_values_iter(&self) -> impl Iterator<Item = Option<&[u8]>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ByteArrayColumnIndex", "path": "ByteArrayColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 1], "end": [447, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:444`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns an iterator over the max values.

Values may be `None` when [`ColumnIndex::is_null_page()`](../operations/parquet.file.page_index.column_index.ColumnIndex.md#op-43d32a8c552edfe30b4e1f52) is `true`.

<a id="op-6ea330cfab81b90f99edc565"></a>
## min_value

`function` · `parquet::file::page_index::column_index::ByteArrayColumnIndex::min_value` · parquet 59.3.0

```rust
fn min_value(&self, idx: usize) -> Option<&[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ByteArrayColumnIndex", "path": "ByteArrayColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 1], "end": [447, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:411`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the min value for the page indexed by `idx`

It is `None` when all values are null

<a id="op-8c921e3c3d7ded95f5aaa65f"></a>
## min_values_iter

`function` · `parquet::file::page_index::column_index::ByteArrayColumnIndex::min_values_iter` · parquet 59.3.0

```rust
fn min_values_iter(&self) -> impl Iterator<Item = Option<&[u8]>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ByteArrayColumnIndex", "path": "ByteArrayColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 1], "end": [447, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:437`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns an iterator over the min values.

Values may be `None` when [`ColumnIndex::is_null_page()`](../operations/parquet.file.page_index.column_index.ColumnIndex.md#op-43d32a8c552edfe30b4e1f52) is `true`.
