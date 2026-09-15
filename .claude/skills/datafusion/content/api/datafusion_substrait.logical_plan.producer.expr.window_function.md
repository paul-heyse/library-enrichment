# `datafusion_substrait::logical_plan::producer::expr::window_function`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.expr.window_function.json`](../model/datafusion_substrait.logical_plan.producer.expr.window_function.json)

## from_window_function

`function` · `datafusion_substrait::logical_plan::producer::expr::window_function::from_window_function`

```rust
fn from_window_function(producer: &mut impl SubstraitProducer, window_fn: &datafusion::logical_expr::expr::WindowFunction, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

---
