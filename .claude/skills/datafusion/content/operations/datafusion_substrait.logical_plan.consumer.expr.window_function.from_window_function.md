# `datafusion_substrait::logical_plan::consumer::expr::window_function::from_window_function`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.expr.window_function.from_window_function.json).

<a id="op-4c2dafd50a9ee4677d41d2c2"></a>
## from_window_function

`function` · `datafusion_substrait::logical_plan::consumer::expr::window_function::from_window_function` · datafusion-substrait 55.1.0

```rust
async fn from_window_function(consumer: &impl SubstraitConsumer, window: &substrait::proto::expression::WindowFunction, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<datafusion::logical_expr::Expr>
```

Source: `src/logical_plan/consumer/expr/window_function.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
