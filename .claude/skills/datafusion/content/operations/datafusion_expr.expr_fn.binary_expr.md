# `datafusion_expr::expr_fn::binary_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.binary_expr.json).

<a id="op-2e514a1bd27300fe5b02ec21"></a>
## binary_expr

`function` · `datafusion_expr::expr_fn::binary_expr` · datafusion-expr 55.1.0

```rust
fn binary_expr(left: Expr, op: Operator, right: Expr) -> Expr
```

Source: `src/expr_fn.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a new expression `left <op> right`
