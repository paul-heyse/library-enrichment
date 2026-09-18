# `datafusion_functions_aggregate::regr::regr_intercept_udaf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.regr.regr_intercept_udaf.json).

<a id="op-5b85c4b11627ecd98ac374cb"></a>
## regr_intercept_udaf

`function` · `datafusion_functions_aggregate::regr::regr_intercept_udaf` · datafusion-functions-aggregate 55.1.0

```rust
fn regr_intercept_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

Source: `src/regr.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_intercept`](../operations/datafusion_functions_aggregate.regr.regr_intercept.md#op-f19b1e443a62b26dd5b1da32)
