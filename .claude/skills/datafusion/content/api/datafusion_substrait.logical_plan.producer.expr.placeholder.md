# `datafusion_substrait::logical_plan::producer::expr::placeholder`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.expr.placeholder.json`](../model/datafusion_substrait.logical_plan.producer.expr.placeholder.json)

## from_placeholder

`function` · `datafusion_substrait::logical_plan::producer::expr::placeholder::from_placeholder`

```rust
fn from_placeholder(producer: &mut impl SubstraitProducer, placeholder: &datafusion::logical_expr::expr::Placeholder) -> datafusion::common::Result<substrait::proto::Expression>
```

---
