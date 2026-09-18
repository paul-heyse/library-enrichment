# `datafusion_spark::function::string::expr_fn::concat`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.expr_fn.concat.json).

<a id="op-2494bc5a3a1679ca51d98fea"></a>
## concat

`function` · `datafusion_spark::function::string::expr_fn::concat` · datafusion-spark 55.1.0

```rust
fn concat(args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/string/mod.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Concatenates multiple input strings into a single string. Returns NULL if any input is NULL.
