# `datafusion_functions::planner`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.planner.json`](../model/datafusion_functions.planner.json)

## UserDefinedFunctionPlanner

`struct` · `datafusion_functions::planner::UserDefinedFunctionPlanner`

> **Deprecated** — since 50.0.0: Use UnicodeFunctionPlanner and DateTimeFunctionPlanner instead

```rust
struct UserDefinedFunctionPlanner
```

**Implements**: `datafusion_expr::planner::ExprPlanner`

**Derives**: Debug, Default

**via `datafusion_expr::planner::ExprPlanner`**

```rust
fn plan_extract(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
fn plan_position(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
fn plan_substring(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.planner.UserDefinedFunctionPlanner.md).


---
