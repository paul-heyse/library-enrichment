# `datafusion_expr::planner::ExprPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.planner.ExprPlanner.json).

<a id="op-c0ce2d948f3fc34627ff15dd"></a>
## ExprPlanner

`trait` · `datafusion_expr::planner::ExprPlanner` · datafusion-expr 55.1.0

```rust
trait ExprPlanner: Debug + Send + Sync
```

Source: `src/planner.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Customize planning of SQL AST expressions to [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc)s

For more background, please also see the [Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]

[Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]: https://datafusion.apache.org/blog/2026/01/12/extending-sql

<a id="op-7006a7617f2a9087809ed90d"></a>
## plan_aggregate

`function` · `datafusion_expr::planner::ExprPlanner::plan_aggregate` · datafusion-expr 55.1.0

```rust
fn plan_aggregate(&self, expr: RawAggregateExpr) -> Result<PlannerResult<RawAggregateExpr>>
```

Source: `src/planner.rs:269`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Plans aggregate functions, such as `COUNT(<expr>)`

Returns original expression arguments if not possible

<a id="op-32c89fe8e692d104b39b9dc0"></a>
## plan_array_literal

`function` · `datafusion_expr::planner::ExprPlanner::plan_array_literal` · datafusion-expr 55.1.0

```rust
fn plan_array_literal(&self, exprs: Vec<Expr>, _schema: &DFSchema) -> Result<PlannerResult<Vec<Expr>>>
```

Source: `src/planner.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Plan an array literal, such as `[1, 2, 3]`

Returns original expression arguments if not possible

<a id="op-67676022796971b606982f4d"></a>
## plan_binary_op

`function` · `datafusion_expr::planner::ExprPlanner::plan_binary_op` · datafusion-expr 55.1.0

```rust
fn plan_binary_op(&self, expr: RawBinaryExpr, _schema: &DFSchema) -> Result<PlannerResult<RawBinaryExpr>>
```

Source: `src/planner.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Plan the binary operation between two expressions, returns original
BinaryExpr if not possible

<a id="op-986bd566e6e25f83eab10ab8"></a>
## plan_compound_identifier

`function` · `datafusion_expr::planner::ExprPlanner::plan_compound_identifier` · datafusion-expr 55.1.0

```rust
fn plan_compound_identifier(&self, _field: &Field, _qualifier: Option<&TableReference>, _nested_names: &[String]) -> Result<PlannerResult<Vec<Expr>>>
```

Source: `src/planner.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Plans compound identifier such as `db.schema.table` for non-empty nested names

# Note:
Currently compound identifier for outer query schema is not supported.

Returns original expression if not possible

<a id="op-118ff07075ec06bff44fc2a2"></a>
## plan_dictionary_literal

`function` · `datafusion_expr::planner::ExprPlanner::plan_dictionary_literal` · datafusion-expr 55.1.0

```rust
fn plan_dictionary_literal(&self, expr: RawDictionaryExpr, _schema: &DFSchema) -> Result<PlannerResult<RawDictionaryExpr>>
```

Source: `src/planner.rs:199`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Plan a dictionary literal, such as `{ key: value, ...}`

Returns original expression arguments if not possible

<a id="op-5a161f90a23f4d030b259548"></a>
## plan_extract

`function` · `datafusion_expr::planner::ExprPlanner::plan_extract` · datafusion-expr 55.1.0

```rust
fn plan_extract(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
```

Source: `src/planner.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Plan an extract expression, such as`EXTRACT(month FROM foo)`

Returns original expression arguments if not possible

<a id="op-fd422e4fd68976a3f9e7d676"></a>
## plan_field_access

`function` · `datafusion_expr::planner::ExprPlanner::plan_field_access` · datafusion-expr 55.1.0

```rust
fn plan_field_access(&self, expr: RawFieldAccessExpr, _schema: &DFSchema) -> Result<PlannerResult<RawFieldAccessExpr>>
```

Source: `src/planner.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Plan the field access expression, such as `foo.bar`

returns original [`RawFieldAccessExpr`](../operations/datafusion_expr.planner.RawFieldAccessExpr.md#op-a93d4058b51c70a9d498ab77) if not possible

<a id="op-4c33861a21551647c60beeb9"></a>
## plan_make_map

`function` · `datafusion_expr::planner::ExprPlanner::plan_make_map` · datafusion-expr 55.1.0

```rust
fn plan_make_map(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
```

Source: `src/planner.rs:245`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Plans a `make_map` expression, such as `make_map(key1, value1, key2, value2, ...)`

Returns original expression arguments if not possible

<a id="op-673f57fe161d83e084c631c0"></a>
## plan_overlay

`function` · `datafusion_expr::planner::ExprPlanner::plan_overlay` · datafusion-expr 55.1.0

```rust
fn plan_overlay(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
```

Source: `src/planner.rs:238`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Plans an overlay expression, such as `overlay(str PLACING substr FROM pos [FOR count])`

Returns original expression arguments if not possible

<a id="op-c7b71e02aa9619480b150013"></a>
## plan_position

`function` · `datafusion_expr::planner::ExprPlanner::plan_position` · datafusion-expr 55.1.0

```rust
fn plan_position(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
```

Source: `src/planner.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Plan a `POSITION` expression, such as `POSITION(<expr> in <expr>)`

Returns original expression arguments if not possible

<a id="op-bc90d92e631ab706f4cb2422"></a>
## plan_struct_literal

`function` · `datafusion_expr::planner::ExprPlanner::plan_struct_literal` · datafusion-expr 55.1.0

```rust
fn plan_struct_literal(&self, args: Vec<Expr>, _is_named_struct: bool) -> Result<PlannerResult<Vec<Expr>>>
```

Source: `src/planner.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Plans a struct literal, such as  `{'field1' : expr1, 'field2' : expr2, ...}`

This function takes a vector of expressions and a boolean flag
indicating whether the struct uses the optional name

Returns the original input expressions if planning is not possible.

<a id="op-dd6af6e9cb597ff9d0896de6"></a>
## plan_substring

`function` · `datafusion_expr::planner::ExprPlanner::plan_substring` · datafusion-expr 55.1.0

```rust
fn plan_substring(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
```

Source: `src/planner.rs:217`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Plan an substring expression, such as `SUBSTRING(<expr> [FROM <expr>] [FOR <expr>])`

Returns original expression arguments if not possible

<a id="op-b5126b7f68c09eeba1aa23a0"></a>
## plan_window

`function` · `datafusion_expr::planner::ExprPlanner::plan_window` · datafusion-expr 55.1.0

```rust
fn plan_window(&self, expr: RawWindowExpr) -> Result<PlannerResult<RawWindowExpr>>
```

Source: `src/planner.rs:279`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Plans window functions, such as `COUNT(<expr>)`

Returns original expression arguments if not possible
