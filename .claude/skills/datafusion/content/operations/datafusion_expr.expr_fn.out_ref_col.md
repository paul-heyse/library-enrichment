# `datafusion_expr::expr_fn::out_ref_col`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.out_ref_col.json).

<a id="op-541031887e3448f2f6a608a9"></a>
## out_ref_col

`function` · `datafusion_expr::expr_fn::out_ref_col` · datafusion-expr 55.1.0

```rust
fn out_ref_col(dt: arrow::datatypes::DataType, ident: impl Into<datafusion_common::Column>) -> Expr
```

Source: `src/expr_fn.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create an out reference column which hold a reference that has been resolved to a field
outside of the current plan.
The expression created by this function does not preserve the metadata of the outer column.
Please use `out_ref_col_with_metadata` if you want to preserve the metadata.
