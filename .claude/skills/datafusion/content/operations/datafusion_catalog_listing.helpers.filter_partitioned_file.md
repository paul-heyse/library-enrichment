# `datafusion_catalog_listing::helpers::filter_partitioned_file`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog_listing.helpers.filter_partitioned_file.json).

<a id="op-30ad6bfd4b8dc48c223939a9"></a>
## filter_partitioned_file

`function` · `datafusion_catalog_listing::helpers::filter_partitioned_file` · datafusion-catalog-listing 55.1.0

```rust
fn filter_partitioned_file(pf: datafusion_datasource::PartitionedFile, filters: &[datafusion_expr::Expr], df_schema: &datafusion_common::DFSchema) -> datafusion_common::Result<Option<datafusion_datasource::PartitionedFile>>
```

Source: `src/helpers.rs:311`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
