# `datafusion_catalog_listing::helpers::evaluate_partition_prefix`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog_listing.helpers.evaluate_partition_prefix.json).

<a id="op-e048fbbd48d518d5327867b4"></a>
## evaluate_partition_prefix

`function` · `datafusion_catalog_listing::helpers::evaluate_partition_prefix` · datafusion-catalog-listing 55.1.0

```rust
fn evaluate_partition_prefix<'a>(partition_cols: &'a [(String, arrow::datatypes::DataType)], filters: &'a [datafusion_expr::Expr]) -> Option<object_store::path::Path>
```

Source: `src/helpers.rs:262`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
