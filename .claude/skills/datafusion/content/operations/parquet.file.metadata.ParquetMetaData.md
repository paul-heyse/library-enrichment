# `parquet::file::metadata::ParquetMetaData`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.ParquetMetaData.json).

<a id="op-02f1f382aa720b3a6b89476b"></a>
## ParquetMetaData

`struct` · `parquet::file::metadata::ParquetMetaData` · parquet 59.3.0

```rust
struct ParquetMetaData
```

Source: `src/file/metadata/mod.rs:186`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parsed metadata for a single Parquet file

This structure is stored in the footer of Parquet files, in the format
defined by [`parquet.thrift`].

# Overview
The fields of this structure are:
* [`FileMetaData`](../operations/parquet.file.metadata.FileMetaData.md#op-b5fcc860595f5c38cea8c79d): Information about the overall file (such as the schema) (See [`Self::file_metadata`](../operations/parquet.file.metadata.ParquetMetaData.md#op-ff75d7f2bd1fa53e2208c831))
* [`RowGroupMetaData`](../operations/parquet.file.metadata.RowGroupMetaData.md#op-f84a9d3c17c9ccf2d622296f): Information about each Row Group (see [`Self::row_groups`](../operations/parquet.file.metadata.ParquetMetaData.md#op-d0b300b997ab46e3d9bacadd))
* [`ParquetColumnIndex`](../operations/parquet.file.metadata.ParquetColumnIndex.md#op-80ab0cca370795f8250c6d69) and [`ParquetOffsetIndex`](../operations/parquet.file.metadata.ParquetOffsetIndex.md#op-ab57bc084f55b3929a33a434): Optional "Page Index" structures (see [`Self::column_index`](../operations/parquet.file.metadata.ParquetMetaData.md#op-6d352f79b2c893f5dda369f6) and [`Self::offset_index`](../operations/parquet.file.metadata.ParquetMetaData.md#op-88ce7460522c1cf94e1361c1))

This structure is read by the various readers in this crate or can be read
directly from a file using the [`ParquetMetaDataReader`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-ecb8df4cd7e741eec25c5144) struct.

See the [`ParquetMetaDataBuilder`](../operations/parquet.file.metadata.ParquetMetaDataBuilder.md#op-7a6a0f0fba706a5590e33bc1) to create and modify this structure.

[`parquet.thrift`]: https://github.com/apache/parquet-format/blob/master/src/main/thrift/parquet.thrift

<a id="op-50a67ac28f8b1fd7bca16d66"></a>
## clone

`function` · `parquet::file::metadata::ParquetMetaData::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ParquetMetaData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaData", "path": "ParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 17], "end": [185, 22], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/metadata/mod.rs:185`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d352f79b2c893f5dda369f6"></a>
## column_index

`function` · `parquet::file::metadata::ParquetMetaData::column_index` · parquet 59.3.0

```rust
fn column_index(&self) -> Option<&ParquetColumnIndex>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaData", "path": "ParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [310, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:259`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the column index for this file if loaded

Returns `None` if the parquet file does not have a `ColumnIndex` or
[ArrowReaderOptions::with_page_index] was set to false.

[ArrowReaderOptions::with_page_index]: https://docs.rs/parquet/latest/parquet/arrow/arrow_reader/struct.ArrowReaderOptions.html#method.with_page_index

<a id="op-9580aaaec6e44b68fc763f11"></a>
## eq

`function` · `parquet::file::metadata::ParquetMetaData::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &ParquetMetaData) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaData", "path": "ParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 24], "end": [185, 33], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/metadata/mod.rs:185`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff75d7f2bd1fa53e2208c831"></a>
## file_metadata

`function` · `parquet::file::metadata::ParquetMetaData::file_metadata` · parquet 59.3.0

```rust
fn file_metadata(&self) -> &FileMetaData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaData", "path": "ParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [310, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:227`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns file metadata as reference.

<a id="op-66cc9d1531090dc5357e728c"></a>
## fmt

`function` · `parquet::file::metadata::ParquetMetaData::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaData", "path": "ParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 10], "end": [185, 15], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/metadata/mod.rs:185`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2c4d589bab994b041126f1e"></a>
## into_builder

`function` · `parquet::file::metadata::ParquetMetaData::into_builder` · parquet 59.3.0

```rust
fn into_builder(self) -> ParquetMetaDataBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaData", "path": "ParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [310, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:222`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Convert this ParquetMetaData into a [`ParquetMetaDataBuilder`](../operations/parquet.file.metadata.ParquetMetaDataBuilder.md#op-7a6a0f0fba706a5590e33bc1)

<a id="op-19dd3820f5a02870cda88bbd"></a>
## memory_size

`function` · `parquet::file::metadata::ParquetMetaData::memory_size` · parquet 59.3.0

```rust
fn memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaData", "path": "ParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [310, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:287`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Estimate of the bytes allocated to store `ParquetMetadata`

# Notes:

1. Includes size of self

2. Includes heap memory for sub fields such as [`FileMetaData`](../operations/parquet.file.metadata.FileMetaData.md#op-b5fcc860595f5c38cea8c79d) and
   [`RowGroupMetaData`](../operations/parquet.file.metadata.RowGroupMetaData.md#op-f84a9d3c17c9ccf2d622296f).

3. Includes memory from shared pointers (e.g. [`SchemaDescPtr`](../operations/parquet.schema.types.SchemaDescPtr.md#op-eb89590f5ea7f99df17bf043)). This
   means `memory_size` will over estimate the memory size if such pointers
   are shared.

4. Does not include any allocator overheads

<a id="op-318ec39518cd17bc81e011c7"></a>
## new

`function` · `parquet::file::metadata::ParquetMetaData::new` · parquet 59.3.0

```rust
fn new(file_metadata: FileMetaData, row_groups: Vec<RowGroupMetaData>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaData", "path": "ParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [310, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:203`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates Parquet metadata from file metadata and a list of row
group metadata

<a id="op-23c49bb55c4251c1d57f17bf"></a>
## num_row_groups

`function` · `parquet::file::metadata::ParquetMetaData::num_row_groups` · parquet 59.3.0

```rust
fn num_row_groups(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaData", "path": "ParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [310, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:238`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns number of row groups in this file.

<a id="op-88ce7460522c1cf94e1361c1"></a>
## offset_index

`function` · `parquet::file::metadata::ParquetMetaData::offset_index` · parquet 59.3.0

```rust
fn offset_index(&self) -> Option<&ParquetOffsetIndex>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaData", "path": "ParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [310, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:269`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns offset indexes in this file, if loaded

Returns `None` if the parquet file does not have a `OffsetIndex` or
[ArrowReaderOptions::with_page_index] was set to false.

[ArrowReaderOptions::with_page_index]: https://docs.rs/parquet/latest/parquet/arrow/arrow_reader/struct.ArrowReaderOptions.html#method.with_page_index

<a id="op-621051540221c5580903dc8c"></a>
## row_group

`function` · `parquet::file::metadata::ParquetMetaData::row_group` · parquet 59.3.0

```rust
fn row_group(&self, i: usize) -> &RowGroupMetaData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaData", "path": "ParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [310, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:244`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns row group metadata for `i`th position.
Position should be less than number of row groups `num_row_groups`.

<a id="op-d0b300b997ab46e3d9bacadd"></a>
## row_groups

`function` · `parquet::file::metadata::ParquetMetaData::row_groups` · parquet 59.3.0

```rust
fn row_groups(&self) -> &[RowGroupMetaData]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaData", "path": "ParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [310, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:249`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns slice of row groups in this file.
