# `datafusion_physical_plan::projection::new_projections_for_columns`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.projection.new_projections_for_columns.json).

<a id="op-9bc1fa459314f3f0fef008d1"></a>
## new_projections_for_columns

`function` · `datafusion_physical_plan::projection::new_projections_for_columns` · datafusion-physical-plan 55.1.0

```rust
fn new_projections_for_columns(projection: &[ProjectionExpr], source: &[usize]) -> Vec<usize>
```

Source: `src/projection.rs:1034`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Updates a source provider's projected columns according to the given
projection operator's expressions. To use this function safely, one must
ensure that all expressions are `Column` expressions without aliases.
