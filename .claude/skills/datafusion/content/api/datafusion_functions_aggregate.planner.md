# `datafusion_functions_aggregate::planner`

Crate `datafusion-functions-aggregate` · 1 public items · structured records in [`model/datafusion_functions_aggregate.planner.json`](../model/datafusion_functions_aggregate.planner.json)

## AggregateFunctionPlanner

`struct` · `datafusion_functions_aggregate::planner::AggregateFunctionPlanner`

```rust
struct AggregateFunctionPlanner
```

**Implements**: `datafusion_expr::planner::ExprPlanner`

**Derives**: Debug

**via `datafusion_expr::planner::ExprPlanner`**

```rust
fn plan_aggregate(&self, raw_expr: RawAggregateExpr) -> Result<PlannerResult<RawAggregateExpr>>
```

---
