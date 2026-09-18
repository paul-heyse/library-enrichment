# `datafusion_physical_expr::physical_expr::add_offset_to_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.physical_expr.add_offset_to_expr.json).

<a id="op-5878149d9dc7d9e0158e32e6"></a>
## add_offset_to_expr

`function` · `datafusion_physical_expr::physical_expr::add_offset_to_expr` · datafusion-physical-expr 55.1.0

```rust
fn add_offset_to_expr(expr: std::sync::Arc<dyn PhysicalExpr>, offset: isize) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/physical_expr.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Adds the `offset` value to `Column` indices inside `expr`. This function is
generally used during the update of the right table schema in join operations.
