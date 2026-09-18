# `datafusion_substrait::logical_plan::consumer::plan::from_substrait_plan_with_consumer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.plan.from_substrait_plan_with_consumer.json).

<a id="op-12fb8dfb7f5f2372b70290bb"></a>
## from_substrait_plan_with_consumer

`function` · `datafusion_substrait::logical_plan::consumer::plan::from_substrait_plan_with_consumer` · datafusion-substrait 55.1.0

```rust
async fn from_substrait_plan_with_consumer(consumer: &impl SubstraitConsumer, plan: &substrait::proto::Plan) -> datafusion::common::Result<datafusion::logical_expr::LogicalPlan>
```

Source: `src/logical_plan/consumer/plan.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Convert Substrait Plan to DataFusion LogicalPlan using the given consumer
