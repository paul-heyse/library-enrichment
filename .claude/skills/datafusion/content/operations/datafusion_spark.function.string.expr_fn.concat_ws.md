# `datafusion_spark::function::string::expr_fn::concat_ws`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.expr_fn.concat_ws.json).

<a id="op-6f27d09f18782df28dbf6b0c"></a>
## concat_ws

`function` · `datafusion_spark::function::string::expr_fn::concat_ws` · datafusion-spark 55.1.0

```rust
fn concat_ws(sep: datafusion_expr::Expr, args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/string/mod.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Concatenates strings with separator. Supports arrays. Null values are skipped.
