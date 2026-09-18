# `datafusion_catalog_listing::helpers::expr_applicable_for_cols`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog_listing.helpers.expr_applicable_for_cols.json).

<a id="op-7e4a8513ad99402fde7e67a2"></a>
## expr_applicable_for_cols

`function` · `datafusion_catalog_listing::helpers::expr_applicable_for_cols` · datafusion-catalog-listing 55.1.0

```rust
fn expr_applicable_for_cols(col_names: &[&str], expr: &datafusion_expr::Expr) -> bool
```

Source: `src/helpers.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Check whether the given expression can be resolved using only the columns `col_names`.
This means that if this function returns true:
- the table provider can filter the table partition values with this expression
- the expression can be marked as `TableProviderFilterPushDown::Exact` once this filtering
  was performed
