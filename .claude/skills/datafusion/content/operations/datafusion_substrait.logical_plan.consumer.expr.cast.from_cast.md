# `datafusion_substrait::logical_plan::consumer::expr::cast::from_cast`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.expr.cast.from_cast.json).

<a id="op-86bbe6a94fb6fff67d978c6f"></a>
## from_cast

`function` · `datafusion_substrait::logical_plan::consumer::expr::cast::from_cast` · datafusion-substrait 55.1.0

```rust
async fn from_cast(consumer: &impl SubstraitConsumer, cast: &substrait_expression::Cast, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<datafusion::logical_expr::Expr>
```

Source: `src/logical_plan/consumer/expr/cast.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
