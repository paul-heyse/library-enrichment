# `datafusion_functions_window::lead_lag::lag_udwf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.lead_lag.lag_udwf.json).

<a id="op-d3793690d084be39356a5273"></a>
## lag_udwf

`function` · `datafusion_functions_window::lead_lag::lag_udwf` · datafusion-functions-window 55.1.0

```rust
fn lag_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

Source: `src/lead_lag.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`lag`](../operations/datafusion_functions_window.lead_lag.lag.md#op-40e308cff8ba80368ff89a79).

Returns the row value that precedes the current row by a specified offset within partition. If no such row exists, then returns the default value.
