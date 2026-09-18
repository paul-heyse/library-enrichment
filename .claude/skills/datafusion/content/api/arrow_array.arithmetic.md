# `arrow_array::arithmetic`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.arithmetic.json`](../model/arrow_array.arithmetic.json)

## ArrowNativeTypeOp

`trait` · `arrow_array::arithmetic::ArrowNativeTypeOp`

Also reachable as `arrow::array::ArrowNativeTypeOp`, `arrow::datatypes::ArrowNativeTypeOp`, `arrow_array::ArrowNativeTypeOp`

```rust
trait ArrowNativeTypeOp: ArrowNativeType
```

**Implementors** (4)

- `arrow_buffer::bigint::i256`
- `arrow_buffer::interval::IntervalDayTime`
- `arrow_buffer::interval::IntervalMonthDayNano`
- `half::binary16::f16`

**Methods** (22)

```rust
fn add_checked(self, rhs: Self) -> Result<Self, ArrowError>
fn add_wrapping(self, rhs: Self) -> Self
fn compare(self, rhs: Self) -> Ordering
fn div_checked(self, rhs: Self) -> Result<Self, ArrowError>
fn div_wrapping(self, rhs: Self) -> Self
fn is_eq(self, rhs: Self) -> bool
fn is_ge(self, rhs: Self) -> bool
fn is_gt(self, rhs: Self) -> bool
fn is_le(self, rhs: Self) -> bool
fn is_lt(self, rhs: Self) -> bool
fn is_ne(self, rhs: Self) -> bool
fn is_zero(self) -> bool
fn mod_checked(self, rhs: Self) -> Result<Self, ArrowError>
fn mod_wrapping(self, rhs: Self) -> Self
fn mul_checked(self, rhs: Self) -> Result<Self, ArrowError>
fn mul_wrapping(self, rhs: Self) -> Self
fn neg_checked(self) -> Result<Self, ArrowError>
fn neg_wrapping(self) -> Self
fn pow_checked(self, exp: u32) -> Result<Self, ArrowError>
fn pow_wrapping(self, exp: u32) -> Self
fn sub_checked(self, rhs: Self) -> Result<Self, ArrowError>
fn sub_wrapping(self, rhs: Self) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_array.arithmetic.ArrowNativeTypeOp.md).


Trait for [`ArrowNativeType`] that adds checked and unchecked arithmetic operations,
and totally ordered comparison operations

The APIs with `_wrapping` suffix do not perform overflow-checking. For integer
types they will wrap around the boundary of the type. For floating point types they
will overflow to INF or -INF preserving the expected sign value

Note `div_wrapping` and `mod_wrapping` will panic for integer types if `rhs` is zero
although this may be subject to change <https://github.com/apache/arrow-rs/issues/2647>

The APIs with `_checked` suffix perform overflow-checking. For integer types
these will return `Err` instead of wrapping. For floating point types they will
overflow to INF or -INF preserving the expected sign value

Comparison of integer types is as per normal integer comparison rules, floating
point values are compared as per IEEE 754's totalOrder predicate see [`f32::total_cmp`]

---
