# `datafusion_substrait::logical_plan::consumer::expr::if_then`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.expr.if_then.json`](../model/datafusion_substrait.logical_plan.consumer.expr.if_then.json)

## from_if_then

`function` · `datafusion_substrait::logical_plan::consumer::expr::if_then::from_if_then`

```rust
async fn from_if_then(consumer: &impl SubstraitConsumer, if_then: &substrait::proto::expression::IfThen, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<datafusion::logical_expr::Expr>
```

---
