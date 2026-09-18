# `datafusion_catalog_listing::helpers::parse_partitions_for_path`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog_listing.helpers.parse_partitions_for_path.json).

<a id="op-e4721de1634026541d10c56c"></a>
## parse_partitions_for_path

`function` · `datafusion_catalog_listing::helpers::parse_partitions_for_path` · datafusion-catalog-listing 55.1.0

```rust
fn parse_partitions_for_path<'a, I>(table_path: &datafusion_datasource::ListingTableUrl, file_path: &'a object_store::path::Path, table_partition_cols: I) -> Option<Vec<std::borrow::Cow<'a, str>>> where I: IntoIterator<Item = &'a str>
```

Source: `src/helpers.rs:466`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Extract the partition values for the given `file_path` (in the given `table_path`)
associated to the partitions defined by `table_partition_cols`.

Partition values are percent-decoded to match Hive-style object-store paths
that encode special characters in path segments.
