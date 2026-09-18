# `datafusion::physical_planner::create_aggregate_expr_with_name_and_maybe_filter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.physical_planner.create_aggregate_expr_with_name_and_maybe_filter.json).

<a id="op-dd770b55953151429e30f57c"></a>
## create_aggregate_expr_with_name_and_maybe_filter

`function` · `datafusion::physical_planner::create_aggregate_expr_with_name_and_maybe_filter` · datafusion 55.1.0

```rust
fn create_aggregate_expr_with_name_and_maybe_filter(e: &logical_expr::Expr, name: Option<String>, human_display: String, logical_input_schema: &datafusion_common::DFSchema, physical_input_schema: &arrow::datatypes::Schema, execution_props: &execution::context::ExecutionProps) -> error::Result<(std::sync::Arc<datafusion_physical_expr::aggregate::AggregateFunctionExpr>, Option<std::sync::Arc<dyn PhysicalExpr>>, Vec<datafusion_physical_expr::PhysicalSortExpr>)>
```

Source: `src/physical_planner.rs:2537`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create an aggregate expression with a name from a logical expression
