# `datafusion::physical_planner::create_aggregate_expr_and_maybe_filter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.physical_planner.create_aggregate_expr_and_maybe_filter.json).

<a id="op-a7a90c87d4d4fcd5ef18e47a"></a>
## create_aggregate_expr_and_maybe_filter

`function` · `datafusion::physical_planner::create_aggregate_expr_and_maybe_filter` · datafusion 55.1.0

```rust
fn create_aggregate_expr_and_maybe_filter(e: &logical_expr::Expr, logical_input_schema: &datafusion_common::DFSchema, physical_input_schema: &arrow::datatypes::Schema, execution_props: &execution::context::ExecutionProps) -> error::Result<(std::sync::Arc<datafusion_physical_expr::aggregate::AggregateFunctionExpr>, Option<std::sync::Arc<dyn PhysicalExpr>>, Vec<datafusion_physical_expr::PhysicalSortExpr>)>
```

Source: `src/physical_planner.rs:2564`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create an aggregate expression from a logical expression or an alias
