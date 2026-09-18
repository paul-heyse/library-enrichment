# `datafusion_physical_plan::aggregates::aggregate_expressions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.aggregate_expressions.json).

<a id="op-fbeda7f6b717b52fe9f87315"></a>
## aggregate_expressions

`function` · `datafusion_physical_plan::aggregates::aggregate_expressions` · datafusion-physical-plan 55.1.0

```rust
fn aggregate_expressions(aggr_expr: &[std::sync::Arc<datafusion_physical_expr::aggregate::AggregateFunctionExpr>], mode: &AggregateMode, col_idx_base: usize) -> datafusion_common::Result<Vec<Vec<std::sync::Arc<dyn PhysicalExpr>>>>
```

Source: `src/aggregates/mod.rs:2875`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns physical expressions for arguments to evaluate against a batch.

The expressions are different depending on `mode`:
* Partial: AggregateFunctionExpr::expressions
* Final: columns of `AggregateFunctionExpr::state_fields()`
