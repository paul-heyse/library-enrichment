# `datafusion_physical_expr::planner::logical2physical`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.planner.logical2physical.json).

<a id="op-cb137123849e5ea4eddae004"></a>
## logical2physical

`function` · `datafusion_physical_expr::planner::logical2physical` · datafusion-physical-expr 55.1.0

```rust
fn logical2physical(expr: &datafusion_expr::Expr, schema: &arrow::datatypes::Schema) -> std::sync::Arc<dyn PhysicalExpr>
```

Source: `src/planner.rs:718`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Convert a logical expression to a physical expression (without any simplification, etc)
