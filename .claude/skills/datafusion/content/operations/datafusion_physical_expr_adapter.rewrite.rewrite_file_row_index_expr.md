# `datafusion_physical_expr_adapter::rewrite::rewrite_file_row_index_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_adapter.rewrite.rewrite_file_row_index_expr.json).

<a id="op-4af44b63045305e330abc41d"></a>
## rewrite_file_row_index_expr

`function` · `datafusion_physical_expr_adapter::rewrite::rewrite_file_row_index_expr` · datafusion-physical-expr-adapter 55.1.0

```rust
fn rewrite_file_row_index_expr(expr: std::sync::Arc<dyn PhysicalExpr>, row_index_name: &str, row_index_idx: usize) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/rewrite.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

Rewrite [`file_row_index()`][FileRowIndexFunc](../operations/datafusion_functions.core.file_row_index.FileRowIndexFunc.md#op-3693ac97957c4259d0e82ca7) in a [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) to
read from a source-provided row-index column.

`row_index_idx` is the index of `row_index_name` in the schema that the
rewritten expression will be evaluated against. The rewrite uses ordinary
physical expressions: a [`Column`](../operations/datafusion_physical_expr.expressions.column.Column.md#op-8a412ca3bf3f178f4986ec28) that reads the source row-index values
wrapped in a [`CastExpr`](../operations/datafusion_physical_expr.expressions.cast.CastExpr.md#op-b2ec9a951f65fbe4161b17ed) that exposes the public `file_row_index: Int64`
return field without source-specific extension metadata.
