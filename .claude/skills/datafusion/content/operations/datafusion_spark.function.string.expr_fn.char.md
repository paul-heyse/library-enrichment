# `datafusion_spark::function::string::expr_fn::char`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.expr_fn.char.json).

<a id="op-9349f13942f32d81674c8685"></a>
## char

`function` · `datafusion_spark::function::string::expr_fn::char` · datafusion-spark 55.1.0

```rust
fn char(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/string/mod.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the ASCII character having the binary equivalent to col. If col is larger than 256 the result is equivalent to char(col % 256).
