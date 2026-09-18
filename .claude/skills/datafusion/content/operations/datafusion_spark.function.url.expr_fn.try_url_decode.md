# `datafusion_spark::function::url::expr_fn::try_url_decode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.url.expr_fn.try_url_decode.json).

<a id="op-b6a6af26f108de6bf03b7b70"></a>
## try_url_decode

`function` · `datafusion_spark::function::url::expr_fn::try_url_decode` · datafusion-spark 55.1.0

```rust
fn try_url_decode(args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/url/mod.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Same as url_decode but returns NULL if an invalid URL-encoded string is provided
