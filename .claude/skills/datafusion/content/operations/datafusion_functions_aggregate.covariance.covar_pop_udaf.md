# `datafusion_functions_aggregate::covariance::covar_pop_udaf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.covariance.covar_pop_udaf.json).

<a id="op-8153193bf395a2ebc515d16a"></a>
## covar_pop_udaf

`function` · `datafusion_functions_aggregate::covariance::covar_pop_udaf` · datafusion-functions-aggregate 55.1.0

```rust
fn covar_pop_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

Source: `src/covariance.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`CovariancePopulation`](../operations/datafusion_functions_aggregate.covariance.CovariancePopulation.md#op-51f89cb4fc4bc2cae12c79c1)
