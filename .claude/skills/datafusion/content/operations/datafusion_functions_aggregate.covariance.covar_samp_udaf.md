# `datafusion_functions_aggregate::covariance::covar_samp_udaf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.covariance.covar_samp_udaf.json).

<a id="op-d043ce36b4d9e76b6a9b53ba"></a>
## covar_samp_udaf

`function` · `datafusion_functions_aggregate::covariance::covar_samp_udaf` · datafusion-functions-aggregate 55.1.0

```rust
fn covar_samp_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

Source: `src/covariance.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`CovarianceSample`](../operations/datafusion_functions_aggregate.covariance.CovarianceSample.md#op-e5fa762d69a4fae51b9b03bb)
