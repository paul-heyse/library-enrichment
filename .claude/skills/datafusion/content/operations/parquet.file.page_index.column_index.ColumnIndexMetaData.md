# `parquet::file::page_index::column_index::ColumnIndexMetaData`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.page_index.column_index.ColumnIndexMetaData.json).

<a id="op-6700acf40da7fd4eab386f3e"></a>
## ColumnIndexMetaData

`enum` · `parquet::file::page_index::column_index::ColumnIndexMetaData` · parquet 59.3.0

```rust
enum ColumnIndexMetaData
```

Source: `src/file/page_index/column_index.rs:540`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parsed [`ColumnIndex`] information for a Parquet file.

See [`ParquetColumnIndex`] for more information.

[`ParquetColumnIndex`]: crate::file::metadata::ParquetColumnIndex
[`ColumnIndex`]: https://github.com/apache/parquet-format/blob/master/PageIndex.md

<a id="op-3f69d5f6dc52556743608744"></a>
## BOOLEAN

`variant` · `parquet::file::page_index::column_index::ColumnIndexMetaData::BOOLEAN` · parquet 59.3.0

```rust
BOOLEAN
```

Source: `src/file/page_index/column_index.rs:546`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Boolean type index

<a id="op-8758fa2d3103d7749e210d63"></a>
## BYTE_ARRAY

`variant` · `parquet::file::page_index::column_index::ColumnIndexMetaData::BYTE_ARRAY` · parquet 59.3.0

```rust
BYTE_ARRAY
```

Source: `src/file/page_index/column_index.rs:558`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Byte array type index

<a id="op-dd666c0836997af96ccb6f27"></a>
## DOUBLE

`variant` · `parquet::file::page_index::column_index::ColumnIndexMetaData::DOUBLE` · parquet 59.3.0

```rust
DOUBLE
```

Source: `src/file/page_index/column_index.rs:556`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

64-bit floating point type index

<a id="op-76272bfa17ce71539de18a0b"></a>
## FIXED_LEN_BYTE_ARRAY

`variant` · `parquet::file::page_index::column_index::ColumnIndexMetaData::FIXED_LEN_BYTE_ARRAY` · parquet 59.3.0

```rust
FIXED_LEN_BYTE_ARRAY
```

Source: `src/file/page_index/column_index.rs:560`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Fixed length byte array type index

<a id="op-ef6e43f5a337d0c78ad45cef"></a>
## FLOAT

`variant` · `parquet::file::page_index::column_index::ColumnIndexMetaData::FLOAT` · parquet 59.3.0

```rust
FLOAT
```

Source: `src/file/page_index/column_index.rs:554`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

32-bit floating point type index

<a id="op-73d28ba2e21683719d53fbdf"></a>
## INT32

`variant` · `parquet::file::page_index::column_index::ColumnIndexMetaData::INT32` · parquet 59.3.0

```rust
INT32
```

Source: `src/file/page_index/column_index.rs:548`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

32-bit integer type index

<a id="op-bd10276e4bad804f9ee04c68"></a>
## INT64

`variant` · `parquet::file::page_index::column_index::ColumnIndexMetaData::INT64` · parquet 59.3.0

```rust
INT64
```

Source: `src/file/page_index/column_index.rs:550`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

64-bit integer type index

<a id="op-26b722e984f0f91b386838af"></a>
## INT96

`variant` · `parquet::file::page_index::column_index::ColumnIndexMetaData::INT96` · parquet 59.3.0

```rust
INT96
```

Source: `src/file/page_index/column_index.rs:552`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

96-bit integer type (timestamp) index

<a id="op-8fa7521b5cc4bdd0dc2b005c"></a>
## NONE

`variant` · `parquet::file::page_index::column_index::ColumnIndexMetaData::NONE` · parquet 59.3.0

```rust
NONE
```

Source: `src/file/page_index/column_index.rs:544`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sometimes reading page index from parquet file
will only return pageLocations without min_max index,
`NONE` represents this lack of index information

<a id="op-a560d634e11a57964066ac52"></a>
## clone

