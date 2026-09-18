# `datafusion_functions::core::planner`

Crate `datafusion-functions` · 1 public items · structured records in [`model/datafusion_functions.core.planner.json`](../model/datafusion_functions.core.planner.json)

## CoreFunctionPlanner

`struct` · `datafusion_functions::core::planner::CoreFunctionPlanner`

```rust
struct CoreFunctionPlanner
```

**Implements**: `datafusion_expr::planner::ExprPlanner`

**Derives**: Debug, Default

**via `datafusion_expr::planner::ExprPlanner`**

```rust
fn plan_compound_identifier(&self, field: &Field, qualifier: Option<&TableReference>, nested_names: &[String]) -> Result<PlannerResult<Vec<Expr>>>
fn plan_dictionary_literal(&self, expr: RawDictionaryExpr, _schema: &DFSchema) -> Result<PlannerResult<RawDictionaryExpr>>
fn plan_overlay(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
fn plan_struct_literal(&self, args: Vec<Expr>, is_named_struct: bool) -> Result<PlannerResult<Vec<Expr>>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.core.planner.CoreFunctionPlanner.md).


---
