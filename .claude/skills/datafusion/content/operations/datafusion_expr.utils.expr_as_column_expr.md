# `datafusion_expr::utils::expr_as_column_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.expr_as_column_expr.json).

<a id="op-8b03b57dea2ee7286955389f"></a>
## expr_as_column_expr

`function` · `datafusion_expr::utils::expr_as_column_expr` · datafusion-expr 55.1.0

```rust
fn expr_as_column_expr(expr: &Expr, plan: &LogicalPlan) -> datafusion_common::Result<Expr>
```

Source: `src/utils.rs:923`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Convert any `Expr` to an `Expr::Column`.
