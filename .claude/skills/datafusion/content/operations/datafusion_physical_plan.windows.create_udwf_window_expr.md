# `datafusion_physical_plan::windows::create_udwf_window_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.windows.create_udwf_window_expr.json).

<a id="op-9829d02f39f9fd6455fafe0d"></a>
## create_udwf_window_expr

`function` · `datafusion_physical_plan::windows::create_udwf_window_expr` · datafusion-physical-plan 55.1.0

```rust
fn create_udwf_window_expr(fun: &std::sync::Arc<datafusion_expr::WindowUDF>, args: &[std::sync::Arc<dyn PhysicalExpr>], input_schema: &arrow::datatypes::Schema, name: String, ignore_nulls: bool) -> datafusion_common::Result<std::sync::Arc<dyn StandardWindowFunctionExpr>>
```

Source: `src/windows/mod.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Creates a `StandardWindowFunctionExpr` suitable for a user defined window function
