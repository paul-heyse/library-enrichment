# `datafusion_functions_window::ntile::ntile_udwf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.ntile.ntile_udwf.json).

<a id="op-27e24b85d174aa532313cc2f"></a>
## ntile_udwf

`function` · `datafusion_functions_window::ntile::ntile_udwf` · datafusion-functions-window 55.1.0

```rust
fn ntile_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

Source: `src/ntile.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`ntile`](../operations/datafusion_functions_window.ntile.ntile.md#op-b53932c8b405a4af79aae2f9).

Integer ranging from 1 to the argument value, dividing the partition as equally as possible.
