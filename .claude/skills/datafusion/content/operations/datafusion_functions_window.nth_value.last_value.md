# `datafusion_functions_window::nth_value::last_value`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.nth_value.last_value.json).

<a id="op-48b05c1c049c5cd063ab372a"></a>
## last_value

`function` · `datafusion_functions_window::nth_value::last_value` · datafusion-functions-window 55.1.0

```rust
fn last_value(arg: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/nth_value.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`Last` user-defined window function.

Returns the last value in the window frame
