# `arrow_buffer::bigint`

Crate `arrow-buffer` · 2 public items · structured records in [`model/arrow_buffer.bigint.json`](../model/arrow_buffer.bigint.json)

## ParseI256Error

`struct` · `arrow_buffer::bigint::ParseI256Error`

```rust
struct ParseI256Error
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.bigint.ParseI256Error.md).


An opaque error similar to [`std::num::ParseIntError`]

---

## i256

`struct` · `arrow_buffer::bigint::i256`

Also reachable as `arrow::datatypes::i256`, `arrow_buffer::i256`

```rust
struct i256
```

**Implements**: `arrow_array::arithmetic::ArrowNativeTypeOp`, `arrow_buffer::native::ArrowNativeType`, `arrow_cast::cast::decimal::DecimalCast`, `core::convert::From`, `core::convert::TryFrom`, `core::fmt::Display`, `core::ops::arith::Add`, `core::ops::arith::AddAssign`, `core::ops::arith::Div`, `core::ops::arith::DivAssign`, `core::ops::arith::Mul`, `core::ops::arith::MulAssign`, `core::ops::arith::Neg`, `core::ops::arith::Rem`, `core::ops::arith::RemAssign`, `core::ops::arith::Sub`, `core::ops::arith::SubAssign`, `core::ops::bit::BitAnd`, `core::ops::bit::BitOr`, `core::ops::bit::BitXor`, `core::ops::bit::Not`, `core::ops::bit::Shl`, `core::ops::bit::Shr`, `core::str::traits::FromStr`, `datafusion_common::hash_utils::HashValue`, `datafusion_common::heap_size::DFHeapSize`, `num_traits::Num`, `num_traits::bounds::Bounded`, `num_traits::cast::ToPrimitive`, `num_traits::identities::ConstOne`, `num_traits::identities::ConstZero`, `num_traits::identities::One`, `num_traits::identities::Zero`, `num_traits::ops::checked::CheckedAdd`, `num_traits::ops::checked::CheckedDiv`, `num_traits::ops::checked::CheckedMul`, `num_traits::ops::checked::CheckedNeg`, `num_traits::ops::checked::CheckedRem`, `num_traits::ops::checked::CheckedShl`, `num_traits::ops::checked::CheckedShr`, `num_traits::ops::checked::CheckedSub`, `num_traits::ops::mul_add::MulAdd`, `num_traits::ops::mul_add::MulAddAssign`, `num_traits::ops::saturating::SaturatingAdd`, `num_traits::ops::saturating::SaturatingMul`, `num_traits::ops::saturating::SaturatingSub`, `num_traits::ops::wrapping::WrappingAdd`, `num_traits::ops::wrapping::WrappingMul`, `num_traits::ops::wrapping::WrappingNeg`, `num_traits::ops::wrapping::WrappingShl`, `num_traits::ops::wrapping::WrappingShr`, `num_traits::ops::wrapping::WrappingSub`, `num_traits::sign::Signed`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (40)

```rust
const fn as_i128(self) -> i128
const fn checked_abs(self) -> Option<Self>
const fn checked_add(self, other: Self) -> Option<Self>
fn checked_div(self, other: Self) -> Option<Self>
fn checked_ilog(self, base: i256) -> Option<u32>
fn checked_ilog10(self) -> Option<u32>
fn checked_ilog2(self) -> Option<u32>
const fn checked_mul(self, other: Self) -> Option<Self>
const fn checked_neg(self) -> Option<Self>
const fn checked_pow(self, exp: u32) -> Option<Self>
fn checked_rem(self, other: Self) -> Option<Self>
const fn checked_sub(self, other: Self) -> Option<Self>
const fn from_be_bytes(b: [u8; 32]) -> Self
fn from_f64(v: f64) -> Option<Self>
const fn from_i128(v: i128) -> Self
const fn from_le_bytes(b: [u8; 32]) -> Self
const fn from_parts(low: u128, high: i128) -> Self
fn from_string(value_str: &str) -> Option<Self>
fn ilog(self, base: i256) -> u32
fn ilog10(self) -> u32
fn ilog2(self) -> u32
const fn is_negative(self) -> bool
const fn is_positive(self) -> bool
const fn leading_zeros(&self) -> u32
const fn overflowing_add(self, rhs: Self) -> (Self, bool)
const fn overflowing_sub(self, rhs: Self) -> (Self, bool)
const fn signum(self) -> Self
const fn to_be_bytes(self) -> [u8; 32]
const fn to_i128(self) -> Option<i128>
const fn to_le_bytes(self) -> [u8; 32]
const fn to_parts(self) -> (u128, i128)
const fn trailing_zeros(&self) -> u32
const fn wrapping_abs(self) -> Self
const fn wrapping_add(self, other: Self) -> Self
fn wrapping_div(self, other: Self) -> Self
const fn wrapping_mul(self, other: Self) -> Self
const fn wrapping_neg(self) -> Self
const fn wrapping_pow(self, exp: u32) -> Self
fn wrapping_rem(self, other: Self) -> Self
const fn wrapping_sub(self, other: Self) -> Self
```

**via `arrow_buffer::native::ArrowNativeType`**

```rust
fn as_usize(self) -> usize
fn from_usize(u: usize) -> Option<Self>
fn to_i64(self) -> Option<i64>
fn to_isize(self) -> Option<isize>
fn to_usize(self) -> Option<usize>
fn usize_as(i: usize) -> Self
```

**via `core::convert::From`**

```rust
fn from(value: i64) -> Self
fn from(value: i16) -> Self
fn from(value: i128) -> Self
fn from(value: i32) -> Self
fn from(value: i8) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `core::ops::arith::Add`**

