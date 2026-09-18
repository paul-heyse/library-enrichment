# `datafusion_physical_plan::projection::physical_to_column_exprs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.projection.physical_to_column_exprs.json).

<a id="op-ab90b1212a94b31983d5a447"></a>
## physical_to_column_exprs

`function` · `datafusion_physical_plan::projection::physical_to_column_exprs` · datafusion-physical-plan 55.1.0

```rust
fn physical_to_column_exprs(exprs: &[ProjectionExpr]) -> Option<Vec<(super::expressions::Column, String)>>
```

Source: `src/projection.rs:1106`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Downcasts all the expressions in `exprs` to `Column`s. If any of the given
expressions is not a `Column`, returns `None`.
