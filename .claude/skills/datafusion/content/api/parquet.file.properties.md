# `parquet::file::properties`

Crate `parquet` · 35 public items · structured records in [`model/parquet.file.properties.json`](../model/parquet.file.properties.json)

## DEFAULT_BLOOM_FILTER_FPP

`constant` · `parquet::file::properties::DEFAULT_BLOOM_FILTER_FPP`

```rust
const DEFAULT_BLOOM_FILTER_FPP: f64 = 0.05
```

Default value for [`BloomFilterProperties::fpp()`]

---

## DEFAULT_BLOOM_FILTER_NDV

`constant` · `parquet::file::properties::DEFAULT_BLOOM_FILTER_NDV`

```rust
const DEFAULT_BLOOM_FILTER_NDV: u64 = _
```

Default value for [`BloomFilterProperties::ndv()`].

Note: this is only the fallback default used when constructing [`BloomFilterProperties`]
directly. When using [`WriterPropertiesBuilder`], columns with bloom filters enabled
but without an explicit NDV will have their NDV resolved at build time to
[`WriterProperties::max_row_group_row_count`], which may differ from this constant
if the user configured a custom row group size.

---

## DEFAULT_BLOOM_FILTER_POSITION

`constant` · `parquet::file::properties::DEFAULT_BLOOM_FILTER_POSITION`

```rust
const DEFAULT_BLOOM_FILTER_POSITION: BloomFilterPosition = BloomFilterPosition::AfterRowGroup
```

Default value for [`WriterProperties::bloom_filter_position`]

---

## DEFAULT_CDC_MAX_CHUNK_SIZE

`constant` · `parquet::file::properties::DEFAULT_CDC_MAX_CHUNK_SIZE`

```rust
const DEFAULT_CDC_MAX_CHUNK_SIZE: usize = _
```

Default maximum chunk size for content-defined chunking: 1024 KiB.

---

## DEFAULT_CDC_MIN_CHUNK_SIZE

`constant` · `parquet::file::properties::DEFAULT_CDC_MIN_CHUNK_SIZE`

```rust
const DEFAULT_CDC_MIN_CHUNK_SIZE: usize = _
```

Default minimum chunk size for content-defined chunking: 256 KiB.

---

## DEFAULT_CDC_NORM_LEVEL

`constant` · `parquet::file::properties::DEFAULT_CDC_NORM_LEVEL`

```rust
const DEFAULT_CDC_NORM_LEVEL: i32 = 0
```

Default normalization level for content-defined chunking.

---

## DEFAULT_COERCE_TYPES

`constant` · `parquet::file::properties::DEFAULT_COERCE_TYPES`

```rust
const DEFAULT_COERCE_TYPES: bool = false
```

Default values for [`WriterProperties::coerce_types`]

---

## DEFAULT_COLUMN_INDEX_TRUNCATE_LENGTH

`constant` · `parquet::file::properties::DEFAULT_COLUMN_INDEX_TRUNCATE_LENGTH`

```rust
const DEFAULT_COLUMN_INDEX_TRUNCATE_LENGTH: Option<usize> = _
```

Default value for [`WriterProperties::column_index_truncate_length`]

---

## DEFAULT_COMPRESSION

`constant` · `parquet::file::properties::DEFAULT_COMPRESSION`

```rust
const DEFAULT_COMPRESSION: basic::Compression = Compression::UNCOMPRESSED
```

Default value for [`WriterProperties::compression`]

---

## DEFAULT_CREATED_BY

`constant` · `parquet::file::properties::DEFAULT_CREATED_BY`

```rust
const DEFAULT_CREATED_BY: &str = "parquet-rs version 59.3.0"
```

Default value for [`WriterProperties::created_by`]

---

## DEFAULT_DATA_PAGE_ROW_COUNT_LIMIT

`constant` · `parquet::file::properties::DEFAULT_DATA_PAGE_ROW_COUNT_LIMIT`

```rust
const DEFAULT_DATA_PAGE_ROW_COUNT_LIMIT: usize = 20_000
```

