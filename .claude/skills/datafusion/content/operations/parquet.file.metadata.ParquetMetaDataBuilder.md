# `parquet::file::metadata::ParquetMetaDataBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.ParquetMetaDataBuilder.json).

<a id="op-7a6a0f0fba706a5590e33bc1"></a>
## ParquetMetaDataBuilder

`struct` · `parquet::file::metadata::ParquetMetaDataBuilder` · parquet 59.3.0

```rust
struct ParquetMetaDataBuilder
```

Source: `src/file/metadata/mod.rs:349`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

 A builder for creating / manipulating [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b)

 # Example creating a new [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b)

```no_run
 # use parquet::file::metadata::{FileMetaData, ParquetMetaData, ParquetMetaDataBuilder, RowGroupMetaData, RowGroupMetaDataBuilder};
 # fn get_file_metadata() -> FileMetaData { unimplemented!(); }
 // Create a new builder given the file metadata
 let file_metadata = get_file_metadata();
 // Create a row group
 let row_group = RowGroupMetaData::builder(file_metadata.schema_descr_ptr())
    .set_num_rows(100)
    // ... (A real row group needs more than just the number of rows)
    .build()
    .unwrap();
 // Create the final metadata
 let metadata: ParquetMetaData = ParquetMetaDataBuilder::new(file_metadata)
   .add_row_group(row_group)
   .build();
 ```

 # Example modifying an existing [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b)
 ```no_run
 # use parquet::file::metadata::ParquetMetaData;
 # fn load_metadata() -> ParquetMetaData { unimplemented!(); }
 // Modify the metadata so only the last RowGroup remains
 let metadata: ParquetMetaData = load_metadata();
 let mut builder = metadata.into_builder();

 // Take existing row groups to modify
 let mut row_groups = builder.take_row_groups();
 let last_row_group = row_groups.pop().unwrap();

 let metadata = builder
   .add_row_group(last_row_group)
   .build();
 ```

<a id="op-f670f7148b41d238e3e3df02"></a>
## add_row_group

`function` · `parquet::file::metadata::ParquetMetaDataBuilder::add_row_group` · parquet 59.3.0

```rust
fn add_row_group(self, row_group: RowGroupMetaData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaDataBuilder", "path": "ParquetMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [351, 1], "end": [432, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:363`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Adds a row group to the metadata

<a id="op-ed1d60842c4cafe03fa73362"></a>
## build

`function` · `parquet::file::metadata::ParquetMetaDataBuilder::build` · parquet 59.3.0

```rust
fn build(self) -> ParquetMetaData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaDataBuilder", "path": "ParquetMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [351, 1], "end": [432, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:428`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a new ParquetMetaData from the builder

<a id="op-ca38d0f95b6e56431f95c8cb"></a>
## column_index

`function` · `parquet::file::metadata::ParquetMetaDataBuilder::column_index` · parquet 59.3.0

```rust
fn column_index(&self) -> Option<&ParquetColumnIndex>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaDataBuilder", "path": "ParquetMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [351, 1], "end": [432, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:400`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return a reference to the current column index, if any

<a id="op-516cab2657ac354c2283b81b"></a>
## from

`function` · `parquet::file::metadata::ParquetMetaDataBuilder::from` · parquet 59.3.0

```rust
fn from(meta_data: ParquetMetaData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaDataBuilder", "path": "ParquetMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [434, 1], "end": [438, 2], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaData", "path": "ParquetMetaData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/file/metadata/mod.rs:435`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ad8ec34dc879373f13b0f35"></a>
## new

`function` · `parquet::file::metadata::ParquetMetaDataBuilder::new` · parquet 59.3.0

```rust
fn new(file_meta_data: FileMetaData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaDataBuilder", "path": "ParquetMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [351, 1], "end": [432, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:353`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new builder from a file metadata, with no row groups

<a id="op-36d9513ca3f4df1f07219c39"></a>
## new_from_metadata

`function` · `parquet::file::metadata::ParquetMetaDataBuilder::new_from_metadata` · parquet 59.3.0

```rust
fn new_from_metadata(metadata: ParquetMetaData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaDataBuilder", "path": "ParquetMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [351, 1], "end": [432, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:358`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new builder from an existing ParquetMetaData

<a id="op-b49eeaa062214735217d6ad4"></a>
## offset_index

`function` · `parquet::file::metadata::ParquetMetaDataBuilder::offset_index` · parquet 59.3.0

```rust
fn offset_index(&self) -> Option<&ParquetOffsetIndex>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaDataBuilder", "path": "ParquetMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [351, 1], "end": [432, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:416`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return a reference to the current offset index, if any

<a id="op-15908846fee06e1985059ac6"></a>
## row_groups

`function` · `parquet::file::metadata::ParquetMetaDataBuilder::row_groups` · parquet 59.3.0

```rust
fn row_groups(&self) -> &[RowGroupMetaData]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaDataBuilder", "path": "ParquetMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [351, 1], "end": [432, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:384`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return a reference to the current row groups

<a id="op-d9006e306eecf58a252eb548"></a>
## set_column_index

`function` · `parquet::file::metadata::ParquetMetaDataBuilder::set_column_index` · parquet 59.3.0

```rust
fn set_column_index(self, column_index: Option<ParquetColumnIndex>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaDataBuilder", "path": "ParquetMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [351, 1], "end": [432, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:389`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the column index

<a id="op-250b43ff46249800eee40083"></a>
## set_offset_index

`function` · `parquet::file::metadata::ParquetMetaDataBuilder::set_offset_index` · parquet 59.3.0

```rust
fn set_offset_index(self, offset_index: Option<ParquetOffsetIndex>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaDataBuilder", "path": "ParquetMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [351, 1], "end": [432, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:405`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the offset index

<a id="op-0e24405e315f5cc3f1ca179d"></a>
## set_row_groups

`function` · `parquet::file::metadata::ParquetMetaDataBuilder::set_row_groups` · parquet 59.3.0

```rust
fn set_row_groups(self, row_groups: Vec<RowGroupMetaData>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaDataBuilder", "path": "ParquetMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [351, 1], "end": [432, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:369`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets all the row groups to the specified list

<a id="op-87a9fbae57f9a8173791baa1"></a>
## take_column_index

`function` · `parquet::file::metadata::ParquetMetaDataBuilder::take_column_index` · parquet 59.3.0

```rust
fn take_column_index(&mut self) -> Option<ParquetColumnIndex>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaDataBuilder", "path": "ParquetMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [351, 1], "end": [432, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:395`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the current column index from the builder, replacing it with `None`

<a id="op-dc3ceaf9c985d65522eb28af"></a>
## take_offset_index

`function` · `parquet::file::metadata::ParquetMetaDataBuilder::take_offset_index` · parquet 59.3.0

```rust
fn take_offset_index(&mut self) -> Option<ParquetOffsetIndex>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaDataBuilder", "path": "ParquetMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [351, 1], "end": [432, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:411`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the current offset index from the builder, replacing it with `None`

<a id="op-475a38a65ba87e054913e8a1"></a>
## take_row_groups

`function` · `parquet::file::metadata::ParquetMetaDataBuilder::take_row_groups` · parquet 59.3.0

```rust
fn take_row_groups(&mut self) -> Vec<RowGroupMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ParquetMetaDataBuilder", "path": "ParquetMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [351, 1], "end": [432, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:379`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Takes ownership of the row groups in this builder, and clears the list
of row groups.

This can be used for more efficient creation of a new ParquetMetaData
from an existing one.
