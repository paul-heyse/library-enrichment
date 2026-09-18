# `datafusion_functions_window::nth_value::last_value_udwf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.nth_value.last_value_udwf.json).

<a id="op-eb07546f772e8317179d8466"></a>
## last_value_udwf

`function` · `datafusion_functions_window::nth_value::last_value_udwf` · datafusion-functions-window 55.1.0

```rust
fn last_value_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

Source: `src/nth_value.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`last_value`](../operations/datafusion_functions_window.nth_value.last_value.md#op-48b05c1c049c5cd063ab372a).

Returns the last value in the window frame
