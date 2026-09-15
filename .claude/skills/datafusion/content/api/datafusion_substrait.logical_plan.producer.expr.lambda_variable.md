# `datafusion_substrait::logical_plan::producer::expr::lambda_variable`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.expr.lambda_variable.json`](../model/datafusion_substrait.logical_plan.producer.expr.lambda_variable.json)

## from_lambda_variable

`function` · `datafusion_substrait::logical_plan::producer::expr::lambda_variable::from_lambda_variable`

```rust
fn from_lambda_variable(producer: &mut impl SubstraitProducer, lambda_variable: &datafusion::logical_expr::expr::LambdaVariable, _schema: &datafusion::common::DFSchema) -> Result<substrait::proto::Expression, datafusion::error::DataFusionError>
```

---
