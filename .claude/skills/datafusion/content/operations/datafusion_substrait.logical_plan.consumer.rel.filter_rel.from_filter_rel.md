# `datafusion_substrait::logical_plan::consumer::rel::filter_rel::from_filter_rel`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.rel.filter_rel.from_filter_rel.json).

<a id="op-08f80f53a4a027bdb561e7bd"></a>
## from_filter_rel

`function` · `datafusion_substrait::logical_plan::consumer::rel::filter_rel::from_filter_rel` · datafusion-substrait 55.1.0

```rust
async fn from_filter_rel<'async_recursion>(consumer: &impl SubstraitConsumer, filter: &substrait::proto::FilterRel) -> datafusion::common::Result<datafusion::logical_expr::LogicalPlan> where : 'async_recursion, : 'async_recursion
```

Source: `src/logical_plan/consumer/rel/filter_rel.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
