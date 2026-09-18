# `datafusion_physical_plan::projection::update_join_on`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.projection.update_join_on.json).

<a id="op-077d533951ba4eb4803246f5"></a>
## update_join_on

`function` · `datafusion_physical_plan::projection::update_join_on` · datafusion-physical-plan 55.1.0

```rust
fn update_join_on(proj_left_exprs: &[(super::expressions::Column, String)], proj_right_exprs: &[(super::expressions::Column, String)], hash_join_on: &[(datafusion_physical_expr_common::physical_expr::PhysicalExprRef, datafusion_physical_expr_common::physical_expr::PhysicalExprRef)], left_field_size: usize) -> Option<Vec<(datafusion_physical_expr_common::physical_expr::PhysicalExprRef, datafusion_physical_expr_common::physical_expr::PhysicalExprRef)>>
```

Source: `src/projection.rs:1243`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Tries to update the equi-join `Column`'s of a join as if the input of
the join was replaced by a projection.
