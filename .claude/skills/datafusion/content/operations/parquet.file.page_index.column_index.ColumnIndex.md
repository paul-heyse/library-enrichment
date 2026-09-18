# `parquet::file::page_index::column_index::ColumnIndex`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.page_index.column_index.ColumnIndex.json).

<a id="op-f47f71bd772bd3a601b1b3f2"></a>
## ColumnIndex

`struct` · `parquet::file::page_index::column_index::ColumnIndex` · parquet 59.3.0

```rust
struct ColumnIndex
```

Source: `src/file/page_index/column_index.rs:40`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Common bits of the column index

<a id="op-c27733474b58c62702b08ca8"></a>
## clone

`function` · `parquet::file::page_index::column_index::ColumnIndex::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ColumnIndex
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndex", "path": "ColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 17], "end": [39, 22], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/page_index/column_index.rs:39`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-491c6d0146bd38e863abc171"></a>
## definition_level_histogram

`function` · `parquet::file::page_index::column_index::ColumnIndex::definition_level_histogram` · parquet 59.3.0

```rust
fn definition_level_histogram(&self, idx: usize) -> Option<&[i64]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndex", "path": "ColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [87, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:73`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the definition level histogram for the page indexed by `idx`

<a id="op-dfa381f704b3870c88fc4833"></a>
## eq

`function` · `parquet::file::page_index::column_index::ColumnIndex::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &ColumnIndex) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndex", "path": "ColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 24], "end": [39, 33], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/page_index/column_index.rs:39`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ecef7e9ab10a382addea698"></a>
## fmt

`function` · `parquet::file::page_index::column_index::ColumnIndex::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndex", "path": "ColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 10], "end": [39, 15], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/page_index/column_index.rs:39`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43d32a8c552edfe30b4e1f52"></a>
## is_null_page

`function` · `parquet::file::page_index::column_index::ColumnIndex::is_null_page` · parquet 59.3.0

```rust
fn is_null_page(&self, idx: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndex", "path": "ColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [87, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:84`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns whether the page indexed by `idx` consists of all null values

<a id="op-bb1d1b48195e9fb146ab78d8"></a>
## null_count

`function` · `parquet::file::page_index::column_index::ColumnIndex::null_count` · parquet 59.3.0

```rust
fn null_count(&self, idx: usize) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndex", "path": "ColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [87, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:57`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the number of null values in the page indexed by `idx`

Returns `None` if no null counts have been set in the index

<a id="op-4813651a5303522d28c978e9"></a>
## num_pages

`function` · `parquet::file::page_index::column_index::ColumnIndex::num_pages` · parquet 59.3.0

```rust
fn num_pages(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndex", "path": "ColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [87, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:50`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the number of pages

<a id="op-25714e63b7435443d09f8d22"></a>
## repetition_level_histogram

`function` · `parquet::file::page_index::column_index::ColumnIndex::repetition_level_histogram` · parquet 59.3.0

```rust
fn repetition_level_histogram(&self, idx: usize) -> Option<&[i64]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndex", "path": "ColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [87, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:62`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the repetition level histogram for the page indexed by `idx`
