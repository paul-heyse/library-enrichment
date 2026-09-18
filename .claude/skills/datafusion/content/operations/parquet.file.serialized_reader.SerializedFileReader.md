# `parquet::file::serialized_reader::SerializedFileReader`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.serialized_reader.SerializedFileReader.json).

<a id="op-770b38e09c96846980922df5"></a>
## SerializedFileReader

`struct` · `parquet::file::serialized_reader::SerializedFileReader` · parquet 59.3.0

```rust
struct SerializedFileReader<R: ChunkReader>
```

Source: `src/file/serialized_reader.rs:94`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A serialized implementation for Parquet [`FileReader`](../operations/parquet.file.reader.FileReader.md#op-9b66de5d3c7d389f75983bc1).

<a id="op-876e2a20840a445a08f9d587"></a>
## Error

`assoc_type` · `parquet::file::serialized_reader::SerializedFileReader::Error` · parquet 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "std::fs::File", "path": "std::fs::File"}}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedFileReader", "path": "SerializedFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [61, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "std::path::Path", "path": "Path"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file/serialized_reader.rs:55`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3ae91bea7d68bca630b474b"></a>
## Error

`assoc_type` · `parquet::file::serialized_reader::SerializedFileReader::Error` · parquet 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "std::fs::File", "path": "std::fs::File"}}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedFileReader", "path": "SerializedFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [52, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "std::fs::File", "path": "File"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file/serialized_reader.rs:47`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc35a42ad0988105c1d9fb42"></a>
## Error

`assoc_type` · `parquet::file::serialized_reader::SerializedFileReader::Error` · parquet 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "std::fs::File", "path": "std::fs::File"}}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedFileReader", "path": "SerializedFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [69, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file/serialized_reader.rs:64`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c441849cff560d33be628e20"></a>
## Error

`assoc_type` · `parquet::file::serialized_reader::SerializedFileReader::Error` · parquet 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "std::fs::File", "path": "std::fs::File"}}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedFileReader", "path": "SerializedFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [77, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file/serialized_reader.rs:72`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9bc70bc9c063873a2a6eb81"></a>
## IntoIter

`assoc_type` · `parquet::file::serialized_reader::SerializedFileReader::IntoIter` · parquet 59.3.0

```rust
IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "std::fs::File", "path": "std::fs::File"}}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedFileReader", "path": "SerializedFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [88, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/file/serialized_reader.rs:83`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9098a27996ee728f10ffadd"></a>
## Item

`assoc_type` · `parquet::file::serialized_reader::SerializedFileReader::Item` · parquet 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "std::fs::File", "path": "std::fs::File"}}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedFileReader", "path": "SerializedFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [88, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/file/serialized_reader.rs:82`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa29f17fe4a037968f6f5c6f"></a>
## get_row_group

`function` · `parquet::file::serialized_reader::SerializedFileReader::get_row_group` · parquet 59.3.0

```rust
fn get_row_group(&self, i: usize) -> Result<Box<dyn RowGroupReader + '_>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedFileReader", "path": "SerializedFileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [294, 1], "end": [319, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": null, "id": "parquet::file::reader::FileReader", "path": "FileReader"}, "trait_path": "parquet::file::reader::FileReader"}`

Source: `src/file/serialized_reader.rs:303`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1599fa5a3b86b7f33f441c98"></a>
## get_row_iter

`function` · `parquet::file::serialized_reader::SerializedFileReader::get_row_iter` · parquet 59.3.0

```rust
fn get_row_iter(&self, projection: Option<SchemaType>) -> Result<RowIter<'_>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedFileReader", "path": "SerializedFileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [294, 1], "end": [319, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": null, "id": "parquet::file::reader::FileReader", "path": "FileReader"}, "trait_path": "parquet::file::reader::FileReader"}`

Source: `src/file/serialized_reader.rs:316`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-088c72aeda7bc087d3d90ed0"></a>
## into_iter

`function` · `parquet::file::serialized_reader::SerializedFileReader::into_iter` · parquet 59.3.0

```rust
fn into_iter(self) -> Self::IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "std::fs::File", "path": "std::fs::File"}}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedFileReader", "path": "SerializedFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [88, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/file/serialized_reader.rs:85`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bef9a718aa8d598f8c52c9d3"></a>
## metadata

`function` · `parquet::file::serialized_reader::SerializedFileReader::metadata` · parquet 59.3.0

```rust
fn metadata(&self) -> &ParquetMetaData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedFileReader", "path": "SerializedFileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [294, 1], "end": [319, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": null, "id": "parquet::file::reader::FileReader", "path": "FileReader"}, "trait_path": "parquet::file::reader::FileReader"}`

Source: `src/file/serialized_reader.rs:295`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a54289aaeeae49348d3ff77b"></a>
## new

`function` · `parquet::file::serialized_reader::SerializedFileReader::new` · parquet 59.3.0

```rust
fn new(chunk_reader: R) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedFileReader", "path": "SerializedFileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 1], "end": [280, 2], "filename": "src/file/serialized_reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/serialized_reader.rs:231`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates file reader from a Parquet file.
Returns an error if the Parquet file does not exist or is corrupt.

<a id="op-b0cd341ba07788d2718bcfd6"></a>
## new_with_options

`function` · `parquet::file::serialized_reader::SerializedFileReader::new_with_options` · parquet 59.3.0

```rust
fn new_with_options(chunk_reader: R, options: ReadOptions) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedFileReader", "path": "SerializedFileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 1], "end": [280, 2], "filename": "src/file/serialized_reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/serialized_reader.rs:243`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates file reader from a Parquet file with read options.
Returns an error if the Parquet file does not exist or is corrupt.

<a id="op-7981fc44e5c217aff1bcd082"></a>
## num_row_groups

`function` · `parquet::file::serialized_reader::SerializedFileReader::num_row_groups` · parquet 59.3.0

```rust
fn num_row_groups(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedFileReader", "path": "SerializedFileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [294, 1], "end": [319, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": null, "id": "parquet::file::reader::FileReader", "path": "FileReader"}, "trait_path": "parquet::file::reader::FileReader"}`

Source: `src/file/serialized_reader.rs:299`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11ebb8cd2e00a70fc80f9c51"></a>
## try_from

`function` · `parquet::file::serialized_reader::SerializedFileReader::try_from` · parquet 59.3.0

```rust
fn try_from(path: String) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "std::fs::File", "path": "std::fs::File"}}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedFileReader", "path": "SerializedFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [69, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file/serialized_reader.rs:66`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfe82d215561427d32ec76d8"></a>
## try_from

`function` · `parquet::file::serialized_reader::SerializedFileReader::try_from` · parquet 59.3.0

```rust
fn try_from(path: &Path) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "std::fs::File", "path": "std::fs::File"}}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedFileReader", "path": "SerializedFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [61, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "std::path::Path", "path": "Path"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file/serialized_reader.rs:57`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4a28a1acdfe1b8953086169"></a>
## try_from

`function` · `parquet::file::serialized_reader::SerializedFileReader::try_from` · parquet 59.3.0

```rust
fn try_from(path: &str) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "std::fs::File", "path": "std::fs::File"}}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedFileReader", "path": "SerializedFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [77, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file/serialized_reader.rs:74`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e436ed74acc4514497050fea"></a>
## try_from

`function` · `parquet::file::serialized_reader::SerializedFileReader::try_from` · parquet 59.3.0

```rust
fn try_from(file: File) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "std::fs::File", "path": "std::fs::File"}}}], "constraints": []}}, "id": "parquet::file::serialized_reader::SerializedFileReader", "path": "SerializedFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [52, 2], "filename": "src/file/serialized_reader.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "std::fs::File", "path": "File"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file/serialized_reader.rs:49`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
