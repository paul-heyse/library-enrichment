# `datafusion_expr::expr_rewriter`

Crate `datafusion-expr` · 14 public items · structured records in [`model/datafusion_expr.expr_rewriter.json`](../model/datafusion_expr.expr_rewriter.json)

## SavedName

`enum` · `datafusion_expr::expr_rewriter::SavedName`

```rust
enum SavedName
```

**Variants**: `Saved`, `None`

**Derives**: Debug

**Methods** (1)

```rust
fn restore(self, expr: Expr) -> Expr
```

If the qualified name of an expression is remembered, it will be preserved
when rewriting the expression

---

## coerce_plan_expr_for_schema

`function` · `datafusion_expr::expr_rewriter::coerce_plan_expr_for_schema`

```rust
fn coerce_plan_expr_for_schema(plan: LogicalPlan, schema: &datafusion_common::DFSchema) -> datafusion_common::Result<LogicalPlan>
```

Returns plan with expressions coerced to types compatible with
schema types

---

## create_col_from_scalar_expr

`function` · `datafusion_expr::expr_rewriter::create_col_from_scalar_expr`

```rust
fn create_col_from_scalar_expr(scalar_expr: &Expr, subqry_alias: String) -> datafusion_common::Result<datafusion_common::Column>
```

Create a Column from the Scalar Expr

---

## normalize_col

`function` · `datafusion_expr::expr_rewriter::normalize_col`

```rust
fn normalize_col(expr: Expr, plan: &LogicalPlan) -> datafusion_common::Result<Expr>
```

Recursively call `LogicalPlanBuilder::normalize` on all [`Column`] expressions
in the `expr` expression tree.

---

## normalize_col_with_schemas_and_ambiguity_check

`function` · `datafusion_expr::expr_rewriter::normalize_col_with_schemas_and_ambiguity_check`

```rust
fn normalize_col_with_schemas_and_ambiguity_check(expr: Expr, schemas: &[&[&datafusion_common::DFSchema]], using_columns: &[std::collections::HashSet<datafusion_common::Column>]) -> datafusion_common::Result<Expr>
```

See [`Column::normalize_with_schemas_and_ambiguity_check`] for usage

---

## normalize_cols

`function` · `datafusion_expr::expr_rewriter::normalize_cols`

```rust
fn normalize_cols(exprs: impl IntoIterator<Item = impl Into<Expr>>, plan: &LogicalPlan) -> datafusion_common::Result<Vec<Expr>>
```

Recursively normalize all [`Column`] expressions in a list of expression trees

---

## normalize_sorts

`function` · `datafusion_expr::expr_rewriter::normalize_sorts`

```rust
fn normalize_sorts(sorts: impl IntoIterator<Item = impl Into<expr::Sort>>, plan: &LogicalPlan) -> datafusion_common::Result<Vec<expr::Sort>>
```

---

## replace_col

`function` · `datafusion_expr::expr_rewriter::replace_col`

```rust
fn replace_col(expr: Expr, replace_map: &std::collections::HashMap<&datafusion_common::Column, &datafusion_common::Column>) -> datafusion_common::Result<Expr>
```

Recursively replace all [`Column`] expressions in a given expression tree with
`Column` expressions provided by the hash map argument.

---

## strip_outer_reference

`function` · `datafusion_expr::expr_rewriter::strip_outer_reference`

```rust
fn strip_outer_reference(expr: Expr) -> Expr
```

Recursively remove all the ['OuterReferenceColumn'] and return the inside Column
in the expression tree.

---

## unalias

`function` · `datafusion_expr::expr_rewriter::unalias`

```rust
fn unalias(expr: Expr) -> Expr
```

Recursively un-alias an expressions

---

## unnormalize_col

`function` · `datafusion_expr::expr_rewriter::unnormalize_col`

```rust
fn unnormalize_col(expr: Expr) -> Expr
```

Recursively 'unnormalize' (remove all qualifiers) from an
expression tree.

For example, if there were expressions like `foo.bar` this would
rewrite it to just `bar`.

---

## unnormalize_cols

`function` · `datafusion_expr::expr_rewriter::unnormalize_cols`

```rust
fn unnormalize_cols(exprs: impl IntoIterator<Item = Expr>) -> Vec<Expr>
```

Recursively un-normalize all [`Column`] expressions in a list of expression trees

---

## NamePreserver

`struct` · `datafusion_expr::expr_rewriter::NamePreserver`

Also reachable as `datafusion_optimizer::utils::NamePreserver`

```rust
struct NamePreserver
```

**Methods** (3)

```rust
fn new(plan: &LogicalPlan) -> Self
fn new_for_projection() -> Self
fn save(&self, expr: &Expr) -> SavedName
```

Handles ensuring the name of rewritten expressions is not changed.

This is important when optimizing plans to ensure the output
schema of plan nodes don't change after optimization.
For example, if an expression `1 + 2` is rewritten to `3`, the name of the
expression should be preserved: `3 as "1 + 2"`

See <https://github.com/apache/datafusion/issues/3555> for details

---

## FunctionRewrite

`trait` · `datafusion_expr::expr_rewriter::FunctionRewrite`

```rust
trait FunctionRewrite: Debug
```

**Methods** (2)

```rust
fn name(&self) -> &str
fn rewrite(&self, expr: Expr, schema: &DFSchema, config: &ConfigOptions) -> Result<Transformed<Expr>>
```

Trait for rewriting [`Expr`]s into function calls.

This trait is used with `FunctionRegistry::register_function_rewrite` to
to evaluating `Expr`s using functions that may not be built in to DataFusion

For example, concatenating arrays `a || b` is represented as
`Operator::ArrowAt`, but can be implemented by calling a function
`array_concat` from the `functions-nested` crate.

---
