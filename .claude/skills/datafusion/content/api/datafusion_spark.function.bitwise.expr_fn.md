# `datafusion_spark::function::bitwise::expr_fn`

Crate `datafusion-spark` · 6 public items · structured records in [`model/datafusion_spark.function.bitwise.expr_fn.json`](../model/datafusion_spark.function.bitwise.expr_fn.json)

## bit_count

`function` · `datafusion_spark::function::bitwise::expr_fn::bit_count`

Also reachable as `datafusion_spark::expr_fn::bit_count`

```rust
fn bit_count(col: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.bitwise.expr_fn.bit_count.md).


Returns the number of bits set in the binary representation of the argument.

---

## bit_get

`function` · `datafusion_spark::function::bitwise::expr_fn::bit_get`

Also reachable as `datafusion_spark::expr_fn::bit_get`

```rust
fn bit_get(col: datafusion_expr::Expr, pos: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.bitwise.expr_fn.bit_get.md).


Returns the value of the bit (0 or 1) at the specified position.

---

## bitwise_not

`function` · `datafusion_spark::function::bitwise::expr_fn::bitwise_not`

Also reachable as `datafusion_spark::expr_fn::bitwise_not`

```rust
fn bitwise_not(col: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.bitwise.expr_fn.bitwise_not.md).


Returns the result of a bitwise negation operation on the argument, where each bit in the binary representation is flipped, following two's complement arithmetic for signed integers.

---

## shiftleft

`function` · `datafusion_spark::function::bitwise::expr_fn::shiftleft`

Also reachable as `datafusion_spark::expr_fn::shiftleft`

```rust
fn shiftleft(value: datafusion_expr::Expr, shift: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.bitwise.expr_fn.shiftleft.md).


Shifts the bits of the first argument left by the number of positions specified by the second argument. If the shift amount is negative or greater than or equal to the bit width, it is normalized to the bit width (i.e., pmod(shift, bit_width)).

---

## shiftright

`function` · `datafusion_spark::function::bitwise::expr_fn::shiftright`

Also reachable as `datafusion_spark::expr_fn::shiftright`

```rust
fn shiftright(value: datafusion_expr::Expr, shift: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.bitwise.expr_fn.shiftright.md).


Shifts the bits of the first argument right by the number of positions specified by the second argument (arithmetic/signed shift). If the shift amount is negative or greater than or equal to the bit width, it is normalized to the bit width (i.e., pmod(shift, bit_width)).

---

## shiftrightunsigned

`function` · `datafusion_spark::function::bitwise::expr_fn::shiftrightunsigned`

Also reachable as `datafusion_spark::expr_fn::shiftrightunsigned`

```rust
fn shiftrightunsigned(value: datafusion_expr::Expr, shift: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.bitwise.expr_fn.shiftrightunsigned.md).


Shifts the bits of the first argument right by the number of positions specified by the second argument (logical/unsigned shift). If the shift amount is negative or greater than or equal to the bit width, it is normalized to the bit width (i.e., pmod(shift, bit_width)).

---
