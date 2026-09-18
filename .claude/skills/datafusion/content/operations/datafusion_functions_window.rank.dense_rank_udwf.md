# `datafusion_functions_window::rank::dense_rank_udwf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.rank.dense_rank_udwf.json).

<a id="op-5ea0ce7299a39938c7ce12ff"></a>
## dense_rank_udwf

`function` · `datafusion_functions_window::rank::dense_rank_udwf` · datafusion-functions-window 55.1.0

```rust
fn dense_rank_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

Source: `src/rank.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`dense_rank`](../operations/datafusion_functions_window.rank.dense_rank.md#op-52d323a9ee443a8713b98654).

Returns rank of the current row without gaps. This function counts peer groups
