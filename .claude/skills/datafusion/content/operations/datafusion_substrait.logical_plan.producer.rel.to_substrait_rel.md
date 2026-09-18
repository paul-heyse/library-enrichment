# `datafusion_substrait::logical_plan::producer::rel::to_substrait_rel`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.rel.to_substrait_rel.json).

<a id="op-4b1062059389424d44a37f5b"></a>
## to_substrait_rel

`function` · `datafusion_substrait::logical_plan::producer::rel::to_substrait_rel` · datafusion-substrait 55.1.0

```rust
fn to_substrait_rel(producer: &mut impl SubstraitProducer, plan: &datafusion::logical_expr::LogicalPlan) -> datafusion::common::Result<Box<substrait::proto::Rel>>
```

Source: `src/logical_plan/producer/rel/mod.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
