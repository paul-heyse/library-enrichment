# `datafusion_substrait::logical_plan::consumer::expr::lambda::from_lambda`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.expr.lambda.from_lambda.json).

<a id="op-93aea4d1e09b2ab5771e1210"></a>
## from_lambda

`function` · `datafusion_substrait::logical_plan::consumer::expr::lambda::from_lambda` · datafusion-substrait 55.1.0

```rust
async fn from_lambda(consumer: &impl SubstraitConsumer, expr: &proto::expression::Lambda, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<datafusion::prelude::Expr>
```

Source: `src/logical_plan/consumer/expr/lambda.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
