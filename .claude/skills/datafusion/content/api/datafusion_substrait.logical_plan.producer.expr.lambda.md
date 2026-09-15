# `datafusion_substrait::logical_plan::producer::expr::lambda`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.expr.lambda.json`](../model/datafusion_substrait.logical_plan.producer.expr.lambda.json)

## from_lambda

`function` · `datafusion_substrait::logical_plan::producer::expr::lambda::from_lambda`

```rust
fn from_lambda(producer: &mut impl SubstraitProducer, lambda: &datafusion::logical_expr::expr::Lambda, schema: &datafusion::common::DFSchemaRef) -> Result<substrait::proto::Expression, datafusion::error::DataFusionError>
```

---
