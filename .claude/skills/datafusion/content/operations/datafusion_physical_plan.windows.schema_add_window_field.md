# `datafusion_physical_plan::windows::schema_add_window_field`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.windows.schema_add_window_field.json).

<a id="op-f5039767f1738642bb18e8fa"></a>
## schema_add_window_field

`function` · `datafusion_physical_plan::windows::schema_add_window_field` · datafusion-physical-plan 55.1.0

```rust
fn schema_add_window_field(args: &[std::sync::Arc<dyn PhysicalExpr>], schema: &arrow::datatypes::Schema, window_fn: &datafusion_expr::WindowFunctionDefinition, fn_name: &str) -> datafusion_common::Result<std::sync::Arc<arrow::datatypes::Schema>>
```

Source: `src/windows/mod.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Build field from window function and add it into schema
