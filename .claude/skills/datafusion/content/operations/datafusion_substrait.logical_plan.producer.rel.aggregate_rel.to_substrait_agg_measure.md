# `datafusion_substrait::logical_plan::producer::rel::aggregate_rel::to_substrait_agg_measure`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.rel.aggregate_rel.to_substrait_agg_measure.json).

<a id="op-185d4f40f6fd5e3a3d3a24a1"></a>
## to_substrait_agg_measure

`function` · `datafusion_substrait::logical_plan::producer::rel::aggregate_rel::to_substrait_agg_measure` · datafusion-substrait 55.1.0

```rust
fn to_substrait_agg_measure(producer: &mut impl SubstraitProducer, expr: &datafusion::logical_expr::Expr, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::aggregate_rel::Measure>
```

Source: `src/logical_plan/producer/rel/aggregate_rel.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
