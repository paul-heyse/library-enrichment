# `buoyant_kernel::ParquetHandler`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.ParquetHandler.json).

<a id="op-935e1b04f902c9a95af7c376"></a>
## ParquetHandler

`trait` · `buoyant_kernel::ParquetHandler` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait ParquetHandler: AsAny
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L744).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:744`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Provides Parquet file related functionalities to Delta Kernel.

Connectors can leverage this trait to provide their own custom
implementation of Parquet data file functionalities to Delta Kernel.

<a id="op-9bb577f7cae9ed4c6da91a35"></a>
## read_parquet_files

`function` · `buoyant_kernel::ParquetHandler::read_parquet_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_parquet_files(&self, files: &[FileMeta], physical_schema: SchemaRef, predicate: Option<PredicateRef>) -> DeltaResult<FileDataReadResultIterator>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L885).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:885`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Read and parse the Parquet file at given locations and return the data as EngineData with
the columns requested by physical schema. The ParquetHandler _must_ return exactly the
columns specified in `physical_schema`, and they _must_ be in schema order.

# Resolving Parquet schema to the physical schema

When reading the Parquet file, the columns are resolved from the Parquet schema to the
kernel's `physical_schema`. To do so, the parquet reader must match each Parquet column
to a [`StructField`](../operations/buoyant_kernel.schema.StructField.md#op-ebbc3bdce7a671f7a7d1f58d) in the `physical_schema`. All columns in the returned `EngineData`
must be in the same order as specified in `physical_schema`.

Parquet columns are matched to `physical_schema` [`StructField`](../operations/buoyant_kernel.schema.StructField.md#op-ebbc3bdce7a671f7a7d1f58d)s using the following rules:
1. **Field ID**: If a [`StructField`](../operations/buoyant_kernel.schema.StructField.md#op-ebbc3bdce7a671f7a7d1f58d) in `physical_schema` contains a field ID (specified in
   [`ColumnMetadataKey::ParquetFieldId`] metadata), use the ID to match the Parquet column's
   field id
2. **Field Name**: If no field ID is present in the `physical_schema`'s [`StructField`](../operations/buoyant_kernel.schema.StructField.md#op-ebbc3bdce7a671f7a7d1f58d) or
   no matching parquet field ID is found, fall back to matching by column name

# Type coercion

A matched Parquet column whose physical type differs from the `physical_schema`
[`StructField`](../operations/buoyant_kernel.schema.StructField.md#op-ebbc3bdce7a671f7a7d1f58d) must be coerced to the requested type. In particular, timestamp columns MUST
be normalized to the protocol specified microsecond precision: a `TIMESTAMP(MILLIS)` (or
any other non-microsecond unit) column read into a `TIMESTAMP` / `TIMESTAMP_NTZ` field
must be rescaled to microseconds (a finer unit such as nanosecond is truncated). The
default engine does this via `arrow::compute::cast` while reordering columns to the
requested schema.

# Metadata Columns

The ParquetHandler must support virtual metadata columns that provide additional information
about each row. These columns are not stored in the Parquet file but are generated at read
time.

## Row Index Column

When a column in `physical_schema` is marked as a row index metadata column (via
[`StructField::create_metadata_column`](../operations/buoyant_kernel.schema.StructField.md#op-8ab7b61e79cb25d86c9adfd3) with [`schema::MetadataColumnSpec::RowIndex`](../operations/buoyant_kernel.schema.MetadataColumnSpec.md#op-1427ded1f654fc5b4ab200ba)), the
ParquetHandler must populate it with the 0-based row position within the Parquet file:

- **Column name**: User-specified (commonly `"row_index"` or `"_metadata.row_index"`)
- **Type**: `LONG` (non-nullable)
- **Values**: Sequential integers starting at 0 for each file
- **Use case**: Track row positions for downstream processing, or internally used to compute
  Row IDs

Example: A file with 5 rows would have row_index values `[0, 1, 2, 3, 4]`.

## File Name Column (Reserved Field ID)

When a column in `physical_schema` has the reserved field ID
[`reserved_field_ids::FILE_NAME`](../operations/buoyant_kernel.reserved_field_ids.FILE_NAME.md#op-a2b742a3c77ba475fda2c18b) (2147483646), the ParquetHandler must populate it
with the file path/name:

- **Column name**: `"_file"`
- **Type**: `STRING` (non-nullable)
- **Field ID**: 2147483646 (reserved)
- **Values**: The file path/URL (e.g., `"s3://bucket/path/file.parquet"`)
- **Use case**: Track which file each row came from in multi-file reads

Example: All rows from the same file would have the same `_file` value.

## Metadata Column Examples

```rust,ignore
# use buoyant_kernel as delta_kernel;
use delta_kernel::schema::{StructType, StructField, DataType, MetadataColumnSpec};