`function` · `parquet::file::page_index::column_index::ColumnIndexMetaData::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ColumnIndexMetaData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexMetaData", "path": "ColumnIndexMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 17], "end": [538, 22], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/page_index/column_index.rs:538`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2ec6d6e6d4b10f34cf2923f"></a>
## definition_level_histogram

`function` · `parquet::file::page_index::column_index::ColumnIndexMetaData::definition_level_histogram` · parquet 59.3.0

```rust
fn definition_level_histogram(&self, idx: usize) -> Option<&[i64]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexMetaData", "path": "ColumnIndexMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [563, 1], "end": [633, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:624`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the definition level histogram for the page indexed by `idx`

<a id="op-7657ce37acb864bb98a1cfb8"></a>
## eq

`function` · `parquet::file::page_index::column_index::ColumnIndexMetaData::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &ColumnIndexMetaData) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexMetaData", "path": "ColumnIndexMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 24], "end": [538, 33], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/page_index/column_index.rs:538`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71470ea53cd2c0c62c88fe4b"></a>
## fmt

`function` · `parquet::file::page_index::column_index::ColumnIndexMetaData::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexMetaData", "path": "ColumnIndexMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 10], "end": [538, 15], "filename": "src/file/page_index/column_index.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/page_index/column_index.rs:538`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c0ecbe9b5082223934976d8"></a>
## get_boundary_order

`function` · `parquet::file::page_index::column_index::ColumnIndexMetaData::get_boundary_order` · parquet 59.3.0

```rust
fn get_boundary_order(&self) -> Option<BoundaryOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexMetaData", "path": "ColumnIndexMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [563, 1], "end": [633, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:575`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get boundary_order of this page index.

<a id="op-1fc35843ae56b77de9b53cd4"></a>
## is_null_page

`function` · `parquet::file::page_index::column_index::ColumnIndexMetaData::is_null_page` · parquet 59.3.0

```rust
fn is_null_page(&self, idx: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexMetaData", "path": "ColumnIndexMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [563, 1], "end": [633, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:630`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns whether the page indexed by `idx` consists of all null values

<a id="op-8f4e98eb633413ec205074ff"></a>
## is_sorted

`function` · `parquet::file::page_index::column_index::ColumnIndexMetaData::is_sorted` · parquet 59.3.0

```rust
fn is_sorted(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexMetaData", "path": "ColumnIndexMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [563, 1], "end": [633, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:565`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return min/max elements inside ColumnIndex are ordered or not.

<a id="op-4f78d5266d64c106cfb839f3"></a>
## null_count

`function` · `parquet::file::page_index::column_index::ColumnIndexMetaData::null_count` · parquet 59.3.0

```rust
fn null_count(&self, idx: usize) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexMetaData", "path": "ColumnIndexMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [563, 1], "end": [633, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:614`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the number of null values in the page indexed by `idx`

Returns `None` if no null counts have been set in the index

<a id="op-57dae384f838f3b46c3aab98"></a>
## null_counts

`function` · `parquet::file::page_index::column_index::ColumnIndexMetaData::null_counts` · parquet 59.3.0

```rust
fn null_counts(&self) -> Option<&Vec<i64>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexMetaData", "path": "ColumnIndexMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [563, 1], "end": [633, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:592`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns array of null counts, one per page.

Returns `None` if now null counts have been set in the index

<a id="op-ac4cde0220dc424b45161a98"></a>
## num_pages

`function` · `parquet::file::page_index::column_index::ColumnIndexMetaData::num_pages` · parquet 59.3.0

```rust
fn num_pages(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexMetaData", "path": "ColumnIndexMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [563, 1], "end": [633, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:607`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the number of pages

<a id="op-24bbb8ff815743d0d2666e0e"></a>
## repetition_level_histogram

`function` · `parquet::file::page_index::column_index::ColumnIndexMetaData::repetition_level_histogram` · parquet 59.3.0

```rust
fn repetition_level_histogram(&self, idx: usize) -> Option<&[i64]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::column_index::ColumnIndexMetaData", "path": "ColumnIndexMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [563, 1], "end": [633, 2], "filename": "src/file/page_index/column_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/column_index.rs:619`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the repetition level histogram for the page indexed by `idx`
