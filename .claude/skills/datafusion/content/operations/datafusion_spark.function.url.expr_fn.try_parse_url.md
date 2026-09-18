# `datafusion_spark::function::url::expr_fn::try_parse_url`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.url.expr_fn.try_parse_url.json).

<a id="op-26346cdb7c5e2ef015583627"></a>
## try_parse_url

`function` · `datafusion_spark::function::url::expr_fn::try_parse_url` · datafusion-spark 55.1.0

```rust
fn try_parse_url(args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/url/mod.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Same as parse_url but returns NULL if an invalid URL is provided.
