# `datafusion_datasource_parquet::metadata::DFParquetMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.metadata.DFParquetMetadata.json).

<a id="op-6a48587f65e3f810753b80a2"></a>
## DFParquetMetadata

`struct` · `datafusion_datasource_parquet::metadata::DFParquetMetadata` · datafusion-datasource-parquet 55.1.0

```rust
struct DFParquetMetadata<'a>
```

Source: `src/metadata.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Handles fetching Parquet file schema, metadata and statistics
from object store.

This component is exposed for low level integrations through
[`ParquetFileReaderFactory`].

[`ParquetFileReaderFactory`]: crate::ParquetFileReaderFactory

<a id="op-eaaa0ed35b4d421f1cc2a13f"></a>
## coerce_int96

`struct_field` · `datafusion_datasource_parquet::metadata::DFParquetMetadata::coerce_int96` · datafusion-datasource-parquet 55.1.0

```rust
coerce_int96: Option<arrow::datatypes::TimeUnit>
```

Source: `src/metadata.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

timeunit to coerce INT96 timestamps to

<a id="op-e259d71b6e855bc9550d88a1"></a>
## coerce_int96_tz

`struct_field` · `datafusion_datasource_parquet::metadata::DFParquetMetadata::coerce_int96_tz` · datafusion-datasource-parquet 55.1.0

```rust
coerce_int96_tz: Option<std::sync::Arc<str>>
```

