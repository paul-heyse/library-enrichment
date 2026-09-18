# `datafusion_spark::function::string::expr_fn::format_string`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.expr_fn.format_string.json).

<a id="op-d0a1b46ca3ee61e37dfc7efe"></a>
## format_string

`function` · `datafusion_spark::function::string::expr_fn::format_string` · datafusion-spark 55.1.0

```rust
fn format_string(strfmt: datafusion_expr::Expr, args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/string/mod.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns a formatted string from printf-style format strings.
