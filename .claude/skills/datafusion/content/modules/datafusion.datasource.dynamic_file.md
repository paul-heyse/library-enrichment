# `datafusion::datasource::dynamic_file`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.datasource.dynamic_file.json).

<a id="op-492e6fc51bdc9259992bb983"></a>
## dynamic_file

`module` · `datafusion::datasource::dynamic_file` · datafusion 55.1.0

```rust
mod dynamic_file
```

Source: `src/datasource/dynamic_file.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

dynamic_file_schema contains an [`UrlTableFactory`](../operations/datafusion_catalog.dynamic_file.catalog.UrlTableFactory.md#op-f86a61dc7e8f7beddfbc8d27) implementation that
can create a [`ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a) from the given url.
