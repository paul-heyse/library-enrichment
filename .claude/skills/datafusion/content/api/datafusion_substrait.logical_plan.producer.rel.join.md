# `datafusion_substrait::logical_plan::producer::rel::join`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.rel.join.json`](../model/datafusion_substrait.logical_plan.producer.rel.join.json)

## from_join

`function` · `datafusion_substrait::logical_plan::producer::rel::join::from_join`

```rust
fn from_join(producer: &mut impl SubstraitProducer, join: &datafusion::logical_expr::Join) -> datafusion::common::Result<Box<substrait::proto::Rel>>
```

---
