# `datafusion_functions_window::rank::rank`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.rank.rank.json).

<a id="op-4141fbde2f5ff248191c1421"></a>
## rank

`function` · `datafusion_functions_window::rank::rank` · datafusion-functions-window 55.1.0

```rust
fn rank() -> datafusion_expr::Expr
```

Source: `src/rank.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`Rank` user-defined window function.

Returns rank of the current row with gaps. Same as `row_number` of its first peer