Default value for [`WriterProperties::data_page_row_count_limit`]

---

## DEFAULT_DATA_PAGE_V2_COMPRESSION_RATIO_THRESHOLD

`constant` · `parquet::file::properties::DEFAULT_DATA_PAGE_V2_COMPRESSION_RATIO_THRESHOLD`

```rust
const DEFAULT_DATA_PAGE_V2_COMPRESSION_RATIO_THRESHOLD: f64 = 1.0
```

Default value for [`WriterProperties::data_page_v2_compression_ratio_threshold`]

---

## DEFAULT_DICTIONARY_ENABLED

`constant` · `parquet::file::properties::DEFAULT_DICTIONARY_ENABLED`

```rust
const DEFAULT_DICTIONARY_ENABLED: bool = true
```

Default value for [`WriterProperties::dictionary_enabled`]

---

## DEFAULT_DICTIONARY_PAGE_SIZE_LIMIT

`constant` · `parquet::file::properties::DEFAULT_DICTIONARY_PAGE_SIZE_LIMIT`

```rust
const DEFAULT_DICTIONARY_PAGE_SIZE_LIMIT: usize = DEFAULT_PAGE_SIZE
```

Default value for [`WriterProperties::dictionary_page_size_limit`]

---

## DEFAULT_MAX_ROW_GROUP_ROW_COUNT

`constant` · `parquet::file::properties::DEFAULT_MAX_ROW_GROUP_ROW_COUNT`

```rust
const DEFAULT_MAX_ROW_GROUP_ROW_COUNT: usize = _
```

Default value for [`WriterProperties::max_row_group_row_count`]

---

## DEFAULT_OFFSET_INDEX_DISABLED

`constant` · `parquet::file::properties::DEFAULT_OFFSET_INDEX_DISABLED`

```rust
const DEFAULT_OFFSET_INDEX_DISABLED: bool = false
```

Default value for [`WriterProperties::offset_index_disabled`]

---

## DEFAULT_PAGE_SIZE

`constant` · `parquet::file::properties::DEFAULT_PAGE_SIZE`

```rust
const DEFAULT_PAGE_SIZE: usize = _
```

Default value for [`WriterProperties::data_page_size_limit`]

---

## DEFAULT_STATISTICS_ENABLED

`constant` · `parquet::file::properties::DEFAULT_STATISTICS_ENABLED`

```rust
const DEFAULT_STATISTICS_ENABLED: EnabledStatistics = EnabledStatistics::Page
```

Default value for [`WriterProperties::statistics_enabled`]

---

## DEFAULT_STATISTICS_TRUNCATE_LENGTH

`constant` · `parquet::file::properties::DEFAULT_STATISTICS_TRUNCATE_LENGTH`

```rust
const DEFAULT_STATISTICS_TRUNCATE_LENGTH: Option<usize> = _
```

Default values for [`WriterProperties::statistics_truncate_length`]

---

## DEFAULT_WRITER_VERSION

`constant` · `parquet::file::properties::DEFAULT_WRITER_VERSION`

```rust
const DEFAULT_WRITER_VERSION: WriterVersion = WriterVersion::PARQUET_1_0
```

Default value for [`WriterProperties::writer_version`]

---

## DEFAULT_WRITE_BATCH_SIZE

`constant` · `parquet::file::properties::DEFAULT_WRITE_BATCH_SIZE`

```rust
const DEFAULT_WRITE_BATCH_SIZE: usize = 1024
```

Default value for [`WriterProperties::write_batch_size`]

---

## DEFAULT_WRITE_PAGE_HEADER_STATISTICS

`constant` · `parquet::file::properties::DEFAULT_WRITE_PAGE_HEADER_STATISTICS`

```rust
const DEFAULT_WRITE_PAGE_HEADER_STATISTICS: bool = false
```

Default value for [`WriterProperties::write_page_header_statistics`]

---

## DEFAULT_WRITE_PATH_IN_SCHEMA

`constant` · `parquet::file::properties::DEFAULT_WRITE_PATH_IN_SCHEMA`

