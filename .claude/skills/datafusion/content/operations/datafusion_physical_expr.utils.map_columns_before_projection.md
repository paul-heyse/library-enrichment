# `datafusion_physical_expr::utils::map_columns_before_projection`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.utils.map_columns_before_projection.json).

<a id="op-7f98c3ee64d3d2ff26c706fa"></a>
## map_columns_before_projection

`function` · `datafusion_physical_expr::utils::map_columns_before_projection` · datafusion-physical-expr 55.1.0

```rust
fn map_columns_before_projection(parent_required: &[std::sync::Arc<dyn PhysicalExpr>], proj_exprs: &[(std::sync::Arc<dyn PhysicalExpr>, String)]) -> Vec<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/utils/mod.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This function maps back requirement after ProjectionExec
to the Executor for its input.
