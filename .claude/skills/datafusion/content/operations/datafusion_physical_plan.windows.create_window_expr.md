# `datafusion_physical_plan::windows::create_window_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.windows.create_window_expr.json).

<a id="op-bf2ac3e25758907fc6ceaf67"></a>
## create_window_expr

`function` · `datafusion_physical_plan::windows::create_window_expr` · datafusion-physical-plan 55.1.0

```rust
fn create_window_expr(fun: &datafusion_expr::WindowFunctionDefinition, name: String, args: &[std::sync::Arc<dyn PhysicalExpr>], partition_by: &[std::sync::Arc<dyn PhysicalExpr>], order_by: &[expressions::PhysicalSortExpr], window_frame: std::sync::Arc<datafusion_expr::WindowFrame>, input_schema: arrow::datatypes::SchemaRef, ignore_nulls: bool, distinct: bool, filter: Option<std::sync::Arc<dyn PhysicalExpr>>) -> datafusion_common::Result<std::sync::Arc<dyn WindowExpr>>
```

Source: `src/windows/mod.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a physical expression for window function
