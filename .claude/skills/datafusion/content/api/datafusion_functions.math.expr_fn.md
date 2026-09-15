# `datafusion_functions::math::expr_fn`

Crate `datafusion-functions` · 38 public items · structured records in [`model/datafusion_functions.math.expr_fn.json`](../model/datafusion_functions.math.expr_fn.json)

## abs

`function` · `datafusion_functions::math::expr_fn::abs`

Also reachable as `datafusion::prelude::abs`, `datafusion_functions::expr_fn::abs`

```rust
fn abs(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns the absolute value of a given number

---

## acos

`function` · `datafusion_functions::math::expr_fn::acos`

Also reachable as `datafusion::prelude::acos`, `datafusion_functions::expr_fn::acos`

```rust
fn acos(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns the arc cosine or inverse cosine of a number

---

## acosh

`function` · `datafusion_functions::math::expr_fn::acosh`

Also reachable as `datafusion::prelude::acosh`, `datafusion_functions::expr_fn::acosh`

```rust
fn acosh(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns inverse hyperbolic cosine

---

## asin

`function` · `datafusion_functions::math::expr_fn::asin`

Also reachable as `datafusion::prelude::asin`, `datafusion_functions::expr_fn::asin`

```rust
fn asin(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns the arc sine or inverse sine of a number

---

## asinh

`function` · `datafusion_functions::math::expr_fn::asinh`

Also reachable as `datafusion::prelude::asinh`, `datafusion_functions::expr_fn::asinh`

```rust
fn asinh(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns inverse hyperbolic sine

---

## atan

`function` · `datafusion_functions::math::expr_fn::atan`

Also reachable as `datafusion::prelude::atan`, `datafusion_functions::expr_fn::atan`

```rust
fn atan(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns inverse tangent

---

## atan2

`function` · `datafusion_functions::math::expr_fn::atan2`

Also reachable as `datafusion::prelude::atan2`, `datafusion_functions::expr_fn::atan2`

```rust
fn atan2(y: datafusion_expr::Expr, x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns inverse tangent of a division given in the argument

---

## atanh

`function` · `datafusion_functions::math::expr_fn::atanh`

Also reachable as `datafusion::prelude::atanh`, `datafusion_functions::expr_fn::atanh`

```rust
fn atanh(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns inverse hyperbolic tangent

---

## cbrt

`function` · `datafusion_functions::math::expr_fn::cbrt`

Also reachable as `datafusion::prelude::cbrt`, `datafusion_functions::expr_fn::cbrt`

```rust
fn cbrt(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

cube root of a number

---

## ceil

`function` · `datafusion_functions::math::expr_fn::ceil`

Also reachable as `datafusion::prelude::ceil`, `datafusion_functions::expr_fn::ceil`

```rust
fn ceil(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

nearest integer greater than or equal to argument

---

## cos

`function` · `datafusion_functions::math::expr_fn::cos`

Also reachable as `datafusion::prelude::cos`, `datafusion_functions::expr_fn::cos`

```rust
fn cos(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

cosine

---

## cosh

`function` · `datafusion_functions::math::expr_fn::cosh`

Also reachable as `datafusion::prelude::cosh`, `datafusion_functions::expr_fn::cosh`

```rust
fn cosh(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

hyperbolic cosine

---

## cot

`function` · `datafusion_functions::math::expr_fn::cot`

Also reachable as `datafusion::prelude::cot`, `datafusion_functions::expr_fn::cot`

```rust
fn cot(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

cotangent of a number

---

## degrees

`function` · `datafusion_functions::math::expr_fn::degrees`

Also reachable as `datafusion::prelude::degrees`, `datafusion_functions::expr_fn::degrees`

```rust
fn degrees(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

converts radians to degrees

---

## exp

`function` · `datafusion_functions::math::expr_fn::exp`

Also reachable as `datafusion::prelude::exp`, `datafusion_functions::expr_fn::exp`

```rust
fn exp(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

exponential

---

## factorial

`function` · `datafusion_functions::math::expr_fn::factorial`

Also reachable as `datafusion::prelude::factorial`, `datafusion_functions::expr_fn::factorial`

```rust
fn factorial(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

factorial

---

## floor

`function` · `datafusion_functions::math::expr_fn::floor`

Also reachable as `datafusion::prelude::floor`, `datafusion_functions::expr_fn::floor`

```rust
fn floor(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

nearest integer less than or equal to argument

---

## gcd

`function` · `datafusion_functions::math::expr_fn::gcd`

Also reachable as `datafusion::prelude::gcd`, `datafusion_functions::expr_fn::gcd`

```rust
fn gcd(x: datafusion_expr::Expr, y: datafusion_expr::Expr) -> datafusion_expr::Expr
```

greatest common divisor

---

## isnan

`function` · `datafusion_functions::math::expr_fn::isnan`

Also reachable as `datafusion::prelude::isnan`, `datafusion_functions::expr_fn::isnan`

```rust
fn isnan(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns true if a given number is +NaN or -NaN otherwise returns false

---

## iszero

`function` · `datafusion_functions::math::expr_fn::iszero`

Also reachable as `datafusion::prelude::iszero`, `datafusion_functions::expr_fn::iszero`

```rust
fn iszero(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns true if a given number is +0.0 or -0.0 otherwise returns false

---

## lcm

`function` · `datafusion_functions::math::expr_fn::lcm`

Also reachable as `datafusion::prelude::lcm`, `datafusion_functions::expr_fn::lcm`

```rust
fn lcm(x: datafusion_expr::Expr, y: datafusion_expr::Expr) -> datafusion_expr::Expr
```

least common multiple

---

## ln

`function` · `datafusion_functions::math::expr_fn::ln`

Also reachable as `datafusion::prelude::ln`, `datafusion_functions::expr_fn::ln`

```rust
fn ln(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

natural logarithm (base e) of a number

---

## log

`function` · `datafusion_functions::math::expr_fn::log`

Also reachable as `datafusion::prelude::log`, `datafusion_functions::expr_fn::log`

```rust
fn log(base: datafusion_expr::Expr, num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

logarithm of a number for a particular `base`

---

## log10

`function` · `datafusion_functions::math::expr_fn::log10`

Also reachable as `datafusion::prelude::log10`, `datafusion_functions::expr_fn::log10`

```rust
fn log10(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

base 10 logarithm of a number

---

## log2

`function` · `datafusion_functions::math::expr_fn::log2`

Also reachable as `datafusion::prelude::log2`, `datafusion_functions::expr_fn::log2`

```rust
fn log2(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

base 2 logarithm of a number

---

## nanvl

`function` · `datafusion_functions::math::expr_fn::nanvl`

Also reachable as `datafusion::prelude::nanvl`, `datafusion_functions::expr_fn::nanvl`

```rust
fn nanvl(x: datafusion_expr::Expr, y: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns x if x is not NaN otherwise returns y

---

## pi

`function` · `datafusion_functions::math::expr_fn::pi`

Also reachable as `datafusion::prelude::pi`, `datafusion_functions::expr_fn::pi`

```rust
fn pi() -> datafusion_expr::Expr
```

Returns an approximate value of π

---

## power

`function` · `datafusion_functions::math::expr_fn::power`

Also reachable as `datafusion::prelude::power`, `datafusion_functions::expr_fn::power`

```rust
fn power(base: datafusion_expr::Expr, exponent: datafusion_expr::Expr) -> datafusion_expr::Expr
```

`base` raised to the power of `exponent`

---

## radians

`function` · `datafusion_functions::math::expr_fn::radians`

Also reachable as `datafusion::prelude::radians`, `datafusion_functions::expr_fn::radians`

```rust
fn radians(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

converts degrees to radians

---

## random

`function` · `datafusion_functions::math::expr_fn::random`

Also reachable as `datafusion::prelude::random`, `datafusion_functions::expr_fn::random`

```rust
fn random() -> datafusion_expr::Expr
```

Returns a random value in the range 0.0 <= x < 1.0

---

## round

`function` · `datafusion_functions::math::expr_fn::round`

Also reachable as `datafusion::prelude::round`, `datafusion_functions::expr_fn::round`

```rust
fn round(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

round to nearest integer

---

## signum

`function` · `datafusion_functions::math::expr_fn::signum`

Also reachable as `datafusion::prelude::signum`, `datafusion_functions::expr_fn::signum`

```rust
fn signum(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

sign of the argument (-1, 0, +1)

---

## sin

`function` · `datafusion_functions::math::expr_fn::sin`

Also reachable as `datafusion::prelude::sin`, `datafusion_functions::expr_fn::sin`

```rust
fn sin(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

sine

---

## sinh

`function` · `datafusion_functions::math::expr_fn::sinh`

Also reachable as `datafusion::prelude::sinh`, `datafusion_functions::expr_fn::sinh`

```rust
fn sinh(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

hyperbolic sine

---

## sqrt

`function` · `datafusion_functions::math::expr_fn::sqrt`

Also reachable as `datafusion::prelude::sqrt`, `datafusion_functions::expr_fn::sqrt`

```rust
fn sqrt(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

square root of a number

---

## tan

`function` · `datafusion_functions::math::expr_fn::tan`

Also reachable as `datafusion::prelude::tan`, `datafusion_functions::expr_fn::tan`

```rust
fn tan(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns the tangent of a number

---

## tanh

`function` · `datafusion_functions::math::expr_fn::tanh`

Also reachable as `datafusion::prelude::tanh`, `datafusion_functions::expr_fn::tanh`

```rust
fn tanh(num: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns the hyperbolic tangent of a number

---

## trunc

`function` · `datafusion_functions::math::expr_fn::trunc`

Also reachable as `datafusion::prelude::trunc`, `datafusion_functions::expr_fn::trunc`

```rust
fn trunc(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

truncate toward zero, with optional precision

---
