# `parquet::file::properties::WriterPropertiesBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.properties.WriterPropertiesBuilder.json).

<a id="op-bff56448e2488ad16504b642"></a>
## WriterPropertiesBuilder

`struct` · `parquet::file::properties::WriterPropertiesBuilder` · parquet 59.3.0

```rust
struct WriterPropertiesBuilder
```

Source: `src/file/properties.rs:590`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Builder for  [`WriterProperties`](../operations/parquet.file.properties.WriterProperties.md#op-1a8b0462c7a132d1d5004af2) Parquet writer configuration.

See example on [`WriterProperties`](../operations/parquet.file.properties.WriterProperties.md#op-1a8b0462c7a132d1d5004af2)

<a id="op-f363f4f056ce3fd31c3b63f0"></a>
## build

`function` · `parquet::file::properties::WriterPropertiesBuilder::build` · parquet 59.3.0

```rust
fn build(self) -> WriterProperties
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:641`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Finalizes the configuration and returns immutable writer properties struct.

<a id="op-2c74c4b78a7ab0684069f165"></a>
## clone

`function` · `parquet::file::properties::WriterPropertiesBuilder::clone` · parquet 59.3.0

```rust
fn clone(&self) -> WriterPropertiesBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [589, 17], "end": [589, 22], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/properties.rs:589`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24a6847dd6c37078968ee03f"></a>
## default

`function` · `parquet::file::properties::WriterPropertiesBuilder::default` · parquet 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 1], "end": [637, 2], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file/properties.rs:614`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns default state of the builder.

<a id="op-c31e1667aebbec6135d2c99d"></a>
## fmt

`function` · `parquet::file::properties::WriterPropertiesBuilder::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [589, 10], "end": [589, 15], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/properties.rs:589`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73930d9d7d01181caa0e444d"></a>
## from

`function` · `parquet::file::properties::WriterPropertiesBuilder::from` · parquet 59.3.0

```rust
fn from(props: WriterProperties) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1328, 1], "end": [1355, 2], "filename": "src/file/properties.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterProperties", "path": "WriterProperties"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/file/properties.rs:1329`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-714cffbe6a6fe9449d8d585e"></a>
## set_bloom_filter_enabled

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_bloom_filter_enabled` · parquet 59.3.0

```rust
fn set_bloom_filter_enabled(self, value: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1128`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets if bloom filter should be written for all columns (defaults to `false`).

# Notes

* If the bloom filter is enabled previously then it is a no-op.

* If the bloom filter is not enabled, default values for ndv and fpp
  value are used used. See [`set_bloom_filter_max_ndv`] and
  [`set_bloom_filter_fpp`] to further adjust the ndv and fpp.

[`set_bloom_filter_max_ndv`]: Self::set_bloom_filter_max_ndv
[`set_bloom_filter_fpp`]: Self::set_bloom_filter_fpp

