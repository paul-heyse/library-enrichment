# `datafusion_spark::function::bitwise::expr_fn::shiftrightunsigned`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.bitwise.expr_fn.shiftrightunsigned.json).

<a id="op-14298b9f750f8523954a1534"></a>
## shiftrightunsigned

`function` · `datafusion_spark::function::bitwise::expr_fn::shiftrightunsigned` · datafusion-spark 55.1.0

```rust
fn shiftrightunsigned(value: datafusion_expr::Expr, shift: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/bitwise/mod.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Shifts the bits of the first argument right by the number of positions specified by the second argument (logical/unsigned shift). If the shift amount is negative or greater than or equal to the bit width, it is normalized to the bit width (i.e., pmod(shift, bit_width)).
