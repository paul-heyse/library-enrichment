# `datafusion_substrait::logical_plan::producer::expr::window_function::from_window_function`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.expr.window_function.from_window_function.json).

<a id="op-6e71bd534ff8727817709857"></a>
## from_window_function

`function` · `datafusion_substrait::logical_plan::producer::expr::window_function::from_window_function` · datafusion-substrait 55.1.0

```rust
fn from_window_function(producer: &mut impl SubstraitProducer, window_fn: &datafusion::logical_expr::expr::WindowFunction, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

Source: `src/logical_plan/producer/expr/window_function.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
