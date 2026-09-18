# `datafusion_functions_window::row_number::row_number`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.row_number.row_number.json).

<a id="op-6ab26d86026e8eb7a4598bbc"></a>
## row_number

`function` · `datafusion_functions_window::row_number::row_number` · datafusion-functions-window 55.1.0

```rust
fn row_number() -> datafusion_expr::Expr
```

Source: `src/row_number.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`RowNumber` user-defined window function.

Returns a unique row number for each row in window partition beginning at 1.
