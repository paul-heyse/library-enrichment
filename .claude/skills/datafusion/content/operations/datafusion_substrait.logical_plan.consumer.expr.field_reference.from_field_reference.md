# `datafusion_substrait::logical_plan::consumer::expr::field_reference::from_field_reference`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.expr.field_reference.from_field_reference.json).

<a id="op-59792d30bdf3c24f7eaf2eea"></a>
## from_field_reference

`function` · `datafusion_substrait::logical_plan::consumer::expr::field_reference::from_field_reference` · datafusion-substrait 55.1.0

```rust
async fn from_field_reference(consumer: &impl SubstraitConsumer, field_ref: &substrait::proto::expression::FieldReference, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<datafusion::logical_expr::Expr>
```

Source: `src/logical_plan/consumer/expr/field_reference.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
