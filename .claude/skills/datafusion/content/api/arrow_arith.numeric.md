# `arrow_arith::numeric`

Crate `arrow-arith` · 10 public items · structured records in [`model/arrow_arith.numeric.json`](../model/arrow_arith.numeric.json)

## add

`function` · `arrow_arith::numeric::add`

Also reachable as `arrow::compute::kernels::numeric::add`

```rust
fn add(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<ArrayRef, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.numeric.add.md).


Perform `lhs + rhs`, returning an error on overflow

---

## add_wrapping

`function` · `arrow_arith::numeric::add_wrapping`

Also reachable as `arrow::compute::kernels::numeric::add_wrapping`

```rust
fn add_wrapping(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<ArrayRef, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.numeric.add_wrapping.md).


Perform `lhs + rhs`, wrapping on overflow for [`DataType::is_integer`]

---

## div

`function` · `arrow_arith::numeric::div`

Also reachable as `arrow::compute::kernels::numeric::div`

```rust
fn div(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<ArrayRef, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.numeric.div.md).


Perform `lhs / rhs`

Overflow or division by zero will result in an error, with exception to
floating point numbers, which instead follow the IEEE 754 rules

---

## mul

`function` · `arrow_arith::numeric::mul`

Also reachable as `arrow::compute::kernels::numeric::mul`

```rust
fn mul(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<ArrayRef, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.numeric.mul.md).


Perform `lhs * rhs`, returning an error on overflow

---

## mul_wrapping

`function` · `arrow_arith::numeric::mul_wrapping`

Also reachable as `arrow::compute::kernels::numeric::mul_wrapping`

```rust
fn mul_wrapping(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<ArrayRef, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.numeric.mul_wrapping.md).


Perform `lhs * rhs`, wrapping on overflow for [`DataType::is_integer`]

---

## neg

`function` · `arrow_arith::numeric::neg`

Also reachable as `arrow::compute::kernels::numeric::neg`

```rust
fn neg(array: &dyn Array) -> Result<ArrayRef, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.numeric.neg.md).


Negates each element of  `array`, returning an error on overflow

Note: negation of unsigned arrays is not supported and will return in an error,
for wrapping unsigned negation consider using [`neg_wrapping`][neg_wrapping()]

---

## neg_wrapping

`function` · `arrow_arith::numeric::neg_wrapping`

Also reachable as `arrow::compute::kernels::numeric::neg_wrapping`

```rust
fn neg_wrapping(array: &dyn Array) -> Result<ArrayRef, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.numeric.neg_wrapping.md).


Negates each element of  `array`, wrapping on overflow for [`DataType::is_integer`]

---

## rem

`function` · `arrow_arith::numeric::rem`

Also reachable as `arrow::compute::kernels::numeric::rem`

```rust
fn rem(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<ArrayRef, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.numeric.rem.md).


Perform `lhs % rhs`

Division by zero will result in an error, with exception to
floating point numbers, which instead follow the IEEE 754 rules

`signed_integer::MIN % -1` will not result in an error but return 0

---

## sub

`function` · `arrow_arith::numeric::sub`

Also reachable as `arrow::compute::kernels::numeric::sub`

```rust
fn sub(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<ArrayRef, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.numeric.sub.md).


Perform `lhs - rhs`, returning an error on overflow

---

## sub_wrapping

`function` · `arrow_arith::numeric::sub_wrapping`

Also reachable as `arrow::compute::kernels::numeric::sub_wrapping`

```rust
fn sub_wrapping(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<ArrayRef, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.numeric.sub_wrapping.md).


Perform `lhs - rhs`, wrapping on overflow for [`DataType::is_integer`]

---
