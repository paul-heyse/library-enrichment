# `datafusion_physical_expr::utils::reassign_expr_columns`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.utils.reassign_expr_columns.json).

<a id="op-d73630644d098553088f9ff9"></a>
## reassign_expr_columns

`function` · `datafusion_physical_expr::utils::reassign_expr_columns` · datafusion-physical-expr 55.1.0

```rust
fn reassign_expr_columns(expr: std::sync::Arc<dyn PhysicalExpr>, schema: &arrow::datatypes::Schema) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/utils/mod.rs:308`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Re-assign indices of [`Column`](../operations/datafusion_physical_expr.expressions.column.Column.md#op-8a412ca3bf3f178f4986ec28)s within the given [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) according to
the provided [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050).

This can be useful when attempting to map an expression onto a different schema.

# Errors

This function will return an error if any column in the expression cannot be found
in the provided schema.
