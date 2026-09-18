# `datafusion::physical_planner::create_window_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.physical_planner.create_window_expr.json).

<a id="op-efac2770f3d61dc737e6f62e"></a>
## create_window_expr

`function` · `datafusion::physical_planner::create_window_expr` · datafusion 55.1.0

```rust
fn create_window_expr(e: &logical_expr::Expr, logical_schema: &datafusion_common::DFSchema, execution_props: &execution::context::ExecutionProps, planning_ctx: &datafusion_expr::physical_planning_context::PhysicalPlanningContext) -> error::Result<std::sync::Arc<dyn WindowExpr>>
```

Source: `src/physical_planner.rs:2510`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create a window expression from a logical expression or an alias

See [`create_physical_expr`](../operations/datafusion_physical_expr.planner.create_physical_expr.md#op-b01a3449753ecee1419f6104) for details on the `planning_ctx` argument.
