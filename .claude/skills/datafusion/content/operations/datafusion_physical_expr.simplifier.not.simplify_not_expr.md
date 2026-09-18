# `datafusion_physical_expr::simplifier::not::simplify_not_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.simplifier.not.simplify_not_expr.json).

<a id="op-d5d2eb246a269eaa32d8cc51"></a>
## simplify_not_expr

`function` · `datafusion_physical_expr::simplifier::not::simplify_not_expr` · datafusion-physical-expr 55.1.0

```rust
fn simplify_not_expr(expr: std::sync::Arc<dyn PhysicalExpr>, schema: &arrow::datatypes::Schema) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<std::sync::Arc<dyn PhysicalExpr>>>
```

Source: `src/simplifier/not.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Attempts to simplify NOT expressions by applying one level of transformation

This function applies a single simplification rule and returns. When used with
TreeNodeRewriter, multiple passes will automatically be applied until no more
transformations are possible.
