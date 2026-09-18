# `datafusion_physical_expr::planner::create_physical_exprs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.planner.create_physical_exprs.json).

<a id="op-862cd17ab52e5025d57c674b"></a>
## create_physical_exprs

`function` · `datafusion_physical_expr::planner::create_physical_exprs` · datafusion-physical-expr 55.1.0

```rust
fn create_physical_exprs<'a, I>(exprs: I, input_dfschema: &datafusion_common::DFSchema, execution_props: &datafusion_expr::execution_props::ExecutionProps, planning_ctx: &datafusion_expr::physical_planning_context::PhysicalPlanningContext) -> datafusion_common::Result<Vec<std::sync::Arc<dyn PhysicalExpr>>> where I: IntoIterator<Item = &'a datafusion_expr::Expr>
```

Source: `src/planner.rs:700`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create vector of Physical Expression from a vector of logical expression

See [`create_physical_expr`](../operations/datafusion_physical_expr.planner.create_physical_expr.md#op-b01a3449753ecee1419f6104) for details on the `planning_ctx` argument.
