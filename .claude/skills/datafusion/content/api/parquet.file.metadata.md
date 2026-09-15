# `parquet::file::metadata`

Crate `parquet` · 17 public items · structured records in [`model/parquet.file.metadata.json`](../model/parquet.file.metadata.json)

## ColumnChunkMetaData

`struct` · `parquet::file::metadata::ColumnChunkMetaData`

```rust
struct ColumnChunkMetaData
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (33)

```rust
fn bloom_filter_length(&self) -> Option<i32>
fn bloom_filter_offset(&self) -> Option<i64>
fn builder(column_descr: ColumnDescPtr) -> ColumnChunkMetaDataBuilder
fn byte_range(&self) -> (u64, u64)
fn column_descr(&self) -> &ColumnDescriptor
fn column_descr_ptr(&self) -> ColumnDescPtr
fn column_index_length(&self) -> Option<i32>
fn column_index_offset(&self) -> Option<i64>
fn column_path(&self) -> &ColumnPath
fn column_type(&self) -> Type
fn compressed_size(&self) -> i64
fn compression(&self) -> Compression
fn compression_codec(&self) -> CompressionCodec
fn crypto_metadata(&self) -> Option<&ColumnCryptoMetaData>
fn data_page_offset(&self) -> i64
fn definition_level_histogram(&self) -> Option<&LevelHistogram>
fn dictionary_page_offset(&self) -> Option<i64>
fn encodings(&self) -> impl Iterator<Item = Encoding>
fn encodings_mask(&self) -> &EncodingMask
fn file_offset(&self) -> i64
fn file_path(&self) -> Option<&str>
fn geo_statistics(&self) -> Option<&geo_statistics::GeospatialStatistics>
fn index_page_offset(&self) -> Option<i64>
fn into_builder(self) -> ColumnChunkMetaDataBuilder
fn num_values(&self) -> i64
fn offset_index_length(&self) -> Option<i32>
fn offset_index_offset(&self) -> Option<i64>
fn page_encoding_stats(&self) -> Option<&Vec<PageEncodingStats>>
fn page_encoding_stats_mask(&self) -> Option<&EncodingMask>
fn repetition_level_histogram(&self) -> Option<&LevelHistogram>
fn statistics(&self) -> Option<&Statistics>
fn uncompressed_size(&self) -> i64
fn unencoded_byte_array_data_bytes(&self) -> Option<i64>
```

Metadata for a column chunk.

---

## ColumnChunkMetaDataBuilder

`struct` · `parquet::file::metadata::ColumnChunkMetaDataBuilder`

```rust
struct ColumnChunkMetaDataBuilder
```

**Implements**: `core::convert::From`

**Methods** (29)

```rust
fn build(self) -> Result<ColumnChunkMetaData>
fn clear_page_encoding_stats(self) -> Self
fn clear_statistics(self) -> Self
fn set_bloom_filter_length(self, value: Option<i32>) -> Self
fn set_bloom_filter_offset(self, value: Option<i64>) -> Self
fn set_column_crypto_metadata(self, value: Option<ColumnCryptoMetaData>) -> Self
fn set_column_index_length(self, value: Option<i32>) -> Self
fn set_column_index_offset(self, value: Option<i64>) -> Self
fn set_compression(self, value: Compression) -> Self
fn set_compression_codec(self, value: CompressionCodec) -> Self
fn set_data_page_offset(self, value: i64) -> Self
fn set_definition_level_histogram(self, value: Option<LevelHistogram>) -> Self
fn set_dictionary_page_offset(self, value: Option<i64>) -> Self
fn set_encodings(self, encodings: Vec<Encoding>) -> Self
fn set_encodings_mask(self, encodings: EncodingMask) -> Self
fn set_encrypted_column_metadata(self, value: Option<Vec<u8>>) -> Self
fn set_file_path(self, value: String) -> Self
fn set_geo_statistics(self, value: Box<geo_statistics::GeospatialStatistics>) -> Self
fn set_index_page_offset(self, value: Option<i64>) -> Self
fn set_num_values(self, value: i64) -> Self
fn set_offset_index_length(self, value: Option<i32>) -> Self
fn set_offset_index_offset(self, value: Option<i64>) -> Self
fn set_page_encoding_stats(self, value: Vec<PageEncodingStats>) -> Self
fn set_page_encoding_stats_mask(self, value: EncodingMask) -> Self
fn set_repetition_level_histogram(self, value: Option<LevelHistogram>) -> Self
fn set_statistics(self, value: Statistics) -> Self
fn set_total_compressed_size(self, value: i64) -> Self
fn set_total_uncompressed_size(self, value: i64) -> Self
fn set_unencoded_byte_array_data_bytes(self, value: Option<i64>) -> Self
```

**via `core::convert::From`**

```rust
fn from(value: ColumnChunkMetaData) -> Self
```

Builder for [`ColumnChunkMetaData`]

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

---

## ColumnIndexBuilder

`struct` · `parquet::file::metadata::ColumnIndexBuilder`

```rust
struct ColumnIndexBuilder
```

**Methods** (7)

```rust
fn append(&mut self, null_page: bool, min_value: Vec<u8>, max_value: Vec<u8>, null_count: i64)
fn append_histograms(&mut self, repetition_level_histogram: &Option<LevelHistogram>, definition_level_histogram: &Option<LevelHistogram>)
fn build(self) -> Result<ColumnIndexMetaData>
fn new(column_type: Type) -> Self
fn set_boundary_order(&mut self, boundary_order: BoundaryOrder)
fn to_invalid(&mut self)
fn valid(&self) -> bool
```

Builder for Parquet [`ColumnIndex`], part of the Parquet [PageIndex]

[PageIndex]: https://github.com/apache/parquet-format/blob/master/PageIndex.md
[`ColumnIndex`]: https://github.com/apache/parquet-format/blob/master/PageIndex.md

---

## FileMetaData

`struct` · `parquet::file::metadata::FileMetaData`

```rust
struct FileMetaData
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (10)

