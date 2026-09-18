# `datafusion_expr::expr_rewriter::normalize_col`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_rewriter.normalize_col.json).

<a id="op-c749b8bb90461336ad1c4a45"></a>
## normalize_col

`function` · `datafusion_expr::expr_rewriter::normalize_col` · datafusion-expr 55.1.0

```rust
fn normalize_col(expr: Expr, plan: &LogicalPlan) -> datafusion_common::Result<Expr>
```

Source: `src/expr_rewriter/mod.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Recursively call `LogicalPlanBuilder::normalize` on all [`Column`](../operations/datafusion_common.column.Column.md#op-099cc6d1a52c20c065bf8bc6) expressions
in the `expr` expression tree.
