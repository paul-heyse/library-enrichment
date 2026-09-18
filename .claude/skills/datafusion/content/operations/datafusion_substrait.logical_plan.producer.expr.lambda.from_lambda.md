# `datafusion_substrait::logical_plan::producer::expr::lambda::from_lambda`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.expr.lambda.from_lambda.json).

<a id="op-b6445337a7b9a3cba3f5f764"></a>
## from_lambda

`function` · `datafusion_substrait::logical_plan::producer::expr::lambda::from_lambda` · datafusion-substrait 55.1.0

```rust
fn from_lambda(producer: &mut impl SubstraitProducer, lambda: &datafusion::logical_expr::expr::Lambda, schema: &datafusion::common::DFSchemaRef) -> Result<substrait::proto::Expression, datafusion::error::DataFusionError>
```

Source: `src/logical_plan/producer/expr/lambda.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
