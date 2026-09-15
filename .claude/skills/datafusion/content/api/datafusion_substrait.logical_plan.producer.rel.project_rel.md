# `datafusion_substrait::logical_plan::producer::rel::project_rel`

Crate `datafusion-substrait` · 2 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.rel.project_rel.json`](../model/datafusion_substrait.logical_plan.producer.rel.project_rel.json)

## from_projection

`function` · `datafusion_substrait::logical_plan::producer::rel::project_rel::from_projection`

```rust
fn from_projection(producer: &mut impl SubstraitProducer, p: &datafusion::logical_expr::Projection) -> datafusion::common::Result<Box<substrait::proto::Rel>>
```

---

## from_window

`function` · `datafusion_substrait::logical_plan::producer::rel::project_rel::from_window`

```rust
fn from_window(producer: &mut impl SubstraitProducer, window: &datafusion::logical_expr::Window) -> datafusion::common::Result<Box<substrait::proto::Rel>>
```

---
