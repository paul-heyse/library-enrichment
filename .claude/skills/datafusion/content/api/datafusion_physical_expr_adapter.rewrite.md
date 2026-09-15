# `datafusion_physical_expr_adapter::rewrite`

Crate `datafusion-physical-expr-adapter` · 4 public items · structured records in [`model/datafusion_physical_expr_adapter.rewrite.json`](../model/datafusion_physical_expr_adapter.rewrite.json)

## expr_references_scalar_udf

`function` · `datafusion_physical_expr_adapter::rewrite::expr_references_scalar_udf`

```rust
fn expr_references_scalar_udf<T: ScalarUDFImpl>(expr: &std::sync::Arc<dyn PhysicalExpr>) -> bool
```

Return true if a [`PhysicalExpr`] references scalar UDF `T`.

This matches the concrete [`ScalarUDFImpl`] type rather than the function
name, so unrelated UDFs with the same name are not treated as matches.

---

## rewrite_file_row_index_expr

`function` · `datafusion_physical_expr_adapter::rewrite::rewrite_file_row_index_expr`

```rust
fn rewrite_file_row_index_expr(expr: std::sync::Arc<dyn PhysicalExpr>, row_index_name: &str, row_index_idx: usize) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Rewrite [`file_row_index()`][FileRowIndexFunc] in a [`PhysicalExpr`] to
read from a source-provided row-index column.

`row_index_idx` is the index of `row_index_name` in the schema that the
rewritten expression will be evaluated against. The rewrite uses ordinary
physical expressions: a [`Column`] that reads the source row-index values
wrapped in a [`CastExpr`] that exposes the public `file_row_index: Int64`
return field without source-specific extension metadata.

---

## rewrite_file_row_index_projection

`function` · `datafusion_physical_expr_adapter::rewrite::rewrite_file_row_index_projection`

```rust
fn rewrite_file_row_index_projection(base_projection: &datafusion_physical_expr::projection::ProjectionExprs, projection: &datafusion_physical_expr::projection::ProjectionExprs, row_index_col: &datafusion_physical_expr::expressions::Column) -> datafusion_common::Result<datafusion_physical_expr::projection::ProjectionExprs>
```

Rewrite [`file_row_index()`][FileRowIndexFunc] in pushed [`ProjectionExprs`]
to read from a source-provided row-index column.


For example if `row_index_column` is `__datafusion_row_idx` this function rewrites all
instances of [`file_row_index()`][FileRowIndexFunc] to
`__datafusion_row_index` [`Column`] references.

`base_projection` is the current projection already pushed into a source.
The row-index source column is appended to that base projection if it is not
already present. `projection` is rewritten to read from the projected
row-index column and then merged on top of the extended base projection.

---

## rewrite_input_file_name_in_projection

`function` · `datafusion_physical_expr_adapter::rewrite::rewrite_input_file_name_in_projection`

```rust
fn rewrite_input_file_name_in_projection(projection: datafusion_physical_expr::projection::ProjectionExprs, file_name: &str) -> datafusion_common::Result<datafusion_physical_expr::projection::ProjectionExprs>
```

Rewrite [`input_file_name()`][InputFileNameFunc] in pushed
[`ProjectionExprs`] to a per-file [`Literal`] holding `file_name`.

If the projection contains no [`input_file_name()`][InputFileNameFunc] UDF it
is returned unchanged, without allocating the literal or rebuilding the
projection tree (the common case for queries that don't use the function).

---
