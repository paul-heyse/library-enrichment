# `datafusion_physical_plan::projection::all_alias_free_columns`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.projection.all_alias_free_columns.json).

<a id="op-b719746f3118a15904e0654c"></a>
## all_alias_free_columns

`function` · `datafusion_physical_plan::projection::all_alias_free_columns` · datafusion-physical-plan 55.1.0

```rust
fn all_alias_free_columns(exprs: &[ProjectionExpr]) -> bool
```

Source: `src/projection.rs:1021`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Given the expression set of a projection, checks if the projection causes
any renaming or constructs a non-`Column` physical expression.
