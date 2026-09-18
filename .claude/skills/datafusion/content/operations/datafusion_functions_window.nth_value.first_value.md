# `datafusion_functions_window::nth_value::first_value`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.nth_value.first_value.json).

<a id="op-0b42df266b733bf5d87f9104"></a>
## first_value

`function` · `datafusion_functions_window::nth_value::first_value` · datafusion-functions-window 55.1.0

```rust
fn first_value(arg: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/nth_value.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`First` user-defined window function.

Returns the first value in the window frame
