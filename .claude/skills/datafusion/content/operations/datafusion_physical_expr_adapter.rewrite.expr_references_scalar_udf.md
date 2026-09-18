# `datafusion_physical_expr_adapter::rewrite::expr_references_scalar_udf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_adapter.rewrite.expr_references_scalar_udf.json).

<a id="op-3629eae0b1612a304656d270"></a>
## expr_references_scalar_udf

`function` · `datafusion_physical_expr_adapter::rewrite::expr_references_scalar_udf` · datafusion-physical-expr-adapter 55.1.0

```rust
fn expr_references_scalar_udf<T: ScalarUDFImpl>(expr: &std::sync::Arc<dyn PhysicalExpr>) -> bool
```

Source: `src/rewrite.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

Return true if a [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) references scalar UDF `T`.

This matches the concrete [`ScalarUDFImpl`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-7caee2ff206563b536e983e0) type rather than the function
name, so unrelated UDFs with the same name are not treated as matches.
