# `datafusion_spark::function::string::expr_fn::substring`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.expr_fn.substring.json).

<a id="op-a10b7f6c7c7cd73e23c341ec"></a>
## substring

`function` · `datafusion_spark::function::string::expr_fn::substring` · datafusion-spark 55.1.0

```rust
fn substring(str: datafusion_expr::Expr, pos: datafusion_expr::Expr, length: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/string/mod.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the substring from string `str` starting at position `pos` with length `length.
