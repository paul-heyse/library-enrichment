# `datafusion_sql::planner::SqlToRel`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.planner.SqlToRel.json).

<a id="op-60df62ca6d594410f7745e1c"></a>
## SqlToRel

`struct` · `datafusion_sql::planner::SqlToRel` · datafusion-sql 55.1.0

```rust
struct SqlToRel<'a, S: ContextProvider>
```

Source: `src/planner.rs:454`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

SQL query planner and binder

This struct is used to convert a SQL AST into a [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da).

You can control the behavior of the planner by providing [`ParserOptions`](../operations/datafusion_sql.planner.ParserOptions.md#op-334a25f33d603162ddd6667c).

It performs the following tasks:

1. Name and type resolution (called "binding" in other systems). This
   phase looks up table and column names using the [`ContextProvider`](../operations/datafusion_expr.planner.ContextProvider.md#op-97af42065a69e4e56202d0d2).
2. Mechanical translation of the AST into a [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da).

It does not perform type coercion, or perform optimization, which are done
by subsequent passes.

Key interfaces are:
* [`Self::sql_statement_to_plan`](../operations/datafusion_sql.planner.SqlToRel.md#op-50bf54bdf45fab8ca6c8d0ba): Convert a statement
  (e.g. `SELECT ...`) into a [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da)
* [`Self::sql_to_expr`](../operations/datafusion_sql.planner.SqlToRel.md#op-8db68b37224ff29b99e7799b): Convert an expression (e.g. `1 + 2`) into an [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc)

<a id="op-dcf9f64990f3b26f6c536a0f"></a>
## build_schema

`function` · `datafusion_sql::planner::SqlToRel::build_schema` · datafusion-sql 55.1.0

```rust
fn build_schema(&self, columns: Vec<SQLColumnDef>) -> Result<Schema>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "datafusion_sql::planner::SqlToRel", "path": "SqlToRel"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_expr::planner::ContextProvider", "path": "ContextProvider"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [461, 1], "end": [926, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:502`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77dd04fb58ebc47ebe079007"></a>
## new

`function` · `datafusion_sql::planner::SqlToRel::new` · datafusion-sql 55.1.0

```rust
fn new(context_provider: &'a S) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "datafusion_sql::planner::SqlToRel", "path": "SqlToRel"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_expr::planner::ContextProvider", "path": "ContextProvider"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [461, 1], "end": [926, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:465`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Create a new query planner.

The query planner derives the parser options from the context provider.

<a id="op-5471738b769b5056aa2431f3"></a>
## new_constraint_from_table_constraints

`function` · `datafusion_sql::planner::SqlToRel::new_constraint_from_table_constraints` · datafusion-sql 55.1.0

```rust
fn new_constraint_from_table_constraints(&self, constraints: &[TableConstraint], df_schema: &DFSchemaRef) -> Result<Constraints>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "datafusion_sql::planner::SqlToRel", "path": "crate::planner::SqlToRel"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_expr::planner::ContextProvider", "path": "ContextProvider"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [3190, 2], "filename": "src/statement.rs"}, "trait": null, "trait_path": null}`

Source: `src/statement.rs:1921`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Convert each [TableConstraint](../operations/sqlparser.ast.table_constraints.TableConstraint.md#op-6cf7ad3c767f63a7e0b2a59a) to corresponding [Constraint](../operations/datafusion_common.functional_dependencies.Constraint.md#op-6a1d40180d911ee98e5b60ee)

<a id="op-b0c05c4f687db493489241ac"></a>
## new_with_options

`function` · `datafusion_sql::planner::SqlToRel::new_with_options` · datafusion-sql 55.1.0

```rust
fn new_with_options(context_provider: &'a S, options: ParserOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "datafusion_sql::planner::SqlToRel", "path": "SqlToRel"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_expr::planner::ContextProvider", "path": "ContextProvider"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [461, 1], "end": [926, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:474`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Create a new query planner with the given parser options.

The query planner ignores the parser options from the context provider
and uses the given parser options instead.

<a id="op-50bf54bdf45fab8ca6c8d0ba"></a>
## sql_statement_to_plan

`function` · `datafusion_sql::planner::SqlToRel::sql_statement_to_plan` · datafusion-sql 55.1.0

```rust
fn sql_statement_to_plan(&self, statement: Statement) -> Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "datafusion_sql::planner::SqlToRel", "path": "crate::planner::SqlToRel"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_expr::planner::ContextProvider", "path": "ContextProvider"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [3190, 2], "filename": "src/statement.rs"}, "trait": null, "trait_path": null}`

Source: `src/statement.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Generate a logical plan from an SQL statement

<a id="op-f3722c6a740ee16dead8069f"></a>
## sql_statement_to_plan_with_context

`function` · `datafusion_sql::planner::SqlToRel::sql_statement_to_plan_with_context` · datafusion-sql 55.1.0

```rust
fn sql_statement_to_plan_with_context(&self, statement: Statement, planner_context: &mut PlannerContext) -> Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "datafusion_sql::planner::SqlToRel", "path": "crate::planner::SqlToRel"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_expr::planner::ContextProvider", "path": "ContextProvider"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [3190, 2], "filename": "src/statement.rs"}, "trait": null, "trait_path": null}`

Source: `src/statement.rs:250`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Generate a logical plan from an SQL statement

<a id="op-8db68b37224ff29b99e7799b"></a>
## sql_to_expr

`function` · `datafusion_sql::planner::SqlToRel::sql_to_expr` · datafusion-sql 55.1.0

```rust
fn sql_to_expr(&self, sql: SQLExpr, schema: &DFSchema, planner_context: &mut PlannerContext) -> Result<Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "datafusion_sql::planner::SqlToRel", "path": "crate::planner::SqlToRel"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_expr::planner::ContextProvider", "path": "ContextProvider"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [1324, 2], "filename": "src/expr/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr/mod.rs:245`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Generate a relational expression from a SQL expression

<a id="op-c77fe4efc14627a8d978ba7b"></a>
## sql_to_expr_with_alias

`function` · `datafusion_sql::planner::SqlToRel::sql_to_expr_with_alias` · datafusion-sql 55.1.0

```rust
fn sql_to_expr_with_alias(&self, sql: SQLExprWithAlias, schema: &DFSchema, planner_context: &mut PlannerContext) -> Result<Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "datafusion_sql::planner::SqlToRel", "path": "crate::planner::SqlToRel"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_expr::planner::ContextProvider", "path": "ContextProvider"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [1324, 2], "filename": "src/expr/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr/mod.rs:230`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75b535c1ddc6f8e49db4cb83"></a>
## statement_to_plan

`function` · `datafusion_sql::planner::SqlToRel::statement_to_plan` · datafusion-sql 55.1.0

```rust
fn statement_to_plan(&self, statement: DFStatement) -> Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "datafusion_sql::planner::SqlToRel", "path": "crate::planner::SqlToRel"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_expr::planner::ContextProvider", "path": "ContextProvider"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [3190, 2], "filename": "src/statement.rs"}, "trait": null, "trait_path": null}`

Source: `src/statement.rs:229`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Generate a logical plan from an DataFusion SQL statement

<a id="op-b089b788935eb700de099d47"></a>
## take_warnings

`function` · `datafusion_sql::planner::SqlToRel::take_warnings` · datafusion-sql 55.1.0

```rust
fn take_warnings(&self) -> Vec<Diagnostic>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "datafusion_sql::planner::SqlToRel", "path": "SqlToRel"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_expr::planner::ContextProvider", "path": "ContextProvider"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [461, 1], "end": [926, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:493`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Drain and return non-fatal warnings collected during SQL planning.
