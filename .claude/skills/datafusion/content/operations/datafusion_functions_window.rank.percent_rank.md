# `datafusion_functions_window::rank::percent_rank`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.rank.percent_rank.json).

<a id="op-52bb53625ae9817671c39362"></a>
## percent_rank

`function` · `datafusion_functions_window::rank::percent_rank` · datafusion-functions-window 55.1.0

```rust
fn percent_rank() -> datafusion_expr::Expr
```

Source: `src/rank.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`PercentRank` user-defined window function.

Returns the relative rank of the current row: (rank - 1) / (total rows - 1)
