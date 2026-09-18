# `datafusion_physical_expr::expressions::try_cast::try_cast`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.try_cast.try_cast.json).

<a id="op-3c08d157f4664e94c8440dac"></a>
## try_cast

`function` · `datafusion_physical_expr::expressions::try_cast::try_cast` · datafusion-physical-expr 55.1.0

```rust
fn try_cast(expr: std::sync::Arc<dyn PhysicalExpr>, input_schema: &arrow::datatypes::Schema, cast_type: arrow::datatypes::DataType) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/expressions/try_cast.rs:275`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Return a PhysicalExpression representing `expr` casted to
`cast_type`, if any casting is needed.

Note that such casts may lose type information
