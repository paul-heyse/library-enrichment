# `datafusion_functions_window::planner`

Crate `datafusion-functions-window` · 1 public items · structured records in [`model/datafusion_functions_window.planner.json`](../model/datafusion_functions_window.planner.json)

## WindowFunctionPlanner

`struct` · `datafusion_functions_window::planner::WindowFunctionPlanner`

```rust
struct WindowFunctionPlanner
```

**Implements**: `datafusion_expr::planner::ExprPlanner`

**Derives**: Debug

**via `datafusion_expr::planner::ExprPlanner`**

```rust
fn plan_window(&self, raw_expr: RawWindowExpr) -> Result<PlannerResult<RawWindowExpr>>
```

---
