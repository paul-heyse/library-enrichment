# `datafusion_catalog_listing::helpers::list_partitions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog_listing.helpers.list_partitions.json).

<a id="op-fdb2780c29cbc7bfa0616f55"></a>
## list_partitions

`function` · `datafusion_catalog_listing::helpers::list_partitions` · datafusion-catalog-listing 55.1.0

```rust
async fn list_partitions(store: &dyn ObjectStore, table_path: &datafusion_datasource::ListingTableUrl, max_depth: usize, partition_prefix: Option<object_store::path::Path>) -> datafusion_common::Result<Vec<Partition>>
```

Source: `src/helpers.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Returns a recursive list of the partitions in `table_path` up to `max_depth`
