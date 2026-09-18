# `parquet::file::metadata::ColumnChunkMetaDataBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.ColumnChunkMetaDataBuilder.json).

<a id="op-b5bee7e1fe4e5a85ce43c3f4"></a>
## ColumnChunkMetaDataBuilder

`struct` · `parquet::file::metadata::ColumnChunkMetaDataBuilder` · parquet 59.3.0

```rust
struct ColumnChunkMetaDataBuilder
```

Source: `src/file/metadata/mod.rs:1231`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Builder for [`ColumnChunkMetaData`](../operations/parquet.file.metadata.ColumnChunkMetaData.md#op-664b3834ca25da410177f90f)

This builder is used to create a new column chunk metadata or modify an
existing one.

# Example
```no_run
# use parquet::file::metadata::{ColumnChunkMetaData, ColumnChunkMetaDataBuilder};
# fn get_column_chunk_metadata() -> ColumnChunkMetaData { unimplemented!(); }
let column_chunk_metadata = get_column_chunk_metadata();
// create a new builder from existing column chunk metadata
let builder = ColumnChunkMetaDataBuilder::from(column_chunk_metadata);
// clear the statistics:
let column_chunk_metadata: ColumnChunkMetaData = builder
  .clear_statistics()
  .build()
  .unwrap();
```

<a id="op-d7a98bd8b4e126ee7e665454"></a>
## build

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::build` · parquet 59.3.0

```rust
fn build(self) -> Result<ColumnChunkMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1446`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Builds column chunk metadata.

<a id="op-ff0545a8955d0167f3fc6717"></a>
## clear_page_encoding_stats

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::clear_page_encoding_stats` · parquet 59.3.0

```rust
fn clear_page_encoding_stats(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1372`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Clears the page encoding stats for this column chunk.

<a id="op-bebd6cf2d0e396794a085429"></a>
## clear_statistics

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::clear_statistics` · parquet 59.3.0

```rust
fn clear_statistics(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1350`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Clears the statistics for this column chunk.

<a id="op-77ecc1c86465eb9705b15651"></a>
## from

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::from` · parquet 59.3.0

```rust
fn from(value: ColumnChunkMetaData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1619, 1], "end": [1623, 2], "filename": "src/file/metadata/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaData", "path": "ColumnChunkMetaData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/file/metadata/mod.rs:1620`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0358d66f5aafe9244cd14a1"></a>
## set_bloom_filter_length

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_bloom_filter_length` · parquet 59.3.0

```rust
fn set_bloom_filter_length(self, value: Option<i32>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1384`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets optional bloom filter length in bytes.

<a id="op-f6b0cbab061f476f5c7cd8fa"></a>
## set_bloom_filter_offset

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_bloom_filter_offset` · parquet 59.3.0

```rust
fn set_bloom_filter_offset(self, value: Option<i64>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1378`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets optional bloom filter offset in bytes.

<a id="op-729e09a7909058035abed051"></a>
## set_column_crypto_metadata

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_column_crypto_metadata` · parquet 59.3.0

```rust
fn set_column_crypto_metadata(self, value: Option<ColumnCryptoMetaData>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1433`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set the encryption metadata for an encrypted column

<a id="op-9e4a9b4cccbcb71077c22441"></a>
## set_column_index_length

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_column_index_length` · parquet 59.3.0

```rust
fn set_column_index_length(self, value: Option<i32>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1408`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets optional column index length in bytes.

<a id="op-d321b541ede2401aec59c8bb"></a>
## set_column_index_offset

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_column_index_offset` · parquet 59.3.0

```rust
fn set_column_index_offset(self, value: Option<i64>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1402`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets optional column index offset in bytes.

<a id="op-10b83553e661f04f2526f8d8"></a>
## set_compression

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_compression` · parquet 59.3.0

```rust
fn set_compression(self, value: Compression) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1296`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets compression codec given a [`Compression`](../operations/parquet.basic.Compression.md#op-c5a5132a156c388f8a10fcad) configuration value.

<a id="op-b6c8fd28281723f72e2403ca"></a>
## set_compression_codec

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_compression_codec` · parquet 59.3.0

```rust
fn set_compression_codec(self, value: CompressionCodec) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1302`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets compression codec.

<a id="op-885e00887e8876bd99579b55"></a>
## set_data_page_offset

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_data_page_offset` · parquet 59.3.0

```rust
fn set_data_page_offset(self, value: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1320`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets data page offset in bytes.

<a id="op-4858ae7f3e5134970e27e776"></a>
## set_definition_level_histogram

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_definition_level_histogram` · parquet 59.3.0

```rust
fn set_definition_level_histogram(self, value: Option<LevelHistogram>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1426`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets optional repetition level histogram

<a id="op-4e25c9d287ad47760b8eaf59"></a>
## set_dictionary_page_offset

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_dictionary_page_offset` · parquet 59.3.0

```rust
fn set_dictionary_page_offset(self, value: Option<i64>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1326`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets optional dictionary page offset in bytes.

<a id="op-bf1d4b807919ec91da2e36db"></a>
## set_encodings

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_encodings` · parquet 59.3.0

```rust
fn set_encodings(self, encodings: Vec<Encoding>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1272`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets list of encodings for this column chunk.

<a id="op-839e699877a3666adc51b8d3"></a>
## set_encodings_mask

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_encodings_mask` · parquet 59.3.0

```rust
fn set_encodings_mask(self, encodings: EncodingMask) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1278`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the encodings mask for this column chunk.

<a id="op-96bb685abb6400f2f18d4553"></a>
## set_encrypted_column_metadata

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_encrypted_column_metadata` · parquet 59.3.0

```rust
fn set_encrypted_column_metadata(self, value: Option<Vec<u8>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1440`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set the encryption metadata for an encrypted column

<a id="op-814b2ab1c6ef5680f38d5ec5"></a>
## set_file_path

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_file_path` · parquet 59.3.0

```rust
fn set_file_path(self, value: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1284`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets optional file path for this column chunk.

<a id="op-facf7768ee3af4ab249b0153"></a>
## set_geo_statistics

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_geo_statistics` · parquet 59.3.0

```rust
fn set_geo_statistics(self, value: Box<geo_statistics::GeospatialStatistics>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1344`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets geospatial statistics for this column chunk.

<a id="op-05ad5af2381fd39221863915"></a>
## set_index_page_offset

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_index_page_offset` · parquet 59.3.0

```rust
fn set_index_page_offset(self, value: Option<i64>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1332`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets optional index page offset in bytes.

<a id="op-53bcdd4762c685ab0c8ad6c8"></a>
## set_num_values

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_num_values` · parquet 59.3.0

```rust
fn set_num_values(self, value: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1290`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets number of values.

<a id="op-fed75bdb096b27f063b3ace9"></a>
## set_offset_index_length

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_offset_index_length` · parquet 59.3.0

```rust
fn set_offset_index_length(self, value: Option<i32>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1396`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets optional offset index length in bytes.

<a id="op-e9a3af83e5ad565405c243c2"></a>
## set_offset_index_offset

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_offset_index_offset` · parquet 59.3.0

```rust
fn set_offset_index_offset(self, value: Option<i64>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1390`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets optional offset index offset in bytes.

<a id="op-0e3c82261f5f6dedf1e66e54"></a>
## set_page_encoding_stats

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_page_encoding_stats` · parquet 59.3.0

```rust
fn set_page_encoding_stats(self, value: Vec<PageEncodingStats>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1358`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets page encoding stats for this column chunk.

This will overwrite any existing stats, either `Vec` based or bitmask.

<a id="op-af3d795ca4dc5626983c0a94"></a>
## set_page_encoding_stats_mask

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_page_encoding_stats_mask` · parquet 59.3.0

```rust
fn set_page_encoding_stats_mask(self, value: EncodingMask) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1366`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets page encoding stats mask for this column chunk.

This will overwrite any existing stats, either `Vec` based or bitmask.

<a id="op-fbb9417c91cb332f4f80f157"></a>
## set_repetition_level_histogram

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_repetition_level_histogram` · parquet 59.3.0

```rust
fn set_repetition_level_histogram(self, value: Option<LevelHistogram>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1420`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets optional repetition level histogram

<a id="op-a187ac6fbeb8ddfe4a406baf"></a>
## set_statistics

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_statistics` · parquet 59.3.0

```rust
fn set_statistics(self, value: Statistics) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1338`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets statistics for this column chunk.

<a id="op-d7a06b41bcf33f9150fc69b8"></a>
## set_total_compressed_size

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_total_compressed_size` · parquet 59.3.0

```rust
fn set_total_compressed_size(self, value: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1308`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets total compressed size in bytes.

<a id="op-80e05690d8f40c9d4d111984"></a>
## set_total_uncompressed_size

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_total_uncompressed_size` · parquet 59.3.0

```rust
fn set_total_uncompressed_size(self, value: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1314`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets total uncompressed size in bytes.

<a id="op-30f761ea72febfcead61570f"></a>
## set_unencoded_byte_array_data_bytes

`function` · `parquet::file::metadata::ColumnChunkMetaDataBuilder::set_unencoded_byte_array_data_bytes` · parquet 59.3.0

```rust
fn set_unencoded_byte_array_data_bytes(self, value: Option<i64>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::ColumnChunkMetaDataBuilder", "path": "ColumnChunkMetaDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1233, 1], "end": [1449, 2], "filename": "src/file/metadata/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/mod.rs:1414`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets optional length of variable length data in bytes.
