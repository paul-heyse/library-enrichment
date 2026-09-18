# `datafusion_physical_expr_adapter::rewrite::rewrite_input_file_name_in_projection`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_adapter.rewrite.rewrite_input_file_name_in_projection.json).

<a id="op-deaf87ac92c9d4e139ee5075"></a>
## rewrite_input_file_name_in_projection

`function` · `datafusion_physical_expr_adapter::rewrite::rewrite_input_file_name_in_projection` · datafusion-physical-expr-adapter 55.1.0

```rust
fn rewrite_input_file_name_in_projection(projection: datafusion_physical_expr::projection::ProjectionExprs, file_name: &str) -> datafusion_common::Result<datafusion_physical_expr::projection::ProjectionExprs>
```

Source: `src/rewrite.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

Rewrite [`input_file_name()`][InputFileNameFunc](../operations/datafusion_functions.core.input_file_name.InputFileNameFunc.md#op-bad2789281d7646c1d88e91f) in pushed
[`ProjectionExprs`](../operations/datafusion_physical_expr.projection.ProjectionExprs.md#op-9af61cebb79ab505edeb3c19) to a per-file [`Literal`](../operations/datafusion_physical_expr.expressions.literal.Literal.md#op-ea98b9107dc90c46e8d38265) holding `file_name`.

If the projection contains no [`input_file_name()`][InputFileNameFunc](../operations/datafusion_functions.core.input_file_name.InputFileNameFunc.md#op-bad2789281d7646c1d88e91f) UDF it
is returned unchanged, without allocating the literal or rebuilding the
projection tree (the common case for queries that don't use the function).
