# `datafusion_spark::function::string::expr_fn::unbase64`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.expr_fn.unbase64.json).

<a id="op-0268aa6b271912a6816b7f0a"></a>
## unbase64

`function` · `datafusion_spark::function::string::expr_fn::unbase64` · datafusion-spark 55.1.0

```rust
fn unbase64(str: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/string/mod.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Decodes the input string `str` from a base64 string into binary data.
