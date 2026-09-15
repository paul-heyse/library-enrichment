# `datafusion_functions_nested::planner`

Crate `datafusion-functions-nested` · 2 public items · structured records in [`model/datafusion_functions_nested.planner.json`](../model/datafusion_functions_nested.planner.json)

## FieldAccessPlanner

`struct` · `datafusion_functions_nested::planner::FieldAccessPlanner`

```rust
struct FieldAccessPlanner
```

**Implements**: `datafusion_expr::planner::ExprPlanner`

**Derives**: Debug

**via `datafusion_expr::planner::ExprPlanner`**

```rust
fn plan_field_access(&self, expr: RawFieldAccessExpr, schema: &DFSchema) -> Result<PlannerResult<RawFieldAccessExpr>>
```

---

## NestedFunctionPlanner

`struct` · `datafusion_functions_nested::planner::NestedFunctionPlanner`

```rust
struct NestedFunctionPlanner
```

**Implements**: `datafusion_expr::planner::ExprPlanner`

**Derives**: Debug

**via `datafusion_expr::planner::ExprPlanner`**

```rust
fn plan_array_literal(&self, exprs: Vec<Expr>, _schema: &DFSchema) -> Result<PlannerResult<Vec<Expr>>>
fn plan_binary_op(&self, expr: RawBinaryExpr, schema: &DFSchema) -> Result<PlannerResult<RawBinaryExpr>>
fn plan_make_map(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
```

---
