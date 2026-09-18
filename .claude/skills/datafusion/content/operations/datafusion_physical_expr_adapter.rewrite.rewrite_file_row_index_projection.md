# `datafusion_physical_expr_adapter::rewrite::rewrite_file_row_index_projection`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_adapter.rewrite.rewrite_file_row_index_projection.json).

<a id="op-2db1101f4cd4a0dc1ec977af"></a>
## rewrite_file_row_index_projection

`function` · `datafusion_physical_expr_adapter::rewrite::rewrite_file_row_index_projection` · datafusion-physical-expr-adapter 55.1.0

```rust
fn rewrite_file_row_index_projection(base_projection: &datafusion_physical_expr::projection::ProjectionExprs, projection: &datafusion_physical_expr::projection::ProjectionExprs, row_index_col: &datafusion_physical_expr::expressions::Column) -> datafusion_common::Result<datafusion_physical_expr::projection::ProjectionExprs>
```

Source: `src/rewrite.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

Rewrite [`file_row_index()`][FileRowIndexFunc](../operations/datafusion_functions.core.file_row_index.FileRowIndexFunc.md#op-3693ac97957c4259d0e82ca7) in pushed [`ProjectionExprs`](../operations/datafusion_physical_expr.projection.ProjectionExprs.md#op-9af61cebb79ab505edeb3c19)
to read from a source-provided row-index column.


For example if `row_index_column` is `__datafusion_row_idx` this function rewrites all
instances of [`file_row_index()`][FileRowIndexFunc](../operations/datafusion_functions.core.file_row_index.FileRowIndexFunc.md#op-3693ac97957c4259d0e82ca7) to
`__datafusion_row_index` [`Column`](../operations/datafusion_physical_expr.expressions.column.Column.md#op-8a412ca3bf3f178f4986ec28) references.

`base_projection` is the current projection already pushed into a source.
The row-index source column is appended to that base projection if it is not
already present. `projection` is rewritten to read from the projected
row-index column and then merged on top of the extended base projection.
