# `datafusion_expr::expr_fn::qualified_wildcard_with_options`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.qualified_wildcard_with_options.json).

<a id="op-b3656f23f93626e156786c0d"></a>
## qualified_wildcard_with_options

`function` · `datafusion_expr::expr_fn::qualified_wildcard_with_options` · datafusion-expr 55.1.0

```rust
fn qualified_wildcard_with_options(qualifier: impl Into<datafusion_common::TableReference>, options: expr::WildcardOptions) -> select_expr::SelectExpr
```

Source: `src/expr_fn.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create an 't.*' [`Expr::Wildcard`](../operations/datafusion_expr.expr.Expr.md#op-ea63f88ecb3f6e144471edcb) expression with the wildcard options