```rust
fn column_order(&self, i: usize) -> ColumnOrder
fn column_orders(&self) -> Option<&Vec<ColumnOrder>>
fn created_by(&self) -> Option<&str>
fn key_value_metadata(&self) -> Option<&Vec<KeyValue>>
fn new(version: i32, num_rows: i64, created_by: Option<String>, key_value_metadata: Option<Vec<KeyValue>>, schema_descr: SchemaDescPtr, column_orders: Option<Vec<ColumnOrder>>) -> Self
fn num_rows(&self) -> i64
fn schema(&self) -> &SchemaType
fn schema_descr(&self) -> &SchemaDescriptor
fn schema_descr_ptr(&self) -> SchemaDescPtr
fn version(&self) -> i32
```

File level metadata for a Parquet file.

Includes the version of the file, metadata, number of rows, schema, and column orders

---

## KeyValue

`struct` · `parquet::file::metadata::KeyValue`

```rust
struct KeyValue
```

**Fields**: `key`, `value`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new<F2>(key: String, value: F2) -> KeyValue where F2: Into<Option<String>>
```

A key-value pair for [`FileMetaData`].

---

## LevelHistogram

`struct` · `parquet::file::metadata::LevelHistogram`

```rust
struct LevelHistogram
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (10)

```rust
fn add(&mut self, other: &Self)
fn get(&self, index: usize) -> Option<i64>
fn increment_by(&mut self, level: i16, count: i64)
fn into_inner(self) -> Vec<i64>
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn reset(&mut self)
fn try_new(max_level: i16) -> Option<Self>
fn update_from_levels(&mut self, levels: &[i16])
fn values(&self) -> &[i64]
```

**via `core::convert::From`**

```rust
fn from(inner: Vec<i64>) -> Self
```

Histograms for repetition and definition levels.

Each histogram is a vector of length `max_level + 1`. The value at index `i` is the number of
values at level `i`.

For example, `vec[0]` is the number of rows with level 0, `vec[1]` is the
number of rows with level 1, and so on.

---

## OffsetIndexBuilder

`struct` · `parquet::file::metadata::OffsetIndexBuilder`

```rust
struct OffsetIndexBuilder
```

**Derives**: Default

**Methods** (5)

