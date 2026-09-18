# `datafusion_expr::expr_rewriter::normalize_cols`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_rewriter.normalize_cols.json).

<a id="op-c1cfc4baf0eb203504f2548e"></a>
## normalize_cols

`function` · `datafusion_expr::expr_rewriter::normalize_cols` · datafusion-expr 55.1.0

```rust
fn normalize_cols(exprs: impl IntoIterator<Item = impl Into<Expr>>, plan: &LogicalPlan) -> datafusion_common::Result<Vec<Expr>>
```

Source: `src/expr_rewriter/mod.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Recursively normalize all [`Column`](../operations/datafusion_common.column.Column.md#op-099cc6d1a52c20c065bf8bc6) expressions in a list of expression trees
