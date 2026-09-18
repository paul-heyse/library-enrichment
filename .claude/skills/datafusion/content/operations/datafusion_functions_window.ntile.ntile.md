# `datafusion_functions_window::ntile::ntile`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.ntile.ntile.json).

<a id="op-b53932c8b405a4af79aae2f9"></a>
## ntile

`function` · `datafusion_functions_window::ntile::ntile` · datafusion-functions-window 55.1.0

```rust
fn ntile(arg: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/ntile.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`Ntile` user-defined window function.

Integer ranging from 1 to the argument value, dividing the partition as equally as possible.
