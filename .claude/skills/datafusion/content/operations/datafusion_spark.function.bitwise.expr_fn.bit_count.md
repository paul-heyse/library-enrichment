# `datafusion_spark::function::bitwise::expr_fn::bit_count`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.bitwise.expr_fn.bit_count.json).

<a id="op-a7cf02ab1c184fce88dc68da"></a>
## bit_count

`function` · `datafusion_spark::function::bitwise::expr_fn::bit_count` · datafusion-spark 55.1.0

```rust
fn bit_count(col: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/bitwise/mod.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the number of bits set in the binary representation of the argument.
