# `datafusion::physical_planner::create_window_expr_with_name`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.physical_planner.create_window_expr_with_name.json).

<a id="op-e2863620d828f26934c77a51"></a>
## create_window_expr_with_name

`function` · `datafusion::physical_planner::create_window_expr_with_name` · datafusion 55.1.0

```rust
fn create_window_expr_with_name(e: &logical_expr::Expr, name: impl Into<String>, logical_schema: &datafusion_common::DFSchema, execution_props: &execution::context::ExecutionProps, planning_ctx: &datafusion_expr::physical_planning_context::PhysicalPlanningContext) -> error::Result<std::sync::Arc<dyn WindowExpr>>
```

Source: `src/physical_planner.rs:2429`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create a window expression with a name from a logical expression

See [`create_physical_expr`](../operations/datafusion_physical_expr.planner.create_physical_expr.md#op-b01a3449753ecee1419f6104) for details on the `planning_ctx` argument.