```rust
fn append_offset_and_size(&mut self, offset: i64, compressed_page_size: i32)
fn append_row_count(&mut self, row_count: i64)
fn append_unencoded_byte_array_data_bytes(&mut self, unencoded_byte_array_data_bytes: Option<i64>)
fn build(self) -> OffsetIndexMetaData
fn new() -> Self
```

Builder for offset index, part of the Parquet [PageIndex].

[PageIndex]: https://github.com/apache/parquet-format/blob/master/PageIndex.md

---

## PageEncodingStats

`struct` · `parquet::file::metadata::PageEncodingStats`

```rust
struct PageEncodingStats
```

**Fields**: `page_type`, `encoding`, `count`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

PageEncodingStats for a column chunk and data page.

---

## ParquetMetaData

`struct` · `parquet::file::metadata::ParquetMetaData`

```rust
struct ParquetMetaData
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (9)

```rust
fn column_index(&self) -> Option<&ParquetColumnIndex>
fn file_metadata(&self) -> &FileMetaData
fn into_builder(self) -> ParquetMetaDataBuilder
fn memory_size(&self) -> usize
fn new(file_metadata: FileMetaData, row_groups: Vec<RowGroupMetaData>) -> Self
fn num_row_groups(&self) -> usize
fn offset_index(&self) -> Option<&ParquetOffsetIndex>
fn row_group(&self, i: usize) -> &RowGroupMetaData
fn row_groups(&self) -> &[RowGroupMetaData]
```

Parsed metadata for a single Parquet file

This structure is stored in the footer of Parquet files, in the format
defined by [`parquet.thrift`].

# Overview
The fields of this structure are:
* [`FileMetaData`]: Information about the overall file (such as the schema) (See [`Self::file_metadata`])
* [`RowGroupMetaData`]: Information about each Row Group (see [`Self::row_groups`])
* [`ParquetColumnIndex`] and [`ParquetOffsetIndex`]: Optional "Page Index" structures (see [`Self::column_index`] and [`Self::offset_index`])

This structure is read by the various readers in this crate or can be read
directly from a file using the [`ParquetMetaDataReader`] struct.

See the [`ParquetMetaDataBuilder`] to create and modify this structure.

[`parquet.thrift`]: https://github.com/apache/parquet-format/blob/master/src/main/thrift/parquet.thrift

---

## ParquetMetaDataBuilder

`struct` · `parquet::file::metadata::ParquetMetaDataBuilder`

```rust
struct ParquetMetaDataBuilder
```

**Implements**: `core::convert::From`

**Methods** (13)

```rust
fn add_row_group(self, row_group: RowGroupMetaData) -> Self
fn build(self) -> ParquetMetaData
fn column_index(&self) -> Option<&ParquetColumnIndex>
fn new(file_meta_data: FileMetaData) -> Self
fn new_from_metadata(metadata: ParquetMetaData) -> Self
fn offset_index(&self) -> Option<&ParquetOffsetIndex>
fn row_groups(&self) -> &[RowGroupMetaData]
fn set_column_index(self, column_index: Option<ParquetColumnIndex>) -> Self
fn set_offset_index(self, offset_index: Option<ParquetOffsetIndex>) -> Self
fn set_row_groups(self, row_groups: Vec<RowGroupMetaData>) -> Self
fn take_column_index(&mut self) -> Option<ParquetColumnIndex>
fn take_offset_index(&mut self) -> Option<ParquetOffsetIndex>
fn take_row_groups(&mut self) -> Vec<RowGroupMetaData>
```

**via `core::convert::From`**

```rust
fn from(meta_data: ParquetMetaData) -> Self
```

 A builder for creating / manipulating [`ParquetMetaData`]

 # Example creating a new [`ParquetMetaData`]

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

 # Example modifying an existing [`ParquetMetaData`]
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

---

## RowGroupMetaData

`struct` · `parquet::file::metadata::RowGroupMetaData`

```rust
struct RowGroupMetaData
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (14)

