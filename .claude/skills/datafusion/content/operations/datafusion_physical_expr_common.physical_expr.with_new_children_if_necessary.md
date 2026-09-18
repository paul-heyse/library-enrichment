# `datafusion_physical_expr_common::physical_expr::with_new_children_if_necessary`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.physical_expr.with_new_children_if_necessary.json).

<a id="op-8323cbe3b5e5c7b89aae5eee"></a>
## with_new_children_if_necessary

`function` · `datafusion_physical_expr_common::physical_expr::with_new_children_if_necessary` · datafusion-physical-expr-common 55.1.0

```rust
fn with_new_children_if_necessary(expr: std::sync::Arc<dyn PhysicalExpr>, children: Vec<std::sync::Arc<dyn PhysicalExpr>>) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/physical_expr.rs:821`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns a copy of this expr if we change any child according to the pointer comparison.
The size of `children` must be equal to the size of `PhysicalExpr::children()`.
