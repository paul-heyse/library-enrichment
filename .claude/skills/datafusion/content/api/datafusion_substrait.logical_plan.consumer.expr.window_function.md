# `datafusion_substrait::logical_plan::consumer::expr::window_function`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.expr.window_function.json`](../model/datafusion_substrait.logical_plan.consumer.expr.window_function.json)

## from_window_function

`function` · `datafusion_substrait::logical_plan::consumer::expr::window_function::from_window_function`

```rust
async fn from_window_function(consumer: &impl SubstraitConsumer, window: &substrait::proto::expression::WindowFunction, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<datafusion::logical_expr::Expr>
```

---