```rust
fn builder(schema_descr: SchemaDescPtr) -> RowGroupMetaDataBuilder
fn column(&self, i: usize) -> &ColumnChunkMetaData
fn columns(&self) -> &[ColumnChunkMetaData]
fn columns_mut(&mut self) -> &mut [ColumnChunkMetaData]
fn compressed_size(&self) -> i64
fn file_offset(&self) -> Option<i64>
fn into_builder(self) -> RowGroupMetaDataBuilder
fn num_columns(&self) -> usize
fn num_rows(&self) -> i64
fn ordinal(&self) -> Option<i16>
fn schema_descr(&self) -> &SchemaDescriptor
fn schema_descr_ptr(&self) -> SchemaDescPtr
fn sorting_columns(&self) -> Option<&Vec<SortingColumn>>
fn total_byte_size(&self) -> i64
```

Metadata for a row group

Includes [`ColumnChunkMetaData`] for each column in the row group, the number of rows
the total byte size of the row group, and the [`SchemaDescriptor`] for the row group.

---

## RowGroupMetaDataBuilder

`struct` · `parquet::file::metadata::RowGroupMetaDataBuilder`

```rust
struct RowGroupMetaDataBuilder
```

**Methods** (9)

```rust
fn add_column_metadata(self, value: ColumnChunkMetaData) -> Self
fn build(self) -> Result<RowGroupMetaData>
fn set_column_metadata(self, value: Vec<ColumnChunkMetaData>) -> Self
fn set_file_offset(self, value: i64) -> Self
fn set_num_rows(self, value: i64) -> Self
fn set_ordinal(self, value: i16) -> Self
fn set_sorting_columns(self, value: Option<Vec<SortingColumn>>) -> Self
fn set_total_byte_size(self, value: i64) -> Self
fn take_columns(&mut self) -> Vec<ColumnChunkMetaData>
```

Builder for row group metadata.

---

## SortingColumn

`struct` · `parquet::file::metadata::SortingColumn`

```rust
struct SortingColumn
```

**Fields**: `column_idx`, `descending`, `nulls_first`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

Sort order within a RowGroup of a leaf column

---

## FileMetaDataPtr

`type_alias` · `parquet::file::metadata::FileMetaDataPtr`

```rust
type FileMetaDataPtr = std::sync::Arc<FileMetaData>
```

Reference counted pointer for [`FileMetaData`].

---

## ParquetColumnIndex

`type_alias` · `parquet::file::metadata::ParquetColumnIndex`

```rust
type ParquetColumnIndex = Vec<Vec<file::page_index::column_index::ColumnIndexMetaData>>
```

Page level statistics for each column chunk of each row group.

This structure is an in-memory representation of multiple [`ColumnIndex`]
structures in a parquet file footer, as described in the Parquet [PageIndex
documentation]. Each [`ColumnIndex`] holds statistics about all the pages in a
particular column chunk.

`column_index[row_group_number][column_number]` holds the
[`ColumnIndex`] corresponding to column `column_number` of row group
`row_group_number`.

For example `column_index[2][3]` holds the [`ColumnIndex`] for the fourth
column in the third row group of the parquet file.

[PageIndex documentation]: https://github.com/apache/parquet-format/blob/master/PageIndex.md
[`ColumnIndex`]: crate::file::page_index::column_index::ColumnIndexMetaData

---

## ParquetOffsetIndex

`type_alias` · `parquet::file::metadata::ParquetOffsetIndex`

```rust
type ParquetOffsetIndex = Vec<Vec<file::page_index::offset_index::OffsetIndexMetaData>>
```

[`OffsetIndexMetaData`] for each data page of each row group of each column

This structure is the parsed representation of the [`OffsetIndex`] from the
Parquet file footer, as described in the Parquet [PageIndex documentation].

`offset_index[row_group_number][column_number]` holds
the [`OffsetIndexMetaData`] corresponding to column
`column_number`of row group `row_group_number`.

[PageIndex documentation]: https://github.com/apache/parquet-format/blob/master/PageIndex.md
[`OffsetIndex`]: https://github.com/apache/parquet-format/blob/master/PageIndex.md

---

## RowGroupMetaDataPtr

`type_alias` · `parquet::file::metadata::RowGroupMetaDataPtr`

```rust
type RowGroupMetaDataPtr = std::sync::Arc<RowGroupMetaData>
```

Reference counted pointer for [`RowGroupMetaData`].

---
