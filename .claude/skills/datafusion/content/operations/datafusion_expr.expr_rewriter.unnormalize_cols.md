# `datafusion_expr::expr_rewriter::unnormalize_cols`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_rewriter.unnormalize_cols.json).

<a id="op-95577763238425a67f2e68bc"></a>
## unnormalize_cols

`function` · `datafusion_expr::expr_rewriter::unnormalize_cols` · datafusion-expr 55.1.0

```rust
fn unnormalize_cols(exprs: impl IntoIterator<Item = Expr>) -> Vec<Expr>
```

Source: `src/expr_rewriter/mod.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Recursively un-normalize all [`Column`](../operations/datafusion_common.column.Column.md#op-099cc6d1a52c20c065bf8bc6) expressions in a list of expression trees
