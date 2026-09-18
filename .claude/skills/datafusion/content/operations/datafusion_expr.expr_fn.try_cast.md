# `datafusion_expr::expr_fn::try_cast`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.try_cast.json).

<a id="op-876c93a03237cbe967b5619c"></a>
## try_cast

`function` · `datafusion_expr::expr_fn::try_cast` · datafusion-expr 55.1.0

```rust
fn try_cast(expr: Expr, data_type: arrow::datatypes::DataType) -> Expr
```

Source: `src/expr_fn.rs:335`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a try cast expression
