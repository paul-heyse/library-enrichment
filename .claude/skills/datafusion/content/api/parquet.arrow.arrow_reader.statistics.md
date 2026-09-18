# `parquet::arrow::arrow_reader::statistics`

Crate `parquet` · 1 public items · structured records in [`model/parquet.arrow.arrow_reader.statistics.json`](../model/parquet.arrow.arrow_reader.statistics.json)

## StatisticsConverter

`struct` · `parquet::arrow::arrow_reader::statistics::StatisticsConverter`

```rust
struct StatisticsConverter<'a>
```

**Derives**: Debug

**Methods** (15)

```rust
fn arrow_field(&self) -> &'a Field
fn data_page_maxes<I>(&self, column_page_index: &ParquetColumnIndex, column_offset_index: &ParquetOffsetIndex, row_group_indices: I) -> Result<ArrayRef> where I: IntoIterator<Item = &'a usize>
fn data_page_mins<I>(&self, column_page_index: &ParquetColumnIndex, column_offset_index: &ParquetOffsetIndex, row_group_indices: I) -> Result<ArrayRef> where I: IntoIterator<Item = &'a usize>
fn data_page_null_counts<I>(&self, column_page_index: &ParquetColumnIndex, column_offset_index: &ParquetOffsetIndex, row_group_indices: I) -> Result<UInt64Array> where I: IntoIterator<Item = &'a usize>
fn data_page_row_counts<I>(&self, column_offset_index: &ParquetOffsetIndex, row_group_metadatas: &'a [RowGroupMetaData], row_group_indices: I) -> Result<Option<UInt64Array>> where I: IntoIterator<Item = &'a usize>
fn from_column_index(parquet_column_index: usize, arrow_field: &'a Field, parquet_schema: &'a SchemaDescriptor) -> Result<Self>
fn parquet_column_index(&self) -> Option<usize>
fn row_group_is_max_value_exact<I>(&self, metadatas: I) -> Result<BooleanArray> where I: IntoIterator<Item = &'a RowGroupMetaData>
fn row_group_is_min_value_exact<I>(&self, metadatas: I) -> Result<BooleanArray> where I: IntoIterator<Item = &'a RowGroupMetaData>
fn row_group_maxes<I>(&self, metadatas: I) -> Result<ArrayRef> where I: IntoIterator<Item = &'a RowGroupMetaData>
fn row_group_mins<I>(&self, metadatas: I) -> Result<ArrayRef> where I: IntoIterator<Item = &'a RowGroupMetaData>
fn row_group_null_counts<I>(&self, metadatas: I) -> Result<UInt64Array> where I: IntoIterator<Item = &'a RowGroupMetaData>
fn row_group_row_counts<I>(&self, metadatas: I) -> Result<Option<UInt64Array>> where I: IntoIterator<Item = &'a RowGroupMetaData>
fn try_new<'b>(column_name: &'b str, arrow_schema: &'a Schema, parquet_schema: &'a SchemaDescriptor) -> Result<Self>
fn with_missing_null_counts_as_zero(self, missing_null_counts_as_zero: bool) -> Self
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_reader.statistics.StatisticsConverter.md).


Extracts Parquet statistics as Arrow arrays

This is used to convert Parquet statistics to Arrow [`ArrayRef`], with
proper type conversions. This information can be used for pruning Parquet
files, row groups, and data pages based on the statistics embedded in
Parquet metadata.

# Schemas

The converter uses the schema of the Parquet file and the Arrow schema to
convert the underlying statistics value (stored as a parquet value) into the
corresponding Arrow value. For example, Decimals are stored as binary in
parquet files and this structure handles mapping them to the `i128`
representation used in Arrow.

Note: The Parquet schema and Arrow schema do not have to be identical (for
example, the columns may be in different orders and one or the other schemas
may have additional columns). The function [`parquet_column`] is used to
match the column in the Parquet schema to the column in the Arrow schema
when using [`Self::try_new`]. For nested fields (e.g., struct fields),
where `parquet_column` does not support schema resolution, use
[`Self::from_column_index`] instead with a pre-resolved leaf column index.

---
