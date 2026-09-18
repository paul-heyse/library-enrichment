# `datafusion_functions_window::rank::percent_rank_udwf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.rank.percent_rank_udwf.json).

<a id="op-0a1af0d90afe5cbe48fa5ccc"></a>
## percent_rank_udwf

`function` · `datafusion_functions_window::rank::percent_rank_udwf` · datafusion-functions-window 55.1.0

```rust
fn percent_rank_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

Source: `src/rank.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`percent_rank`](../operations/datafusion_functions_window.rank.percent_rank.md#op-52bb53625ae9817671c39362).

Returns the relative rank of the current row: (rank - 1) / (total rows - 1)
