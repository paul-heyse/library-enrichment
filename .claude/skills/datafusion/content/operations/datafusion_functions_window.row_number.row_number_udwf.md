# `datafusion_functions_window::row_number::row_number_udwf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.row_number.row_number_udwf.json).

<a id="op-2341e725eb48f223ef50e640"></a>
## row_number_udwf

`function` · `datafusion_functions_window::row_number::row_number_udwf` · datafusion-functions-window 55.1.0

```rust
fn row_number_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

Source: `src/row_number.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`row_number`](../operations/datafusion_functions_window.row_number.row_number.md#op-6ab26d86026e8eb7a4598bbc).

Returns a unique row number for each row in window partition beginning at 1.
