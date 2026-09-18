# `datafusion_functions_window::nth_value::nth_value_udwf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.nth_value.nth_value_udwf.json).

<a id="op-76c2f1e8aa3d06cb15e7854d"></a>
## nth_value_udwf

`function` · `datafusion_functions_window::nth_value::nth_value_udwf` · datafusion-functions-window 55.1.0

```rust
fn nth_value_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

Source: `src/nth_value.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`nth_value`](../operations/datafusion_functions_window.nth_value.nth_value.md#op-05a69fbc77691cd5be723131).

Returns the nth value in the window frame
