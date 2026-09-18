# `datafusion_spark::planner`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.planner.json`](../model/datafusion_spark.planner.json)

## SparkFunctionPlanner

`struct` · `datafusion_spark::planner::SparkFunctionPlanner`

```rust
struct SparkFunctionPlanner
```

**Implements**: `datafusion_expr::planner::ExprPlanner`

**Derives**: Debug, Default

**via `datafusion_expr::planner::ExprPlanner`**

```rust
fn plan_extract(&self, args: Vec<Expr>) -> datafusion_common::Result<PlannerResult<Vec<Expr>>>
fn plan_substring(&self, args: Vec<Expr>) -> datafusion_common::Result<PlannerResult<Vec<Expr>>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.planner.SparkFunctionPlanner.md).


---
