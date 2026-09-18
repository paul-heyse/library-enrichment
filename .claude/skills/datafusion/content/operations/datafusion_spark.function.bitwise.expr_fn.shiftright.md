# `datafusion_spark::function::bitwise::expr_fn::shiftright`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.bitwise.expr_fn.shiftright.json).

<a id="op-567f0f5666762311e1c603eb"></a>
## shiftright

`function` · `datafusion_spark::function::bitwise::expr_fn::shiftright` · datafusion-spark 55.1.0

```rust
fn shiftright(value: datafusion_expr::Expr, shift: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/function/bitwise/mod.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Shifts the bits of the first argument right by the number of positions specified by the second argument (arithmetic/signed shift). If the shift amount is negative or greater than or equal to the bit width, it is normalized to the bit width (i.e., pmod(shift, bit_width)).
