# `datafusion_physical_expr::projection::project_ordering`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.projection.project_ordering.json).

<a id="op-540b7f60b0a4510e8b41a2b9"></a>
## project_ordering

`function` · `datafusion_physical_expr::projection::project_ordering` · datafusion-physical-expr 55.1.0

```rust
fn project_ordering(ordering: &datafusion_physical_expr_common::sort_expr::LexOrdering, schema: &arrow::datatypes::SchemaRef) -> Option<datafusion_physical_expr_common::sort_expr::LexOrdering>
```

Source: `src/projection.rs:1397`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Projects a single [LexOrdering](../operations/datafusion_physical_expr_common.sort_expr.LexOrdering.md#op-d19b6df9e9a4be59c4b4dfd1) onto the given schema.

This function attempts to rewrite every [PhysicalSortExpr](../operations/datafusion_physical_expr_common.sort_expr.PhysicalSortExpr.md#op-80fd045cf8466b7fe98e5ea8) in the provided
[LexOrdering](../operations/datafusion_physical_expr_common.sort_expr.LexOrdering.md#op-d19b6df9e9a4be59c4b4dfd1) so that any [Column](../operations/datafusion_physical_expr.expressions.column.Column.md#op-8a412ca3bf3f178f4986ec28) expressions point at the correct field
indices in `schema`.

Key details:
- Columns are matched by name, not by index. The index of each matched
  column is looked up with [Schema::column_with_name](arrow::datatypes::Schema::column_with_name) and a new
  [Column](../operations/datafusion_physical_expr.expressions.column.Column.md#op-8a412ca3bf3f178f4986ec28) with the correct [index](Column::index) is substituted.
- If an expression references a column name that does not exist in
  `schema`, projection of the current ordering stops and only the already
  rewritten prefix is kept. This models the fact that a lexicographical
  ordering remains valid for any leading prefix whose expressions are
  present in the projected schema.
- If no expressions can be projected (i.e. the first one is missing), the
  function returns `None`.

Return value:
- `Some(LexOrdering)` if at least one sort expression could be projected.
  The returned ordering may be a strict prefix of the input ordering.
- `None` if no part of the ordering can be projected onto `schema`.

Example

Suppose we have an input ordering `[col("a@0"), col("b@1")]` but the projected
schema only contains b and not a. The result will be `Some([col("a@0")])`. In other
words, the column reference is reindexed to match the projected schema.
If neither a nor b is present, the result will be None.

Unresolved upstream links (retained, not inferred): `arrow::datatypes::Schema::column_with_name`.
