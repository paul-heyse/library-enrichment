# `datafusion_functions_window::cume_dist::cume_dist_udwf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.cume_dist.cume_dist_udwf.json).

<a id="op-40119cb5747df571b06f781f"></a>
## cume_dist_udwf

`function` · `datafusion_functions_window::cume_dist::cume_dist_udwf` · datafusion-functions-window 55.1.0

```rust
fn cume_dist_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

Source: `src/cume_dist.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`cume_dist`](../operations/datafusion_functions_window.cume_dist.cume_dist.md#op-3bd9711ac9ae294f84f7ca1c).

Calculates the cumulative distribution of a value in a group of values.
