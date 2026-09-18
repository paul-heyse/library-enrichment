# `datafusion_functions_aggregate::variance::var_pop_udaf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.variance.var_pop_udaf.json).

<a id="op-f73cd313e61f1a5eff1b53e7"></a>
## var_pop_udaf

`function` · `datafusion_functions_aggregate::variance::var_pop_udaf` · datafusion-functions-aggregate 55.1.0

```rust
fn var_pop_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

Source: `src/variance.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`VariancePopulation`](../operations/datafusion_functions_aggregate.variance.VariancePopulation.md#op-2aba8181d0116c756b60f60f)
