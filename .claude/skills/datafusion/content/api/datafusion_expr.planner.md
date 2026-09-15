# `datafusion_expr::planner`

Crate `datafusion-expr` · 13 public items · structured records in [`model/datafusion_expr.planner.json`](../model/datafusion_expr.planner.json)

## PlannerResult

`enum` · `datafusion_expr::planner::PlannerResult`

```rust
enum PlannerResult<T>
```

**Variants**: `Planned`, `Original`

**Derives**: Clone, Debug

Result of planning a raw expr with [`ExprPlanner`]

---

## RelationPlanning

`enum` · `datafusion_expr::planner::RelationPlanning`

```rust
enum RelationPlanning
```

**Variants**: `Planned`, `Original`

**Derives**: Debug

Result of attempting to plan a relation with extension planners

---

## PlannedRelation

`struct` · `datafusion_expr::planner::PlannedRelation`

```rust
struct PlannedRelation
```

**Fields**: `plan`, `alias`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(plan: LogicalPlan, alias: Option<TableAlias>) -> Self
```

Result of planning a relation with [`RelationPlanner`]

---

## RawAggregateExpr

`struct` · `datafusion_expr::planner::RawAggregateExpr`

```rust
struct RawAggregateExpr
```

**Fields**: `func`, `args`, `distinct`, `filter`, `order_by`, `null_treatment`

**Derives**: Clone, Debug

This structure is used by `AggregateFunctionPlanner` to plan operators with
custom expressions.

---

## RawBinaryExpr

`struct` · `datafusion_expr::planner::RawBinaryExpr`

```rust
struct RawBinaryExpr
```

**Fields**: `op`, `left`, `right`

**Derives**: Clone, Debug

An operator with two arguments to plan

Note `left` and `right` are DataFusion [`Expr`]s but the `op` is the SQL AST
operator.

This structure is used by [`ExprPlanner`] to plan operators with
custom expressions.

---

## RawDictionaryExpr

`struct` · `datafusion_expr::planner::RawDictionaryExpr`

```rust
struct RawDictionaryExpr
```

**Fields**: `keys`, `values`

**Derives**: Clone, Debug

A Dictionary literal expression `{ key: value, ...}`

This structure is used by [`ExprPlanner`] to plan operators with
custom expressions.

---

## RawFieldAccessExpr

`struct` · `datafusion_expr::planner::RawFieldAccessExpr`

```rust
struct RawFieldAccessExpr
```

**Fields**: `field_access`, `expr`

**Derives**: Clone, Debug

An expression with GetFieldAccess to plan

This structure is used by [`ExprPlanner`] to plan operators with
custom expressions.

---

## RawWindowExpr

`struct` · `datafusion_expr::planner::RawWindowExpr`

```rust
struct RawWindowExpr
```

**Fields**: `func_def`, `args`, `partition_by`, `order_by`, `window_frame`, `filter`, `null_treatment`, `distinct`

**Derives**: Clone, Debug

This structure is used by `WindowFunctionPlanner` to plan operators with
custom expressions.

---

## ContextProvider

`trait` · `datafusion_expr::planner::ContextProvider`

Also reachable as `datafusion_sql::planner::ContextProvider`

```rust
trait ContextProvider
```

**Methods** (18)

```rust
fn create_cte_work_table(&self, _name: &str, _schema: SchemaRef) -> Result<Arc<dyn TableSource>>
fn get_aggregate_meta(&self, name: &str) -> Option<Arc<AggregateUDF>>
fn get_expr_planners(&self) -> &[Arc<dyn ExprPlanner>]
fn get_file_type(&self, _ext: &str) -> Result<Arc<dyn FileType>>
fn get_function_meta(&self, name: &str) -> Option<Arc<ScalarUDF>>
fn get_higher_order_meta(&self, name: &str) -> Option<Arc<HigherOrderUDF>>
fn get_relation_planners(&self) -> &[Arc<dyn RelationPlanner>]
fn get_table_function_source(&self, _name: &str, _args: Vec<Expr>) -> Result<Arc<dyn TableSource>>
fn get_table_source(&self, name: TableReference) -> Result<Arc<dyn TableSource>>
fn get_type_planner(&self) -> Option<Arc<dyn TypePlanner>>
fn get_variable_field(&self, variable_names: &[String]) -> Option<FieldRef>
fn get_variable_type(&self, variable_names: &[String]) -> Option<DataType>
fn get_window_meta(&self, name: &str) -> Option<Arc<WindowUDF>>
fn higher_order_function_names(&self) -> Vec<String>
fn options(&self) -> &ConfigOptions
fn udaf_names(&self) -> Vec<String>
fn udf_names(&self) -> Vec<String>
fn udwf_names(&self) -> Vec<String>
```

Provides the `SQL` query planner meta-data about tables and
functions referenced in SQL statements, without a direct dependency on the
`datafusion` Catalog structures such as [`TableProvider`]

[`TableProvider`]: https://docs.rs/datafusion/latest/datafusion/catalog/trait.TableProvider.html

---

## ExprPlanner

`trait` · `datafusion_expr::planner::ExprPlanner`

```rust
trait ExprPlanner: Debug + Send + Sync
```

**Implementors** (9)

- `datafusion_functions::core::planner::CoreFunctionPlanner`
- `datafusion_functions::datetime::planner::DatetimeFunctionPlanner`
- `datafusion_functions::planner::UserDefinedFunctionPlanner`
- `datafusion_functions::unicode::planner::UnicodeFunctionPlanner`
- `datafusion_functions_aggregate::planner::AggregateFunctionPlanner`
- `datafusion_functions_nested::planner::FieldAccessPlanner`
- `datafusion_functions_nested::planner::NestedFunctionPlanner`
- `datafusion_functions_window::planner::WindowFunctionPlanner`
- `datafusion_spark::planner::SparkFunctionPlanner`

**Methods** (13)

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

Customize planning of SQL AST expressions to [`Expr`]s

For more background, please also see the [Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]

[Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]: https://datafusion.apache.org/blog/2026/01/12/extending-sql

---

## RelationPlanner

`trait` · `datafusion_expr::planner::RelationPlanner`

```rust
trait RelationPlanner: Debug + Send + Sync
```

**Methods** (1)

```rust
fn plan_relation(&self, relation: TableFactor, context: &mut dyn RelationPlannerContext) -> Result<RelationPlanning>
```

Customize planning SQL table factors to [`LogicalPlan`]s.
For more background, please also see the [Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]

[Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]: https://datafusion.apache.org/blog/2026/01/12/extending-sql

---

## RelationPlannerContext

`trait` · `datafusion_expr::planner::RelationPlannerContext`

```rust
trait RelationPlannerContext
```

**Methods** (6)

```rust
fn context_provider(&self) -> &dyn ContextProvider
fn normalize_ident(&self, ident: Ident) -> String
fn object_name_to_table_reference(&self, name: ObjectName) -> Result<TableReference>
fn plan(&mut self, relation: TableFactor) -> Result<LogicalPlan>
fn sql_expr_to_logical_expr(&mut self, expr: SQLExpr, schema: &DFSchema) -> Result<Expr>
fn sql_to_expr(&mut self, expr: SQLExpr, schema: &DFSchema) -> Result<Expr>
```

Provides utilities for relation planners to interact with DataFusion's SQL
planner.

This trait provides SQL planning utilities specific to relation planning,
such as converting SQL expressions to logical expressions and normalizing
identifiers. It uses composition to provide access to session context via
[`ContextProvider`].

---

## TypePlanner

`trait` · `datafusion_expr::planner::TypePlanner`

```rust
trait TypePlanner: Debug + Send + Sync
```

**Methods** (2)

```rust
fn plan_type(&self, _sql_type: &sqlparser::ast::DataType) -> Result<Option<DataType>>
fn plan_type_field(&self, sql_type: &sqlparser::ast::DataType) -> Result<Option<FieldRef>>
```

Customize planning SQL types to DataFusion (Arrow) types.
For more background, please also see the [Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]

[Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]: https://datafusion.apache.org/blog/2026/01/12/extending-sql

---
