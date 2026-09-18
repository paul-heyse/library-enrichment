# `parquet::file::metadata::RowGroupMetaDataBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.RowGroupMetaDataBuilder.json).

<a id="op-ffe5b89c2ec5fc4ef8e8260d"></a>
## RowGroupMetaDataBuilder

`struct` · `parquet::file::metadata::RowGroupMetaDataBuilder` · parquet 59.3.0

```rust
struct RowGroupMetaDataBuilder
```

Source: `src/file/metadata/mod.rs:720`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Builder for row group metadata.

<a id="op-7f106f5eb78a84faeb1c8fb3"></a>
## add_column_metadata

`function` · `parquet::file::metadata::RowGroupMetaDataBuilder::add_column_metadata` · parquet 59.3.0

```rust
fn add_column_metadata(self, value: ColumnChunkMetaData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaDataBuilder", "path": "RowGroupMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [722, 1], "end": [804, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:770`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Adds a column metadata to this row group

<a id="op-846ab8da68ddc58568cd5356"></a>
## build

`function` · `parquet::file::metadata::RowGroupMetaDataBuilder::build` · parquet 59.3.0

```rust
fn build(self) -> Result<RowGroupMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaDataBuilder", "path": "RowGroupMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [722, 1], "end": [804, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:788`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Builds row group metadata.

<a id="op-9476c6462d1247fb0d44cb1c"></a>
## set_column_metadata

`function` · `parquet::file::metadata::RowGroupMetaDataBuilder::set_column_metadata` · parquet 59.3.0

```rust
fn set_column_metadata(self, value: Vec<ColumnChunkMetaData>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaDataBuilder", "path": "RowGroupMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [722, 1], "end": [804, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:764`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets column metadata for this row group.

<a id="op-41bf7a7ca424ca43bdc61adf"></a>
## set_file_offset

`function` · `parquet::file::metadata::RowGroupMetaDataBuilder::set_file_offset` · parquet 59.3.0

```rust
fn set_file_offset(self, value: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaDataBuilder", "path": "RowGroupMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [722, 1], "end": [804, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:782`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets file offset for this row group.

<a id="op-ef3e0698b874d687dcfac140"></a>
## set_num_rows

`function` · `parquet::file::metadata::RowGroupMetaDataBuilder::set_num_rows` · parquet 59.3.0

```rust
fn set_num_rows(self, value: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaDataBuilder", "path": "RowGroupMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [722, 1], "end": [804, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:737`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets number of rows in this row group.

<a id="op-117b634940177d0573547bb6"></a>
## set_ordinal

`function` · `parquet::file::metadata::RowGroupMetaDataBuilder::set_ordinal` · parquet 59.3.0

```rust
fn set_ordinal(self, value: i16) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaDataBuilder", "path": "RowGroupMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [722, 1], "end": [804, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:776`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets ordinal for this row group.

<a id="op-5d41de261d3ccf520625cd36"></a>
## set_sorting_columns

`function` · `parquet::file::metadata::RowGroupMetaDataBuilder::set_sorting_columns` · parquet 59.3.0

```rust
fn set_sorting_columns(self, value: Option<Vec<SortingColumn>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaDataBuilder", "path": "RowGroupMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [722, 1], "end": [804, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:743`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the sorting order for columns

<a id="op-b5ef4ce27f5f16c5acdcda5b"></a>
## set_total_byte_size

`function` · `parquet::file::metadata::RowGroupMetaDataBuilder::set_total_byte_size` · parquet 59.3.0

```rust
fn set_total_byte_size(self, value: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaDataBuilder", "path": "RowGroupMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [722, 1], "end": [804, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:749`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets total size in bytes for this row group.

<a id="op-d1fe08715e3331a98a2256db"></a>
## take_columns

`function` · `parquet::file::metadata::RowGroupMetaDataBuilder::take_columns` · parquet 59.3.0

```rust
fn take_columns(&mut self) -> Vec<ColumnChunkMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaDataBuilder", "path": "RowGroupMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [722, 1], "end": [804, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:759`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Takes ownership of the the column metadata in this builder, and clears
the list of columns.

This can be used for more efficient creation of a new RowGroupMetaData
from an existing one.
