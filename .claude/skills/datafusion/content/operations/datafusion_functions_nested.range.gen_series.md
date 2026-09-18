# `datafusion_functions_nested::range::gen_series`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.range.gen_series.json).

<a id="op-633d8d3b0a986dd1bf2aba51"></a>
## gen_series

`function` · `datafusion_functions_nested::range::gen_series` · datafusion-functions-nested 55.1.0

```rust
fn gen_series(start: datafusion_expr::Expr, stop: datafusion_expr::Expr, step: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/range.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

create a list of values in the range between start and stop, include upper bound
