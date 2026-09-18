# `parquet::arrow::arrow_reader::ArrowReaderOptions`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.ArrowReaderOptions.json).

<a id="op-d1d83f9f3dc4572b085ef8a4"></a>
## ArrowReaderOptions

`struct` · `parquet::arrow::arrow_reader::ArrowReaderOptions` · parquet 59.3.0

```rust
struct ArrowReaderOptions
```

Source: `src/arrow/arrow_reader/mod.rs:462`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Options that control how [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) is read when constructing
an Arrow reader.

To use these options, pass them to one of the following methods:
* [`ParquetRecordBatchReaderBuilder::try_new_with_options`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-61bc90b220729954175c1066)
* [`ParquetRecordBatchStreamBuilder::new_with_options`]

For fine-grained control over metadata loading, use
[`ArrowReaderMetadata::load`](../operations/parquet.arrow.arrow_reader.ArrowReaderMetadata.md#op-af27598b22047b6ad64b6068) to load metadata with these options,

See [`ArrowReaderBuilder`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-2cb4803ec228a018e60491ee) for how to configure how the column data
is then read from the file, including projection and filter pushdown

[`ParquetRecordBatchStreamBuilder::new_with_options`]: crate::arrow::async_reader::ParquetRecordBatchStreamBuilder::new_with_options

<a id="op-2f103236f6efa3d39fd2dec7"></a>
## clone

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ArrowReaderOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [461, 17], "end": [461, 22], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow/arrow_reader/mod.rs:461`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71bc6b9c387e0b17d5f969bf"></a>
## column_index_policy

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::column_index_policy` · parquet 59.3.0

```rust
fn column_index_policy(&self) -> PageIndexPolicy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [846, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:829`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Retrieve the currently set [`PageIndexPolicy`](../operations/parquet.file.metadata.reader.PageIndexPolicy.md#op-7dc5da782b207c53dc34d37c) for the column index.

This can be set via [`with_column_index_policy`][Self::with_column_index_policy](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-80fac375041df611ef5894f9)
or [`with_page_index_policy`][Self::with_page_index_policy](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-d9cba3f4586f96db031a6533).

<a id="op-9c9d58d87ba81862518f2e09"></a>
## default

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::default` · parquet 59.3.0

```rust
fn default() -> ArrowReaderOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [461, 24], "end": [461, 31], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/arrow/arrow_reader/mod.rs:461`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a4b7e51de0c872e62059354"></a>
## file_decryption_properties

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::file_decryption_properties` · parquet 59.3.0

```rust
fn file_decryption_properties(&self) -> Option<&Arc<FileDecryptionProperties>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [846, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:843`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Retrieve the currently set file decryption properties.

This can be set via
[`file_decryption_properties`][Self::with_file_decryption_properties](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-f144af5f895d0cfb46e589e0).

<a id="op-0bc0d5c32709652e42e7bc42"></a>
## fmt

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [461, 10], "end": [461, 15], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/arrow_reader/mod.rs:461`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e35842ab206fb45680444953"></a>
## metadata_options

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::metadata_options` · parquet 59.3.0

```rust
fn metadata_options(&self) -> &ParquetMetaDataOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [846, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:834`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Retrieve the currently set metadata decoding options.

<a id="op-0ffb6809b4cb80c558579393"></a>
## new

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::new` · parquet 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [846, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:485`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [`ArrowReaderOptions`](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-d1d83f9f3dc4572b085ef8a4) with the default settings

<a id="op-ed8117f40b93e901a75d8cec"></a>
## offset_index_policy

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::offset_index_policy` · parquet 59.3.0

```rust
fn offset_index_policy(&self) -> PageIndexPolicy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [846, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:821`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Retrieve the currently set [`PageIndexPolicy`](../operations/parquet.file.metadata.reader.PageIndexPolicy.md#op-7dc5da782b207c53dc34d37c) for the offset index.

This can be set via [`with_offset_index_policy`][Self::with_offset_index_policy](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-20e7e67b14aa2ee987f9bab3)
or [`with_page_index_policy`][Self::with_page_index_policy](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-d9cba3f4586f96db031a6533).

<a id="op-e7fc1549327bbe9fabe4750b"></a>
## page_index

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::page_index` · parquet 59.3.0

```rust
fn page_index(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [846, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:813`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns whether page index reading is enabled.

This returns `true` if both the column index and offset index policies are not [`PageIndexPolicy::Skip`](../operations/parquet.file.metadata.reader.PageIndexPolicy.md#op-6f2d1d654ecec39294ec7a53).

This can be set via [`with_page_index`][Self::with_page_index](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-4c872ea4ae6007c2db6ecd25) or
[`with_page_index_policy`][Self::with_page_index_policy](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-d9cba3f4586f96db031a6533).

<a id="op-80fac375041df611ef5894f9"></a>
## with_column_index_policy

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::with_column_index_policy` · parquet 59.3.0

```rust
fn with_column_index_policy(self, policy: PageIndexPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [846, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:653`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the [`PageIndexPolicy`](../operations/parquet.file.metadata.reader.PageIndexPolicy.md#op-7dc5da782b207c53dc34d37c) for the Parquet [ColumnIndex] structure.

The `ColumnIndex` contains min/max statistics for each page, which can be used
for predicate pushdown and page-level pruning.

[ColumnIndex]: https://github.com/apache/parquet-format/blob/master/PageIndex.md

<a id="op-ba2893d2c7e5902ecd460432"></a>
## with_column_stats_policy

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::with_column_stats_policy` · parquet 59.3.0

```rust
fn with_column_stats_policy(self, policy: ParquetStatisticsPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [846, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:707`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the decoding policy for [`statistics`] in the Parquet `ColumnMetaData`.

[`statistics`]:
https://github.com/apache/parquet-format/blob/786142e26740487930ddc3ec5e39d780bd930907/src/main/thrift/parquet.thrift#L912

<a id="op-654feec70bd5d44884badac1"></a>
## with_encoding_stats_as_mask

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::with_encoding_stats_as_mask` · parquet 59.3.0

```rust
fn with_encoding_stats_as_mask(self, val: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [846, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:689`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set whether to convert the [`encoding_stats`] in the Parquet `ColumnMetaData` to a bitmask
(defaults to `false`).

See [`ColumnChunkMetaData::page_encoding_stats_mask`] for an explanation of why this
might be desirable.

[`ColumnChunkMetaData::page_encoding_stats_mask`]:
crate::file::metadata::ColumnChunkMetaData::page_encoding_stats_mask
[`encoding_stats`]:
https://github.com/apache/parquet-format/blob/786142e26740487930ddc3ec5e39d780bd930907/src/main/thrift/parquet.thrift#L917

<a id="op-48dad2f5ad71cb1f61b06ae8"></a>
## with_encoding_stats_policy

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::with_encoding_stats_policy` · parquet 59.3.0

```rust
fn with_encoding_stats_policy(self, policy: ParquetStatisticsPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [846, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:698`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the decoding policy for [`encoding_stats`] in the Parquet `ColumnMetaData`.

[`encoding_stats`]:
https://github.com/apache/parquet-format/blob/786142e26740487930ddc3ec5e39d780bd930907/src/main/thrift/parquet.thrift#L917

<a id="op-f144af5f895d0cfb46e589e0"></a>
## with_file_decryption_properties

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::with_file_decryption_properties` · parquet 59.3.0

```rust
fn with_file_decryption_properties(self, file_decryption_properties: Arc<FileDecryptionProperties>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [846, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:725`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Provide the file decryption properties to use when reading encrypted parquet files.

If encryption is enabled and the file is encrypted, the `file_decryption_properties` must be provided.

<a id="op-20e7e67b14aa2ee987f9bab3"></a>
## with_offset_index_policy

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::with_offset_index_policy` · parquet 59.3.0

```rust
fn with_offset_index_policy(self, policy: PageIndexPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [846, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:664`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the [`PageIndexPolicy`](../operations/parquet.file.metadata.reader.PageIndexPolicy.md#op-7dc5da782b207c53dc34d37c) for the Parquet [OffsetIndex] structure.

The `OffsetIndex` contains the locations and sizes of each page, which enables
efficient page-level skipping and random access within column chunks.

[OffsetIndex]: https://github.com/apache/parquet-format/blob/master/PageIndex.md

<a id="op-4c872ea4ae6007c2db6ecd25"></a>
## with_page_index

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::with_page_index` · parquet 59.3.0

```rust
fn with_page_index(self, page_index: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [846, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:631`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Enable reading the [`PageIndex`] from the metadata, if present (defaults to `false`)

The `PageIndex` can be used to push down predicates to the parquet scan,
potentially eliminating unnecessary IO, by some query engines.

If this is enabled, [`ParquetMetaData::column_index`] and
[`ParquetMetaData::offset_index`] will be populated if the corresponding
information is present in the file.

[`PageIndex`]: https://github.com/apache/parquet-format/blob/master/PageIndex.md
[`ParquetMetaData::column_index`]: crate::file::metadata::ParquetMetaData::column_index
[`ParquetMetaData::offset_index`]: crate::file::metadata::ParquetMetaData::offset_index

<a id="op-d9cba3f4586f96db031a6533"></a>
## with_page_index_policy

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::with_page_index_policy` · parquet 59.3.0

```rust
fn with_page_index_policy(self, policy: PageIndexPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [846, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:642`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the [`PageIndexPolicy`](../operations/parquet.file.metadata.reader.PageIndexPolicy.md#op-7dc5da782b207c53dc34d37c) for both the column and offset indexes.

The `PageIndex` consists of two structures: the `ColumnIndex` and `OffsetIndex`.
This method sets the same policy for both. For fine-grained control, use
[`Self::with_column_index_policy`](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-80fac375041df611ef5894f9) and [`Self::with_offset_index_policy`](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-20e7e67b14aa2ee987f9bab3).

See [`Self::with_page_index`](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-4c872ea4ae6007c2db6ecd25) for more details on page indexes.

<a id="op-afef2c6ad97ed13d4fb1ef7b"></a>
## with_parquet_schema

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::with_parquet_schema` · parquet 59.3.0

```rust
fn with_parquet_schema(self, schema: Arc<SchemaDescriptor>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [846, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:674`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Provide a Parquet schema to use when decoding the metadata. The schema in the Parquet
footer will be skipped.

This can be used to avoid reparsing the schema from the file when it is
already known.

<a id="op-549a43157f215c416cd8b1a9"></a>
## with_schema

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::with_schema` · parquet 59.3.0

```rust
fn with_schema(self, schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [846, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:610`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Provide a schema hint to use when reading the Parquet file.

If provided, this schema takes precedence over any arrow schema embedded
in the metadata (see the [`arrow`] documentation for more details).

If the provided schema is not compatible with the data stored in the
parquet file schema, an error will be returned when constructing the
builder.

This option is only required if you want to explicitly control the
conversion of Parquet types to Arrow types, such as casting a column to
a different type. For example, if you wanted to read an Int64 in
a Parquet file to a [`TimestampMicrosecondArray`] in the Arrow schema.

[`arrow`]: crate::arrow
[`TimestampMicrosecondArray`]: arrow_array::TimestampMicrosecondArray

# Notes

The provided schema must have the same number of columns as the parquet schema and
the column names must be the same.

# Example
```
# use std::sync::Arc;
# use bytes::Bytes;
# use arrow_array::{ArrayRef, Int32Array, RecordBatch};
# use arrow_schema::{DataType, Field, Schema, TimeUnit};
# use parquet::arrow::arrow_reader::{ArrowReaderOptions, ParquetRecordBatchReaderBuilder};
# use parquet::arrow::ArrowWriter;
// Write data - schema is inferred from the data to be Int32
let mut file = Vec::new();
let batch = RecordBatch::try_from_iter(vec![
    ("col_1", Arc::new(Int32Array::from(vec![1, 2, 3])) as ArrayRef),
]).unwrap();
let mut writer = ArrowWriter::try_new(&mut file, batch.schema(), None).unwrap();
writer.write(&batch).unwrap();
writer.close().unwrap();
let file = Bytes::from(file);

// Read the file back.
// Supply a schema that interprets the Int32 column as a Timestamp.
let supplied_schema = Arc::new(Schema::new(vec![
    Field::new("col_1", DataType::Timestamp(TimeUnit::Nanosecond, None), false)
]));
let options = ArrowReaderOptions::new().with_schema(supplied_schema.clone());
let mut builder = ParquetRecordBatchReaderBuilder::try_new_with_options(
    file.clone(),
    options
).expect("Error if the schema is not compatible with the parquet file schema.");

// Create the reader and read the data using the supplied schema.
let mut reader = builder.build().unwrap();
let _batch = reader.next().unwrap().unwrap();
```

# Example: Preserving Dictionary Encoding

By default, Parquet string columns are read as `Utf8Array` (or `LargeUtf8Array`),
even if the underlying Parquet data uses dictionary encoding. You can preserve
the dictionary encoding by specifying a `Dictionary` type in the schema hint:

```
# use std::sync::Arc;
# use bytes::Bytes;
# use arrow_array::{ArrayRef, RecordBatch, StringArray};
# use arrow_schema::{DataType, Field, Schema};
# use parquet::arrow::arrow_reader::{ArrowReaderOptions, ParquetRecordBatchReaderBuilder};
# use parquet::arrow::ArrowWriter;
// Write a Parquet file with string data
let mut file = Vec::new();
let schema = Arc::new(Schema::new(vec![
    Field::new("city", DataType::Utf8, false)
]));
let cities = StringArray::from(vec!["Berlin", "Berlin", "Paris", "Berlin", "Paris"]);
let batch = RecordBatch::try_new(schema.clone(), vec![Arc::new(cities)]).unwrap();

let mut writer = ArrowWriter::try_new(&mut file, batch.schema(), None).unwrap();
writer.write(&batch).unwrap();
writer.close().unwrap();
let file = Bytes::from(file);

// Read the file back, requesting dictionary encoding preservation
let dict_schema = Arc::new(Schema::new(vec![
    Field::new("city", DataType::Dictionary(
        Box::new(DataType::Int32),
        Box::new(DataType::Utf8)
    ), false)
]));
let options = ArrowReaderOptions::new().with_schema(dict_schema);
let builder = ParquetRecordBatchReaderBuilder::try_new_with_options(
    file.clone(),
    options
).unwrap();

let mut reader = builder.build().unwrap();
let batch = reader.next().unwrap().unwrap();

// The column is now a DictionaryArray
assert!(matches!(
    batch.column(0).data_type(),
    DataType::Dictionary(_, _)
));
```

**Note**: Dictionary encoding preservation works best when:
1. The original column was dictionary encoded (the default for string columns)
2. There are a small number of distinct values

<a id="op-c3a72245ec21e6e39e4d6b26"></a>
## with_size_stats_policy

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::with_size_stats_policy` · parquet 59.3.0

```rust
fn with_size_stats_policy(self, policy: ParquetStatisticsPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [846, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:716`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the decoding policy for [`size_statistics`] in the Parquet `ColumnMetaData`.

[`size_statistics`]:
https://github.com/apache/parquet-format/blob/786142e26740487930ddc3ec5e39d780bd930907/src/main/thrift/parquet.thrift#L936

<a id="op-e6de6e02e6885e0e699b0ad3"></a>
## with_skip_arrow_metadata

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::with_skip_arrow_metadata` · parquet 59.3.0

```rust
fn with_skip_arrow_metadata(self, skip_arrow_metadata: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [846, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:495`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Skip decoding the embedded arrow metadata (defaults to `false`)

Parquet files generated by some writers may contain embedded arrow
schema and metadata.
This may not be correct or compatible with your system,
for example, see [ARROW-16184](https://issues.apache.org/jira/browse/ARROW-16184)

<a id="op-af341cc6514f3303e9d05a88"></a>
## with_virtual_columns

`function` · `parquet::arrow::arrow_reader::ArrowReaderOptions::with_virtual_columns` · parquet 59.3.0

```rust
fn with_virtual_columns(self, virtual_columns: Vec<FieldRef>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::ArrowReaderOptions", "path": "ArrowReaderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [846, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:787`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Include virtual columns in the output.

Virtual columns are columns that are not part of the Parquet schema, but are added to the output by the reader such as row numbers and row group indices.

# Example
```
# use std::sync::Arc;
# use bytes::Bytes;
# use arrow_array::{ArrayRef, Int64Array, RecordBatch};
# use arrow_schema::{DataType, Field, Schema};
# use parquet::arrow::{ArrowWriter, RowNumber};
# use parquet::arrow::arrow_reader::{ArrowReaderOptions, ParquetRecordBatchReaderBuilder};
#
# fn main() -> Result<(), Box<dyn std::error::Error>> {
// Create a simple record batch with some data
let values = Arc::new(Int64Array::from(vec![1, 2, 3])) as ArrayRef;
let batch = RecordBatch::try_from_iter(vec![("value", values)])?;

// Write the batch to an in-memory buffer
let mut file = Vec::new();
let mut writer = ArrowWriter::try_new(
    &mut file,
    batch.schema(),
    None
)?;
writer.write(&batch)?;
writer.close()?;
let file = Bytes::from(file);

// Create a virtual column for row numbers
let row_number_field = Arc::new(Field::new("row_number", DataType::Int64, false)
    .with_extension_type(RowNumber));

// Configure options with virtual columns
let options = ArrowReaderOptions::new()
    .with_virtual_columns(vec![row_number_field])?;

// Create a reader with the options
let mut reader = ParquetRecordBatchReaderBuilder::try_new_with_options(
    file,
    options
)?
.build()?;

// Read the batch - it will include both the original column and the virtual row_number column
let result_batch = reader.next().unwrap()?;
assert_eq!(result_batch.num_columns(), 2); // "value" + "row_number"
assert_eq!(result_batch.num_rows(), 3);
#
# Ok(())
# }
```
