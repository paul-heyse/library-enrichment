# `datafusion_expr::expr_rewriter::unnormalize_col`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_rewriter.unnormalize_col.json).

<a id="op-232d02647510936e62b10faf"></a>
## unnormalize_col

`function` · `datafusion_expr::expr_rewriter::unnormalize_col` · datafusion-expr 55.1.0

```rust
fn unnormalize_col(expr: Expr) -> Expr
```

Source: `src/expr_rewriter/mod.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Recursively 'unnormalize' (remove all qualifiers) from an
expression tree.

For example, if there were expressions like `foo.bar` this would
rewrite it to just `bar`.
