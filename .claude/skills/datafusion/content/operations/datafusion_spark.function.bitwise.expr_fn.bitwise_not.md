# `datafusion_spark::function::bitwise::expr_fn::bitwise_not`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.bitwise.expr_fn.bitwise_not.json).

<a id="op-9f4545c5a594d091aa81befa"></a>
## bitwise_not

`function` · `datafusion_spark::function::bitwise::expr_fn::bitwise_not` · datafusion-spark 55.1.0

```rust
fn bitwise_not(col: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/bitwise/mod.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the result of a bitwise negation operation on the argument, where each bit in the binary representation is flipped, following two's complement arithmetic for signed integers.
