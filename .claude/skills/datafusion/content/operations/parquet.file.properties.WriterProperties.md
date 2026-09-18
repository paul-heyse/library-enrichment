# `parquet::file::properties::WriterProperties`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.properties.WriterProperties.json).

<a id="op-1a8b0462c7a132d1d5004af2"></a>
## WriterProperties

`struct` · `parquet::file::properties::WriterProperties` · parquet 59.3.0

```rust
struct WriterProperties
```

Source: `src/file/properties.rs:241`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Configuration settings for writing parquet files.

Use [`Self::builder`](../operations/parquet.file.properties.WriterProperties.md#op-a7053a49c5186d414ab405c2) to create a [`WriterPropertiesBuilder`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-bff56448e2488ad16504b642) to change settings.

# Example

```rust
# use parquet::{
#    basic::{Compression, Encoding},
#    file::properties::*,
#    schema::types::ColumnPath,
# };
#
// Create properties with default configuration.
let props = WriterProperties::default();

// Use properties builder to set certain options and assemble the configuration.
let props = WriterProperties::builder()
    .set_writer_version(WriterVersion::PARQUET_1_0)
    .set_encoding(Encoding::PLAIN)
    .set_column_encoding(ColumnPath::from("col1"), Encoding::DELTA_BINARY_PACKED)
    .set_compression(Compression::SNAPPY)
    .build();

assert_eq!(props.writer_version(), WriterVersion::PARQUET_1_0);
assert_eq!(
    props.encoding(&ColumnPath::from("col1")),
    Some(Encoding::DELTA_BINARY_PACKED)
);
assert_eq!(
    props.encoding(&ColumnPath::from("col2")),
    Some(Encoding::PLAIN)
);
```

<a id="op-17411813b13f020c9fa60913"></a>
## bloom_filter_position

`function` · `parquet::file::properties::WriterProperties::bloom_filter_position` · parquet 59.3.0

```rust
fn bloom_filter_position(&self) -> BloomFilterPosition
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:378`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns bloom filter position.

For more details see [`WriterPropertiesBuilder::set_bloom_filter_position`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-430970c80eb184c86d619b25)

<a id="op-fb17c5ffd7369e1ff38b4d8e"></a>
## bloom_filter_properties

`function` · `parquet::file::properties::WriterProperties::bloom_filter_properties` · parquet 59.3.0

```rust
fn bloom_filter_properties(&self, col: &ColumnPath) -> Option<&BloomFilterProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:570`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the [`BloomFilterProperties`](../operations/parquet.file.properties.BloomFilterProperties.md#op-d8c6090fa6f2cfe781d4070c) for the given column

Returns `None` if bloom filter is disabled

For more details see [`WriterPropertiesBuilder::set_column_bloom_filter_enabled`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-21e35abb9e041a7b25067c56)

<a id="op-a7053a49c5186d414ab405c2"></a>
## builder

`function` · `parquet::file::properties::WriterProperties::builder` · parquet 59.3.0

```rust
fn builder() -> WriterPropertiesBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:279`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a new default [`WriterPropertiesBuilder`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-bff56448e2488ad16504b642) for creating writer
properties.

<a id="op-a463fedad56c0bbc0e6ef21d"></a>
## clone

`function` · `parquet::file::properties::WriterProperties::clone` · parquet 59.3.0

```rust
fn clone(&self) -> WriterProperties
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 17], "end": [240, 22], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/properties.rs:240`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a49031df6239f8d8ac55f7e"></a>
## coerce_types

`function` · `parquet::file::properties::WriterProperties::coerce_types` · parquet 59.3.0

```rust
fn coerce_types(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:440`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns `true` if type coercion is enabled.

For more details see [`WriterPropertiesBuilder::set_coerce_types`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-63b4c369a8027e5344df6a8e)

<a id="op-6267e11e3a660199e79b1228"></a>
## column_data_page_size_limit

`function` · `parquet::file::properties::WriterProperties::column_data_page_size_limit` · parquet 59.3.0

```rust
fn column_data_page_size_limit(&self, col: &ColumnPath) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:305`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns data page size limit for a specific column.

Takes precedence over [`Self::data_page_size_limit`](../operations/parquet.file.properties.WriterProperties.md#op-632467579b7740c34186c49c).

Note: this is a best effort limit based on the write batch size.

<a id="op-0fe9779fdf942d91ec83141a"></a>
## column_data_page_v2_compression_ratio_threshold

`function` · `parquet::file::properties::WriterProperties::column_data_page_v2_compression_ratio_threshold` · parquet 59.3.0

```rust
fn column_data_page_v2_compression_ratio_threshold(&self, col: &ColumnPath) -> f64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:472`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the Data Page v2 compression ratio threshold for a specific column.

Takes precedence over [`Self::data_page_v2_compression_ratio_threshold`](../operations/parquet.file.properties.WriterProperties.md#op-4f4d0807b7f0390d1a1e3d34).

<a id="op-60cbbf9be20b0b0d1925a2c6"></a>
## column_dictionary_page_size_limit

`function` · `parquet::file::properties::WriterProperties::column_dictionary_page_size_limit` · parquet 59.3.0

```rust
fn column_dictionary_page_size_limit(&self, col: &ColumnPath) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:325`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns dictionary page size limit for a specific column.

<a id="op-9a1ff525940244d1064fbcd1"></a>
## column_index_truncate_length

`function` · `parquet::file::properties::WriterProperties::column_index_truncate_length` · parquet 59.3.0

```rust
fn column_index_truncate_length(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:422`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the maximum length of truncated min/max values in the column index.

`None` if truncation is disabled, must be greater than 0 otherwise.

For more details see [`WriterPropertiesBuilder::set_column_index_truncate_length`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-c0724eb2567cbd868212af4f)

<a id="op-566d2ac3369e59f2e4717dd1"></a>
## compression

`function` · `parquet::file::properties::WriterProperties::compression` · parquet 59.3.0

```rust
fn compression(&self, col: &ColumnPath) -> Compression
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:519`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns compression codec for a column.

For more details see [`WriterPropertiesBuilder::set_column_compression`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-da0a1634c6a08f7e0b304abd)

<a id="op-b610f5c10fcc98c4a8c18d2f"></a>
## content_defined_chunking

`function` · `parquet::file::properties::WriterProperties::content_defined_chunking` · parquet 59.3.0

```rust
fn content_defined_chunking(&self) -> Option<&CdcOptions>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:455`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

EXPERIMENTAL: Returns content-defined chunking options, or `None` if CDC is disabled.

For more details see [`WriterPropertiesBuilder::set_content_defined_chunking`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-7645b076cb242e9294c7add7)

<a id="op-cad64d848a164fef999eda92"></a>
## created_by

`function` · `parquet::file::properties::WriterProperties::created_by` · parquet 59.3.0

```rust
fn created_by(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:392`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns `created_by` string.

For more details see [`WriterPropertiesBuilder::set_created_by`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-12d6a903816c7fdd78669b1e)

<a id="op-403790435ef3dc9afa3b37fc"></a>
## data_page_row_count_limit

`function` · `parquet::file::properties::WriterProperties::data_page_row_count_limit` · parquet 59.3.0

```rust
fn data_page_row_count_limit(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:338`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the maximum page row count

Note: this is a best effort limit based on the write batch size

For more details see [`WriterPropertiesBuilder::set_data_page_row_count_limit`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-9176054306ba78aab8d67d3b)

<a id="op-632467579b7740c34186c49c"></a>
## data_page_size_limit

`function` · `parquet::file::properties::WriterProperties::data_page_size_limit` · parquet 59.3.0

```rust
fn data_page_size_limit(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:294`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns data page size limit.

Note: this is a best effort limit based on the write batch size

For more details see [`WriterPropertiesBuilder::set_data_page_size_limit`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-490588989259edf39d0dca79)

<a id="op-4f4d0807b7f0390d1a1e3d34"></a>
## data_page_v2_compression_ratio_threshold

`function` · `parquet::file::properties::WriterProperties::data_page_v2_compression_ratio_threshold` · parquet 59.3.0

```rust
fn data_page_v2_compression_ratio_threshold(&self) -> f64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:463`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the compression ratio threshold at or above which a Data Page v2's
compressed values are discarded in favor of writing the values uncompressed.

For more details see [`WriterPropertiesBuilder::set_data_page_v2_compression_ratio_threshold`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-7d9b1a934c69bd8ce949063c)

<a id="op-566ffbbd7cfbfd0d22abc02a"></a>
## default

`function` · `parquet::file::properties::WriterProperties::default` · parquet 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [263, 1], "end": [267, 2], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file/properties.rs:264`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-827ac650ea73e15b0e024552"></a>
## dictionary_data_page_encoding

`function` · `parquet::file::properties::WriterProperties::dictionary_data_page_encoding` · parquet 59.3.0

```rust
fn dictionary_data_page_encoding(&self) -> Encoding
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:487`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns encoding for a data page, when dictionary encoding is enabled.

This is not configurable.

<a id="op-510331825cd85ac085217df8"></a>
## dictionary_enabled

`function` · `parquet::file::properties::WriterProperties::dictionary_enabled` · parquet 59.3.0

```rust
fn dictionary_enabled(&self, col: &ColumnPath) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:530`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns `true` if dictionary encoding is enabled for a column.

For more details see [`WriterPropertiesBuilder::set_dictionary_enabled`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-56401508cb567ab5decef901)

<a id="op-41f4f25ebef0fd1bf731617c"></a>
## dictionary_page_encoding

`function` · `parquet::file::properties::WriterProperties::dictionary_page_encoding` · parquet 59.3.0

```rust
fn dictionary_page_encoding(&self) -> Encoding
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:497`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns encoding for dictionary page, when dictionary encoding is enabled.

This is not configurable.

<a id="op-d445e9d8beb1d03591566769"></a>
## dictionary_page_size_limit

`function` · `parquet::file::properties::WriterProperties::dictionary_page_size_limit` · parquet 59.3.0

```rust
fn dictionary_page_size_limit(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:318`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns dictionary page size limit.

Note: this is a best effort limit based on the write batch size

For more details see [`WriterPropertiesBuilder::set_dictionary_page_size_limit`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-2856ca2e66cdde8896b9c50a)

<a id="op-dfc69e14b69f9b4d95df5323"></a>
## encoding

`function` · `parquet::file::properties::WriterProperties::encoding` · parquet 59.3.0

```rust
fn encoding(&self, col: &ColumnPath) -> Option<Encoding>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:509`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns encoding for a column, if set.

In case when dictionary is enabled, returns fallback encoding.

If encoding is not set, then column writer will choose the best encoding
based on the column type.

<a id="op-c9339dfc072df6ff904dcf2b"></a>
## file_encryption_properties

`function` · `parquet::file::properties::WriterProperties::file_encryption_properties` · parquet 59.3.0

```rust
fn file_encryption_properties(&self) -> Option<&Arc<FileEncryptionProperties>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:581`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return file encryption properties

For more details see [`WriterPropertiesBuilder::with_file_encryption_properties`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-0d20fdc86ca0ff1f833633ca)

<a id="op-ccd364510fd5a7371a2f85f4"></a>
## fmt

`function` · `parquet::file::properties::WriterProperties::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 10], "end": [240, 15], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/properties.rs:240`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d3777ff2b44af24f74734f1"></a>
## into_builder

`function` · `parquet::file::properties::WriterProperties::into_builder` · parquet 59.3.0

```rust
fn into_builder(self) -> WriterPropertiesBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:285`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts this [`WriterProperties`](../operations/parquet.file.properties.WriterProperties.md#op-1a8b0462c7a132d1d5004af2) into a [`WriterPropertiesBuilder`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-bff56448e2488ad16504b642)
Used for mutating existing property settings

<a id="op-609f26fad83746ae5ca94aff"></a>
## key_value_metadata

`function` · `parquet::file::properties::WriterProperties::key_value_metadata` · parquet 59.3.0

```rust
fn key_value_metadata(&self) -> Option<&Vec<KeyValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:406`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns `key_value_metadata` KeyValue pairs.

For more details see [`WriterPropertiesBuilder::set_key_value_metadata`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-92857223a57417f03df74df4)

<a id="op-48d7f3ca1609a19a044b460d"></a>
## max_row_group_bytes

`function` · `parquet::file::properties::WriterProperties::max_row_group_bytes` · parquet 59.3.0

```rust
fn max_row_group_bytes(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:371`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns maximum size of a row group in bytes, or `None` if unlimited.

For more details see [`WriterPropertiesBuilder::set_max_row_group_bytes`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-e52254a054f8facad4f6b7a0)

<a id="op-59c83d225f83bcf8d0c3dbd2"></a>
## max_row_group_row_count

`function` · `parquet::file::properties::WriterProperties::max_row_group_row_count` · parquet 59.3.0

```rust
fn max_row_group_row_count(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:364`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns maximum number of rows in a row group, or `None` if unlimited.

For more details see [`WriterPropertiesBuilder::set_max_row_group_row_count`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-a565c50d4ca38f5adc42342c)

<a id="op-6f0521c5661fab93d80f5df9"></a>
## max_row_group_size

`function` · `parquet::file::properties::WriterProperties::max_row_group_size` · parquet 59.3.0

```rust
fn max_row_group_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:357`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns maximum number of rows in a row group, or `usize::MAX` if unlimited.

For more details see [`WriterPropertiesBuilder::set_max_row_group_size`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-eccd50d4af68241acc2a7fe3)

<a id="op-87df78ca75f9677a15fec66c"></a>
## new

`function` · `parquet::file::properties::WriterProperties::new` · parquet 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:273`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [`WriterProperties`](../operations/parquet.file.properties.WriterProperties.md#op-1a8b0462c7a132d1d5004af2) with the default settings

See [`WriterProperties::builder`](../operations/parquet.file.properties.WriterProperties.md#op-a7053a49c5186d414ab405c2) for customising settings

<a id="op-4b020df33f13c36322861ebc"></a>
## offset_index_disabled

`function` · `parquet::file::properties::WriterProperties::offset_index_disabled` · parquet 59.3.0

```rust
fn offset_index_disabled(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:399`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns `true` if offset index writing is disabled.

For more details see [`WriterPropertiesBuilder::set_offset_index_disabled`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-c1a591a8fd5d82670e247042)

<a id="op-8402e0235561433f9361a548"></a>
## sorting_columns

`function` · `parquet::file::properties::WriterProperties::sorting_columns` · parquet 59.3.0

```rust
fn sorting_columns(&self) -> Option<&Vec<SortingColumn>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:413`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns sorting columns.

For more details see [`WriterPropertiesBuilder::set_sorting_columns`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-846d4a67bc1309d88fe40347)

<a id="op-895c1c05fbde2ced95030c15"></a>
## statistics_enabled

`function` · `parquet::file::properties::WriterProperties::statistics_enabled` · parquet 59.3.0

```rust
fn statistics_enabled(&self, col: &ColumnPath) -> EnabledStatistics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:541`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns which statistics are written for a column.

For more details see [`WriterPropertiesBuilder::set_statistics_enabled`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-65f020db802f7d7d62769b46)

<a id="op-3377350e4db650f1b03ae4cc"></a>
## statistics_truncate_length

`function` · `parquet::file::properties::WriterProperties::statistics_truncate_length` · parquet 59.3.0

```rust
fn statistics_truncate_length(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:433`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the maximum length of truncated min/max values in [`Statistics`].

`None` if truncation is disabled, must be greater than 0 otherwise.

For more details see [`WriterPropertiesBuilder::set_statistics_truncate_length`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-9ed34fa0eb0d2ccd829ec9ac)

[`Statistics`]: crate::file::statistics::Statistics

<a id="op-9de12ec1be117038b2a59c80"></a>
## write_batch_size

`function` · `parquet::file::properties::WriterProperties::write_batch_size` · parquet 59.3.0

```rust
fn write_batch_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:349`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns configured batch size for writes.

When writing a batch of data, this setting allows to split it internally into
smaller batches so we can better estimate the size of a page currently being
written.

For more details see [`WriterPropertiesBuilder::set_write_batch_size`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-93d333cdc60d997ed60a3681)

<a id="op-47aa164868ddac92e5442290"></a>
## write_page_header_statistics

`function` · `parquet::file::properties::WriterProperties::write_page_header_statistics` · parquet 59.3.0

```rust
fn write_page_header_statistics(&self, col: &ColumnPath) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:554`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns `true` if [`Statistics`] are to be written to the page header for a column.

For more details see [`WriterPropertiesBuilder::set_write_page_header_statistics`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-d83a04ff4005399fb7744eab)

[`Statistics`]: crate::file::statistics::Statistics

<a id="op-7c9aa7725326ac520f718064"></a>
## write_path_in_schema

`function` · `parquet::file::properties::WriterProperties::write_path_in_schema` · parquet 59.3.0

```rust
fn write_path_in_schema(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:448`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns `true` if the `path_in_schema` field of the `ColumnMetaData` Thrift struct
should be written.

For more details see [`WriterPropertiesBuilder::set_write_path_in_schema`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-cc00350bab402b7a6791c571)

<a id="op-225aaf6ba79250b3e6fcf4f5"></a>
## writer_version

`function` · `parquet::file::properties::WriterProperties::writer_version` · parquet 59.3.0

```rust
fn writer_version(&self) -> WriterVersion
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [584, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:385`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns configured writer version.

For more details see [`WriterPropertiesBuilder::set_writer_version`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-b1f0c60a3b828540a8e253f6)
