# `datafusion_functions_aggregate::approx_distinct::approx_distinct_udaf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.approx_distinct.approx_distinct_udaf.json).

<a id="op-004d81d02ece903ba102ba92"></a>
## approx_distinct_udaf

`function` · `datafusion_functions_aggregate::approx_distinct::approx_distinct_udaf` · datafusion-functions-aggregate 55.1.0

```rust
fn approx_distinct_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

Source: `src/approx_distinct.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`ApproxDistinct`](../operations/datafusion_functions_aggregate.approx_distinct.ApproxDistinct.md#op-c9af29cd8e14fdb320d3e537)
