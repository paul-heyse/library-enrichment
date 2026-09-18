# `datafusion_spark::function::hash::expr_fn::sha2`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.hash.expr_fn.sha2.json).

<a id="op-bbfff405e046a9ab14e7a1b6"></a>
## sha2

`function` · `datafusion_spark::function::hash::expr_fn::sha2` · datafusion-spark 55.1.0

```rust
fn sha2(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/hash/mod.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

sha2(expr, bitLength) - Returns a checksum of SHA-2 family as a hex string of expr. SHA-224, SHA-256, SHA-384, and SHA-512 are supported. Bit length of 0 is equivalent to 256.
