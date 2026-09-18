# `datafusion_physical_plan::projection::all_columns`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.projection.all_columns.json).

<a id="op-28129bca0ca99cfdbdb6d339"></a>
## all_columns

`function` · `datafusion_physical_plan::projection::all_columns` · datafusion-physical-plan 55.1.0

```rust
fn all_columns(exprs: &[ProjectionExpr]) -> bool
```

Source: `src/projection.rs:1064`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns `true` if all the expressions in the argument are `Column`s.
