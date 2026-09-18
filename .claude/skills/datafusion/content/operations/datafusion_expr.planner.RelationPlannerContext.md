# `datafusion_expr::planner::RelationPlannerContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.planner.RelationPlannerContext.json).

<a id="op-2ca1c50135eec6ca1376865b"></a>
## RelationPlannerContext

`trait` · `datafusion_expr::planner::RelationPlannerContext` · datafusion-expr 55.1.0

```rust
trait RelationPlannerContext
```

Source: `src/planner.rs:410`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Provides utilities for relation planners to interact with DataFusion's SQL
planner.

This trait provides SQL planning utilities specific to relation planning,
such as converting SQL expressions to logical expressions and normalizing
identifiers. It uses composition to provide access to session context via
[`ContextProvider`](../operations/datafusion_expr.planner.ContextProvider.md#op-97af42065a69e4e56202d0d2).

<a id="op-ae405de58bb7d8eefcd19b61"></a>
## context_provider

`function` · `datafusion_expr::planner::RelationPlannerContext::context_provider` · datafusion-expr 55.1.0

```rust
fn context_provider(&self) -> &dyn ContextProvider
```

Source: `src/planner.rs:413`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Provides access to the underlying context provider for reading session
configuration, accessing tables, functions, and other metadata.

<a id="op-d12612220735fcaa28ce9aec"></a>
## normalize_ident

`function` · `datafusion_expr::planner::RelationPlannerContext::normalize_ident` · datafusion-expr 55.1.0

```rust
fn normalize_ident(&self, ident: Ident) -> String
```

Source: `src/planner.rs:432`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Normalizes an identifier according to session settings.

<a id="op-ff52afc6172a5b8c1c72fd28"></a>
## object_name_to_table_reference

`function` · `datafusion_expr::planner::RelationPlannerContext::object_name_to_table_reference` · datafusion-expr 55.1.0

```rust
fn object_name_to_table_reference(&self, name: ObjectName) -> Result<TableReference>
```

Source: `src/planner.rs:435`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Normalizes a SQL object name into a [`TableReference`](../operations/datafusion_common.table_reference.TableReference.md#op-dafce6f1cf123e142b4fcab0).

<a id="op-1aa88409f342009791cf3e26"></a>
## plan

`function` · `datafusion_expr::planner::RelationPlannerContext::plan` · datafusion-expr 55.1.0

```rust
fn plan(&mut self, relation: TableFactor) -> Result<LogicalPlan>
```

Source: `src/planner.rs:417`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Plans the specified relation through the full planner pipeline, starting
from the first registered relation planner.

<a id="op-077fc32582e8494670c033c5"></a>
## sql_expr_to_logical_expr

`function` · `datafusion_expr::planner::RelationPlannerContext::sql_expr_to_logical_expr` · datafusion-expr 55.1.0

```rust
fn sql_expr_to_logical_expr(&mut self, expr: SQLExpr, schema: &DFSchema) -> Result<Expr>
```

Source: `src/planner.rs:425`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Converts a SQL expression into a logical expression without DataFusion
rewrites.

<a id="op-290ec455b2cb0a988f30cee4"></a>
## sql_to_expr

`function` · `datafusion_expr::planner::RelationPlannerContext::sql_to_expr` · datafusion-expr 55.1.0

```rust
fn sql_to_expr(&mut self, expr: SQLExpr, schema: &DFSchema) -> Result<Expr>
```

Source: `src/planner.rs:421`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Converts a SQL expression into a logical expression using the current
planner context.
