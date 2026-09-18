# `parquet::file::metadata::OffsetIndexBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.OffsetIndexBuilder.json).

<a id="op-f374a8e96e3c901563419960"></a>
## OffsetIndexBuilder

`struct` · `parquet::file::metadata::OffsetIndexBuilder` · parquet 59.3.0

```rust
struct OffsetIndexBuilder
```

Source: `src/file/metadata/mod.rs:1628`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Builder for offset index, part of the Parquet [PageIndex].

[PageIndex]: https://github.com/apache/parquet-format/blob/master/PageIndex.md

<a id="op-4b8a792434ef6a6961ee042f"></a>
## append_offset_and_size

`function` · `parquet::file::metadata::OffsetIndexBuilder::append_offset_and_size` · parquet 59.3.0

```rust
fn append_offset_and_size(&mut self, offset: i64, compressed_page_size: i32)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::OffsetIndexBuilder", "path": "OffsetIndexBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1642, 1], "end": [1697, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1662`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Append the offset and size of the next page.

<a id="op-07cbf490759ff3ab3a52a67f"></a>
## append_row_count

`function` · `parquet::file::metadata::OffsetIndexBuilder::append_row_count` · parquet 59.3.0

```rust
fn append_row_count(&mut self, row_count: i64)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::OffsetIndexBuilder", "path": "OffsetIndexBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1642, 1], "end": [1697, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1655`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Append the row count of the next page.

<a id="op-59059e345f2805ab26857f6c"></a>
## append_unencoded_byte_array_data_bytes

`function` · `parquet::file::metadata::OffsetIndexBuilder::append_unencoded_byte_array_data_bytes` · parquet 59.3.0

```rust
fn append_unencoded_byte_array_data_bytes(&mut self, unencoded_byte_array_data_bytes: Option<i64>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::OffsetIndexBuilder", "path": "OffsetIndexBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1642, 1], "end": [1697, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1668`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Append the unencoded byte array data bytes of the next page.

<a id="op-cdd59cbcd3e396b47c636391"></a>
## build

`function` · `parquet::file::metadata::OffsetIndexBuilder::build` · parquet 59.3.0

```rust
fn build(self) -> OffsetIndexMetaData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::OffsetIndexBuilder", "path": "OffsetIndexBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1642, 1], "end": [1697, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1680`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Build and get the thrift metadata of offset index

<a id="op-35e235eb65540829671fe435"></a>
## default

`function` · `parquet::file::metadata::OffsetIndexBuilder::default` · parquet 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::OffsetIndexBuilder", "path": "OffsetIndexBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1636, 1], "end": [1640, 2], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file/metadata/mod.rs:1637`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de7535979dd0cc577b0b92c7"></a>
## new

`function` · `parquet::file::metadata::OffsetIndexBuilder::new` · parquet 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::OffsetIndexBuilder", "path": "OffsetIndexBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1642, 1], "end": [1697, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1644`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a new offset index builder.
