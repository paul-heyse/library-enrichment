# `datafusion_physical_expr::expressions::try_cast::try_cast_with_target_field`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.try_cast.try_cast_with_target_field.json).

<a id="op-503845958e42842ab191fb5d"></a>
## try_cast_with_target_field

`function` · `datafusion_physical_expr::expressions::try_cast::try_cast_with_target_field` · datafusion-physical-expr 55.1.0

```rust
fn try_cast_with_target_field(expr: std::sync::Arc<dyn PhysicalExpr>, input_schema: &arrow::datatypes::Schema, target_field: &arrow::datatypes::FieldRef) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/expressions/try_cast.rs:298`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Return a PhysicalExpression representing `expr` casted to `target_field`,
preserving any explicit field semantics such as metadata.

TRY_CAST results are always nullable since failed casts return NULL.

If the input expression already has the same data type, the target field
has no explicit metadata constraints, and the source has no extension
metadata to strip, the original expression is returned unchanged.