// Example 1: Schema with row_index metadata column
let schema_with_row_index = StructType::try_new([
    StructField::nullable("id", DataType::INTEGER),
    StructField::create_metadata_column("row_index", MetadataColumnSpec::RowIndex),
    StructField::nullable("value", DataType::STRING),
])?;

// Example 2: Schema with _file metadata column (using reserved field ID)
let schema_with_file_path = StructType::try_new([
    StructField::nullable("id", DataType::INTEGER),
    StructField::create_metadata_column("_file", MetadataColumnSpec::FilePath),
    StructField::nullable("value", DataType::STRING),
])?;
```

---

 If no matching Parquet column is found, `NULL` values are returned
 for nullable columns in `physical_schema`. For non-nullable columns, an error is returned.


## Column Matching Examples

Consider a `physical_schema` with the following fields:
- Column 0:  `"i_logical"` (integer, non-null) with field ID 1 (via
  [`ColumnMetadataKey::ParquetFieldId`])
- Column 1: `"s"` (string, nullable) with no field ID metadata
- Column 2: `"i2"` (integer, nullable) with no field ID metadata

[`ColumnMetadataKey::ParquetFieldId`]: crate::schema::ColumnMetadataKey::ParquetFieldId

And a Parquet file containing these columns:
- Column 0: `"i2"` (integer, nullable) with field ID 3
- Column 1: `"i"` (integer, non-null) with field ID 1
- No `"s"` column present

The column matching would work as follows:
- `"i_logical"` matches `"i"` by field ID (both have ID 1)
- `"i2"` matches `"i2"` by column name (no field ID to match on)
- `"s"` has no matching Parquet column, so NULL values are returned

The returned data will contain exactly 3 columns in physical schema order:
`{i_logical: parquet[1], s: NULL.., i2: parquet[0]}`

# Parameters

- `files` - File metadata for files to be read.
- `physical_schema` - Select list and order of columns to read from the Parquet file.
- `predicate` - Optional push-down predicate hint (engine is free to ignore it).

# Returns
A [`DeltaResult`](../operations/buoyant_kernel.error.DeltaResult.md#op-3db788f17aa90cfeefaa890f) containing a [`FileDataReadResultIterator`](../operations/buoyant_kernel.FileDataReadResultIterator.md#op-2c38562abfe66da38e1b8f0e).
Each element of the iterator is a [`DeltaResult`](../operations/buoyant_kernel.error.DeltaResult.md#op-3db788f17aa90cfeefaa890f) of [`EngineData`](../operations/buoyant_kernel.engine_data.EngineData.md#op-f9036ab52623c01f96e1f809). The [`EngineData`](../operations/buoyant_kernel.engine_data.EngineData.md#op-f9036ab52623c01f96e1f809)
has the contents of `files` and must match the provided `physical_schema`.

Note: The [`FileDataReadResultIterator`](../operations/buoyant_kernel.FileDataReadResultIterator.md#op-2c38562abfe66da38e1b8f0e) must emit data from files in the order that `files`
is given. For example if files ["a", "b"] is provided, then the engine data iterator must
first return all the engine data from file "a", _then_ all the engine data from file "b".
Moreover, for a given file, all of its [`EngineData`](../operations/buoyant_kernel.engine_data.EngineData.md#op-f9036ab52623c01f96e1f809) and constituent rows must be in order
that they occur in the file. Consider a file with rows
(1, 2, 3). The following are legal iterator batches:
   iter: [EngineData(1, 2), EngineData(3)]
   iter: [EngineData(1), EngineData(2, 3)]
   iter: [EngineData(1, 2, 3)]
The following are illegal batches:
   iter: [EngineData(3), EngineData(1, 2)]
   iter: [EngineData(1), EngineData(3, 2)]
   iter: [EngineData(2, 1, 3)]

Additionally, engines must not merge engine data across file boundaries.

[`ColumnMetadataKey::ParquetFieldId`]: crate::schema::ColumnMetadataKey

<a id="op-aeee3285fa1beca493453464"></a>
## read_parquet_footer

`function` · `buoyant_kernel::ParquetHandler::read_parquet_footer` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_parquet_footer(&self, file: &FileMeta) -> DeltaResult<ParquetFooter>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L969).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:969`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Read the footer metadata from a Parquet file without reading the data.

