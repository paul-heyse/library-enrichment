# `datafusion_functions_aggregate::approx_median::approx_median_udaf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.approx_median.approx_median_udaf.json).

<a id="op-6fbdd5203ed06b61c91c54f6"></a>
## approx_median_udaf

`function` · `datafusion_functions_aggregate::approx_median::approx_median_udaf` · datafusion-functions-aggregate 55.1.0

```rust
fn approx_median_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

Source: `src/approx_median.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`ApproxMedian`](../operations/datafusion_functions_aggregate.approx_median.ApproxMedian.md#op-b3c48ecad789423beae97699)
