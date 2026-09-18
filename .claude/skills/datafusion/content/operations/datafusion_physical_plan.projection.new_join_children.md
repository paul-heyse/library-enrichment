# `datafusion_physical_plan::projection::new_join_children`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.projection.new_join_children.json).

<a id="op-a00bc234062e36b701c6355f"></a>
## new_join_children

`function` · `datafusion_physical_plan::projection::new_join_children` · datafusion-physical-plan 55.1.0

```rust
fn new_join_children(projection_as_columns: &[(super::expressions::Column, String)], far_right_left_col_ind: i32, far_left_right_col_ind: i32, left_child: &std::sync::Arc<dyn ExecutionPlan>, right_child: &std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<(ProjectionExec, ProjectionExec)>
```

Source: `src/projection.rs:1123`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

If pushing down the projection over this join's children seems possible,
this function constructs the new [`ProjectionExec`](../operations/datafusion_physical_plan.projection.ProjectionExec.md#op-b46d9dc006ec8aae1caad158)s that will come on top
of the original children of the join.