Source: `src/metadata.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Optional timezone applied to INT96-coerced timestamps.

<a id="op-bd8af0811fa491114afd4542"></a>
## fetch_metadata

`function` · `datafusion_datasource_parquet::metadata::DFParquetMetadata::fetch_metadata` · datafusion-datasource-parquet 55.1.0

```rust
async fn fetch_metadata(&self) -> Result<Arc<ParquetMetaData>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource_parquet::metadata::DFParquetMetadata", "path": "DFParquetMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [573, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:188`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Fetch the [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) for this file.

Consults the [`FileMetadataCache`](../operations/datafusion_execution.cache.cache_manager.FileMetadataCache.md#op-eae1fe1b1bd732ee9949f01d) first when one is configured and
falls back to reading from the object store via
[`ParquetMetaDataPushDecoder`](../operations/parquet.file.metadata.push_decoder.ParquetMetaDataPushDecoder.md#op-e00d43ab5b4fd446b342b085) on a cache miss.

<a id="op-3d880e6adf5868b9f23b3206"></a>
## fetch_schema

`function` · `datafusion_datasource_parquet::metadata::DFParquetMetadata::fetch_schema` · datafusion-datasource-parquet 55.1.0

```rust
async fn fetch_schema(&self) -> Result<Schema>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource_parquet::metadata::DFParquetMetadata", "path": "DFParquetMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [573, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Fetch this file's [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) and convert its embedded Thrift
schema into an Arrow [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050).

<a id="op-7fe956abd3f670b3588de359"></a>
## fetch_statistics

`function` · `datafusion_datasource_parquet::metadata::DFParquetMetadata::fetch_statistics` · datafusion-datasource-parquet 55.1.0

```rust
async fn fetch_statistics(&self, table_schema: &SchemaRef) -> Result<Statistics>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource_parquet::metadata::DFParquetMetadata", "path": "DFParquetMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [573, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:400`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Fetch the metadata from the Parquet file via [`Self::fetch_metadata`](../operations/datafusion_datasource_parquet.metadata.DFParquetMetadata.md#op-bd8af0811fa491114afd4542) and convert
the statistics in the metadata using [`Self::statistics_from_parquet_metadata`](../operations/datafusion_datasource_parquet.metadata.DFParquetMetadata.md#op-faffd1a309e1904f4cc714b1)

<a id="op-c830d2e30cdca72523f5a4bc"></a>
## fmt

`function` · `datafusion_datasource_parquet::metadata::DFParquetMetadata::fmt` · datafusion-datasource-parquet 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource_parquet::metadata::DFParquetMetadata", "path": "DFParquetMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 10], "end": [67, 15], "filename": "src/metadata.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metadata.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d544087fa1307199e75b480"></a>
## new

`function` · `datafusion_datasource_parquet::metadata::DFParquetMetadata::new` · datafusion-datasource-parquet 55.1.0

```rust
fn new(store: &'a dyn ObjectStore, object_meta: &'a ObjectMeta) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource_parquet::metadata::DFParquetMetadata", "path": "DFParquetMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [573, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Create a new `DFParquetMetadata` for the given file.

Use the `with_*` builder methods to customize behavior
before calling [`Self::fetch_metadata`](../operations/datafusion_datasource_parquet.metadata.DFParquetMetadata.md#op-bd8af0811fa491114afd4542) or [`Self::fetch_schema`](../operations/datafusion_datasource_parquet.metadata.DFParquetMetadata.md#op-3d880e6adf5868b9f23b3206).

<a id="op-faffd1a309e1904f4cc714b1"></a>
## statistics_from_parquet_metadata

`function` · `datafusion_datasource_parquet::metadata::DFParquetMetadata::statistics_from_parquet_metadata` · datafusion-datasource-parquet 55.1.0

```rust
fn statistics_from_parquet_metadata(metadata: &ParquetMetaData, logical_file_schema: &SchemaRef) -> Result<Statistics>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource_parquet::metadata::DFParquetMetadata", "path": "DFParquetMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [573, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:442`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Convert statistics in [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) into [`Statistics`](../operations/datafusion_common.stats.Statistics.md#op-06d2580fbefe2068a74aafb6) using [`StatisticsConverter`](../operations/parquet.arrow.arrow_reader.statistics.StatisticsConverter.md#op-c25ef84c92a9151936db232f)

The statistics are calculated for each column in the table schema
using the row group statistics in the parquet metadata.

# Key behaviors:

1. Extracts row counts and byte sizes from all row groups
2. Applies schema type coercions to align file schema with table schema
3. Collects and aggregates statistics across row groups when available

# When there are no statistics:

If the Parquet file doesn't contain any statistics (has_statistics is false), the function returns a Statistics object with:
- Exact row count
- Exact byte size
- All column statistics marked as unknown via Statistics::unknown_column(&table_schema)
- Column byte sizes are still calculated and recorded

# When only some columns have statistics:

For columns with statistics:
- Min/max values are properly extracted and represented as Precision::Exact
- Null counts are calculated by summing across row groups
- Byte sizes are calculated and recorded

For columns without statistics,
- For min/max, there are two situations:
    1. The column isn't in arrow schema, then min/max values are set to Precision::Absent
    2. The column is in arrow schema, but not in parquet schema due to schema revolution, min/max values are set to Precision::Exact(null)
- Null counts are set to Precision::Exact(num_rows) (conservatively assuming all values could be null)

# Byte Size Calculation:

- For primitive types with known fixed size, exact byte size is calculated as (byte width * number of rows)
- For other types, uncompressed Parquet size is used as an estimate for in-memory size
- If neither method is applicable, byte size is marked as Precision::Absent

<a id="op-462db1d09648ad9f36831a8c"></a>
## with_coerce_int96

`function` · `datafusion_datasource_parquet::metadata::DFParquetMetadata::with_coerce_int96` · datafusion-datasource-parquet 55.1.0

```rust
fn with_coerce_int96(self, time_unit: Option<TimeUnit>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource_parquet::metadata::DFParquetMetadata", "path": "DFParquetMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [573, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Set the [`TimeUnit`](../operations/arrow_schema.datatype.TimeUnit.md#op-4e6bd6e27e392e178d8a5b2e) that INT96 timestamp columns should be coerced
to when reading the schema.

INT96 in Parquet has no defined unit or timezone, so leaving this
`None` reads INT96 columns as nanosecond timestamps with no timezone
— DataFusion's default behavior.

<a id="op-476551ad0442d96494b972c1"></a>
## with_coerce_int96_tz

`function` · `datafusion_datasource_parquet::metadata::DFParquetMetadata::with_coerce_int96_tz` · datafusion-datasource-parquet 55.1.0

```rust
fn with_coerce_int96_tz(self, timezone: Option<Arc<str>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource_parquet::metadata::DFParquetMetadata", "path": "DFParquetMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [573, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Set the optional timezone applied to INT96-coerced timestamps.

Only used when [`Self::with_coerce_int96`](../operations/datafusion_datasource_parquet.metadata.DFParquetMetadata.md#op-462db1d09648ad9f36831a8c) has also been set, and
otherwise has no effect.

<a id="op-50030efed123e4e4581d641e"></a>
## with_decryption_properties

`function` · `datafusion_datasource_parquet::metadata::DFParquetMetadata::with_decryption_properties` · datafusion-datasource-parquet 55.1.0

```rust
fn with_decryption_properties(self, decryption_properties: Option<Arc<FileDecryptionProperties>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource_parquet::metadata::DFParquetMetadata", "path": "DFParquetMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [573, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Set the decryption properties used to read an encrypted Parquet file,
equivalent to [`ParquetMetaDataReader::with_decryption_properties`].

Only needed when the target file was written with Parquet Modular
Encryption.

Unresolved upstream links (retained, not inferred): ``ParquetMetaDataReader::with_decryption_properties``.

<a id="op-b8a52c992258925aff3a5c77"></a>
## with_file_metadata_cache

`function` · `datafusion_datasource_parquet::metadata::DFParquetMetadata::with_file_metadata_cache` · datafusion-datasource-parquet 55.1.0

```rust
fn with_file_metadata_cache(self, file_metadata_cache: Option<Arc<FileMetadataCache>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource_parquet::metadata::DFParquetMetadata", "path": "DFParquetMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [573, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Set an optional [`FileMetadataCache`](../operations/datafusion_execution.cache.cache_manager.FileMetadataCache.md#op-eae1fe1b1bd732ee9949f01d) used to avoid re-fetching
[`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) for files that have already been read.

<a id="op-c01c81fd7e994053361abf00"></a>
## with_metadata_size_hint

`function` · `datafusion_datasource_parquet::metadata::DFParquetMetadata::with_metadata_size_hint` · datafusion-datasource-parquet 55.1.0

```rust
fn with_metadata_size_hint(self, metadata_size_hint: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource_parquet::metadata::DFParquetMetadata", "path": "DFParquetMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [573, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Set a hint for the number of trailing bytes to prefetch from the end
of the file, equivalent to
[`ParquetMetaDataReader::with_prefetch_hint`].

Providing a good estimate of the footer (and, if requested, page index)
size can save an extra I/O round trip when fetching metadata from the
store.

Unresolved upstream links (retained, not inferred): ``ParquetMetaDataReader::with_prefetch_hint``.

<a id="op-797e74d4f099009f805ceb97"></a>
## with_page_index_policy

`function` · `datafusion_datasource_parquet::metadata::DFParquetMetadata::with_page_index_policy` · datafusion-datasource-parquet 55.1.0

```rust
fn with_page_index_policy(self, page_index_policy: Option<PageIndexPolicy>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource_parquet::metadata::DFParquetMetadata", "path": "DFParquetMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [573, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Sets the policy for loading parquet page index structures (column and
offset indexes), equivalent to
[`ParquetMetaDataReader::with_page_index_policy`].

Passing `None` uses a default automatically, based on whether a metadata
cache is configured.

Unresolved upstream links (retained, not inferred): ``ParquetMetaDataReader::with_page_index_policy``.
