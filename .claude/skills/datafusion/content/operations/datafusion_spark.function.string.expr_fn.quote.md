# `datafusion_spark::function::string::expr_fn::quote`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.expr_fn.quote.json).

<a id="op-0c9931133179a5d14cf9d8ee"></a>
## quote

`function` · `datafusion_spark::function::string::expr_fn::quote` · datafusion-spark 55.1.0

```rust
fn quote(str: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/string/mod.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns str enclosed by single quotes and each instance of single quote in it is preceded by a backslash
