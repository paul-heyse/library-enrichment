# `datafusion_substrait::logical_plan::producer::substrait_producer::lambda_parameters_map`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.substrait_producer.lambda_parameters_map.json).

<a id="op-95c1e42c0bf1a83c4d4dad54"></a>
## lambda_parameters_map

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::lambda_parameters_map` · datafusion-substrait 55.1.0

```rust
fn lambda_parameters_map(producer: &mut impl SubstraitProducer, lambda_parameters: Vec<datafusion::arrow::datatypes::FieldRef>) -> datafusion::common::Result<datafusion::common::HashMap<String, (usize, substrait::proto::Type)>>
```

Source: `src/logical_plan/producer/substrait_producer.rs:655`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Produces a map of lambda parameters as expected by [DefaultSubstraitLambdaProducer::push_lambda_parameters](../operations/datafusion_substrait.logical_plan.producer.substrait_producer.DefaultSubstraitLambdaProducer.md#op-76cc0fb6b436d62bccba37b6)
