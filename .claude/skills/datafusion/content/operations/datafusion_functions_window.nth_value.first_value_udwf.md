# `datafusion_functions_window::nth_value::first_value_udwf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.nth_value.first_value_udwf.json).

<a id="op-6cbe640f58465979c312aa90"></a>
## first_value_udwf

`function` · `datafusion_functions_window::nth_value::first_value_udwf` · datafusion-functions-window 55.1.0

```rust
fn first_value_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

Source: `src/nth_value.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`first_value`](../operations/datafusion_functions_window.nth_value.first_value.md#op-0b42df266b733bf5d87f9104).

Returns the first value in the window frame
