# `datafusion_functions_aggregate::nth_value::nth_value_udaf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.nth_value.nth_value_udaf.json).

<a id="op-34f5d50ebc7376d7c642357e"></a>
## nth_value_udaf

`function` · `datafusion_functions_aggregate::nth_value::nth_value_udaf` · datafusion-functions-aggregate 55.1.0

```rust
fn nth_value_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

Source: `src/nth_value.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`NthValueAgg`](../operations/datafusion_functions_aggregate.nth_value.NthValueAgg.md#op-34b49a6dadde0213adc1dfa2)
