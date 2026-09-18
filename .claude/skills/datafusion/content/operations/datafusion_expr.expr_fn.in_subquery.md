# `datafusion_expr::expr_fn::in_subquery`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.in_subquery.json).

<a id="op-266c6f4c4b1fe1037bed8ea9"></a>
## in_subquery

`function` · `datafusion_expr::expr_fn::in_subquery` · datafusion-expr 55.1.0

```rust
fn in_subquery(expr: Expr, subquery: std::sync::Arc<LogicalPlan>) -> Expr
```

Source: `src/expr_fn.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create an IN subquery expression
