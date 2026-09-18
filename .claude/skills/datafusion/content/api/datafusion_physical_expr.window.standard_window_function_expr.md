# `datafusion_physical_expr::window::standard_window_function_expr`

Crate `datafusion-physical-expr` · 1 public items · structured records in [`model/datafusion_physical_expr.window.standard_window_function_expr.json`](../model/datafusion_physical_expr.window.standard_window_function_expr.json)

## StandardWindowFunctionExpr

`trait` · `datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr`

Also reachable as `datafusion_physical_expr::window::StandardWindowFunctionExpr`

```rust
trait StandardWindowFunctionExpr: Send + Sync + std::fmt::Debug
```

**Implementors** (1)

- `datafusion_physical_plan::windows::WindowUDFExpr`

**Methods** (9)

```rust
fn as_any(&self) -> &dyn Any
fn create_evaluator(&self) -> Result<Box<dyn PartitionEvaluator>>
fn evaluate_args(&self, batch: &RecordBatch) -> Result<Vec<ArrayRef>>
fn expressions(&self) -> Vec<Arc<dyn PhysicalExpr>>
fn field(&self) -> Result<FieldRef>
fn get_result_ordering(&self, _schema: &SchemaRef) -> Option<PhysicalSortExpr>
fn limit_effect(&self) -> LimitEffect
fn name(&self) -> &str
fn reverse_expr(&self) -> Option<Arc<dyn StandardWindowFunctionExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.window.standard_window_function_expr.StandardWindowFunctionExpr.md).


Evaluates a window function by instantiating a
[`PartitionEvaluator`] for calculating the function's output in
that partition.

Note that unlike aggregation based window functions, some window
functions such as `rank` ignore the values in the window frame,
but others such as `first_value`, `last_value`, and
`nth_value` need the value.

---
