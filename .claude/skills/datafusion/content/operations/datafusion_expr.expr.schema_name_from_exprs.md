# `datafusion_expr::expr::schema_name_from_exprs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.schema_name_from_exprs.json).

<a id="op-161d9a31e9ce9e55f61080fd"></a>
## schema_name_from_exprs

`function` · `datafusion_expr::expr::schema_name_from_exprs` · datafusion-expr 55.1.0

```rust
fn schema_name_from_exprs(exprs: &[Expr]) -> datafusion_common::Result<String, fmt::Error>
```

Source: `src/expr.rs:3523`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Get schema_name for Vector of expressions