```rust
const DEFAULT_WRITE_PATH_IN_SCHEMA: bool = true
```

Default value for [`WriterProperties::write_path_in_schema`]

---

## BloomFilterPosition

`enum` · `parquet::file::properties::BloomFilterPosition`

```rust
enum BloomFilterPosition
```

**Variants**: `AfterRowGroup`, `End`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Where in the file [`ArrowWriter`](crate::arrow::arrow_writer::ArrowWriter) should
write Bloom filters

Basic constant, which is not part of the Thrift definition.

---

## EnabledStatistics

`enum` · `parquet::file::properties::EnabledStatistics`

```rust
enum EnabledStatistics
```

**Variants**: `None`, `Chunk`, `Page`

**Implements**: `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Controls the level of statistics to be computed by the writer and stored in
the parquet file.

Enabling statistics makes the resulting Parquet file larger and requires
more time to read the parquet footer.

Statistics can be used to improve query performance by pruning row groups
and pages during query execution if the query engine supports evaluating the
predicate using the statistics.

---

## WriterVersion

`enum` · `parquet::file::properties::WriterVersion`

```rust
enum WriterVersion
```

**Variants**: `PARQUET_1_0`, `PARQUET_2_0`

**Implements**: `core::convert::From`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn as_num(&self) -> i32
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Parquet writer version.

Basic constant, which is not part of the Thrift definition.

---

## BloomFilterProperties

`struct` · `parquet::file::properties::BloomFilterProperties`

```rust
struct BloomFilterProperties
```

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn builder() -> BloomFilterPropertiesBuilder
fn fpp(&self) -> f64
fn ndv(&self) -> u64
```

Controls the bloom filter to be computed by the writer.

The bloom filter is initially sized for `ndv` distinct values at the given `fpp`, then
automatically folded down after all values are inserted to achieve optimal size while
maintaining the target `fpp`. See [`Sbbf::fold_to_target_fpp`] for details on the
folding algorithm.

# Example

```rust
# use parquet::{
#    file::properties::{BloomFilterProperties, WriterProperties},
#    schema::types::ColumnPath,
# };
// Build a BloomFilterProperties via the builder, then apply it to one column.
let bf = BloomFilterProperties::builder()
    .with_fpp(0.01)
    .with_max_ndv(10_000)
    .build();

let props = WriterProperties::builder()
    .set_column_bloom_filter_properties(ColumnPath::from("user_id"), bf.clone())
    .build();

assert_eq!(
    props.bloom_filter_properties(&ColumnPath::from("user_id")),
    Some(&bf)
);
```

[`Sbbf::fold_to_target_fpp`]: crate::bloom_filter::Sbbf::fold_to_target_fpp

---

## BloomFilterPropertiesBuilder

`struct` · `parquet::file::properties::BloomFilterPropertiesBuilder`

```rust
struct BloomFilterPropertiesBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (5)

```rust
fn build(self) -> BloomFilterProperties
fn new() -> Self
fn try_build(self) -> Result<BloomFilterProperties>
fn with_fpp(self, fpp: f64) -> Self
fn with_max_ndv(self, ndv: u64) -> Self
```

Builder for [`BloomFilterProperties`].

Use [`BloomFilterProperties::builder`] or [`BloomFilterPropertiesBuilder::new`]
as the entry point.

---

## CdcOptions

`struct` · `parquet::file::properties::CdcOptions`

```rust
struct CdcOptions
```

**Fields**: `min_chunk_size`, `max_chunk_size`, `norm_level`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

EXPERIMENTAL: Options for content-defined chunking (CDC).

Content-defined chunking is an experimental feature that optimizes parquet
files for content addressable storage (CAS) systems by writing data pages
according to content-defined chunk boundaries. This allows for more
efficient deduplication of data across files, hence more efficient network
transfers and storage.

Each content-defined chunk is written as a separate parquet data page. The
following options control the chunks' size and the chunking process. Note
that the chunk size is calculated based on the logical value of the data,
before any encoding or compression is applied.

---

## ReaderProperties

`struct` · `parquet::file::properties::ReaderProperties`

```rust
struct ReaderProperties
```

**Methods** (1)

```rust
fn builder() -> ReaderPropertiesBuilder
```

Configuration settings for reading parquet files.

All properties are immutable and `Send` + `Sync`.
Use [`ReaderPropertiesBuilder`] to assemble these properties.

# Example

```rust
use parquet::file::properties::ReaderProperties;

