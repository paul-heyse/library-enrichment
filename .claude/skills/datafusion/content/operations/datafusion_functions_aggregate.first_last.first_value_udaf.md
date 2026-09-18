# `datafusion_functions_aggregate::first_last::first_value_udaf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.first_last.first_value_udaf.json).

<a id="op-af4802858b69cb4bcb9d99a6"></a>
## first_value_udaf

`function` · `datafusion_functions_aggregate::first_last::first_value_udaf` · datafusion-functions-aggregate 55.1.0

```rust
fn first_value_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

Source: `src/first_last.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`FirstValue`](../operations/datafusion_functions_aggregate.first_last.FirstValue.md#op-a117c22d6064c7fc2d59cfab)
