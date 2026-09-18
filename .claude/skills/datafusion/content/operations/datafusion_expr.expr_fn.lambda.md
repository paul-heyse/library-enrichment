# `datafusion_expr::expr_fn::lambda`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.lambda.json).

<a id="op-a6bafb79eec1e618d3bba1ac"></a>
## lambda

`function` · `datafusion_expr::expr_fn::lambda` · datafusion-expr 55.1.0

```rust
fn lambda(params: impl IntoIterator<Item = impl Into<String>>, body: Expr) -> Expr
```

Source: `src/expr_fn.rs:725`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a lambda expression
