# `datafusion_functions_aggregate::variance::var_samp_udaf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.variance.var_samp_udaf.json).

<a id="op-bb5f1b4c81ea8077f27d1f98"></a>
## var_samp_udaf

`function` · `datafusion_functions_aggregate::variance::var_samp_udaf` · datafusion-functions-aggregate 55.1.0

```rust
fn var_samp_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

Source: `src/variance.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`VarianceSample`](../operations/datafusion_functions_aggregate.variance.VarianceSample.md#op-a430035d7c3eefc128e96f68)
