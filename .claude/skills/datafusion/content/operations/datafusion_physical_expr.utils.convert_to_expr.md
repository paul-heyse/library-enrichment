# `datafusion_physical_expr::utils::convert_to_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.utils.convert_to_expr.json).

<a id="op-decd1a268cf1991e1195db60"></a>
## convert_to_expr

`function` · `datafusion_physical_expr::utils::convert_to_expr` · datafusion-physical-expr 55.1.0

```rust
fn convert_to_expr<T: Borrow<PhysicalSortExpr>>(sequence: impl IntoIterator<Item = T>) -> Vec<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/utils/mod.rs:194`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This function returns all `Arc<dyn PhysicalExpr>`s inside the given
`PhysicalSortExpr` sequence.
