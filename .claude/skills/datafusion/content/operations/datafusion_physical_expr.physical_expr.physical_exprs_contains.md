# `datafusion_physical_expr::physical_expr::physical_exprs_contains`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.physical_expr.physical_exprs_contains.json).

<a id="op-04081a219791aae994e0f7fa"></a>
## physical_exprs_contains

`function` · `datafusion_physical_expr::physical_expr::physical_exprs_contains` · datafusion-physical-expr 55.1.0

```rust
fn physical_exprs_contains(physical_exprs: &[std::sync::Arc<dyn PhysicalExpr>], expr: &std::sync::Arc<dyn PhysicalExpr>) -> bool
```

Source: `src/physical_expr.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This function is similar to the `contains` method of `Vec`. It finds
whether `expr` is among `physical_exprs`.
