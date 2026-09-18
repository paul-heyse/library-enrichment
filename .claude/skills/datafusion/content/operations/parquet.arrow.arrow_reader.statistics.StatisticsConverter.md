# `parquet::arrow::arrow_reader::statistics::StatisticsConverter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.statistics.StatisticsConverter.json).

<a id="op-c25ef84c92a9151936db232f"></a>
## StatisticsConverter

`struct` · `parquet::arrow::arrow_reader::statistics::StatisticsConverter` · parquet 59.3.0

```rust
struct StatisticsConverter<'a>
```

Source: `src/arrow/arrow_reader/statistics.rs:1434`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Extracts Parquet statistics as Arrow arrays

This is used to convert Parquet statistics to Arrow [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1), with
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
may have additional columns). The function [`parquet_column`](../operations/parquet.arrow.parquet_column.md#op-9f56d57daf07ae08d43a786a) is used to
match the column in the Parquet schema to the column in the Arrow schema
when using [`Self::try_new`](../operations/parquet.arrow.arrow_reader.statistics.StatisticsConverter.md#op-a51fa8889c4a893e0e25ea0c). For nested fields (e.g., struct fields),
where `parquet_column` does not support schema resolution, use
[`Self::from_column_index`](../operations/parquet.arrow.arrow_reader.statistics.StatisticsConverter.md#op-a600bac1031176d86e3f176e) instead with a pre-resolved leaf column index.

<a id="op-f926d9ab5df068ece7f2f244"></a>
## arrow_field

`function` · `parquet::arrow::arrow_reader::statistics::StatisticsConverter::arrow_field` · parquet 59.3.0

```rust
fn arrow_field(&self) -> &'a Field
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::statistics::StatisticsConverter", "path": "StatisticsConverter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1445, 1], "end": [1983, 2], "filename": "src/arrow/arrow_reader/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/statistics.rs:1455`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return the arrow schema's [`Field]` of the column in the Arrow schema

<a id="op-817e7548027f1c1acf64baaa"></a>
## data_page_maxes

`function` · `parquet::arrow::arrow_reader::statistics::StatisticsConverter::data_page_maxes` · parquet 59.3.0

```rust
fn data_page_maxes<I>(&self, column_page_index: &ParquetColumnIndex, column_offset_index: &ParquetOffsetIndex, row_group_indices: I) -> Result<ArrayRef> where I: IntoIterator<Item = &'a usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::statistics::StatisticsConverter", "path": "StatisticsConverter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1445, 1], "end": [1983, 2], "filename": "src/arrow/arrow_reader/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/statistics.rs:1855`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Extract the maximum values from Data Page statistics.

See docs on [`Self::data_page_mins`](../operations/parquet.arrow.arrow_reader.statistics.StatisticsConverter.md#op-7acb72c25f30276f779caa7b) for details.

<a id="op-7acb72c25f30276f779caa7b"></a>
## data_page_mins

`function` · `parquet::arrow::arrow_reader::statistics::StatisticsConverter::data_page_mins` · parquet 59.3.0

```rust
fn data_page_mins<I>(&self, column_page_index: &ParquetColumnIndex, column_offset_index: &ParquetOffsetIndex, row_group_indices: I) -> Result<ArrayRef> where I: IntoIterator<Item = &'a usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::statistics::StatisticsConverter", "path": "StatisticsConverter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1445, 1], "end": [1983, 2], "filename": "src/arrow/arrow_reader/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/statistics.rs:1824`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Extract the minimum values from Data Page statistics.

In Parquet files, in addition to the Column Chunk level statistics
(stored for each column for each row group) there are also
optional statistics stored for each data page, as part of
the [`ParquetColumnIndex`](../operations/parquet.file.metadata.ParquetColumnIndex.md#op-80ab0cca370795f8250c6d69).

Since a single Column Chunk is stored as one or more pages,
page level statistics can prune at a finer granularity.

However since they are stored in a separate metadata
structure ([`ColumnIndexMetaData`](../operations/parquet.file.page_index.column_index.ColumnIndexMetaData.md#op-6700acf40da7fd4eab386f3e)) there is different code to extract them as
compared to arrow statistics.

# Parameters:

* `column_page_index`: The parquet column page indices, read from
  `ParquetMetaData` column_index

* `column_offset_index`: The parquet column offset indices, read from
  `ParquetMetaData` offset_index

* `row_group_indices`: The indices of the row groups, that are used to
  extract the column page index and offset index on a per row group
  per column basis.

# Return Value

The returned array contains 1 value for each `NativeIndex`
in the underlying `Index`es, in the same order as they appear
in `metadatas`.

For example, if there are two `Index`es in `metadatas`:
1. the first having `3` `PageIndex` entries
2. the second having `2` `PageIndex` entries

The returned array would have 5 rows.

Each value is either:
* the minimum value for the page
* a null value, if the statistics can not be extracted

Note that a null value does NOT mean the min value was actually
`null` it means it the requested statistic is unknown

# Errors

Reasons for not being able to extract the statistics include:
* the column is not present in the parquet file
* statistics for the pages are not present in the row group
* the stored statistic value can not be converted to the requested type

<a id="op-f6b728599ab5238c8059ae25"></a>
## data_page_null_counts

`function` · `parquet::arrow::arrow_reader::statistics::StatisticsConverter::data_page_null_counts` · parquet 59.3.0

```rust
fn data_page_null_counts<I>(&self, column_page_index: &ParquetColumnIndex, column_offset_index: &ParquetOffsetIndex, row_group_indices: I) -> Result<UInt64Array> where I: IntoIterator<Item = &'a usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::statistics::StatisticsConverter", "path": "StatisticsConverter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1445, 1], "end": [1983, 2], "filename": "src/arrow/arrow_reader/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/statistics.rs:1886`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a [`UInt64Array`](../operations/arrow_array.array.primitive_array.UInt64Array.md#op-47e7828263162b82ce927cad) with null counts for each data page.

See docs on [`Self::data_page_mins`](../operations/parquet.arrow.arrow_reader.statistics.StatisticsConverter.md#op-7acb72c25f30276f779caa7b) for details.

<a id="op-00f3ebf32c72e9fe11bcca08"></a>
## data_page_row_counts

`function` · `parquet::arrow::arrow_reader::statistics::StatisticsConverter::data_page_row_counts` · parquet 59.3.0

```rust
fn data_page_row_counts<I>(&self, column_offset_index: &ParquetOffsetIndex, row_group_metadatas: &'a [RowGroupMetaData], row_group_indices: I) -> Result<Option<UInt64Array>> where I: IntoIterator<Item = &'a usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::statistics::StatisticsConverter", "path": "StatisticsConverter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1445, 1], "end": [1983, 2], "filename": "src/arrow/arrow_reader/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/statistics.rs:1929`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a [`UInt64Array`](../operations/arrow_array.array.primitive_array.UInt64Array.md#op-47e7828263162b82ce927cad) with row counts for each data page.

This function iterates over the given row group indexes and computes
the row count for each page in the specified column.

# Parameters:

* `column_offset_index`: The parquet column offset indices, read from
  `ParquetMetaData` offset_index

* `row_group_metadatas`: The metadata slice of the row groups, read
  from `ParquetMetaData` row_groups

* `row_group_indices`: The indices of the row groups, that are used to
  extract the column offset index on a per row group per column basis.

See docs on [`Self::data_page_mins`](../operations/parquet.arrow.arrow_reader.statistics.StatisticsConverter.md#op-7acb72c25f30276f779caa7b) for details.

<a id="op-eb0763e6613c7886c077d41e"></a>
## fmt

`function` · `parquet::arrow::arrow_reader::statistics::StatisticsConverter::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::statistics::StatisticsConverter", "path": "StatisticsConverter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1433, 10], "end": [1433, 15], "filename": "src/arrow/arrow_reader/statistics.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/arrow_reader/statistics.rs:1433`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a600bac1031176d86e3f176e"></a>
## from_column_index

`function` · `parquet::arrow::arrow_reader::statistics::StatisticsConverter::from_column_index` · parquet 59.3.0

```rust
fn from_column_index(parquet_column_index: usize, arrow_field: &'a Field, parquet_schema: &'a SchemaDescriptor) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::statistics::StatisticsConverter", "path": "StatisticsConverter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1445, 1], "end": [1983, 2], "filename": "src/arrow/arrow_reader/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/statistics.rs:1598`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new `StatisticsConverter` from a Parquet leaf column index directly.

Unlike [`Self::try_new`](../operations/parquet.arrow.arrow_reader.statistics.StatisticsConverter.md#op-a51fa8889c4a893e0e25ea0c), this constructor bypasses schema resolution and
accepts a Parquet column index directly. This is useful for nested fields
(e.g., struct fields) where the caller has already resolved the mapping
from the Arrow field to the Parquet leaf column.

# Arguments

* `parquet_column_index` - The index of the leaf column in the Parquet schema
* `arrow_field` - The Arrow field describing the column's data type
* `parquet_schema` - The Parquet schema descriptor (used to look up the physical type)

The caller must ensure that `arrow_field` describes the same leaf column as
`parquet_column_index`. This mapping is not validated by the converter; if
the Arrow type does not match the Parquet column statistics, extraction
returns null statistics values rather than an error.

# Errors

* If the `parquet_column_index` is out of bounds

<a id="op-a91bda3c51bd48a711515a65"></a>
## parquet_column_index

`function` · `parquet::arrow::arrow_reader::statistics::StatisticsConverter::parquet_column_index` · parquet 59.3.0

```rust
fn parquet_column_index(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::statistics::StatisticsConverter", "path": "StatisticsConverter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1445, 1], "end": [1983, 2], "filename": "src/arrow/arrow_reader/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/statistics.rs:1450`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return the index of the column in the Parquet schema, if any

Returns `None` if the column is was present in the Arrow schema, but not
present in the parquet file

<a id="op-db6118c6348665c7942f4e86"></a>
## row_group_is_max_value_exact

`function` · `parquet::arrow::arrow_reader::statistics::StatisticsConverter::row_group_is_max_value_exact` · parquet 59.3.0

```rust
fn row_group_is_max_value_exact<I>(&self, metadatas: I) -> Result<BooleanArray> where I: IntoIterator<Item = &'a RowGroupMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::statistics::StatisticsConverter", "path": "StatisticsConverter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1445, 1], "end": [1983, 2], "filename": "src/arrow/arrow_reader/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/statistics.rs:1702`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Extract the `is_max_value_exact` flags from row group statistics in [`RowGroupMetaData`](../operations/parquet.file.metadata.RowGroupMetaData.md#op-f84a9d3c17c9ccf2d622296f)

See docs on [`Self::row_group_maxes`](../operations/parquet.arrow.arrow_reader.statistics.StatisticsConverter.md#op-9dc5641ee85bc8a78b2d9a84) for details

<a id="op-a0abafd9688537d0bb89aec4"></a>
## row_group_is_min_value_exact

`function` · `parquet::arrow::arrow_reader::statistics::StatisticsConverter::row_group_is_min_value_exact` · parquet 59.3.0

```rust
fn row_group_is_min_value_exact<I>(&self, metadatas: I) -> Result<BooleanArray> where I: IntoIterator<Item = &'a RowGroupMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::statistics::StatisticsConverter", "path": "StatisticsConverter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1445, 1], "end": [1983, 2], "filename": "src/arrow/arrow_reader/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/statistics.rs:1724`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Extract the `is_min_value_exact` flags from row group statistics in [`RowGroupMetaData`](../operations/parquet.file.metadata.RowGroupMetaData.md#op-f84a9d3c17c9ccf2d622296f)

See docs on [`Self::row_group_mins`](../operations/parquet.arrow.arrow_reader.statistics.StatisticsConverter.md#op-9fef4ac9657dac4ccb90b9a3) for details

<a id="op-9dc5641ee85bc8a78b2d9a84"></a>
## row_group_maxes

`function` · `parquet::arrow::arrow_reader::statistics::StatisticsConverter::row_group_maxes` · parquet 59.3.0

```rust
fn row_group_maxes<I>(&self, metadatas: I) -> Result<ArrayRef> where I: IntoIterator<Item = &'a RowGroupMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::statistics::StatisticsConverter", "path": "StatisticsConverter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1445, 1], "end": [1983, 2], "filename": "src/arrow/arrow_reader/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/statistics.rs:1683`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Extract the maximum values from row group statistics in [`RowGroupMetaData`](../operations/parquet.file.metadata.RowGroupMetaData.md#op-f84a9d3c17c9ccf2d622296f)

See docs on [`Self::row_group_mins`](../operations/parquet.arrow.arrow_reader.statistics.StatisticsConverter.md#op-9fef4ac9657dac4ccb90b9a3) for details

<a id="op-9fef4ac9657dac4ccb90b9a3"></a>
## row_group_mins

`function` · `parquet::arrow::arrow_reader::statistics::StatisticsConverter::row_group_mins` · parquet 59.3.0

```rust
fn row_group_mins<I>(&self, metadatas: I) -> Result<ArrayRef> where I: IntoIterator<Item = &'a RowGroupMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::statistics::StatisticsConverter", "path": "StatisticsConverter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1445, 1], "end": [1983, 2], "filename": "src/arrow/arrow_reader/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/statistics.rs:1664`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Extract the minimum values from row group statistics in [`RowGroupMetaData`](../operations/parquet.file.metadata.RowGroupMetaData.md#op-f84a9d3c17c9ccf2d622296f)

# Return Value

The returned array contains 1 value for each row group, in the same order as `metadatas`

Each value is either
* the minimum value for the column
* a null value, if the statistics can not be extracted

Note that a null value does NOT mean the min value was actually
`null` it means it the requested statistic is unknown

# Errors

Reasons for not being able to extract the statistics include:
* the column is not present in the parquet file
* statistics for the column are not present in the row group
* the stored statistic value can not be converted to the requested type

# Example
```no_run
# use std::sync::Arc;
# use arrow::datatypes::Schema;
# use arrow_array::{ArrayRef, Float64Array};
# use parquet::arrow::arrow_reader::statistics::StatisticsConverter;
# use parquet::file::metadata::ParquetMetaData;
# fn get_parquet_metadata() -> ParquetMetaData { unimplemented!() }
# fn get_arrow_schema() -> Schema { unimplemented!() }
// Given the metadata for a parquet file and the arrow schema
let metadata: ParquetMetaData = get_parquet_metadata();
let arrow_schema: Schema = get_arrow_schema();
let parquet_schema = metadata.file_metadata().schema_descr();
// create a converter
let converter = StatisticsConverter::try_new("foo", &arrow_schema, parquet_schema)
  .unwrap();
// get the minimum value for the column "foo" in the parquet file
let min_values: ArrayRef = converter
  .row_group_mins(metadata.row_groups().iter())
  .unwrap();
// if "foo" is a Float64 value, the returned array will contain Float64 values
assert_eq!(min_values, Arc::new(Float64Array::from(vec![Some(1.0), Some(2.0)])) as _);
```

<a id="op-9bff88458cffd0044b061585"></a>
## row_group_null_counts

`function` · `parquet::arrow::arrow_reader::statistics::StatisticsConverter::row_group_null_counts` · parquet 59.3.0

```rust
fn row_group_null_counts<I>(&self, metadatas: I) -> Result<UInt64Array> where I: IntoIterator<Item = &'a RowGroupMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::statistics::StatisticsConverter", "path": "StatisticsConverter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1445, 1], "end": [1983, 2], "filename": "src/arrow/arrow_reader/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/statistics.rs:1746`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Extract the null counts from row group statistics in [`RowGroupMetaData`](../operations/parquet.file.metadata.RowGroupMetaData.md#op-f84a9d3c17c9ccf2d622296f)

See docs on [`Self::row_group_mins`](../operations/parquet.arrow.arrow_reader.statistics.StatisticsConverter.md#op-9fef4ac9657dac4ccb90b9a3) for details

<a id="op-44a5dee27ab6e2d5d8430d7c"></a>
## row_group_row_counts

`function` · `parquet::arrow::arrow_reader::statistics::StatisticsConverter::row_group_row_counts` · parquet 59.3.0

```rust
fn row_group_row_counts<I>(&self, metadatas: I) -> Result<Option<UInt64Array>> where I: IntoIterator<Item = &'a RowGroupMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::statistics::StatisticsConverter", "path": "StatisticsConverter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1445, 1], "end": [1983, 2], "filename": "src/arrow/arrow_reader/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/statistics.rs:1506`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a [`UInt64Array`](../operations/arrow_array.array.primitive_array.UInt64Array.md#op-47e7828263162b82ce927cad) with row counts for each row group

# Return Value

The returned array has no nulls, and has one value for each row group.
Each value is the number of rows in the row group.

# Example
```no_run
# use arrow::datatypes::Schema;
# use arrow_array::{ArrayRef, UInt64Array};
# use parquet::arrow::arrow_reader::statistics::StatisticsConverter;
# use parquet::file::metadata::ParquetMetaData;
# fn get_parquet_metadata() -> ParquetMetaData { unimplemented!() }
# fn get_arrow_schema() -> Schema { unimplemented!() }
// Given the metadata for a parquet file and the arrow schema
let metadata: ParquetMetaData = get_parquet_metadata();
let arrow_schema: Schema = get_arrow_schema();
let parquet_schema = metadata.file_metadata().schema_descr();
// create a converter
let converter = StatisticsConverter::try_new("foo", &arrow_schema, parquet_schema)
  .unwrap();
// get the row counts for each row group
let row_counts = converter.row_group_row_counts(metadata
  .row_groups()
  .iter()
).unwrap();
// file had 2 row groups, with 1024 and 23 rows respectively
assert_eq!(row_counts, Some(UInt64Array::from(vec![1024, 23])));
```

<a id="op-a51fa8889c4a893e0e25ea0c"></a>
## try_new

`function` · `parquet::arrow::arrow_reader::statistics::StatisticsConverter::try_new` · parquet 59.3.0

```rust
fn try_new<'b>(column_name: &'b str, arrow_schema: &'a Schema, parquet_schema: &'a SchemaDescriptor) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::statistics::StatisticsConverter", "path": "StatisticsConverter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1445, 1], "end": [1983, 2], "filename": "src/arrow/arrow_reader/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/statistics.rs:1541`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new `StatisticsConverter` to extract statistics for a column

Note if there is no corresponding column in the parquet file, the returned
arrays will be null. This can happen if the column is in the arrow
schema but not in the parquet schema due to schema evolution.

This constructor only supports top-level, non-nested columns. For nested
fields (e.g., fields within a struct), use [`Self::from_column_index`](../operations/parquet.arrow.arrow_reader.statistics.StatisticsConverter.md#op-a600bac1031176d86e3f176e).

See example on [`Self::row_group_mins`](../operations/parquet.arrow.arrow_reader.statistics.StatisticsConverter.md#op-9fef4ac9657dac4ccb90b9a3) for usage

# Errors

* If the column is not found in the arrow schema

<a id="op-d73216b8b784e5a9725f8e1f"></a>
## with_missing_null_counts_as_zero

`function` · `parquet::arrow::arrow_reader::statistics::StatisticsConverter::with_missing_null_counts_as_zero` · parquet 59.3.0

```rust
fn with_missing_null_counts_as_zero(self, missing_null_counts_as_zero: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::statistics::StatisticsConverter", "path": "StatisticsConverter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1445, 1], "end": [1983, 2], "filename": "src/arrow/arrow_reader/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/statistics.rs:1471`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set the statistics converter to treat missing null counts as missing

By default, the converter will treat missing null counts as though
the null count is known to be `0`.

Note that parquet files written by parquet-rs currently do not store
null counts even when it is known there are zero nulls, and the reader
will return 0 for the null counts in that instance. This behavior may
change in a future release.

Both parquet-java and parquet-cpp store null counts as 0 when there are
no nulls, and don't write unknown values to the null count field.
