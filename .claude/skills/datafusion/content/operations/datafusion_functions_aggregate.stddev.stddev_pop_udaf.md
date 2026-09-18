# `datafusion_functions_aggregate::stddev::stddev_pop_udaf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.stddev.stddev_pop_udaf.json).

<a id="op-b4db3d1f3b35c939d41c7440"></a>
## stddev_pop_udaf

`function` · `datafusion_functions_aggregate::stddev::stddev_pop_udaf` · datafusion-functions-aggregate 55.1.0

```rust
fn stddev_pop_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

Source: `src/stddev.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`StddevPop`](../operations/datafusion_functions_aggregate.stddev.StddevPop.md#op-67a17772ae6f7518d7c2275a)
