# `datafusion_expr::expr_rewriter::replace_col`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_rewriter.replace_col.json).

<a id="op-026b57bff5fd610e427772b3"></a>
## replace_col

`function` · `datafusion_expr::expr_rewriter::replace_col` · datafusion-expr 55.1.0

```rust
fn replace_col(expr: Expr, replace_map: &std::collections::HashMap<&datafusion_common::Column, &datafusion_common::Column>) -> datafusion_common::Result<Expr>
```

Source: `src/expr_rewriter/mod.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Recursively replace all [`Column`](../operations/datafusion_common.column.Column.md#op-099cc6d1a52c20c065bf8bc6) expressions in a given expression tree with
`Column` expressions provided by the hash map argument.