```rust
fn add(self, rhs: &'a i256) -> Self::Output
fn add(self, rhs: Self) -> Self::Output
```

**via `core::ops::arith::AddAssign`**

```rust
fn add_assign(&mut self, rhs: Self)
```

**via `core::ops::arith::Div`**

```rust
fn div(self, rhs: &'a i256) -> Self::Output
fn div(self, rhs: Self) -> Self::Output
```

**via `core::ops::arith::DivAssign`**

```rust
fn div_assign(&mut self, rhs: Self)
```

**via `core::ops::arith::Mul`**

```rust
fn mul(self, rhs: &'a i256) -> Self::Output
fn mul(self, rhs: Self) -> Self::Output
```

**via `core::ops::arith::MulAssign`**

```rust
fn mul_assign(&mut self, rhs: Self)
```

**via `core::ops::arith::Neg`**

```rust
fn neg(self) -> Self::Output
```

**via `core::ops::arith::Rem`**

```rust
fn rem(self, rhs: &'a i256) -> Self::Output
fn rem(self, rhs: Self) -> Self::Output
```

**via `core::ops::arith::RemAssign`**

```rust
fn rem_assign(&mut self, rhs: Self)
```

**via `core::ops::arith::Sub`**

```rust
fn sub(self, rhs: &'a i256) -> Self::Output
fn sub(self, rhs: Self) -> Self::Output
```

**via `core::ops::arith::SubAssign`**

```rust
fn sub_assign(&mut self, rhs: Self)
```

**via `core::ops::bit::BitAnd`**

```rust
fn bitand(self, rhs: Self) -> Self::Output
```

**via `core::ops::bit::BitOr`**

```rust
fn bitor(self, rhs: Self) -> Self::Output
```

**via `core::ops::bit::BitXor`**

```rust
fn bitxor(self, rhs: Self) -> Self::Output
```

**via `core::ops::bit::Not`**

```rust
fn not(self) -> Self::Output
```

**via `core::ops::bit::Shl`**

```rust
fn shl(self, rhs: usize) -> Self::Output
fn shl(self, rhs: isize) -> Self::Output
fn shl(self, rhs: u16) -> Self::Output
fn shl(self, rhs: i16) -> Self::Output
fn shl(self, rhs: u32) -> Self::Output
fn shl(self, rhs: i32) -> Self::Output
fn shl(self, rhs: u8) -> Self::Output
fn shl(self, rhs: u64) -> Self::Output
fn shl(self, rhs: i64) -> Self::Output
fn shl(self, rhs: u128) -> Self::Output
fn shl(self, rhs: i128) -> Self::Output
```

**via `core::ops::bit::Shr`**

