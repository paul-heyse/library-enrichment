# `datafusion_spark::function::bitmap::expr_fn::bitmap_count`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.bitmap.expr_fn.bitmap_count.json).

<a id="op-c1e8ece1a8680c7411ad4312"></a>
## bitmap_count

`function` · `datafusion_spark::function::bitmap::expr_fn::bitmap_count` · datafusion-spark 55.1.0

```rust
fn bitmap_count(arg: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/bitmap/mod.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the number of set bits in the input bitmap.
