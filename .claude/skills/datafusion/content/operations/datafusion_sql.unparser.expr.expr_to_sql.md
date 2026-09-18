# `datafusion_sql::unparser::expr::expr_to_sql`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.expr.expr_to_sql.json).

<a id="op-e296114a3665696472053bea"></a>
## expr_to_sql

`function` · `datafusion_sql::unparser::expr::expr_to_sql` · datafusion-sql 55.1.0

```rust
fn expr_to_sql(expr: &datafusion_expr::Expr) -> datafusion_common::Result<ast::Expr>
```

Source: `src/unparser/expr.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Convert a DataFusion [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) to [`ast::Expr`](../operations/sqlparser.ast.Expr.md#op-60bb4348f2010610c575bb63)

This function is the opposite of [`SqlToRel::sql_to_expr`] and can be used
to, among other things, convert [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc)s to SQL strings. Such strings could
be used to pass filters or other expressions to another SQL engine.

# Errors

Throws an error if [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) can not be represented by an [`ast::Expr`](../operations/sqlparser.ast.Expr.md#op-60bb4348f2010610c575bb63)

# See Also

* [`Unparser`](../operations/datafusion_sql.unparser.Unparser.md#op-7e732a2e0e80279190139cfb) for more control over the conversion to SQL
* [`plan_to_sql`] for converting a [`LogicalPlan`] to SQL

# Example
```
use datafusion_expr::{col, lit};
use datafusion_sql::unparser::expr_to_sql;
let expr = col("a").gt(lit(4)); // form an expression `a > 4`
let sql = expr_to_sql(&expr).unwrap(); // convert to ast::Expr, using
assert_eq!(sql.to_string(), "(a > 4)"); // use Display impl for SQL text
```

[`SqlToRel::sql_to_expr`]: crate::planner::SqlToRel::sql_to_expr
[`plan_to_sql`]: crate::unparser::plan_to_sql
[`LogicalPlan`]: datafusion_expr::logical_plan::LogicalPlan
