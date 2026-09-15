# `datafusion_spark::function::math::expr_fn`

Crate `datafusion-spark` · 19 public items · structured records in [`model/datafusion_spark.function.math.expr_fn.json`](../model/datafusion_spark.function.math.expr_fn.json)

## abs

`function` · `datafusion_spark::function::math::expr_fn::abs`

Also reachable as `datafusion_spark::expr_fn::abs`

```rust
fn abs(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns abs(expr)

---

## atan2

`function` · `datafusion_spark::function::math::expr_fn::atan2`

Also reachable as `datafusion_spark::expr_fn::atan2`

```rust
fn atan2(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the angle in radians between the positive x-axis and the point (exprX, exprY).

---

## bin

`function` · `datafusion_spark::function::math::expr_fn::bin`

Also reachable as `datafusion_spark::expr_fn::bin`

```rust
fn bin(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the string representation of the long value represented in binary.

---

## ceil

`function` · `datafusion_spark::function::math::expr_fn::ceil`

Also reachable as `datafusion_spark::expr_fn::ceil`

```rust
fn ceil(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the ceiling of expr.

---

## csc

`function` · `datafusion_spark::function::math::expr_fn::csc`

Also reachable as `datafusion_spark::expr_fn::csc`

```rust
fn csc(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the cosecant of expr.

---

## expm1

`function` · `datafusion_spark::function::math::expr_fn::expm1`

Also reachable as `datafusion_spark::expr_fn::expm1`

```rust
fn expm1(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns exp(expr) - 1 as a Float64.

---

## factorial

`function` · `datafusion_spark::function::math::expr_fn::factorial`

Also reachable as `datafusion_spark::expr_fn::factorial`

```rust
fn factorial(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the factorial of expr. expr is [0..20]. Otherwise, null.

---

## floor

`function` · `datafusion_spark::function::math::expr_fn::floor`

Also reachable as `datafusion_spark::expr_fn::floor`

```rust
fn floor(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns floor of expr.

---

## hex

`function` · `datafusion_spark::function::math::expr_fn::hex`

Also reachable as `datafusion_spark::expr_fn::hex`

```rust
fn hex(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Computes hex value of the given column.

---

## hypot

`function` · `datafusion_spark::function::math::expr_fn::hypot`

Also reachable as `datafusion_spark::expr_fn::hypot`

```rust
fn hypot(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns sqrt(a^2 + b^2) without intermediate overflow or underflow.

---

## modulus

`function` · `datafusion_spark::function::math::expr_fn::modulus`

Also reachable as `datafusion_spark::expr_fn::modulus`

```rust
fn modulus(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the remainder of division of the first argument by the second argument.

---

## negative

`function` · `datafusion_spark::function::math::expr_fn::negative`

Also reachable as `datafusion_spark::expr_fn::negative`

```rust
fn negative(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the negation of expr (unary minus).

---

## pmod

`function` · `datafusion_spark::function::math::expr_fn::pmod`

Also reachable as `datafusion_spark::expr_fn::pmod`

```rust
fn pmod(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the positive remainder of division of the first argument by the second argument.

---

## pow

`function` · `datafusion_spark::function::math::expr_fn::pow`

Also reachable as `datafusion_spark::expr_fn::pow`

```rust
fn pow(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns base raised to the power of exponent. Returns Infinity for pow(0, negative).

---

## rint

`function` · `datafusion_spark::function::math::expr_fn::rint`

Also reachable as `datafusion_spark::expr_fn::rint`

```rust
fn rint(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the double value that is closest in value to the argument and is equal to a mathematical integer.

---

## round

`function` · `datafusion_spark::function::math::expr_fn::round`

Also reachable as `datafusion_spark::expr_fn::round`

```rust
fn round(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Rounds the value of expr to scale decimal places using HALF_UP rounding mode.

---

## sec

`function` · `datafusion_spark::function::math::expr_fn::sec`

Also reachable as `datafusion_spark::expr_fn::sec`

```rust
fn sec(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the secant of expr.

---

## unhex

`function` · `datafusion_spark::function::math::expr_fn::unhex`

Also reachable as `datafusion_spark::expr_fn::unhex`

```rust
fn unhex(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Converts hexadecimal string to binary.

---

## width_bucket

`function` · `datafusion_spark::function::math::expr_fn::width_bucket`

Also reachable as `datafusion_spark::expr_fn::width_bucket`

```rust
fn width_bucket(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr, arg3: datafusion_expr::Expr, arg4: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the bucket number into which the value of this expression would fall after being evaluated.

---
