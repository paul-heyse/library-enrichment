# ExprPlanner

`datafusion_expr::planner::ExprPlanner`

```rust
trait ExprPlanner: Debug + Send + Sync
```

Prose: [`api/datafusion_expr.planner.md`](../api/datafusion_expr.planner.md#exprplanner) · records: [`model/datafusion_expr.planner.json`](../model/datafusion_expr.planner.json)

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
fn plan_aggregate(&self, expr: RawAggregateExpr) -> Result<PlannerResult<RawAggregateExpr>>
fn plan_array_literal(&self, exprs: Vec<Expr>, _schema: &DFSchema) -> Result<PlannerResult<Vec<Expr>>>
fn plan_binary_op(&self, expr: RawBinaryExpr, _schema: &DFSchema) -> Result<PlannerResult<RawBinaryExpr>>
fn plan_compound_identifier(&self, _field: &Field, _qualifier: Option<&TableReference>, _nested_names: &[String]) -> Result<PlannerResult<Vec<Expr>>>
fn plan_dictionary_literal(&self, expr: RawDictionaryExpr, _schema: &DFSchema) -> Result<PlannerResult<RawDictionaryExpr>>
fn plan_extract(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
fn plan_field_access(&self, expr: RawFieldAccessExpr, _schema: &DFSchema) -> Result<PlannerResult<RawFieldAccessExpr>>
fn plan_make_map(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
fn plan_overlay(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
fn plan_position(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
fn plan_struct_literal(&self, args: Vec<Expr>, _is_named_struct: bool) -> Result<PlannerResult<Vec<Expr>>>
fn plan_substring(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
fn plan_window(&self, expr: RawWindowExpr) -> Result<PlannerResult<RawWindowExpr>>
```

## Implementors (9)

Read one before writing your own.

- `datafusion_functions::core::planner::CoreFunctionPlanner`
- `datafusion_functions::datetime::planner::DatetimeFunctionPlanner`
- `datafusion_functions::planner::UserDefinedFunctionPlanner`
- `datafusion_functions::unicode::planner::UnicodeFunctionPlanner`
- `datafusion_functions_aggregate::planner::AggregateFunctionPlanner`
- `datafusion_functions_nested::planner::FieldAccessPlanner`
- `datafusion_functions_nested::planner::NestedFunctionPlanner`
- `datafusion_functions_window::planner::WindowFunctionPlanner`
- `datafusion_spark::planner::SparkFunctionPlanner`

## Documentation

Customize planning of SQL AST expressions to [`Expr`]s

For more background, please also see the [Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]

[Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]: https://datafusion.apache.org/blog/2026/01/12/extending-sql
