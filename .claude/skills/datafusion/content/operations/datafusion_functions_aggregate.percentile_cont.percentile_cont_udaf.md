# `datafusion_functions_aggregate::percentile_cont::percentile_cont_udaf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.percentile_cont.percentile_cont_udaf.json).

<a id="op-b55a912599bd5b272042b779"></a>
## percentile_cont_udaf

`function` · `datafusion_functions_aggregate::percentile_cont::percentile_cont_udaf` · datafusion-functions-aggregate 55.1.0

```rust
fn percentile_cont_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

Source: `src/percentile_cont.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`PercentileCont`](../operations/datafusion_functions_aggregate.percentile_cont.PercentileCont.md#op-fdfc3e38e1fc56e24a3ebc9f)
