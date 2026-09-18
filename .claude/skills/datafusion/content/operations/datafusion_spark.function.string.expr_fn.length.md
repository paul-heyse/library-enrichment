# `datafusion_spark::function::string::expr_fn::length`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.expr_fn.length.json).

<a id="op-0975be186e0547d2c349d137"></a>
## length

`function` · `datafusion_spark::function::string::expr_fn::length` · datafusion-spark 55.1.0

```rust
fn length(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/string/mod.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the character length of string data or number of bytes of binary data. The length of string data includes the trailing spaces. The length of binary data includes binary zeros.