```rust
fn shr(self, rhs: u128) -> Self::Output
fn shr(self, rhs: i128) -> Self::Output
fn shr(self, rhs: usize) -> Self::Output
fn shr(self, rhs: isize) -> Self::Output
fn shr(self, rhs: u16) -> Self::Output
fn shr(self, rhs: i16) -> Self::Output
fn shr(self, rhs: u8) -> Self::Output
fn shr(self, rhs: u32) -> Self::Output
fn shr(self, rhs: i32) -> Self::Output
fn shr(self, rhs: u64) -> Self::Output
fn shr(self, rhs: i64) -> Self::Output
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `num_traits::Num`**

```rust
fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr>
```

**via `num_traits::bounds::Bounded`**

```rust
fn max_value() -> Self
fn min_value() -> Self
```

**via `num_traits::cast::ToPrimitive`**

```rust
fn to_f64(&self) -> Option<f64>
fn to_i64(&self) -> Option<i64>
fn to_u64(&self) -> Option<u64>
```

**via `num_traits::identities::One`**

```rust
fn is_one(&self) -> bool
fn one() -> Self
```

**via `num_traits::identities::Zero`**

```rust
fn is_zero(&self) -> bool
fn zero() -> Self
```

**via `num_traits::ops::checked::CheckedAdd`**

```rust
fn checked_add(&self, v: &i256) -> Option<Self>
```

**via `num_traits::ops::checked::CheckedDiv`**

```rust
fn checked_div(&self, v: &i256) -> Option<Self>
```

**via `num_traits::ops::checked::CheckedMul`**

```rust
fn checked_mul(&self, v: &i256) -> Option<Self>
```

**via `num_traits::ops::checked::CheckedNeg`**

```rust
fn checked_neg(&self) -> Option<Self>
```

**via `num_traits::ops::checked::CheckedRem`**

```rust
fn checked_rem(&self, v: &i256) -> Option<Self>
```

**via `num_traits::ops::checked::CheckedShl`**

```rust
fn checked_shl(&self, rhs: u32) -> Option<Self>
```

**via `num_traits::ops::checked::CheckedShr`**

```rust
fn checked_shr(&self, rhs: u32) -> Option<Self>
```

**via `num_traits::ops::checked::CheckedSub`**

```rust
fn checked_sub(&self, v: &i256) -> Option<Self>
```

**via `num_traits::ops::mul_add::MulAdd`**

```rust
fn mul_add(self, a: Self, b: Self) -> Self::Output
```

**via `num_traits::ops::mul_add::MulAddAssign`**

```rust
fn mul_add_assign(&mut self, a: Self, b: Self)
```

**via `num_traits::ops::saturating::SaturatingAdd`**

```rust
fn saturating_add(&self, v: &Self) -> Self
```

**via `num_traits::ops::saturating::SaturatingMul`**

```rust
fn saturating_mul(&self, v: &Self) -> Self
```

**via `num_traits::ops::saturating::SaturatingSub`**

```rust
fn saturating_sub(&self, v: &Self) -> Self
```

**via `num_traits::ops::wrapping::WrappingAdd`**

```rust
fn wrapping_add(&self, v: &Self) -> Self
```

**via `num_traits::ops::wrapping::WrappingMul`**

```rust
fn wrapping_mul(&self, v: &Self) -> Self
```

**via `num_traits::ops::wrapping::WrappingNeg`**

```rust
fn wrapping_neg(&self) -> Self
```

**via `num_traits::ops::wrapping::WrappingShl`**

```rust
fn wrapping_shl(&self, rhs: u32) -> i256
```

**via `num_traits::ops::wrapping::WrappingShr`**

```rust
fn wrapping_shr(&self, rhs: u32) -> i256
```

**via `num_traits::ops::wrapping::WrappingSub`**

```rust
fn wrapping_sub(&self, v: &Self) -> Self
```

**via `num_traits::sign::Signed`**

```rust
fn abs(&self) -> Self
fn abs_sub(&self, other: &Self) -> Self
fn is_negative(&self) -> bool
fn is_positive(&self) -> bool
fn signum(&self) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.bigint.i256.md).


A signed 256-bit integer

---
