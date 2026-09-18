# `datafusion_physical_expr::projection::project_orderings`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.projection.project_orderings.json).

<a id="op-1430332a2381f2c4a7974546"></a>
## project_orderings

`function` · `datafusion_physical_expr::projection::project_orderings` · datafusion-physical-expr 55.1.0

```rust
fn project_orderings(orderings: &[datafusion_physical_expr_common::sort_expr::LexOrdering], schema: &arrow::datatypes::SchemaRef) -> Vec<datafusion_physical_expr_common::sort_expr::LexOrdering>
```

Source: `src/projection.rs:1355`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Projects a slice of [LexOrdering](../operations/datafusion_physical_expr_common.sort_expr.LexOrdering.md#op-d19b6df9e9a4be59c4b4dfd1)s onto the given schema.

This is a convenience wrapper that applies [project_ordering](../operations/datafusion_physical_expr.projection.project_ordering.md#op-540b7f60b0a4510e8b41a2b9) to each
input ordering and collects the successful projections:
- For each input ordering, the result of [project_ordering](../operations/datafusion_physical_expr.projection.project_ordering.md#op-540b7f60b0a4510e8b41a2b9) is appended to
  the output if it is `Some(...)`.
- Order is preserved and no deduplication is attempted.
- If none of the input orderings can be projected, an empty `Vec` is
  returned.

See [project_ordering](../operations/datafusion_physical_expr.projection.project_ordering.md#op-540b7f60b0a4510e8b41a2b9) for the semantics of projecting a single
[LexOrdering](../operations/datafusion_physical_expr_common.sort_expr.LexOrdering.md#op-d19b6df9e9a4be59c4b4dfd1).
