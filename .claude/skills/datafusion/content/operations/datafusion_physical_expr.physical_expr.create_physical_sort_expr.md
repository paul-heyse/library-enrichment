# `datafusion_physical_expr::physical_expr::create_physical_sort_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.physical_expr.create_physical_sort_expr.json).

<a id="op-79b6eb418f88c8570d81838a"></a>
## create_physical_sort_expr

`function` · `datafusion_physical_expr::physical_expr::create_physical_sort_expr` · datafusion-physical-expr 55.1.0

```rust
fn create_physical_sort_expr(e: &datafusion_expr::SortExpr, input_dfschema: &datafusion_common::DFSchema, execution_props: &datafusion_expr::execution_props::ExecutionProps, planning_ctx: &datafusion_expr::physical_planning_context::PhysicalPlanningContext) -> datafusion_common::Result<PhysicalSortExpr>
```

Source: `src/physical_expr.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a physical sort expression from a logical expression

See [`create_physical_expr`](../operations/datafusion_physical_expr.planner.create_physical_expr.md#op-b01a3449753ecee1419f6104) for details on the `planning_ctx` argument.
