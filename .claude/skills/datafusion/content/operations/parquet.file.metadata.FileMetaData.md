# `parquet::file::metadata::FileMetaData`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.FileMetaData.json).

<a id="op-b5fcc860595f5c38cea8c79d"></a>
## FileMetaData

`struct` · `parquet::file::metadata::FileMetaData` · parquet 59.3.0

```rust
struct FileMetaData
```

Source: `src/file/metadata/mod.rs:487`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

File level metadata for a Parquet file.

Includes the version of the file, metadata, number of rows, schema, and column orders

<a id="op-c400947f0a90031b53193be1"></a>
## clone

`function` · `parquet::file::metadata::FileMetaData::clone` · parquet 59.3.0

```rust
fn clone(&self) -> FileMetaData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::FileMetaData", "path": "FileMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [486, 17], "end": [486, 22], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/metadata/mod.rs:486`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14909f0ae68de6f63b3ce465"></a>
## column_order

`function` · `parquet::file::metadata::FileMetaData::column_order` · parquet 59.3.0

```rust
fn column_order(&self, i: usize) -> ColumnOrder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::FileMetaData", "path": "FileMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 1], "end": [605, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:599`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns column order for `i`th column in this file.
If column orders are not available, returns undefined (legacy) column order.

<a id="op-0f7d4b56bc0a2dda301b7002"></a>
## column_orders

`function` · `parquet::file::metadata::FileMetaData::column_orders` · parquet 59.3.0

```rust
fn column_orders(&self) -> Option<&Vec<ColumnOrder>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::FileMetaData", "path": "FileMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 1], "end": [605, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:593`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Column (sort) order used for `min` and `max` values of each column in this file.

Each column order corresponds to one column, determined by its position in the
list, matching the position of the column in the schema.

When `None` is returned, there are no column orders available, and each column
should be assumed to have undefined (legacy) column order.

<a id="op-0a89aded47ed05b6f0a1966b"></a>
## created_by

`function` · `parquet::file::metadata::FileMetaData::created_by` · parquet 59.3.0

```rust
fn created_by(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::FileMetaData", "path": "FileMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 1], "end": [605, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:560`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

String message for application that wrote this file.

This should have the following format:
`<application> version <application version> (build <application build hash>)`.

```shell
parquet-mr version 1.8.0 (build 0fda28af84b9746396014ad6a415b90592a98b3b)
```

<a id="op-2cdbfee4a58bb8585bb2cc83"></a>
## eq

`function` · `parquet::file::metadata::FileMetaData::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &FileMetaData) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::FileMetaData", "path": "FileMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [486, 24], "end": [486, 33], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/metadata/mod.rs:486`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f43cbe2b7ea497e8af4c5d5"></a>
## fmt

`function` · `parquet::file::metadata::FileMetaData::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::FileMetaData", "path": "FileMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [486, 10], "end": [486, 15], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/metadata/mod.rs:486`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18478888a9ee0bed5bc505f4"></a>
## key_value_metadata

`function` · `parquet::file::metadata::FileMetaData::key_value_metadata` · parquet 59.3.0

```rust
fn key_value_metadata(&self) -> Option<&Vec<KeyValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::FileMetaData", "path": "FileMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 1], "end": [605, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:565`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns key_value_metadata of this file.

<a id="op-ede65077121b21c20853bd8e"></a>
## new

`function` · `parquet::file::metadata::FileMetaData::new` · parquet 59.3.0

```rust
fn new(version: i32, num_rows: i64, created_by: Option<String>, key_value_metadata: Option<Vec<KeyValue>>, schema_descr: SchemaDescPtr, column_orders: Option<Vec<ColumnOrder>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::FileMetaData", "path": "FileMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 1], "end": [605, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:502`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new file metadata.

<a id="op-0fd28788e48f7d0b04f6467f"></a>
## num_rows

`function` · `parquet::file::metadata::FileMetaData::num_rows` · parquet 59.3.0

```rust
fn num_rows(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::FileMetaData", "path": "FileMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 1], "end": [605, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:548`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns number of rows in the file.

<a id="op-6fe117b57a97913c0ffb41aa"></a>
## schema

`function` · `parquet::file::metadata::FileMetaData::schema` · parquet 59.3.0

```rust
fn schema(&self) -> &SchemaType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::FileMetaData", "path": "FileMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 1], "end": [605, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:572`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns Parquet [`Type`] that describes schema in this file.

[`Type`]: crate::schema::types::Type

<a id="op-628d0315ddf06a7c268d0fd4"></a>
## schema_descr

`function` · `parquet::file::metadata::FileMetaData::schema_descr` · parquet 59.3.0

```rust
fn schema_descr(&self) -> &SchemaDescriptor
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::FileMetaData", "path": "FileMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 1], "end": [605, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:577`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a reference to schema descriptor.

<a id="op-ab2caea90a740a904e612a9b"></a>
## schema_descr_ptr

`function` · `parquet::file::metadata::FileMetaData::schema_descr_ptr` · parquet 59.3.0

```rust
fn schema_descr_ptr(&self) -> SchemaDescPtr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::FileMetaData", "path": "FileMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 1], "end": [605, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:582`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns reference counted clone for schema descriptor.

<a id="op-e61f5dda568b82b7b1ea31f8"></a>
## version

`function` · `parquet::file::metadata::FileMetaData::version` · parquet 59.3.0

```rust
fn version(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::FileMetaData", "path": "FileMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 1], "end": [605, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:543`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns version of this file.
