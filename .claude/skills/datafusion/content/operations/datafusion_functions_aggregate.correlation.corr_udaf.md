# `datafusion_functions_aggregate::correlation::corr_udaf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.correlation.corr_udaf.json).

<a id="op-dcefdc82d1ee7d2e00173fd7"></a>
## corr_udaf

`function` · `datafusion_functions_aggregate::correlation::corr_udaf` · datafusion-functions-aggregate 55.1.0

```rust
fn corr_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

Source: `src/correlation.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`Correlation`](../operations/datafusion_functions_aggregate.correlation.Correlation.md#op-3fd6ac9849edb93926f71627)
