# `datafusion_expr::utils::expr_to_columns`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.expr_to_columns.json).

<a id="op-8d66b5472914a0333802de60"></a>
## expr_to_columns

`function` · `datafusion_expr::utils::expr_to_columns` · datafusion-expr 55.1.0

```rust
fn expr_to_columns(expr: &Expr, accum: &mut std::collections::HashSet<datafusion_common::Column>) -> datafusion_common::Result<()>
```

Source: `src/utils.rs:276`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Recursively walk an expression tree, collecting the unique set of columns
referenced in the expression
