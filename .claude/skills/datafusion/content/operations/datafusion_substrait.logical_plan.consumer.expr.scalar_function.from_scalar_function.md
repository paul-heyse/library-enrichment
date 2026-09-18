# `datafusion_substrait::logical_plan::consumer::expr::scalar_function::from_scalar_function`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.expr.scalar_function.from_scalar_function.json).

<a id="op-ac63637dbb576ac6ae9b96e2"></a>
## from_scalar_function

`function` · `datafusion_substrait::logical_plan::consumer::expr::scalar_function::from_scalar_function` · datafusion-substrait 55.1.0

```rust
async fn from_scalar_function(consumer: &impl SubstraitConsumer, f: &substrait::proto::expression::ScalarFunction, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<datafusion::logical_expr::Expr>
```

Source: `src/logical_plan/consumer/expr/scalar_function.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
