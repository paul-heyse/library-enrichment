# `parquet::file::page_index::offset_index::OffsetIndexMetaData`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.page_index.offset_index.OffsetIndexMetaData.json).

<a id="op-c31410ae3eaaa7e7f83822c5"></a>
## OffsetIndexMetaData

`struct` · `parquet::file::page_index::offset_index::OffsetIndexMetaData` · parquet 59.3.0

```rust
struct OffsetIndexMetaData
```

Source: `src/file/page_index/offset_index.rs:47`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

[`OffsetIndex`] information for a column chunk. Contains offsets and sizes for each page
in the chunk. Optionally stores fully decoded page sizes for BYTE_ARRAY columns.

See [`ParquetOffsetIndex`] for more information.

[`ParquetOffsetIndex`]: crate::file::metadata::ParquetOffsetIndex
[`OffsetIndex`]: https://github.com/apache/parquet-format/blob/master/PageIndex.md

<a id="op-6bdf766817607f3f748a021a"></a>
## clone

`function` · `parquet::file::page_index::offset_index::OffsetIndexMetaData::clone` · parquet 59.3.0

```rust
fn clone(&self) -> OffsetIndexMetaData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::offset_index::OffsetIndexMetaData", "path": "OffsetIndexMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [62, 2], "filename": "src/file/page_index/offset_index.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/page_index/offset_index.rs:47`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74aeb3aa326646f4bb246c83"></a>
## eq

`function` · `parquet::file::page_index::offset_index::OffsetIndexMetaData::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &OffsetIndexMetaData) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::offset_index::OffsetIndexMetaData", "path": "OffsetIndexMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [62, 2], "filename": "src/file/page_index/offset_index.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/page_index/offset_index.rs:47`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02eb016815e53895157269e1"></a>
## fmt

`function` · `parquet::file::page_index::offset_index::OffsetIndexMetaData::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::offset_index::OffsetIndexMetaData", "path": "OffsetIndexMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [62, 2], "filename": "src/file/page_index/offset_index.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/page_index/offset_index.rs:47`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f4addcbc4c4004484c442cc"></a>
## page_locations

`struct_field` · `parquet::file::page_index::offset_index::OffsetIndexMetaData::page_locations` · parquet 59.3.0

```rust
page_locations: Vec<PageLocation>
```

Source: `src/file/page_index/offset_index.rs:47`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Vector of [`PageLocation`](../operations/parquet.file.page_index.offset_index.PageLocation.md#op-ae2d7b02d089261641fcb9e0) objects, one per page in the chunk.

<a id="op-cb81085bb1f1f40f275bf544"></a>
## page_locations

`function` · `parquet::file::page_index::offset_index::OffsetIndexMetaData::page_locations` · parquet 59.3.0

```rust
fn page_locations(&self) -> &Vec<PageLocation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::offset_index::OffsetIndexMetaData", "path": "OffsetIndexMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [127, 2], "filename": "src/file/page_index/offset_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/offset_index.rs:66`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Vector of [`PageLocation`](../operations/parquet.file.page_index.offset_index.PageLocation.md#op-ae2d7b02d089261641fcb9e0) objects, one per page in the chunk.

<a id="op-1499780885bd0c2bf27b7512"></a>
## unencoded_byte_array_data_bytes

`function` · `parquet::file::page_index::offset_index::OffsetIndexMetaData::unencoded_byte_array_data_bytes` · parquet 59.3.0

```rust
fn unencoded_byte_array_data_bytes(&self) -> Option<&Vec<i64>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::page_index::offset_index::OffsetIndexMetaData", "path": "OffsetIndexMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [127, 2], "filename": "src/file/page_index/offset_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/page_index/offset_index.rs:72`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Optional vector of unencoded page sizes, one per page in the chunk. Only defined
for BYTE_ARRAY columns.

<a id="op-98f6cf21efac6866b6860cb3"></a>
## unencoded_byte_array_data_bytes

`struct_field` · `parquet::file::page_index::offset_index::OffsetIndexMetaData::unencoded_byte_array_data_bytes` · parquet 59.3.0

```rust
unencoded_byte_array_data_bytes: Option<Vec<i64>>
```

Source: `src/file/page_index/offset_index.rs:47`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Optional vector of unencoded page sizes, one per page in the chunk.
Only defined for BYTE_ARRAY columns.
