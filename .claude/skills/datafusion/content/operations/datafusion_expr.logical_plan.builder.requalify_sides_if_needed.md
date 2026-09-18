# `datafusion_expr::logical_plan::builder::requalify_sides_if_needed`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.builder.requalify_sides_if_needed.json).

<a id="op-1bc532f2cd379630707dbb82"></a>
## requalify_sides_if_needed

`function` · `datafusion_expr::logical_plan::builder::requalify_sides_if_needed` · datafusion-expr 55.1.0

```rust
fn requalify_sides_if_needed(left: LogicalPlanBuilder, right: LogicalPlanBuilder) -> datafusion_common::Result<(LogicalPlanBuilder, LogicalPlanBuilder, bool)>
```

Source: `src/logical_plan/builder.rs:1788`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

(Re)qualify the sides of a join if needed, i.e. if the columns from one side would otherwise
conflict with the columns from the other.
This is especially useful for queries that come as Substrait, since Substrait doesn't currently allow specifying
aliases, neither for columns nor for tables.  DataFusion requires columns to be uniquely identifiable, in some
places (see e.g. DFSchema::check_names).
The function returns:
- The requalified or original left logical plan
- The requalified or original right logical plan
- If a requalification was needed or not
