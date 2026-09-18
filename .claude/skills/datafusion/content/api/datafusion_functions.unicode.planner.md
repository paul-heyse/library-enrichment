# `datafusion_functions::unicode::planner`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.unicode.planner.json`](../model/datafusion_functions.unicode.planner.json)

## UnicodeFunctionPlanner

`struct` · `datafusion_functions::unicode::planner::UnicodeFunctionPlanner`

```rust
struct UnicodeFunctionPlanner
```

**Implements**: `datafusion_expr::planner::ExprPlanner`

**Derives**: Debug, Default

**via `datafusion_expr::planner::ExprPlanner`**

```rust
fn plan_position(&self, args: Vec<Expr>) -> datafusion_common::Result<PlannerResult<Vec<Expr>>>
fn plan_substring(&self, args: Vec<Expr>) -> datafusion_common::Result<PlannerResult<Vec<Expr>>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.unicode.planner.UnicodeFunctionPlanner.md).


---
