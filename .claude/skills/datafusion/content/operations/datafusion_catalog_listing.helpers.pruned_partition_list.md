# `datafusion_catalog_listing::helpers::pruned_partition_list`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog_listing.helpers.pruned_partition_list.json).

<a id="op-652e88cda6934cf394f07d52"></a>
## pruned_partition_list

`function` · `datafusion_catalog_listing::helpers::pruned_partition_list` · datafusion-catalog-listing 55.1.0

```rust
async fn pruned_partition_list<'a>(ctx: &'a dyn Session, store: &'a dyn ObjectStore, table_path: &'a datafusion_datasource::ListingTableUrl, filters: &'a [datafusion_expr::Expr], file_extension: &'a str, partition_cols: &'a [(String, arrow::datatypes::DataType)]) -> datafusion_common::Result<futures::stream::BoxStream<'a, datafusion_common::Result<datafusion_datasource::PartitionedFile>>>
```

Source: `src/helpers.rs:385`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Discover the partitions on the given path and prune out files
that belong to irrelevant partitions using `filters` expressions.
`filters` should only contain expressions that can be evaluated
using only the partition columns.
