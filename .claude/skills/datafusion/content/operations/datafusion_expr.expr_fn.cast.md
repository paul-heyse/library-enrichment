# `datafusion_expr::expr_fn::cast`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.cast.json).

<a id="op-5430395c0f996bd365b8bc35"></a>
## cast

`function` · `datafusion_expr::expr_fn::cast` · datafusion-expr 55.1.0

```rust
fn cast(expr: Expr, data_type: arrow::datatypes::DataType) -> Expr
```

Source: `src/expr_fn.rs:330`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a cast expression
