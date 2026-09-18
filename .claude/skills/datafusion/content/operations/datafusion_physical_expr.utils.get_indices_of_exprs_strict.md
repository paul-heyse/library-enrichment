# `datafusion_physical_expr::utils::get_indices_of_exprs_strict`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.utils.get_indices_of_exprs_strict.json).

<a id="op-25c51267034c3c90ef52b32b"></a>
## get_indices_of_exprs_strict

`function` · `datafusion_physical_expr::utils::get_indices_of_exprs_strict` · datafusion-physical-expr 55.1.0

```rust
fn get_indices_of_exprs_strict<T: Borrow<std::sync::Arc<dyn PhysicalExpr>>>(targets: impl IntoIterator<Item = T>, items: &[std::sync::Arc<dyn PhysicalExpr>]) -> Vec<usize>
```

Source: `src/utils/mod.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This function finds the indices of `targets` within `items` using strict
equality.
