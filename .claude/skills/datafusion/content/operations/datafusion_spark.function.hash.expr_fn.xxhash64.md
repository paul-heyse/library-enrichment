# `datafusion_spark::function::hash::expr_fn::xxhash64`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.hash.expr_fn.xxhash64.json).

<a id="op-8b7d85168faefa47d5a375be"></a>
## xxhash64

`function` · `datafusion_spark::function::hash::expr_fn::xxhash64` · datafusion-spark 55.1.0

```rust
fn xxhash64(args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/hash/mod.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

xxhash64(expr1, expr2, ...) - Returns a 64-bit hash value of the arguments using xxHash.
