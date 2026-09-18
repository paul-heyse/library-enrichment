# `datafusion_substrait::logical_plan::consumer::expr::if_then::from_if_then`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.expr.if_then.from_if_then.json).

<a id="op-348c14bda70d41cea659c3ec"></a>
## from_if_then

`function` · `datafusion_substrait::logical_plan::consumer::expr::if_then::from_if_then` · datafusion-substrait 55.1.0

```rust
async fn from_if_then(consumer: &impl SubstraitConsumer, if_then: &substrait::proto::expression::IfThen, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<datafusion::logical_expr::Expr>
```

Source: `src/logical_plan/consumer/expr/if_then.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
