# `datafusion_substrait::logical_plan::consumer::expr::lambda`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.expr.lambda.json`](../model/datafusion_substrait.logical_plan.consumer.expr.lambda.json)

## from_lambda

`function` · `datafusion_substrait::logical_plan::consumer::expr::lambda::from_lambda`

```rust
async fn from_lambda(consumer: &impl SubstraitConsumer, expr: &proto::expression::Lambda, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<datafusion::prelude::Expr>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.expr.lambda.from_lambda.md).


---
