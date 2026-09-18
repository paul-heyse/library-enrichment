# `datafusion_substrait::logical_plan::producer::rel::filter_rel`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.rel.filter_rel.json`](../model/datafusion_substrait.logical_plan.producer.rel.filter_rel.json)

## from_filter

`function` · `datafusion_substrait::logical_plan::producer::rel::filter_rel::from_filter`

```rust
fn from_filter(producer: &mut impl SubstraitProducer, filter: &datafusion::logical_expr::Filter) -> datafusion::common::Result<Box<substrait::proto::Rel>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.producer.rel.filter_rel.from_filter.md).


---
