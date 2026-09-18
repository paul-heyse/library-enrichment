# `datafusion_physical_plan::projection::join_allows_pushdown`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.projection.join_allows_pushdown.json).

<a id="op-8bd85a4d50dd6b7714b43c5a"></a>
## join_allows_pushdown

`function` · `datafusion_physical_plan::projection::join_allows_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn join_allows_pushdown(projection_as_columns: &[(super::expressions::Column, String)], join_schema: &arrow::datatypes::SchemaRef, far_right_left_col_ind: i32, far_left_right_col_ind: i32) -> bool
```

Source: `src/projection.rs:1193`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Checks three conditions for pushing a projection down through a join:
- Projection must narrow the join output schema.
- Columns coming from left/right tables must be collected at the left/right
  sides of the output table.
- Left or right table is not lost after the projection.
