# `datafusion_functions_window::rank::rank_udwf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.rank.rank_udwf.json).

<a id="op-22c65b8c117f16c4828befd4"></a>
## rank_udwf

`function` · `datafusion_functions_window::rank::rank_udwf` · datafusion-functions-window 55.1.0

```rust
fn rank_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

Source: `src/rank.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`rank`](../operations/datafusion_functions_window.rank.rank.md#op-4141fbde2f5ff248191c1421).

Returns rank of the current row with gaps. Same as `row_number` of its first peer
