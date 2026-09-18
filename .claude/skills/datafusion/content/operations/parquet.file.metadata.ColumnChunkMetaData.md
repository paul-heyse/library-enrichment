# `parquet::file::metadata::ColumnChunkMetaData`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.ColumnChunkMetaData.json).

<a id="op-664b3834ca25da410177f90f"></a>
## ColumnChunkMetaData

`struct` · `parquet::file::metadata::ColumnChunkMetaData` · parquet 59.3.0

```rust
struct ColumnChunkMetaData
```

Source: `src/file/metadata/mod.rs:808`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Metadata for a column chunk.

<a id="op-e3d771fb92cbb3c73ea0f106"></a>
## bloom_filter_length

`function` · `parquet::file::metadata::ColumnChunkMetaData::bloom_filter_length` · parquet 59.3.0

```rust
fn bloom_filter_length(&self) -> Option<i32>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1137`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the offset for the bloom filter.

<a id="op-02c8a0a82b6ba2e9238af347"></a>
## bloom_filter_offset

`function` · `parquet::file::metadata::ColumnChunkMetaData::bloom_filter_offset` · parquet 59.3.0

```rust
fn bloom_filter_offset(&self) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1132`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the offset for the bloom filter.

<a id="op-c3166018650e2fe11ff1937e"></a>
## builder

`function` · `parquet::file::metadata::ColumnChunkMetaData::builder` · parquet 59.3.0

```rust
fn builder(column_descr: ColumnDescPtr) -> ColumnChunkMetaDataBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:959`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns builder for column chunk metadata.

<a id="op-4cbde1856370299151e7e457"></a>
## byte_range

`function` · `parquet::file::metadata::ColumnChunkMetaData::byte_range` · parquet 59.3.0

```rust
fn byte_range(&self) -> (u64, u64)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1057`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the offset and length in bytes of the column chunk within the file

<a id="op-e279f093de08e335d144326b"></a>
## clone

`function` · `parquet::file::metadata::ColumnChunkMetaData::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ColumnChunkMetaData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [807, 17], "end": [807, 22], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/metadata/mod.rs:807`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4432dc255b8eb243ccea1152"></a>
## column_descr

`function` · `parquet::file::metadata::ColumnChunkMetaData::column_descr` · parquet 59.3.0

```rust
fn column_descr(&self) -> &ColumnDescriptor
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:992`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Descriptor for this column.

<a id="op-faf8975e1e17fd0ed59c145e"></a>
## column_descr_ptr

`function` · `parquet::file::metadata::ColumnChunkMetaData::column_descr_ptr` · parquet 59.3.0

```rust
fn column_descr_ptr(&self) -> ColumnDescPtr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:997`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Reference counted clone of descriptor for this column.

<a id="op-15edf875f27feebe36e8d40d"></a>
## column_index_length

`function` · `parquet::file::metadata::ColumnChunkMetaData::column_index_length` · parquet 59.3.0

```rust
fn column_index_length(&self) -> Option<i32>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1147`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the offset for the column index length.

<a id="op-3f39f490ded1516cd6de38cb"></a>
## column_index_offset

`function` · `parquet::file::metadata::ColumnChunkMetaData::column_index_offset` · parquet 59.3.0

```rust
fn column_index_offset(&self) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1142`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the offset for the column index.

<a id="op-1c966361642bd18803e22b18"></a>
## column_path

`function` · `parquet::file::metadata::ColumnChunkMetaData::column_path` · parquet 59.3.0

```rust
fn column_path(&self) -> &ColumnPath
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:987`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Path (or identifier) of this column.

<a id="op-a02b8b12499664992b7c9cba"></a>
## column_type

`function` · `parquet::file::metadata::ColumnChunkMetaData::column_type` · parquet 59.3.0

```rust
fn column_type(&self) -> Type
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:982`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Type of this column. Must be primitive.

<a id="op-27b65044b57f96d92a8c4a52"></a>
## compressed_size

`function` · `parquet::file::metadata::ColumnChunkMetaData::compressed_size` · parquet 59.3.0

```rust
fn compressed_size(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1032`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the total compressed data size of this column chunk.

<a id="op-6795ef9f37510f1513fc02f0"></a>
## compression

`function` · `parquet::file::metadata::ColumnChunkMetaData::compression` · parquet 59.3.0

```rust
fn compression(&self) -> Compression
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1022`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

