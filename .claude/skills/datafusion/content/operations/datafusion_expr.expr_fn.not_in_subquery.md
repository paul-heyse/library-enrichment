# `datafusion_expr::expr_fn::not_in_subquery`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.not_in_subquery.json).

<a id="op-58bf231040481a6cbc32ae1c"></a>
## not_in_subquery

`function` · `datafusion_expr::expr_fn::not_in_subquery` · datafusion-expr 55.1.0

```rust
fn not_in_subquery(expr: Expr, subquery: std::sync::Arc<LogicalPlan>) -> Expr
```

Source: `src/expr_fn.rs:291`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a NOT IN subquery expression
