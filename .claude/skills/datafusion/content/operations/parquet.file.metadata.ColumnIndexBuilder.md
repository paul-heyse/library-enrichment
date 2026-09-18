# `parquet::file::metadata::ColumnIndexBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.ColumnIndexBuilder.json).

<a id="op-3450d0ad645204305f0ab427"></a>
## ColumnIndexBuilder

`struct` · `parquet::file::metadata::ColumnIndexBuilder` · parquet 59.3.0

```rust
struct ColumnIndexBuilder
```

Source: `src/file/metadata/mod.rs:1455`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Builder for Parquet [`ColumnIndex`], part of the Parquet [PageIndex]

[PageIndex]: https://github.com/apache/parquet-format/blob/master/PageIndex.md
[`ColumnIndex`]: https://github.com/apache/parquet-format/blob/master/PageIndex.md

<a id="op-7baeb008409f1fd91fb7cc2d"></a>
## append

`function` · `parquet::file::metadata::ColumnIndexBuilder::append` · parquet 59.3.0

```rust
fn append(&mut self, null_page: bool, min_value: Vec<u8>, max_value: Vec<u8>, null_count: i64)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnIndexBuilder", "path": "ColumnIndexBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1476, 1], "end": [1617, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1493`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Append statistics for the next page

<a id="op-23ea7aec1177b260563a5b13"></a>
## append_histograms

`function` · `parquet::file::metadata::ColumnIndexBuilder::append_histograms` · parquet 59.3.0

```rust
fn append_histograms(&mut self, repetition_level_histogram: &Option<LevelHistogram>, definition_level_histogram: &Option<LevelHistogram>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnIndexBuilder", "path": "ColumnIndexBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1476, 1], "end": [1617, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1510`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Append the given page-level histograms to the [`ColumnIndex`] histograms.
Does nothing if the `ColumnIndexBuilder` is not in the `valid` state.

[`ColumnIndex`]: https://github.com/apache/parquet-format/blob/master/PageIndex.md

<a id="op-3eb111a6c48b8751c115719e"></a>
## build

`function` · `parquet::file::metadata::ColumnIndexBuilder::build` · parquet 59.3.0

```rust
fn build(self) -> Result<ColumnIndexMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnIndexBuilder", "path": "ColumnIndexBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1476, 1], "end": [1617, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1548`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Build and get the column index

Note: callers should check [`Self::valid`](../operations/parquet.file.metadata.ColumnIndexBuilder.md#op-f923f084125b9c0d37b5f392) before calling this method

<a id="op-0001bfa0ad09c8cda095d1e5"></a>
## new

`function` · `parquet::file::metadata::ColumnIndexBuilder::new` · parquet 59.3.0

```rust
fn new(column_type: Type) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnIndexBuilder", "path": "ColumnIndexBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1476, 1], "end": [1617, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1478`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a new column index builder.

<a id="op-6c257d947a3253939a51f04e"></a>
## set_boundary_order

`function` · `parquet::file::metadata::ColumnIndexBuilder::set_boundary_order` · parquet 59.3.0

```rust
fn set_boundary_order(&mut self, boundary_order: BoundaryOrder)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnIndexBuilder", "path": "ColumnIndexBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1476, 1], "end": [1617, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1531`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set the boundary order of the column index

<a id="op-8f4e9430da1b2dccbc649a10"></a>
## to_invalid

`function` · `parquet::file::metadata::ColumnIndexBuilder::to_invalid` · parquet 59.3.0

```rust
fn to_invalid(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnIndexBuilder", "path": "ColumnIndexBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1476, 1], "end": [1617, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1536`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Mark this column index as invalid

<a id="op-f923f084125b9c0d37b5f392"></a>
## valid

`function` · `parquet::file::metadata::ColumnIndexBuilder::valid` · parquet 59.3.0

```rust
fn valid(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnIndexBuilder", "path": "ColumnIndexBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1476, 1], "end": [1617, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1541`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Is the information in the builder valid?
