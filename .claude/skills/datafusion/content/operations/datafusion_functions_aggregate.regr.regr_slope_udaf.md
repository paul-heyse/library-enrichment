# `datafusion_functions_aggregate::regr::regr_slope_udaf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.regr.regr_slope_udaf.json).

<a id="op-cb4f1f1a75b4b2bbc58cbb9f"></a>
## regr_slope_udaf

`function` · `datafusion_functions_aggregate::regr::regr_slope_udaf` · datafusion-functions-aggregate 55.1.0

```rust
fn regr_slope_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

Source: `src/regr.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_slope`](../operations/datafusion_functions_aggregate.regr.regr_slope.md#op-f3723f0595923b67f775eae8)
