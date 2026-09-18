# `datafusion_spark::function::url::expr_fn::url_decode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.url.expr_fn.url_decode.json).

<a id="op-63b32c8060354239281ef00b"></a>
## url_decode

`function` · `datafusion_spark::function::url::expr_fn::url_decode` · datafusion-spark 55.1.0

```rust
fn url_decode(args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/url/mod.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Decodes a URL-encoded string in ‘application/x-www-form-urlencoded’ format to its original format.