<a id="op-3ca60c9b3704cc6b94959616"></a>
## set_bloom_filter_fpp

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_bloom_filter_fpp` · parquet 59.3.0

```rust
fn set_bloom_filter_fpp(self, value: f64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1141`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the default target bloom filter false positive probability (fpp)
for all columns (defaults to `0.05` via [`DEFAULT_BLOOM_FILTER_FPP`](../operations/parquet.file.properties.DEFAULT_BLOOM_FILTER_FPP.md#op-6a638a0205a9f6e75faeba63)).

Implicitly enables bloom writing, as if [`set_bloom_filter_enabled`] had
been called.

[`set_bloom_filter_enabled`]: Self::set_bloom_filter_enabled

<a id="op-57fa972053a9b4d5a03eefc5"></a>
## set_bloom_filter_max_ndv

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_bloom_filter_max_ndv` · parquet 59.3.0

```rust
fn set_bloom_filter_max_ndv(self, value: u64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1158`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets default maximum expected number of distinct values (ndv) for bloom filter
for all columns (defaults to [`DEFAULT_BLOOM_FILTER_NDV`](../operations/parquet.file.properties.DEFAULT_BLOOM_FILTER_NDV.md#op-9c8e92e4f942656d39a72c51)).

The bloom filter is initially sized for this many distinct values at the
configured FPP, then folded down after all values are inserted to achieve
optimal size. A good heuristic is to set this to the expected number of rows
in the row group.

Implicitly enables bloom writing, as if [`set_bloom_filter_enabled`] had
been called.

[`set_bloom_filter_enabled`]: Self::set_bloom_filter_enabled

<a id="op-08269a553f60e0a6840a0a08"></a>
## set_bloom_filter_ndv

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_bloom_filter_ndv` · parquet 59.3.0

```rust
fn set_bloom_filter_ndv(self, value: u64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1165`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Deprecated alias for [`Self::set_bloom_filter_max_ndv`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-57fa972053a9b4d5a03eefc5).

<a id="op-430970c80eb184c86d619b25"></a>
## set_bloom_filter_position

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_bloom_filter_position` · parquet 59.3.0

```rust
fn set_bloom_filter_position(self, value: BloomFilterPosition) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:781`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets where in the final file Bloom Filters are written (defaults to  [`AfterRowGroup`]
via [`DEFAULT_BLOOM_FILTER_POSITION`](../operations/parquet.file.properties.DEFAULT_BLOOM_FILTER_POSITION.md#op-d9fc3f6af5869f30ce5aa9d0))

[`AfterRowGroup`]: BloomFilterPosition::AfterRowGroup

<a id="op-40320e1a8f7ce5abe6ac7814"></a>
## set_bloom_filter_properties

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_bloom_filter_properties` · parquet 59.3.0

```rust
fn set_bloom_filter_properties(self, value: BloomFilterProperties) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1309`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the [`BloomFilterProperties`](../operations/parquet.file.properties.BloomFilterProperties.md#op-d8c6090fa6f2cfe781d4070c) for all columns, implicitly enabling
the bloom filter.

Both `fpp` and `ndv` from `value` are treated as explicit and will not
be overridden by the build-time row-group-size NDV fallback. For
dynamic NDV sizing (resolved to `max_row_group_row_count` at build
time), use [`Self::set_bloom_filter_enabled`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-714cffbe6a6fe9449d8d585e) or
[`Self::set_bloom_filter_fpp`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-3ca60c9b3704cc6b94959616) instead.

<a id="op-63b4c369a8027e5344df6a8e"></a>
## set_coerce_types

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_coerce_types` · parquet 59.3.0

```rust
fn set_coerce_types(self, coerce_types: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:898`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Should the writer coerce types to parquet native types (defaults to `false` via
[`DEFAULT_COERCE_TYPES`](../operations/parquet.file.properties.DEFAULT_COERCE_TYPES.md#op-65eedafa8c4d3753a26c5ffc)).

Leaving this option the default `false` will ensure the exact same data
written to parquet using this library will be read.

Setting this option to `true` will result in parquet files that can be
read by more readers, but potentially lose information in the process.

* Types such as [`DataType::Date64`], which have no direct corresponding
  Parquet type, may be stored with lower precision.

* The internal field names of `List` and `Map` types will be renamed if
  necessary to match what is required by the newest Parquet specification.

See [`ArrowToParquetSchemaConverter::with_coerce_types`] for more details

[`DataType::Date64`]: arrow_schema::DataType::Date64
[`ArrowToParquetSchemaConverter::with_coerce_types`]: crate::arrow::ArrowSchemaConverter::with_coerce_types

Unresolved upstream links (retained, not inferred): `crate::arrow::ArrowSchemaConverter::with_coerce_types`.

<a id="op-21e35abb9e041a7b25067c56"></a>
## set_column_bloom_filter_enabled

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_column_bloom_filter_enabled` · parquet 59.3.0

```rust
fn set_column_bloom_filter_enabled(self, col: ColumnPath, value: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1254`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets whether a bloom filter should be written for a specific column.

Takes precedence over [`Self::set_bloom_filter_enabled`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-714cffbe6a6fe9449d8d585e).

<a id="op-c1b3219e46f04ecb633f48d4"></a>
## set_column_bloom_filter_fpp

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_column_bloom_filter_fpp` · parquet 59.3.0

```rust
fn set_column_bloom_filter_fpp(self, col: ColumnPath, value: f64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1262`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the false positive probability for bloom filter for a specific column.

Takes precedence over [`Self::set_bloom_filter_fpp`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-3ca60c9b3704cc6b94959616).

<a id="op-23e425f8136d525313b1e884"></a>
## set_column_bloom_filter_max_ndv

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_column_bloom_filter_max_ndv` · parquet 59.3.0

```rust
fn set_column_bloom_filter_max_ndv(self, col: ColumnPath, value: u64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1271`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the maximum expected number of distinct values for bloom filter for
a specific column.

Takes precedence over [`Self::set_bloom_filter_max_ndv`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-57fa972053a9b4d5a03eefc5).

<a id="op-e00a8f03a9a15d49f5e0f540"></a>
## set_column_bloom_filter_ndv

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_column_bloom_filter_ndv` · parquet 59.3.0

```rust
fn set_column_bloom_filter_ndv(self, col: ColumnPath, value: u64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1297`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Deprecated alias for [`Self::set_column_bloom_filter_max_ndv`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-23e425f8136d525313b1e884).

<a id="op-d56019b674820458c9c67410"></a>
## set_column_bloom_filter_properties

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_column_bloom_filter_properties` · parquet 59.3.0

```rust
fn set_column_bloom_filter_properties(self, col: ColumnPath, value: BloomFilterProperties) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1318`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the [`BloomFilterProperties`](../operations/parquet.file.properties.BloomFilterProperties.md#op-d8c6090fa6f2cfe781d4070c) for a specific column.

Takes precedence over [`Self::set_bloom_filter_properties`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-40320e1a8f7ce5abe6ac7814).

<a id="op-da0a1634c6a08f7e0b304abd"></a>
## set_column_compression

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_column_compression` · parquet 59.3.0

```rust
fn set_column_compression(self, col: ColumnPath, value: Compression) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1198`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets compression codec for a specific column.

Takes precedence over [`Self::set_compression`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-cb6c633a00d64c4bfbe98de8).

<a id="op-b61948937af335516ea183a6"></a>
## set_column_data_page_size_limit

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_column_data_page_size_limit` · parquet 59.3.0

```rust
fn set_column_data_page_size_limit(self, col: ColumnPath, value: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1223`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets data page size limit for a specific column.

Takes precedence over [`Self::set_data_page_size_limit`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-490588989259edf39d0dca79).

<a id="op-0b0d17d1b9372ca87a18a62d"></a>
## set_column_data_page_v2_compression_ratio_threshold

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_column_data_page_v2_compression_ratio_threshold` · parquet 59.3.0

```rust
fn set_column_data_page_v2_compression_ratio_threshold(self, col: ColumnPath, value: f64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1282`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the Data Page v2 compression ratio threshold for a specific column.

Takes precedence over [`Self::set_data_page_v2_compression_ratio_threshold`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-7d9b1a934c69bd8ce949063c).

# Panics
If `value` is not finite or is not strictly positive.

<a id="op-5fd42abf356587829cefe77e"></a>
## set_column_dictionary_enabled

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_column_dictionary_enabled` · parquet 59.3.0

```rust
fn set_column_dictionary_enabled(self, col: ColumnPath, value: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1206`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets flag to enable/disable dictionary encoding for a specific column.

Takes precedence over [`Self::set_dictionary_enabled`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-56401508cb567ab5decef901).

<a id="op-fb106e2483d89c56f5133deb"></a>
## set_column_dictionary_page_size_limit

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_column_dictionary_page_size_limit` · parquet 59.3.0

```rust
fn set_column_dictionary_page_size_limit(self, col: ColumnPath, value: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1214`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets dictionary page size limit for a specific column.

Takes precedence over [`Self::set_dictionary_page_size_limit`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-2856ca2e66cdde8896b9c50a).

<a id="op-de4fbb48bbecea03359b921d"></a>
## set_column_encoding

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_column_encoding` · parquet 59.3.0

```rust
fn set_column_encoding(self, col: ColumnPath, value: Encoding) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1190`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets encoding for a specific column.

Takes precedence over [`Self::set_encoding`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-7108f9b226b6bb1284068212).

If dictionary is not enabled, this is treated as a primary encoding for this
column. In case when dictionary is enabled for this column, either through
global defaults or explicitly, this value is considered to be a fallback
encoding for this column.

# Panics
If user tries to set dictionary encoding here, regardless of dictionary
encoding flag being set.

<a id="op-c0724eb2567cbd868212af4f"></a>
## set_column_index_truncate_length

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_column_index_truncate_length` · parquet 59.3.0

```rust
fn set_column_index_truncate_length(self, max_length: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:838`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the max length of min/max value fields when writing the column
[`Index`] (defaults to `Some(64)` via [`DEFAULT_COLUMN_INDEX_TRUNCATE_LENGTH`](../operations/parquet.file.properties.DEFAULT_COLUMN_INDEX_TRUNCATE_LENGTH.md#op-97ec8bd40c7dee54fbf01053)).

This can be used to prevent columns with very long values (hundreds of
bytes long) from causing the parquet metadata to become huge.

# Notes

The column [`Index`] is written when [`Self::set_statistics_enabled`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-65f020db802f7d7d62769b46) is
set to [`EnabledStatistics::Page`](../operations/parquet.file.properties.EnabledStatistics.md#op-918e760bc6c733e101ef05af).

* If `Some`, must be greater than 0, otherwise will panic
* If `None`, there's no effective limit.

[`Index`]: crate::file::page_index::column_index::ColumnIndexMetaData

<a id="op-ae2cbf337c470ce18ec922de"></a>
## set_column_statistics_enabled

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_column_statistics_enabled` · parquet 59.3.0

```rust
fn set_column_statistics_enabled(self, col: ColumnPath, value: EnabledStatistics) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1231`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets [`EnabledStatistics`](../operations/parquet.file.properties.EnabledStatistics.md#op-0bdf156eacebacc657f23a92) level for a specific column.

Takes precedence over [`Self::set_statistics_enabled`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-65f020db802f7d7d62769b46).

<a id="op-623db96657b0e8abd9aaeaab"></a>
## set_column_write_page_header_statistics

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_column_write_page_header_statistics` · parquet 59.3.0

```rust
fn set_column_write_page_header_statistics(self, col: ColumnPath, value: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1245`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets whether to write [`Statistics`] in the page header for a specific column.

Takes precedence over [`Self::set_write_page_header_statistics`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-d83a04ff4005399fb7744eab).

[`Statistics`]: crate::file::statistics::Statistics

<a id="op-cb6c633a00d64c4bfbe98de8"></a>
## set_compression

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_compression` · parquet 59.3.0

```rust
fn set_compression(self, value: Compression) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1032`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets default compression codec for all columns (default to [`UNCOMPRESSED`] via
[`DEFAULT_COMPRESSION`](../operations/parquet.file.properties.DEFAULT_COMPRESSION.md#op-ade39c24893565b57b1ea993)).

[`UNCOMPRESSED`]: Compression::UNCOMPRESSED

<a id="op-7645b076cb242e9294c7add7"></a>
## set_content_defined_chunking

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_content_defined_chunking` · parquet 59.3.0

```rust
fn set_content_defined_chunking(self, options: Option<CdcOptions>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:954`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

EXPERIMENTAL: Sets content-defined chunking options, or disables CDC with `None`.

When enabled, data page boundaries are determined by a rolling hash of the
column values, so unchanged data produces identical byte sequences across
file versions. This enables efficient deduplication on content-addressable
storage systems.

Only supported through the Arrow writer interface ([`ArrowWriter`]).

# Panics

Panics if `min_chunk_size == 0` or `max_chunk_size <= min_chunk_size`.

[`ArrowWriter`]: crate::arrow::arrow_writer::ArrowWriter

<a id="op-12d6a903816c7fdd78669b1e"></a>
## set_created_by

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_created_by` · parquet 59.3.0

```rust
fn set_created_by(self, value: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:790`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets "created by" property (defaults to `parquet-rs version <VERSION>` via
[`DEFAULT_CREATED_BY`](../operations/parquet.file.properties.DEFAULT_CREATED_BY.md#op-269e64dea90567dfeca5112a)).

This is a string that will be written into the file metadata

<a id="op-9176054306ba78aab8d67d3b"></a>
## set_data_page_row_count_limit

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_data_page_row_count_limit` · parquet 59.3.0

```rust
fn set_data_page_row_count_limit(self, value: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:717`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets best effort maximum number of rows in a data page (defaults to `20_000`
via [`DEFAULT_DATA_PAGE_ROW_COUNT_LIMIT`](../operations/parquet.file.properties.DEFAULT_DATA_PAGE_ROW_COUNT_LIMIT.md#op-17b948924318ba9cfd93bc68)).

The parquet writer will attempt to limit the number of rows in
each `DataPage` to this value. Reducing this value will result
in larger parquet files, but may improve the effectiveness of
page index based predicate pushdown during reading.

Note: this is a best effort limit based on value of
[`set_write_batch_size`](Self::set_write_batch_size).

<a id="op-490588989259edf39d0dca79"></a>
## set_data_page_size_limit

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_data_page_size_limit` · parquet 59.3.0

```rust
fn set_data_page_size_limit(self, value: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1074`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets best effort maximum size of a data page in bytes (defaults to `1024 * 1024`
via [`DEFAULT_PAGE_SIZE`](../operations/parquet.file.properties.DEFAULT_PAGE_SIZE.md#op-ed16f4888101f11748bcb620)).

The parquet writer will attempt to limit the sizes of each
`DataPage` to this many bytes. Reducing this value will result
in larger parquet files, but may improve the effectiveness of
page index based predicate pushdown during reading.

Note: this is a best effort limit based on value of
[`set_write_batch_size`](Self::set_write_batch_size).

<a id="op-7d9b1a934c69bd8ce949063c"></a>
## set_data_page_v2_compression_ratio_threshold

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_data_page_v2_compression_ratio_threshold` · parquet 59.3.0

```rust
fn set_data_page_v2_compression_ratio_threshold(self, value: f64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:994`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the default compression ratio threshold at or above which a Data Page
v2's compressed values are discarded in favor of writing the values
uncompressed, for all columns (defaults to `1.0` via
[`DEFAULT_DATA_PAGE_V2_COMPRESSION_RATIO_THRESHOLD`](../operations/parquet.file.properties.DEFAULT_DATA_PAGE_V2_COMPRESSION_RATIO_THRESHOLD.md#op-0ecc3e169394b1bd4cf4b1c7)).

When writing a Data Page v2 with a configured compression codec, the writer
first compresses the values and then compares the compressed size to the
uncompressed size. If `compressed_size >= uncompressed_size * threshold`, the
compressed buffer is discarded and the values are written uncompressed for
that page (the page's `is_compressed` flag is set to `false`).

The default of `1.0` preserves the historical behavior of only keeping
compression when it strictly reduces the size. Setting a value below `1.0`
requires a minimum amount of size reduction to keep the compressed page —
for example `0.9` requires at least a 10% reduction. Setting a value above
`1.0` keeps the compressed buffer even if it's somewhat larger than the
uncompressed values.

This setting only affects Data Page v2; Data Page v1 always stores the
compressor's output regardless of the resulting size.

# Panics
If `value` is not finite or is not strictly positive.

<a id="op-56401508cb567ab5decef901"></a>
## set_dictionary_enabled

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_dictionary_enabled` · parquet 59.3.0

```rust
fn set_dictionary_enabled(self, value: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1042`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets default flag to enable/disable dictionary encoding for all columns (defaults to `true`
via [`DEFAULT_DICTIONARY_ENABLED`](../operations/parquet.file.properties.DEFAULT_DICTIONARY_ENABLED.md#op-a5117cd27b3febcd2a705cc1)).

Use this method to set dictionary encoding, instead of explicitly specifying
encoding in `set_encoding` method.

<a id="op-2856ca2e66cdde8896b9c50a"></a>
## set_dictionary_page_size_limit

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_dictionary_page_size_limit` · parquet 59.3.0

```rust
fn set_dictionary_page_size_limit(self, value: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1058`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets best effort maximum dictionary page size, in bytes (defaults to `1024 * 1024`
via [`DEFAULT_DICTIONARY_PAGE_SIZE_LIMIT`](../operations/parquet.file.properties.DEFAULT_DICTIONARY_PAGE_SIZE_LIMIT.md#op-a014a92f4418db70c185a267)).

The parquet writer will attempt to limit the size of each
`DataPage` used to store dictionaries to this many
bytes. Reducing this value will result in larger parquet
files, but may improve the effectiveness of page index based
predicate pushdown during reading.

Note: this is a best effort limit based on value of
[`set_write_batch_size`](Self::set_write_batch_size).

<a id="op-7108f9b226b6bb1284068212"></a>
## set_encoding

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_encoding` · parquet 59.3.0

```rust
fn set_encoding(self, value: Encoding) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1023`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets default encoding for all columns.

If dictionary is not enabled, this is treated as a primary encoding for all
columns. In case when dictionary is enabled for any column, this value is
considered to be a fallback encoding for that column.

# Panics

if dictionary encoding is specified, regardless of dictionary
encoding flag being set.

<a id="op-92857223a57417f03df74df4"></a>
## set_key_value_metadata

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_key_value_metadata` · parquet 59.3.0

```rust
fn set_key_value_metadata(self, value: Option<Vec<KeyValue>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:812`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets "key_value_metadata" property (defaults to `None`).

<a id="op-e52254a054f8facad4f6b7a0"></a>
## set_max_row_group_bytes

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_max_row_group_bytes` · parquet 59.3.0

```rust
fn set_max_row_group_bytes(self, value: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:771`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets maximum size of a row group in bytes, or `None` for unlimited.

Row groups are flushed when their estimated encoded size exceeds this threshold.
This is similar to the official Java implementation for `parquet.block.size`'s behavior.

If both `max_row_group_row_count` and `max_row_group_bytes` are set,
the row group with the smaller limit will be produced.

# Panics
If the value is `Some(0)`.

<a id="op-a565c50d4ca38f5adc42342c"></a>
## set_max_row_group_row_count

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_max_row_group_row_count` · parquet 59.3.0

```rust
fn set_max_row_group_row_count(self, value: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:755`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets maximum number of rows in a row group, or `None` for unlimited.

If both `max_row_group_row_count` and `max_row_group_bytes` are set,
the row group with the smaller limit will be produced.

# Panics
If the value is `Some(0)`.

<a id="op-eccd50d4af68241acc2a7fe3"></a>
## set_max_row_group_size

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_max_row_group_size` · parquet 59.3.0

```rust
fn set_max_row_group_size(self, value: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:742`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets maximum number of rows in a row group (defaults to `1024 * 1024`
via [`DEFAULT_MAX_ROW_GROUP_ROW_COUNT`](../operations/parquet.file.properties.DEFAULT_MAX_ROW_GROUP_ROW_COUNT.md#op-685fa8e2035d1285fcca86eb)).

# Panics
If the value is set to 0.

<a id="op-c1a591a8fd5d82670e247042"></a>
## set_offset_index_disabled

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_offset_index_disabled` · parquet 59.3.0

```rust
fn set_offset_index_disabled(self, value: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:806`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets whether the writing of offset indexes is disabled (defaults to `false` via
[`DEFAULT_OFFSET_INDEX_DISABLED`](../operations/parquet.file.properties.DEFAULT_OFFSET_INDEX_DISABLED.md#op-efaafa43c6d35c9751bda4d1)).

If statistics level is set to [`Page`] this setting will be overridden with `false`.

Note: As the offset indexes are useful for accessing data by row number,
they are always written by default, regardless of whether other statistics
are enabled. Disabling this metadata may result in a degradation in read
performance, so use this option with care.

[`Page`]: EnabledStatistics::Page

<a id="op-846d4a67bc1309d88fe40347"></a>
## set_sorting_columns

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_sorting_columns` · parquet 59.3.0

```rust
fn set_sorting_columns(self, value: Option<Vec<SortingColumn>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:818`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets sorting order of rows in the row group if any (defaults to `None`).

<a id="op-65f020db802f7d7d62769b46"></a>
## set_statistics_enabled

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_statistics_enabled` · parquet 59.3.0

```rust
fn set_statistics_enabled(self, value: EnabledStatistics) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1084`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets default [`EnabledStatistics`](../operations/parquet.file.properties.EnabledStatistics.md#op-0bdf156eacebacc657f23a92) level for all columns (defaults to [`Page`] via
[`DEFAULT_STATISTICS_ENABLED`](../operations/parquet.file.properties.DEFAULT_STATISTICS_ENABLED.md#op-2fd28940b699eaded88b325b)).

[`Page`]: EnabledStatistics::Page

<a id="op-9ed34fa0eb0d2ccd829ec9ac"></a>
## set_statistics_truncate_length

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_statistics_truncate_length` · parquet 59.3.0

```rust
fn set_statistics_truncate_length(self, max_length: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:867`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the max length of min/max value fields in row group and data page header
[`Statistics`] (defaults to `Some(64)` via [`DEFAULT_STATISTICS_TRUNCATE_LENGTH`](../operations/parquet.file.properties.DEFAULT_STATISTICS_TRUNCATE_LENGTH.md#op-1370bd4e5c3deacd56d3179a)).

# Notes
Row group [`Statistics`] are written when [`Self::set_statistics_enabled`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-65f020db802f7d7d62769b46) is
set to [`EnabledStatistics::Chunk`](../operations/parquet.file.properties.EnabledStatistics.md#op-de078fa8f540ac9d7d3949a2) or [`EnabledStatistics::Page`](../operations/parquet.file.properties.EnabledStatistics.md#op-918e760bc6c733e101ef05af). Data page header
[`Statistics`] are written when [`Self::set_statistics_enabled`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-65f020db802f7d7d62769b46) is set to
[`EnabledStatistics::Page`](../operations/parquet.file.properties.EnabledStatistics.md#op-918e760bc6c733e101ef05af).

* If `Some`, must be greater than 0, otherwise will panic
* If `None`, there's no effective limit.

# See also
Truncation of Page Index statistics is controlled separately via
[`WriterPropertiesBuilder::set_column_index_truncate_length`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-c0724eb2567cbd868212af4f)

[`Statistics`]: crate::file::statistics::Statistics

<a id="op-93d333cdc60d997ed60a3681"></a>
## set_write_batch_size

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_write_batch_size` · parquet 59.3.0

```rust
fn set_write_batch_size(self, value: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:731`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets write batch size (defaults to 1024 via [`DEFAULT_WRITE_BATCH_SIZE`](../operations/parquet.file.properties.DEFAULT_WRITE_BATCH_SIZE.md#op-dba84d9f1b3a685060b154e2)).

For performance reasons, data for each column is written in
batches of this size.

Additional limits such as such as
[`set_data_page_row_count_limit`](Self::set_data_page_row_count_limit)
are checked between batches, and thus the write batch size value acts as an
upper-bound on the enforcement granularity of other limits.

<a id="op-d83a04ff4005399fb7744eab"></a>
## set_write_page_header_statistics

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_write_page_header_statistics` · parquet 59.3.0

```rust
fn set_write_page_header_statistics(self, value: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1110`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

enable/disable writing [`Statistics`] in the page header
(defaults to `false` via [`DEFAULT_WRITE_PAGE_HEADER_STATISTICS`](../operations/parquet.file.properties.DEFAULT_WRITE_PAGE_HEADER_STATISTICS.md#op-2bc9c0067054a19009ea0012)).

Only applicable if [`Page`] level statistics are gathered.

Setting this value to `true` can greatly increase the size of the resulting Parquet
file while yielding very little added benefit. Most modern Parquet implementations
will use the min/max values stored in the [`ParquetColumnIndex`] rather than
those in the page header.

# Note

Prior to version 56.0.0, the `parquet` crate always wrote these
statistics (the equivalent of setting this option to `true`). This was
changed in 56.0.0 to follow the recommendation in the Parquet
specification. See [issue #7580] for more details.

[`Statistics`]: crate::file::statistics::Statistics
[`ParquetColumnIndex`]: crate::file::metadata::ParquetColumnIndex
[`Page`]: EnabledStatistics::Page
[issue #7580]: https://github.com/apache/arrow-rs/issues/7580

<a id="op-cc00350bab402b7a6791c571"></a>
## set_write_path_in_schema

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_write_path_in_schema` · parquet 59.3.0

```rust
fn set_write_path_in_schema(self, write_path_in_schema: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:935`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

EXPERIMENTAL: Should the writer emit the `path_in_schema` element of the
`ColumnMetaData` Thrift struct. Defaults to `true` via [`DEFAULT_WRITE_PATH_IN_SCHEMA`](../operations/parquet.file.properties.DEFAULT_WRITE_PATH_IN_SCHEMA.md#op-888d0df60062c33f79c39910).

Because `path_in_schema` is a field on the `ColumnMetaData`, it is repeated
`num_columns * num_rowgroups` times. Compounding this is any level of nesting or
repetition in the schema. For instance, a top-level list column named `foo` will have
a `path_in_schema` of `["foo", "list", "element"]`. A list-of-struct is even worse,
because the necessary list wrapping is repeated for each element of the struct. A
file with a deeply nested schema and many row groups can have a large percentage of the
footer taken up by this field. For example, a file of 38 row groups with a schema containing
several lists of structs containing lists had 36% of the footer taken up by `path_in_schema`.
Removing this redundant information can greatly speed up footer parsing, which is particularly
important in scenarios where one does not wish to read the entire file (e.g. point
lookups).

<div class="warning">

**WARNING:**
Setting this to `false` will break compatibility with Parquet readers that
still expect this field to be present. Virtually all Parquet readers (parquet-java,
Spark, arrow-cpp, pyarrow, pandas to name a few), with the exception
of the one in this crate, expect this field to be present, and will terminate execution
if it is not. This will continue to be the case unless/until the Parquet format
specification is explicitly changed to allow this field to be missing. As a consquence,
users should only set this to `false` if they have verified that any reader(s) they plan
to use can tolerate the absence of this field.

For more context, see [GH-563].

</div>

[GH-563]: https://github.com/apache/parquet-format/issues/563

<a id="op-b1f0c60a3b828540a8e253f6"></a>
## set_writer_version

`function` · `parquet::file::properties::WriterPropertiesBuilder::set_writer_version` · parquet 59.3.0

```rust
fn set_writer_version(self, value: WriterVersion) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:702`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the `WriterVersion` written into the parquet metadata (defaults to [`PARQUET_1_0`]
via [`DEFAULT_WRITER_VERSION`](../operations/parquet.file.properties.DEFAULT_WRITER_VERSION.md#op-ab186c67d56860b55e2825ae))

This value can determine what features some readers will support.

[`PARQUET_1_0`]: [WriterVersion::PARQUET_1_0]

<a id="op-0d20fdc86ca0ff1f833633ca"></a>
## with_file_encryption_properties

`function` · `parquet::file::properties::WriterPropertiesBuilder::with_file_encryption_properties` · parquet 59.3.0

```rust
fn with_file_encryption_properties(self, file_encryption_properties: Arc<FileEncryptionProperties>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterPropertiesBuilder", "path": "WriterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [1326, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1002`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets FileEncryptionProperties (defaults to `None`)
