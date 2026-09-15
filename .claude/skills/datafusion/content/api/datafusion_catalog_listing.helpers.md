# `datafusion_catalog_listing::helpers`

Crate `datafusion-catalog-listing` · 8 public items · structured records in [`model/datafusion_catalog_listing.helpers.json`](../model/datafusion_catalog_listing.helpers.json)

## describe_partition

`function` · `datafusion_catalog_listing::helpers::describe_partition`

Also reachable as `datafusion::datasource::listing::helpers::describe_partition`

```rust
fn describe_partition(partition: &Partition) -> (&str, usize, Vec<&str>)
```

Describe a partition as a (path, depth, files) tuple for easier assertions

---

## evaluate_partition_prefix

`function` · `datafusion_catalog_listing::helpers::evaluate_partition_prefix`

Also reachable as `datafusion::datasource::listing::helpers::evaluate_partition_prefix`

```rust
fn evaluate_partition_prefix<'a>(partition_cols: &'a [(String, arrow::datatypes::DataType)], filters: &'a [datafusion_expr::Expr]) -> Option<object_store::path::Path>
```

---

## expr_applicable_for_cols

`function` · `datafusion_catalog_listing::helpers::expr_applicable_for_cols`

Also reachable as `datafusion::datasource::listing::helpers::expr_applicable_for_cols`

```rust
fn expr_applicable_for_cols(col_names: &[&str], expr: &datafusion_expr::Expr) -> bool
```

Check whether the given expression can be resolved using only the columns `col_names`.
This means that if this function returns true:
- the table provider can filter the table partition values with this expression
- the expression can be marked as `TableProviderFilterPushDown::Exact` once this filtering
  was performed

---

## filter_partitioned_file

`function` · `datafusion_catalog_listing::helpers::filter_partitioned_file`

Also reachable as `datafusion::datasource::listing::helpers::filter_partitioned_file`

```rust
fn filter_partitioned_file(pf: datafusion_datasource::PartitionedFile, filters: &[datafusion_expr::Expr], df_schema: &datafusion_common::DFSchema) -> datafusion_common::Result<Option<datafusion_datasource::PartitionedFile>>
```

---

## list_partitions

`function` · `datafusion_catalog_listing::helpers::list_partitions`

Also reachable as `datafusion::datasource::listing::helpers::list_partitions`

```rust
async fn list_partitions(store: &dyn ObjectStore, table_path: &datafusion_datasource::ListingTableUrl, max_depth: usize, partition_prefix: Option<object_store::path::Path>) -> datafusion_common::Result<Vec<Partition>>
```

Returns a recursive list of the partitions in `table_path` up to `max_depth`

---

## parse_partitions_for_path

`function` · `datafusion_catalog_listing::helpers::parse_partitions_for_path`

Also reachable as `datafusion::datasource::listing::helpers::parse_partitions_for_path`

```rust
fn parse_partitions_for_path<'a, I>(table_path: &datafusion_datasource::ListingTableUrl, file_path: &'a object_store::path::Path, table_partition_cols: I) -> Option<Vec<std::borrow::Cow<'a, str>>> where I: IntoIterator<Item = &'a str>
```

Extract the partition values for the given `file_path` (in the given `table_path`)
associated to the partitions defined by `table_partition_cols`.

Partition values are percent-decoded to match Hive-style object-store paths
that encode special characters in path segments.

---

## pruned_partition_list

`function` · `datafusion_catalog_listing::helpers::pruned_partition_list`

Also reachable as `datafusion::datasource::listing::helpers::pruned_partition_list`

```rust
async fn pruned_partition_list<'a>(ctx: &'a dyn Session, store: &'a dyn ObjectStore, table_path: &'a datafusion_datasource::ListingTableUrl, filters: &'a [datafusion_expr::Expr], file_extension: &'a str, partition_cols: &'a [(String, arrow::datatypes::DataType)]) -> datafusion_common::Result<futures::stream::BoxStream<'a, datafusion_common::Result<datafusion_datasource::PartitionedFile>>>
```

Discover the partitions on the given path and prune out files
that belong to irrelevant partitions using `filters` expressions.
`filters` should only contain expressions that can be evaluated
using only the partition columns.

---

## Partition

`struct` · `datafusion_catalog_listing::helpers::Partition`

Also reachable as `datafusion::datasource::listing::helpers::Partition`

```rust
struct Partition
```

**Derives**: Debug

---
