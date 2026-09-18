# `datafusion_physical_expr::expressions::negative::negative`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.negative.negative.json).

<a id="op-1ee5386b22648f49d7c6c925"></a>
## negative

`function` · `datafusion_physical_expr::expressions::negative::negative` · datafusion-physical-expr 55.1.0

```rust
fn negative(arg: std::sync::Arc<dyn PhysicalExpr>, input_schema: &arrow::datatypes::Schema) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/expressions/negative.rs:225`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Creates a unary expression NEGATIVE

# Errors

This function errors when the argument's type is not signed numeric
