# `datafusion_substrait::logical_plan::producer::expr::scalar_function::from_higher_order_function`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.expr.scalar_function.from_higher_order_function.json).

<a id="op-0c8f73671857b496a80cdc4c"></a>
## from_higher_order_function

`function` · `datafusion_substrait::logical_plan::producer::expr::scalar_function::from_higher_order_function` · datafusion-substrait 55.1.0

```rust
fn from_higher_order_function(producer: &mut impl SubstraitProducer, fun: &expr::HigherOrderFunction, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

Source: `src/logical_plan/producer/expr/scalar_function.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
