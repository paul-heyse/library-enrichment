# `datafusion_physical_expr_common::physical_expr::is_dynamic_physical_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.physical_expr.is_dynamic_physical_expr.json).

<a id="op-bad44ff0cbc035b21d582e5b"></a>
## is_dynamic_physical_expr

`function` · `datafusion_physical_expr_common::physical_expr::is_dynamic_physical_expr` · datafusion-physical-expr-common 55.1.0

```rust
fn is_dynamic_physical_expr(expr: &std::sync::Arc<dyn PhysicalExpr>) -> bool
```

Source: `src/physical_expr.rs:1003`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Check if the given `PhysicalExpr` is dynamic.
Internally this calls [`snapshot_generation`](../operations/datafusion_physical_expr_common.physical_expr.snapshot_generation.md#op-297e75607be761bf81f57611) to check if the generation is non-zero,
any dynamic `PhysicalExpr` should have a non-zero generation.
