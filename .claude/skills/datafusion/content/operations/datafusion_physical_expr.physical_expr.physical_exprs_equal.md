# `datafusion_physical_expr::physical_expr::physical_exprs_equal`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.physical_expr.physical_exprs_equal.json).

<a id="op-f948bd2a3ade6acfea294841"></a>
## physical_exprs_equal

`function` · `datafusion_physical_expr::physical_expr::physical_exprs_equal` · datafusion-physical-expr 55.1.0

```rust
fn physical_exprs_equal(lhs: &[std::sync::Arc<dyn PhysicalExpr>], rhs: &[std::sync::Arc<dyn PhysicalExpr>]) -> bool
```

Source: `src/physical_expr.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Checks whether the given physical expression slices are equal.
