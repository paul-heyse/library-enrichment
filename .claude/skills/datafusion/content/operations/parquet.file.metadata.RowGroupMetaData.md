# `parquet::file::metadata::RowGroupMetaData`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.RowGroupMetaData.json).

<a id="op-f84a9d3c17c9ccf2d622296f"></a>
## RowGroupMetaData

`struct` · `parquet::file::metadata::RowGroupMetaData` · parquet 59.3.0

```rust
struct RowGroupMetaData
```

Source: `src/file/metadata/mod.rs:630`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Metadata for a row group

Includes [`ColumnChunkMetaData`](../operations/parquet.file.metadata.ColumnChunkMetaData.md#op-664b3834ca25da410177f90f) for each column in the row group, the number of rows
the total byte size of the row group, and the [`SchemaDescriptor`](../operations/parquet.schema.types.SchemaDescriptor.md#op-cb960d451851ace5640135f7) for the row group.

<a id="op-d6685ec03691b9524bad6e22"></a>
## builder

`function` · `parquet::file::metadata::RowGroupMetaData::builder` · parquet 59.3.0

```rust
fn builder(schema_descr: SchemaDescPtr) -> RowGroupMetaDataBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaData", "path": "RowGroupMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 1], "end": [717, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:644`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns builder for row group metadata.

<a id="op-01c26bae8972901d0542a750"></a>
## clone

`function` · `parquet::file::metadata::RowGroupMetaData::clone` · parquet 59.3.0

```rust
fn clone(&self) -> RowGroupMetaData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaData", "path": "RowGroupMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 17], "end": [629, 22], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/metadata/mod.rs:629`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b6e84bce8a4a35a1e87bb5d"></a>
## column

`function` · `parquet::file::metadata::RowGroupMetaData::column` · parquet 59.3.0

```rust
fn column(&self, i: usize) -> &ColumnChunkMetaData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaData", "path": "RowGroupMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 1], "end": [717, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:654`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns column chunk metadata for `i`th column.

<a id="op-bb40cf82568e0c77bfee715f"></a>
## columns

`function` · `parquet::file::metadata::RowGroupMetaData::columns` · parquet 59.3.0

```rust
fn columns(&self) -> &[ColumnChunkMetaData]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaData", "path": "RowGroupMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 1], "end": [717, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:659`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns slice of column chunk metadata.

<a id="op-ec37d94540ccb253eddcdc2c"></a>
## columns_mut

`function` · `parquet::file::metadata::RowGroupMetaData::columns_mut` · parquet 59.3.0

```rust
fn columns_mut(&mut self) -> &mut [ColumnChunkMetaData]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaData", "path": "RowGroupMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 1], "end": [717, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:664`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns mutable slice of column chunk metadata.

<a id="op-4bb825745add505788573821"></a>
## compressed_size

`function` · `parquet::file::metadata::RowGroupMetaData::compressed_size` · parquet 59.3.0

```rust
fn compressed_size(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaData", "path": "RowGroupMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 1], "end": [717, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:684`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Total size of all compressed column data in this row group.

<a id="op-d32484130acbbba6e568fcdb"></a>
## eq

`function` · `parquet::file::metadata::RowGroupMetaData::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &RowGroupMetaData) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaData", "path": "RowGroupMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 24], "end": [629, 33], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/metadata/mod.rs:629`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c84e46fcb34762e96bbf2eb0"></a>
## file_offset

`function` · `parquet::file::metadata::RowGroupMetaData::file_offset` · parquet 59.3.0

```rust
fn file_offset(&self) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaData", "path": "RowGroupMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 1], "end": [717, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:709`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns file offset of this row group in file.

<a id="op-2d88fa24d0a9b56cc129706a"></a>
## fmt

`function` · `parquet::file::metadata::RowGroupMetaData::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaData", "path": "RowGroupMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 10], "end": [629, 15], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/metadata/mod.rs:629`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-466cdfb9869eee9f8ad9e1b3"></a>
## into_builder

`function` · `parquet::file::metadata::RowGroupMetaData::into_builder` · parquet 59.3.0

```rust
fn into_builder(self) -> RowGroupMetaDataBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaData", "path": "RowGroupMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 1], "end": [717, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:714`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts this [`RowGroupMetaData`](../operations/parquet.file.metadata.RowGroupMetaData.md#op-f84a9d3c17c9ccf2d622296f) into a [`RowGroupMetaDataBuilder`](../operations/parquet.file.metadata.RowGroupMetaDataBuilder.md#op-ffe5b89c2ec5fc4ef8e8260d)

<a id="op-b8fa3acc39aa68d63be7a9d2"></a>
## num_columns

`function` · `parquet::file::metadata::RowGroupMetaData::num_columns` · parquet 59.3.0

```rust
fn num_columns(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaData", "path": "RowGroupMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 1], "end": [717, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:649`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Number of columns in this row group.

<a id="op-9c9c2ddac67b5c950cae429a"></a>
## num_rows

`function` · `parquet::file::metadata::RowGroupMetaData::num_rows` · parquet 59.3.0

```rust
fn num_rows(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaData", "path": "RowGroupMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 1], "end": [717, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:669`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Number of rows in this row group.

<a id="op-ac910eea2e205f531761424d"></a>
## ordinal

`function` · `parquet::file::metadata::RowGroupMetaData::ordinal` · parquet 59.3.0

```rust
fn ordinal(&self) -> Option<i16>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaData", "path": "RowGroupMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 1], "end": [717, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:703`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns ordinal position of this row group in file.

For example if this is the first row group in the file, this will return 0.
If this is the second row group in the file, this will return 1.

<a id="op-e9c0853eb1e255f1807c2a75"></a>
## schema_descr

`function` · `parquet::file::metadata::RowGroupMetaData::schema_descr` · parquet 59.3.0

```rust
fn schema_descr(&self) -> &SchemaDescriptor
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaData", "path": "RowGroupMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 1], "end": [717, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:689`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns reference to a schema descriptor.

<a id="op-75c6ebcf571369e63ce527e4"></a>
## schema_descr_ptr

`function` · `parquet::file::metadata::RowGroupMetaData::schema_descr_ptr` · parquet 59.3.0

```rust
fn schema_descr_ptr(&self) -> SchemaDescPtr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaData", "path": "RowGroupMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 1], "end": [717, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:694`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns reference counted clone of schema descriptor.

<a id="op-de510a9dc8548af256448374"></a>
## sorting_columns

`function` · `parquet::file::metadata::RowGroupMetaData::sorting_columns` · parquet 59.3.0

```rust
fn sorting_columns(&self) -> Option<&Vec<SortingColumn>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaData", "path": "RowGroupMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 1], "end": [717, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:674`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the sort ordering of the rows in this RowGroup if any

<a id="op-395cc1fd0277a98688dc252e"></a>
## total_byte_size

`function` · `parquet::file::metadata::RowGroupMetaData::total_byte_size` · parquet 59.3.0

```rust
fn total_byte_size(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::RowGroupMetaData", "path": "RowGroupMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 1], "end": [717, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:679`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Total byte size of all uncompressed column data in this row group.
