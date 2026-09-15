# `datafusion_substrait::logical_plan::producer::rel::exchange_rel`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.rel.exchange_rel.json`](../model/datafusion_substrait.logical_plan.producer.rel.exchange_rel.json)

## from_repartition

`function` · `datafusion_substrait::logical_plan::producer::rel::exchange_rel::from_repartition`

```rust
fn from_repartition(producer: &mut impl SubstraitProducer, repartition: &datafusion::logical_expr::Repartition) -> datafusion::common::Result<Box<substrait::proto::Rel>>
```

---
