# `datafusion_expr::expr_fn::wildcard_with_options`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.wildcard_with_options.json).

<a id="op-c571381351e7763c130ff7e1"></a>
## wildcard_with_options

`function` · `datafusion_expr::expr_fn::wildcard_with_options` · datafusion-expr 55.1.0

```rust
fn wildcard_with_options(options: expr::WildcardOptions) -> select_expr::SelectExpr
```

Source: `src/expr_fn.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create an '*' [`Expr::Wildcard`](../operations/datafusion_expr.expr.Expr.md#op-ea63f88ecb3f6e144471edcb) expression with the wildcard options
