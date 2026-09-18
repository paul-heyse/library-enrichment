# `datafusion_expr::expr_fn::out_ref_col_with_metadata`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.out_ref_col_with_metadata.json).

<a id="op-5a4d1e5f860d9fe4a29f831f"></a>
## out_ref_col_with_metadata

`function` · `datafusion_expr::expr_fn::out_ref_col_with_metadata` · datafusion-expr 55.1.0

```rust
fn out_ref_col_with_metadata(dt: arrow::datatypes::DataType, metadata: std::collections::HashMap<String, String>, ident: impl Into<datafusion_common::Column>) -> Expr
```

Source: `src/expr_fn.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create an out reference column from an existing field (preserving metadata)
