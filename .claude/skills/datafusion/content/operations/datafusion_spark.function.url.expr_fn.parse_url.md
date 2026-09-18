# `datafusion_spark::function::url::expr_fn::parse_url`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.url.expr_fn.parse_url.json).

<a id="op-333c5c86a7d801569a37a475"></a>
## parse_url

`function` · `datafusion_spark::function::url::expr_fn::parse_url` · datafusion-spark 55.1.0

```rust
fn parse_url(args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/url/mod.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Extracts a part from a URL, throwing an error if an invalid URL is provided.