[`Compression`](../operations/parquet.basic.Compression.md#op-c5a5132a156c388f8a10fcad) for this column.

This is a default value suitable for passing to [`WriterPropertiesBuilder::set_compression`].
It is constructed from the `codec` field of the Parquet `ColumnMetaData`

[`WriterPropertiesBuilder::set_compression`]: crate::file::properties::WriterPropertiesBuilder

<a id="op-5d9c930ae67c65d193247043"></a>
## compression_codec

`function` · `parquet::file::metadata::ColumnChunkMetaData::compression_codec` · parquet 59.3.0

```rust
fn compression_codec(&self) -> CompressionCodec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1027`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the compression codec used when writing this column.

<a id="op-76e5b09a5ef3863bd5575e02"></a>
## crypto_metadata

`function` · `parquet::file::metadata::ColumnChunkMetaData::crypto_metadata` · parquet 59.3.0

```rust
fn crypto_metadata(&self) -> Option<&ColumnCryptoMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1203`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the encryption metadata for this column chunk.

<a id="op-2f92b2ce6d67082d638797f6"></a>
## data_page_offset

`function` · `parquet::file::metadata::ColumnChunkMetaData::data_page_offset` · parquet 59.3.0

```rust
fn data_page_offset(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1042`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the offset for the column data.

<a id="op-fb60fe6e199fbf3279ceaf7e"></a>
## definition_level_histogram

`function` · `parquet::file::metadata::ColumnChunkMetaData::definition_level_histogram` · parquet 59.3.0

```rust
fn definition_level_histogram(&self) -> Option<&LevelHistogram>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1197`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the definition level histogram.

The returned value `vec[i]` is how many values are at definition level `i`. For example,
`vec[max_definition_level]` indicates how many non-null values are present in the page.
This field may not be set by older writers.

<a id="op-3d26fc613083ac6437c9658d"></a>
## dictionary_page_offset

`function` · `parquet::file::metadata::ColumnChunkMetaData::dictionary_page_offset` · parquet 59.3.0

```rust
fn dictionary_page_offset(&self) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1052`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the offset for the dictionary page, if any.

<a id="op-ab564cf91ebefccc4723bf54"></a>
## encodings

`function` · `parquet::file::metadata::ColumnChunkMetaData::encodings` · parquet 59.3.0

```rust
fn encodings(&self) -> impl Iterator<Item = Encoding>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1002`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

All encodings used for this column.

<a id="op-c8fe68c2075a6d0fdcb50800"></a>
## encodings_mask

`function` · `parquet::file::metadata::ColumnChunkMetaData::encodings_mask` · parquet 59.3.0

```rust
fn encodings_mask(&self) -> &EncodingMask
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1007`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

All encodings used for this column, returned as a bitmask.

<a id="op-59f8c1673b5f6f32abe66675"></a>
## eq

`function` · `parquet::file::metadata::ColumnChunkMetaData::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &ColumnChunkMetaData) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [807, 24], "end": [807, 33], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/metadata/mod.rs:807`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4db77c682756ac54decfb596"></a>
## file_offset

`function` · `parquet::file::metadata::ColumnChunkMetaData::file_offset` · parquet 59.3.0

```rust
fn file_offset(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:977`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Byte offset of `ColumnMetaData` in `file_path()`.

Note that the meaning of this field has been inconsistent between implementations
so its use has since been deprecated in the Parquet specification. Modern implementations
will set this to `0` to indicate that the `ColumnMetaData` is solely contained in the
`ColumnChunk` struct.

<a id="op-005991a3ad9fe148d55ec7f3"></a>
## file_path

`function` · `parquet::file::metadata::ColumnChunkMetaData::file_path` · parquet 59.3.0

```rust
fn file_path(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:967`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

File where the column chunk is stored.

If not set, assumed to belong to the same file as the metadata.
This path is relative to the current file.

<a id="op-356d1a70d81d67446c3702cf"></a>
## fmt

`function` · `parquet::file::metadata::ColumnChunkMetaData::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [807, 10], "end": [807, 15], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/metadata/mod.rs:807`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d4ff6a375651fd2eabfaa14"></a>
## geo_statistics

`function` · `parquet::file::metadata::ColumnChunkMetaData::geo_statistics` · parquet 59.3.0

```rust
fn geo_statistics(&self) -> Option<&geo_statistics::GeospatialStatistics>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1078`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns geospatial statistics that are set for this column chunk,
or `None` if no geospatial statistics are available.

<a id="op-22da2f479084e399506b07cd"></a>
## index_page_offset

`function` · `parquet::file::metadata::ColumnChunkMetaData::index_page_offset` · parquet 59.3.0

```rust
fn index_page_offset(&self) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1047`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the offset for the index page.

<a id="op-bbab7bae0cc9e376f397e5b4"></a>
## into_builder

`function` · `parquet::file::metadata::ColumnChunkMetaData::into_builder` · parquet 59.3.0

```rust
fn into_builder(self) -> ColumnChunkMetaDataBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1208`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts this [`ColumnChunkMetaData`](../operations/parquet.file.metadata.ColumnChunkMetaData.md#op-664b3834ca25da410177f90f) into a [`ColumnChunkMetaDataBuilder`](../operations/parquet.file.metadata.ColumnChunkMetaDataBuilder.md#op-b5bee7e1fe4e5a85ce43c3f4)

<a id="op-4f206c6234394fd8e7da1f8e"></a>
## num_values

`function` · `parquet::file::metadata::ColumnChunkMetaData::num_values` · parquet 59.3.0

```rust
fn num_values(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1012`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Total number of values in this column chunk.

<a id="op-379b2624713d5e1c01b2b2e7"></a>
## offset_index_length

`function` · `parquet::file::metadata::ColumnChunkMetaData::offset_index_length` · parquet 59.3.0

```rust
fn offset_index_length(&self) -> Option<i32>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1164`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the offset for the offset index length.

<a id="op-3450377661c7783d476677aa"></a>
## offset_index_offset

`function` · `parquet::file::metadata::ColumnChunkMetaData::offset_index_offset` · parquet 59.3.0

```rust
fn offset_index_offset(&self) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1159`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the offset for the offset index.

<a id="op-f213784ab5518dc89c53fc82"></a>
## page_encoding_stats

`function` · `parquet::file::metadata::ColumnChunkMetaData::page_encoding_stats` · parquet 59.3.0

```rust
fn page_encoding_stats(&self) -> Option<&Vec<PageEncodingStats>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1088`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the page encoding statistics, or `None` if no page encoding statistics
are available (or they were converted to a mask).

Note: By default, this crate converts page encoding statistics to a mask for performance
reasons. To get the full statistics, you must set [`ParquetMetaDataOptions::with_encoding_stats_as_mask`](../operations/parquet.file.metadata.options.ParquetMetaDataOptions.md#op-5119f1010aa08c4b35a99c5f)
to `false`.

<a id="op-afe0cb25a019198512177e3d"></a>
## page_encoding_stats_mask

`function` · `parquet::file::metadata::ColumnChunkMetaData::page_encoding_stats_mask` · parquet 59.3.0

```rust
fn page_encoding_stats_mask(&self) -> Option<&EncodingMask>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1124`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the page encoding statistics reduced to a bitmask, or `None` if statistics are
not available (or they were left in their original form).

Note: This is the default behavior for this crate.

The [`PageEncodingStats`](../operations/parquet.file.metadata.PageEncodingStats.md#op-3206550f7543aea052c85566) struct was added to the Parquet specification specifically to
enable fast determination of whether all pages in a column chunk are dictionary encoded
(see <https://github.com/apache/parquet-format/pull/16>).
Decoding the full page encoding statistics, however, can be very costly, and is not
necessary to support the aforementioned use case. As an alternative, this crate can
instead distill the list of `PageEncodingStats` down to a bitmask of just the encodings
used for data pages
(see [`ParquetMetaDataOptions::set_encoding_stats_as_mask`](../operations/parquet.file.metadata.options.ParquetMetaDataOptions.md#op-2a5f6446514f2b744ce7034e)).
To test for an all-dictionary-encoded chunk one could use this bitmask in the following way:

```rust
use parquet::basic::Encoding;
use parquet::file::metadata::ColumnChunkMetaData;
// test if all data pages in the column chunk are dictionary encoded
fn is_all_dictionary_encoded(col_meta: &ColumnChunkMetaData) -> bool {
    // check that dictionary encoding was used
    col_meta.dictionary_page_offset().is_some()
        && col_meta.page_encoding_stats_mask().is_some_and(|mask| {
            // mask should only have one bit set, either for PLAIN_DICTIONARY or
            // RLE_DICTIONARY
            mask.is_only(Encoding::PLAIN_DICTIONARY) || mask.is_only(Encoding::RLE_DICTIONARY)
        })
}
```

<a id="op-6701b644cc8ab756d7ce401c"></a>
## repetition_level_histogram

`function` · `parquet::file::metadata::ColumnChunkMetaData::repetition_level_histogram` · parquet 59.3.0

```rust
fn repetition_level_histogram(&self) -> Option<&LevelHistogram>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1188`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the repetition level histogram.

The returned value `vec[i]` is how many values are at repetition level `i`. For example,
`vec[0]` indicates how many rows the page contains.
This field may not be set by older writers.

<a id="op-5250de48a177949d250b917f"></a>
## statistics

`function` · `parquet::file::metadata::ColumnChunkMetaData::statistics` · parquet 59.3.0

```rust
fn statistics(&self) -> Option<&Statistics>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1072`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns statistics that are set for this column chunk,
or `None` if no statistics are available.

<a id="op-b5b10541580ffa4c069874af"></a>
## uncompressed_size

`function` · `parquet::file::metadata::ColumnChunkMetaData::uncompressed_size` · parquet 59.3.0

```rust
fn uncompressed_size(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1037`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the total uncompressed data size of this column chunk.

<a id="op-89d6be46480c01f0a72cb863"></a>
## unencoded_byte_array_data_bytes

`function` · `parquet::file::metadata::ColumnChunkMetaData::unencoded_byte_array_data_bytes` · parquet 59.3.0

```rust
fn unencoded_byte_array_data_bytes(&self) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [1211, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1179`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the number of bytes of variable length data after decoding.

Only set for BYTE_ARRAY columns. This field may not be set by older
writers.
