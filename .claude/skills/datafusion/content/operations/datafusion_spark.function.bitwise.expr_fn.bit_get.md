# `datafusion_spark::function::bitwise::expr_fn::bit_get`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.bitwise.expr_fn.bit_get.json).

<a id="op-6474cf722c110316079e8af2"></a>
## bit_get

`function` · `datafusion_spark::function::bitwise::expr_fn::bit_get` · datafusion-spark 55.1.0

```rust
fn bit_get(col: datafusion_expr::Expr, pos: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/bitwise/mod.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the value of the bit (0 or 1) at the specified position.
