# `datafusion_datasource_parquet::access_plan`

Crate `datafusion-datasource-parquet` · 3 public items · structured records in [`model/datafusion_datasource_parquet.access_plan.json`](../model/datafusion_datasource_parquet.access_plan.json)

## RowGroupAccess

`enum` · `datafusion_datasource_parquet::access_plan::RowGroupAccess`

Also reachable as `datafusion::datasource::physical_plan::parquet::RowGroupAccess`, `datafusion_datasource_parquet::RowGroupAccess`

```rust
enum RowGroupAccess
```

**Variants**: `Skip`, `Scan`, `Selection`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn should_scan(&self) -> bool
```

Describes how the parquet reader will access a row group

---

## ParquetAccessPlan

`struct` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan`

Also reachable as `datafusion::datasource::physical_plan::parquet::ParquetAccessPlan`, `datafusion_datasource_parquet::ParquetAccessPlan`

```rust
struct ParquetAccessPlan
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (16)

```rust
fn inner(&self) -> &[RowGroupAccess]
fn into_inner(self) -> Vec<RowGroupAccess>
fn into_overall_row_selection(self, row_group_meta_data: &[RowGroupMetaData]) -> Result<Option<RowSelection>>
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn new(row_groups: Vec<RowGroupAccess>) -> Self
fn new_all(row_group_count: usize) -> Self
fn new_none(row_group_count: usize) -> Self
fn row_group_index_iter(&self) -> impl Iterator<Item = usize> + '_
fn row_group_indexes(&self) -> Vec<usize>
fn scan(&mut self, idx: usize)
fn scan_selection(&mut self, idx: usize, selection: RowSelection)
fn set(&mut self, idx: usize, access: RowGroupAccess)
fn should_scan(&self, idx: usize) -> bool
fn skip(&mut self, idx: usize)
fn try_new_from_overall_row_selection(selection: RowSelection, row_group_meta_data: &[RowGroupMetaData]) -> Result<Self>
```

A selection of rows and row groups within a ParquetFile to decode.

A `ParquetAccessPlan` is used to limit the row groups and data pages a `DataSourceExec`
will read and decode to improve performance.

Note that page level pruning based on ArrowPredicate is applied after all of
these selections

# Example

For example, given a Parquet file with 4 row groups, a `ParquetAccessPlan`
can be used to specify skipping row group 0 and 2, scanning a range of rows
in row group 1, and scanning all rows in row group 3 as follows:

```rust
# use parquet::arrow::arrow_reader::{RowSelection, RowSelector};
# use datafusion_datasource_parquet::ParquetAccessPlan;
// Default to scan all row groups
let mut access_plan = ParquetAccessPlan::new_all(4);
access_plan.skip(0); // skip row group
// Use parquet reader RowSelector to specify scanning rows 100-200 and 350-400
// in a row group that has 1000 rows
let row_selection = RowSelection::from(vec![
   RowSelector::skip(100),
   RowSelector::select(100),
   RowSelector::skip(150),
   RowSelector::select(50),
   RowSelector::skip(600),  // skip last 600 rows
]);
access_plan.scan_selection(1, row_selection);
access_plan.skip(2); // skip row group 2
// row group 3 is scanned by default
```

The resulting plan would look like:

```text
┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐

│                   │  SKIP

└ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘
 Row Group 0
┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
 ┌────────────────┐    SCAN ONLY ROWS
│└────────────────┘ │  100-200
 ┌────────────────┐    350-400
│└────────────────┘ │
 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
 Row Group 1
┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
                       SKIP
│                   │

└ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘
 Row Group 2
┌───────────────────┐
│                   │  SCAN ALL ROWS
│                   │
│                   │
└───────────────────┘
 Row Group 3
```

For more background, please also see the [Embedding User-Defined Indexes in Apache Parquet Files blog]

[Embedding User-Defined Indexes in Apache Parquet Files blog]: https://datafusion.apache.org/blog/2025/07/14/user-defined-parquet-indexes

---

## ParquetRowSelection

`struct` · `datafusion_datasource_parquet::access_plan::ParquetRowSelection`

Also reachable as `datafusion::datasource::physical_plan::parquet::ParquetRowSelection`, `datafusion_datasource_parquet::ParquetRowSelection`

```rust
struct ParquetRowSelection
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn into_inner(self) -> RowSelection
fn new(selection: RowSelection) -> Self
fn selection(&self) -> &RowSelection
```

**via `core::convert::From`**

```rust
fn from(selection: RowSelection) -> Self
```

A file-level row selection for a parquet scan.

Attach this type to a [`PartitionedFile`](datafusion_datasource::PartitionedFile)
with [`PartitionedFile::with_extension`](datafusion_datasource::PartitionedFile::with_extension)
when an external index produces a [`RowSelection`] across the entire parquet
file. DataFusion will use parquet metadata to split it into row-group-level
access when the file is opened.

---
