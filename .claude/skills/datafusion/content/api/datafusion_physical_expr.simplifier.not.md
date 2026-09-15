# `datafusion_physical_expr::simplifier::not`

Crate `datafusion-physical-expr` · 1 public items · structured records in [`model/datafusion_physical_expr.simplifier.not.json`](../model/datafusion_physical_expr.simplifier.not.json)

## simplify_not_expr

`function` · `datafusion_physical_expr::simplifier::not::simplify_not_expr`

> **Deprecated** — since 53.0.0: This function will be made private in a future release, please file an issue if you have a reason for keeping it public.

```rust
fn simplify_not_expr(expr: std::sync::Arc<dyn PhysicalExpr>, schema: &arrow::datatypes::Schema) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<std::sync::Arc<dyn PhysicalExpr>>>
```

Attempts to simplify NOT expressions by applying one level of transformation

This function applies a single simplification rule and returns. When used with
TreeNodeRewriter, multiple passes will automatically be applied until no more
transformations are possible.

---