This method reads only the Parquet file footer (metadata section), which is useful for
schema inspection, compatibility checking, and determining whether parsed statistics
columns are present and compatible with the current table schema.

# Parameters

- `file` - File metadata for the Parquet file whose footer should be read. The `size` field
  should contain the actual file size to enable efficient footer reads without additional
  I/O operations.

# Returns

A [`DeltaResult`](../operations/buoyant_kernel.error.DeltaResult.md#op-3db788f17aa90cfeefaa890f) containing a [`ParquetFooter`](../operations/buoyant_kernel.ParquetFooter.md#op-b5dbf337ed7e73bfbf58e27d) with the Parquet file's metadata, including
the schema converted to Delta Kernel's format.

# Field IDs

If the Parquet file contains field IDs (written when column mapping is enabled), they are
preserved in each [`StructField`]'s metadata. Callers can access field IDs via
[`StructField::get_config_value`] with [`ColumnMetadataKey::ParquetFieldId`].

# Errors

Returns an error if:
- The file cannot be accessed or does not exist
- The file is not a valid Parquet file
- The footer cannot be read or parsed
- The schema cannot be converted to Delta Kernel's format

[`StructField`]: crate::schema::StructField
[`StructField::get_config_value`]: crate::schema::StructField::get_config_value
[`ColumnMetadataKey::ParquetFieldId`]: crate::schema::ColumnMetadataKey::ParquetFieldId

<a id="op-2219be84eb11bd25e10fde13"></a>
## write_parquet_file

`function` · `buoyant_kernel::ParquetHandler::write_parquet_file` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn write_parquet_file(&self, location: url::Url, data: DeltaResultIteratorStatic<Box<dyn EngineData>>) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L929).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:929`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Write data to a Parquet file at the specified URL.

This method writes the provided `data` to a Parquet file at the given `url`.

This will overwrite the file if it already exists. For filesystem-backed
implementations, the parent directories must be created if they do not exist.

# Parquet field IDs

The engine must write a Parquet `field_id` correctly when the kernel
[`StructField`] carries a field-id related annotation, including:
- [`ColumnMetadataKey::ColumnMappingId`] / [`ColumnMetadataKey::ParquetFieldId`]
- [`ColumnMetadataKey::ColumnMappingNestedIds`]

For how to use these keys, refer to the Delta protocol's [Column Mapping] and
[IcebergCompatV2] sections.

**Non-compliance produces files with incorrect `field_id`s**, which may lead to
read failures when column mapping mode is `id` and to failures when converting
the table to Iceberg.

# Parameters

- `url` - The full URL path where the Parquet file should be written (e.g.,
  `s3://bucket/path/file.parquet`).
- `data` - An iterator of engine data to be written to the Parquet file.

# Returns

A [`DeltaResult`](../operations/buoyant_kernel.error.DeltaResult.md#op-3db788f17aa90cfeefaa890f) indicating success or failure.

[`StructField`]: crate::schema::StructField
[`ColumnMetadataKey::ColumnMappingId`]: crate::schema::ColumnMetadataKey::ColumnMappingId
[`ColumnMetadataKey::ParquetFieldId`]: crate::schema::ColumnMetadataKey::ParquetFieldId
[`ColumnMetadataKey::ColumnMappingNestedIds`]: crate::schema::ColumnMetadataKey::ColumnMappingNestedIds
[Column Mapping]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#column-mapping
[IcebergCompatV2]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#iceberg-compatibility-v2
