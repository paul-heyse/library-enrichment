# `datafusion_substrait::logical_plan::consumer::rel::from_substrait_rel`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.rel.from_substrait_rel.json).

<a id="op-36a538aff6809f7ffaf3aa58"></a>
## from_substrait_rel

`function` · `datafusion_substrait::logical_plan::consumer::rel::from_substrait_rel` · datafusion-substrait 55.1.0

```rust
async fn from_substrait_rel<'async_recursion>(consumer: &impl SubstraitConsumer, relation: &substrait::proto::Rel) -> datafusion::common::Result<datafusion::logical_expr::LogicalPlan> where : 'async_recursion, : 'async_recursion
```

Source: `src/logical_plan/consumer/rel/mod.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Convert Substrait Rel to DataFusion DataFrame
