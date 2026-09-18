# `datafusion_substrait::logical_plan::consumer::expr::nested`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.expr.nested.json`](../model/datafusion_substrait.logical_plan.consumer.expr.nested.json)

## from_nested

`function` · `datafusion_substrait::logical_plan::consumer::expr::nested::from_nested`

```rust
async fn from_nested(consumer: &impl SubstraitConsumer, nested: &substrait::proto::expression::Nested, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<datafusion::logical_expr::Expr>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.logical_plan.consumer.expr.nested.from_nested.md).


Converts a Substrait [Nested] expression into a DataFusion [Expr].

Substrait Nested expressions represent complex type constructors (list, struct, map)
where elements are full expressions rather than just literals. This is used by
producers that emit `Nested { list: ... }` for array construction, as opposed to
`Literal { list: ... }` which only supports scalar values.

---
