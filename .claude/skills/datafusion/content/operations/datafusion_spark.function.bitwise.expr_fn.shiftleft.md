# `datafusion_spark::function::bitwise::expr_fn::shiftleft`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.bitwise.expr_fn.shiftleft.json).

<a id="op-d11ac2db6b7334863aaecea3"></a>
## shiftleft

`function` · `datafusion_spark::function::bitwise::expr_fn::shiftleft` · datafusion-spark 55.1.0

```rust
fn shiftleft(value: datafusion_expr::Expr, shift: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/bitwise/mod.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Shifts the bits of the first argument left by the number of positions specified by the second argument. If the shift amount is negative or greater than or equal to the bit width, it is normalized to the bit width (i.e., pmod(shift, bit_width)).
