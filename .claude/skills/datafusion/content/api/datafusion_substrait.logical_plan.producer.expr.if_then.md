# `datafusion_substrait::logical_plan::producer::expr::if_then`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.expr.if_then.json`](../model/datafusion_substrait.logical_plan.producer.expr.if_then.json)

## from_case

`function` · `datafusion_substrait::logical_plan::producer::expr::if_then::from_case`

```rust
fn from_case(producer: &mut impl SubstraitProducer, case: &datafusion::logical_expr::Case, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.producer.expr.if_then.from_case.md).


---
