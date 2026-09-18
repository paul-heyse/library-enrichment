# `datafusion_functions_aggregate::first_last::last_value_udaf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.first_last.last_value_udaf.json).

<a id="op-3d9c4b44194b32ff4129ef89"></a>
## last_value_udaf

`function` · `datafusion_functions_aggregate::first_last::last_value_udaf` · datafusion-functions-aggregate 55.1.0

```rust
fn last_value_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

Source: `src/first_last.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`LastValue`](../operations/datafusion_functions_aggregate.first_last.LastValue.md#op-11dd90b02b5881e3af6f9ff5)
