# `datafusion_functions_window::lead_lag::lead_udwf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.lead_lag.lead_udwf.json).

<a id="op-5e9125870e8b32520cc783f2"></a>
## lead_udwf

`function` · `datafusion_functions_window::lead_lag::lead_udwf` · datafusion-functions-window 55.1.0

```rust
fn lead_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

Source: `src/lead_lag.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`lead`](../operations/datafusion_functions_window.lead_lag.lead.md#op-e63e45bbaf5bf1f124dca9f9).

Returns the value from a row that follows the current row by a specified offset within the partition. If no such row exists, then returns the default value.
