# `datafusion_substrait::logical_plan::producer::expr::aggregate_function`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.expr.aggregate_function.json`](../model/datafusion_substrait.logical_plan.producer.expr.aggregate_function.json)

## from_aggregate_function

`function` · `datafusion_substrait::logical_plan::producer::expr::aggregate_function::from_aggregate_function`

```rust
fn from_aggregate_function(producer: &mut impl SubstraitProducer, agg_fn: &expr::AggregateFunction, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::aggregate_rel::Measure>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.producer.expr.aggregate_function.from_aggregate_function.md).


---
