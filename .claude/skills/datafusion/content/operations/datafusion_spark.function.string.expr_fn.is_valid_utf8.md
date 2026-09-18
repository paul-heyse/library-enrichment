# `datafusion_spark::function::string::expr_fn::is_valid_utf8`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.expr_fn.is_valid_utf8.json).

<a id="op-862892341725ffe115805857"></a>
## is_valid_utf8

`function` · `datafusion_spark::function::string::expr_fn::is_valid_utf8` · datafusion-spark 55.1.0

```rust
fn is_valid_utf8(str: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/string/mod.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns true if str is a valid UTF-8 string, otherwise returns false
