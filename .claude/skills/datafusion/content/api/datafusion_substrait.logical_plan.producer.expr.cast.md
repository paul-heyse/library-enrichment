# `datafusion_substrait::logical_plan::producer::expr::cast`

Crate `datafusion-substrait` · 2 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.expr.cast.json`](../model/datafusion_substrait.logical_plan.producer.expr.cast.json)

## from_cast

`function` · `datafusion_substrait::logical_plan::producer::expr::cast::from_cast`

```rust
fn from_cast(producer: &mut impl SubstraitProducer, cast: &datafusion::logical_expr::Cast, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.producer.expr.cast.from_cast.md).


---

## from_try_cast

`function` · `datafusion_substrait::logical_plan::producer::expr::cast::from_try_cast`

```rust
fn from_try_cast(producer: &mut impl SubstraitProducer, cast: &datafusion::logical_expr::TryCast, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.producer.expr.cast.from_try_cast.md).


---
