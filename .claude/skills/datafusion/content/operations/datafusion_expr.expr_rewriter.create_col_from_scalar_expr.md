# `datafusion_expr::expr_rewriter::create_col_from_scalar_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_rewriter.create_col_from_scalar_expr.json).

<a id="op-0f24dba0007904a77978cfde"></a>
## create_col_from_scalar_expr

`function` · `datafusion_expr::expr_rewriter::create_col_from_scalar_expr` · datafusion-expr 55.1.0

```rust
fn create_col_from_scalar_expr(scalar_expr: &Expr, subqry_alias: String) -> datafusion_common::Result<datafusion_common::Column>
```

Source: `src/expr_rewriter/mod.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a Column from the Scalar Expr