// Create properties with default configuration.
let props = ReaderProperties::builder().build();

// Use properties builder to set certain options and assemble the configuration.
let props = ReaderProperties::builder()
    .set_backward_compatible_lz4(false)
    .build();
```

---

## ReaderPropertiesBuilder

`struct` · `parquet::file::properties::ReaderPropertiesBuilder`

```rust
struct ReaderPropertiesBuilder
```

**Methods** (4)

```rust
fn build(self) -> ReaderProperties
fn set_backward_compatible_lz4(self, value: bool) -> Self
fn set_read_bloom_filter(self, value: bool) -> Self
fn set_read_page_statistics(self, value: bool) -> Self
```

Builder for parquet file reader configuration. See example on
[`ReaderProperties`]

---

## WriterProperties

`struct` · `parquet::file::properties::WriterProperties`

```rust
struct WriterProperties
```

**Derives**: Clone, Debug, Default

**Methods** (34)

```rust
fn bloom_filter_position(&self) -> BloomFilterPosition
fn bloom_filter_properties(&self, col: &ColumnPath) -> Option<&BloomFilterProperties>
fn builder() -> WriterPropertiesBuilder
fn coerce_types(&self) -> bool
fn column_data_page_size_limit(&self, col: &ColumnPath) -> usize
fn column_data_page_v2_compression_ratio_threshold(&self, col: &ColumnPath) -> f64
fn column_dictionary_page_size_limit(&self, col: &ColumnPath) -> usize
fn column_index_truncate_length(&self) -> Option<usize>
fn compression(&self, col: &ColumnPath) -> Compression
fn content_defined_chunking(&self) -> Option<&CdcOptions>
fn created_by(&self) -> &str
fn data_page_row_count_limit(&self) -> usize
fn data_page_size_limit(&self) -> usize
fn data_page_v2_compression_ratio_threshold(&self) -> f64
fn dictionary_data_page_encoding(&self) -> Encoding
fn dictionary_enabled(&self, col: &ColumnPath) -> bool
fn dictionary_page_encoding(&self) -> Encoding
fn dictionary_page_size_limit(&self) -> usize
fn encoding(&self, col: &ColumnPath) -> Option<Encoding>
fn file_encryption_properties(&self) -> Option<&Arc<FileEncryptionProperties>>
fn into_builder(self) -> WriterPropertiesBuilder
fn key_value_metadata(&self) -> Option<&Vec<KeyValue>>
fn max_row_group_bytes(&self) -> Option<usize>
fn max_row_group_row_count(&self) -> Option<usize>
fn max_row_group_size(&self) -> usize
fn new() -> Self
fn offset_index_disabled(&self) -> bool
fn sorting_columns(&self) -> Option<&Vec<SortingColumn>>
fn statistics_enabled(&self, col: &ColumnPath) -> EnabledStatistics
fn statistics_truncate_length(&self) -> Option<usize>
fn write_batch_size(&self) -> usize
fn write_page_header_statistics(&self, col: &ColumnPath) -> bool
fn write_path_in_schema(&self) -> bool
fn writer_version(&self) -> WriterVersion
```

Configuration settings for writing parquet files.

Use [`Self::builder`] to create a [`WriterPropertiesBuilder`] to change settings.

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

---

## WriterPropertiesBuilder

`struct` · `parquet::file::properties::WriterPropertiesBuilder`

```rust
struct WriterPropertiesBuilder
```

**Implements**: `core::convert::From`, `core::convert::TryFrom`

**Derives**: Clone, Debug, Default

**Methods** (44)

```rust
fn build(self) -> WriterProperties
fn set_bloom_filter_enabled(self, value: bool) -> Self
fn set_bloom_filter_fpp(self, value: f64) -> Self
fn set_bloom_filter_max_ndv(self, value: u64) -> Self
fn set_bloom_filter_ndv(self, value: u64) -> Self
fn set_bloom_filter_position(self, value: BloomFilterPosition) -> Self
fn set_bloom_filter_properties(self, value: BloomFilterProperties) -> Self
fn set_coerce_types(self, coerce_types: bool) -> Self
fn set_column_bloom_filter_enabled(self, col: ColumnPath, value: bool) -> Self
fn set_column_bloom_filter_fpp(self, col: ColumnPath, value: f64) -> Self
fn set_column_bloom_filter_max_ndv(self, col: ColumnPath, value: u64) -> Self
fn set_column_bloom_filter_ndv(self, col: ColumnPath, value: u64) -> Self
fn set_column_bloom_filter_properties(self, col: ColumnPath, value: BloomFilterProperties) -> Self
fn set_column_compression(self, col: ColumnPath, value: Compression) -> Self
fn set_column_data_page_size_limit(self, col: ColumnPath, value: usize) -> Self
fn set_column_data_page_v2_compression_ratio_threshold(self, col: ColumnPath, value: f64) -> Self
fn set_column_dictionary_enabled(self, col: ColumnPath, value: bool) -> Self
fn set_column_dictionary_page_size_limit(self, col: ColumnPath, value: usize) -> Self
fn set_column_encoding(self, col: ColumnPath, value: Encoding) -> Self
fn set_column_index_truncate_length(self, max_length: Option<usize>) -> Self
fn set_column_statistics_enabled(self, col: ColumnPath, value: EnabledStatistics) -> Self
fn set_column_write_page_header_statistics(self, col: ColumnPath, value: bool) -> Self
fn set_compression(self, value: Compression) -> Self
fn set_content_defined_chunking(self, options: Option<CdcOptions>) -> Self
fn set_created_by(self, value: String) -> Self
fn set_data_page_row_count_limit(self, value: usize) -> Self
fn set_data_page_size_limit(self, value: usize) -> Self
fn set_data_page_v2_compression_ratio_threshold(self, value: f64) -> Self
fn set_dictionary_enabled(self, value: bool) -> Self
fn set_dictionary_page_size_limit(self, value: usize) -> Self
fn set_encoding(self, value: Encoding) -> Self
fn set_key_value_metadata(self, value: Option<Vec<KeyValue>>) -> Self
fn set_max_row_group_bytes(self, value: Option<usize>) -> Self
fn set_max_row_group_row_count(self, value: Option<usize>) -> Self
fn set_max_row_group_size(self, value: usize) -> Self
fn set_offset_index_disabled(self, value: bool) -> Self
fn set_sorting_columns(self, value: Option<Vec<SortingColumn>>) -> Self
fn set_statistics_enabled(self, value: EnabledStatistics) -> Self
fn set_statistics_truncate_length(self, max_length: Option<usize>) -> Self
fn set_write_batch_size(self, value: usize) -> Self
fn set_write_page_header_statistics(self, value: bool) -> Self
fn set_write_path_in_schema(self, write_path_in_schema: bool) -> Self
fn set_writer_version(self, value: WriterVersion) -> Self
fn with_file_encryption_properties(self, file_encryption_properties: Arc<FileEncryptionProperties>) -> Self
```

**via `core::convert::From`**

```rust
fn from(props: WriterProperties) -> Self
```

Builder for  [`WriterProperties`] Parquet writer configuration.

See example on [`WriterProperties`]

---

## ReaderPropertiesPtr

`type_alias` · `parquet::file::properties::ReaderPropertiesPtr`

```rust
type ReaderPropertiesPtr = std::sync::Arc<ReaderProperties>
```

Reference counted reader properties.

---

## WriterPropertiesPtr

`type_alias` · `parquet::file::properties::WriterPropertiesPtr`

```rust
type WriterPropertiesPtr = std::sync::Arc<WriterProperties>
```

Reference counted writer properties.

---
