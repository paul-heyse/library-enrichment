# `datafusion_substrait::logical_plan::producer::rel::aggregate_rel::to_substrait_groupings`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.rel.aggregate_rel.to_substrait_groupings.json).

<a id="op-9b789b74209b68c24cde8f8d"></a>
## to_substrait_groupings

`function` · `datafusion_substrait::logical_plan::producer::rel::aggregate_rel::to_substrait_groupings` · datafusion-substrait 55.1.0

```rust
fn to_substrait_groupings(producer: &mut impl SubstraitProducer, exprs: &[datafusion::logical_expr::Expr], schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<(Vec<substrait::proto::Expression>, Vec<substrait::proto::aggregate_rel::Grouping>)>
```

Source: `src/logical_plan/producer/rel/aggregate_rel.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
