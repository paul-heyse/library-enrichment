# `datafusion_functions_window::rank::dense_rank`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.rank.dense_rank.json).

<a id="op-52d323a9ee443a8713b98654"></a>
## dense_rank

`function` · `datafusion_functions_window::rank::dense_rank` · datafusion-functions-window 55.1.0

```rust
fn dense_rank() -> datafusion_expr::Expr
```

Source: `src/rank.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`DenseRank` user-defined window function.

Returns rank of the current row without gaps. This function counts peer groups
