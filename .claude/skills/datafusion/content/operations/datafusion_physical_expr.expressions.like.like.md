# `datafusion_physical_expr::expressions::like::like`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.like.like.json).

<a id="op-b085917805053aaa18a5b7d3"></a>
## like

`function` · `datafusion_physical_expr::expressions::like::like` · datafusion-physical-expr 55.1.0

```rust
fn like(negated: bool, case_insensitive: bool, expr: std::sync::Arc<dyn PhysicalExpr>, pattern: std::sync::Arc<dyn PhysicalExpr>, input_schema: &arrow::datatypes::Schema) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/expressions/like.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a like expression, erroring if the argument types are not compatible.
