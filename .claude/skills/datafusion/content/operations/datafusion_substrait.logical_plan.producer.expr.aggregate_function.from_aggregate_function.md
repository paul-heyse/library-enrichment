# `datafusion_substrait::logical_plan::producer::expr::aggregate_function::from_aggregate_function`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.expr.aggregate_function.from_aggregate_function.json).

<a id="op-8178e0967154385a229fe005"></a>
## from_aggregate_function

`function` · `datafusion_substrait::logical_plan::producer::expr::aggregate_function::from_aggregate_function` · datafusion-substrait 55.1.0

```rust
fn from_aggregate_function(producer: &mut impl SubstraitProducer, agg_fn: &expr::AggregateFunction, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::aggregate_rel::Measure>
```

Source: `src/logical_plan/producer/expr/aggregate_function.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
