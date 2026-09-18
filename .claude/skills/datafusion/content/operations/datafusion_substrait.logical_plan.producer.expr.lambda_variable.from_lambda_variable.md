# `datafusion_substrait::logical_plan::producer::expr::lambda_variable::from_lambda_variable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.expr.lambda_variable.from_lambda_variable.json).

<a id="op-a111b5588fa1be9efd0ebc66"></a>
## from_lambda_variable

`function` · `datafusion_substrait::logical_plan::producer::expr::lambda_variable::from_lambda_variable` · datafusion-substrait 55.1.0

```rust
fn from_lambda_variable(producer: &mut impl SubstraitProducer, lambda_variable: &datafusion::logical_expr::expr::LambdaVariable, _schema: &datafusion::common::DFSchema) -> Result<substrait::proto::Expression, datafusion::error::DataFusionError>
```

Source: `src/logical_plan/producer/expr/lambda_variable.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
