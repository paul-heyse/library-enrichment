# `datafusion_functions_window::cume_dist::cume_dist`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.cume_dist.cume_dist.json).

<a id="op-3bd9711ac9ae294f84f7ca1c"></a>
## cume_dist

`function` · `datafusion_functions_window::cume_dist::cume_dist` · datafusion-functions-window 55.1.0

```rust
fn cume_dist() -> datafusion_expr::Expr
```

Source: `src/cume_dist.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`CumeDist` user-defined window function.

Calculates the cumulative distribution of a value in a group of values.
