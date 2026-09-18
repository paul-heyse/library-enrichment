# `datafusion_physical_expr::expressions::cast::cast`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.cast.cast.json).

<a id="op-11f9459bd5257de2bdd33c47"></a>
## cast

`function` · `datafusion_physical_expr::expressions::cast::cast` · datafusion-physical-expr 55.1.0

```rust
fn cast(expr: std::sync::Arc<dyn PhysicalExpr>, input_schema: &arrow::datatypes::Schema, cast_type: arrow::datatypes::DataType) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/expressions/cast.rs:548`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Return a PhysicalExpression representing `expr` casted to
`cast_type`, if any casting is needed.

Note that such casts may lose type information
