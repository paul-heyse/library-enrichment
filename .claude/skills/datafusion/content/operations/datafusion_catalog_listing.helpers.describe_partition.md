# `datafusion_catalog_listing::helpers::describe_partition`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog_listing.helpers.describe_partition.json).

<a id="op-b8f572af20b0e6541a8ceec8"></a>
## describe_partition

`function` · `datafusion_catalog_listing::helpers::describe_partition` · datafusion-catalog-listing 55.1.0

```rust
fn describe_partition(partition: &Partition) -> (&str, usize, Vec<&str>)
```

Source: `src/helpers.rs:497`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Describe a partition as a (path, depth, files) tuple for easier assertions
