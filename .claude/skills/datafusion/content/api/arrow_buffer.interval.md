# `arrow_buffer::interval`

Crate `arrow-buffer` · 2 public items · structured records in [`model/arrow_buffer.interval.json`](../model/arrow_buffer.interval.json)

## IntervalDayTime

`struct` · `arrow_buffer::interval::IntervalDayTime`

Also reachable as `arrow::datatypes::IntervalDayTime`, `arrow_array::types::IntervalDayTime`

```rust
struct IntervalDayTime
```

**Fields**: `days`, `milliseconds`

**Implements**: `arrow_array::arithmetic::ArrowNativeTypeOp`, `arrow_buffer::native::ArrowNativeType`, `core::ops::arith::Add`, `core::ops::arith::AddAssign`, `core::ops::arith::Div`, `core::ops::arith::DivAssign`, `core::ops::arith::Mul`, `core::ops::arith::MulAssign`, `core::ops::arith::Neg`, `core::ops::arith::Rem`, `core::ops::arith::RemAssign`, `core::ops::arith::Sub`, `core::ops::arith::SubAssign`, `datafusion_common::hash_utils::HashValue`, `datafusion_common::heap_size::DFHeapSize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (17)

```rust
fn checked_abs(self) -> Option<Self>
fn checked_add(self, other: Self) -> Option<Self>
fn checked_div(self, other: Self) -> Option<Self>
fn checked_mul(self, other: Self) -> Option<Self>
fn checked_neg(self) -> Option<Self>
fn checked_pow(self, exp: u32) -> Option<Self>
fn checked_rem(self, other: Self) -> Option<Self>
fn checked_sub(self, other: Self) -> Option<Self>
const fn new(days: i32, milliseconds: i32) -> Self
fn wrapping_abs(self) -> Self
fn wrapping_add(self, other: Self) -> Self
fn wrapping_div(self, other: Self) -> Self
fn wrapping_mul(self, other: Self) -> Self
fn wrapping_neg(self) -> Self
fn wrapping_pow(self, exp: u32) -> Self
fn wrapping_rem(self, other: Self) -> Self
fn wrapping_sub(self, other: Self) -> Self
```

**via `arrow_buffer::native::ArrowNativeType`**

```rust
fn as_usize(self) -> usize
fn from_usize(_: usize) -> Option<Self>
fn to_i64(self) -> Option<i64>
fn to_isize(self) -> Option<isize>
fn to_usize(self) -> Option<usize>
fn usize_as(i: usize) -> Self
```

**via `core::ops::arith::Add`**

```rust
fn add(self, rhs: &'a IntervalDayTime) -> Self::Output
fn add(self, rhs: Self) -> Self::Output
```

**via `core::ops::arith::AddAssign`**

```rust
fn add_assign(&mut self, rhs: Self)
```

**via `core::ops::arith::Div`**

```rust
fn div(self, rhs: &'a IntervalDayTime) -> Self::Output
fn div(self, rhs: Self) -> Self::Output
```

**via `core::ops::arith::DivAssign`**

```rust
fn div_assign(&mut self, rhs: Self)
```

**via `core::ops::arith::Mul`**

```rust
fn mul(self, rhs: &'a IntervalDayTime) -> Self::Output
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
fn rem(self, rhs: &'a IntervalDayTime) -> Self::Output
fn rem(self, rhs: Self) -> Self::Output
```

**via `core::ops::arith::RemAssign`**

```rust
fn rem_assign(&mut self, rhs: Self)
```

**via `core::ops::arith::Sub`**

```rust
fn sub(self, rhs: Self) -> Self::Output
fn sub(self, rhs: &'a IntervalDayTime) -> Self::Output
```

**via `core::ops::arith::SubAssign`**

```rust
fn sub_assign(&mut self, rhs: Self)
```

Value of an IntervalDayTime array

## Representation

This type is stored as a single 64 bit integer, interpreted as two i32
fields:

1. the number of elapsed days
2. The number of milliseconds (no leap seconds),

```text
┌──────────────┬──────────────┐
│     Days     │ Milliseconds │
│  (32 bits)   │  (32 bits)   │
└──────────────┴──────────────┘
0              31            63 bit offset
```

Please see the [Arrow Spec](https://github.com/apache/arrow/blob/081b4022fe6f659d8765efc82b3f4787c5039e3c/format/Schema.fbs#L406-L408) for more details

## Note on Comparing and Ordering for Calendar Types

Values of `IntervalDayTime` are compared using their binary representation,
which can lead to surprising results. Please see the description of ordering on
[`IntervalMonthDayNano`] for more details

---

## IntervalMonthDayNano

`struct` · `arrow_buffer::interval::IntervalMonthDayNano`

Also reachable as `arrow::datatypes::IntervalMonthDayNano`, `arrow_array::types::IntervalMonthDayNano`

```rust
struct IntervalMonthDayNano
```

**Fields**: `months`, `days`, `nanoseconds`

**Implements**: `arrow_array::arithmetic::ArrowNativeTypeOp`, `arrow_buffer::native::ArrowNativeType`, `core::ops::arith::Add`, `core::ops::arith::AddAssign`, `core::ops::arith::Div`, `core::ops::arith::DivAssign`, `core::ops::arith::Mul`, `core::ops::arith::MulAssign`, `core::ops::arith::Neg`, `core::ops::arith::Rem`, `core::ops::arith::RemAssign`, `core::ops::arith::Sub`, `core::ops::arith::SubAssign`, `datafusion_common::hash_utils::HashValue`, `datafusion_common::heap_size::DFHeapSize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (17)

```rust
fn checked_abs(self) -> Option<Self>
fn checked_add(self, other: Self) -> Option<Self>
fn checked_div(self, other: Self) -> Option<Self>
fn checked_mul(self, other: Self) -> Option<Self>
fn checked_neg(self) -> Option<Self>
fn checked_pow(self, exp: u32) -> Option<Self>
fn checked_rem(self, other: Self) -> Option<Self>
fn checked_sub(self, other: Self) -> Option<Self>
const fn new(months: i32, days: i32, nanoseconds: i64) -> Self
fn wrapping_abs(self) -> Self
fn wrapping_add(self, other: Self) -> Self
fn wrapping_div(self, other: Self) -> Self
fn wrapping_mul(self, other: Self) -> Self
fn wrapping_neg(self) -> Self
fn wrapping_pow(self, exp: u32) -> Self
fn wrapping_rem(self, other: Self) -> Self
fn wrapping_sub(self, other: Self) -> Self
```

**via `arrow_buffer::native::ArrowNativeType`**

```rust
fn as_usize(self) -> usize
fn from_usize(_: usize) -> Option<Self>
fn to_i64(self) -> Option<i64>
fn to_isize(self) -> Option<isize>
fn to_usize(self) -> Option<usize>
fn usize_as(i: usize) -> Self
```

**via `core::ops::arith::Add`**

```rust
fn add(self, rhs: &'a IntervalMonthDayNano) -> Self::Output
fn add(self, rhs: Self) -> Self::Output
```

**via `core::ops::arith::AddAssign`**

```rust
fn add_assign(&mut self, rhs: Self)
```

**via `core::ops::arith::Div`**

```rust
fn div(self, rhs: Self) -> Self::Output
fn div(self, rhs: &'a IntervalMonthDayNano) -> Self::Output
```

**via `core::ops::arith::DivAssign`**

```rust
fn div_assign(&mut self, rhs: Self)
```

**via `core::ops::arith::Mul`**

```rust
fn mul(self, rhs: &'a IntervalMonthDayNano) -> Self::Output
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
fn rem(self, rhs: &'a IntervalMonthDayNano) -> Self::Output
fn rem(self, rhs: Self) -> Self::Output
```

**via `core::ops::arith::RemAssign`**

```rust
fn rem_assign(&mut self, rhs: Self)
```

**via `core::ops::arith::Sub`**

```rust
fn sub(self, rhs: &'a IntervalMonthDayNano) -> Self::Output
fn sub(self, rhs: Self) -> Self::Output
```

**via `core::ops::arith::SubAssign`**

```rust
fn sub_assign(&mut self, rhs: Self)
```

 Value of an IntervalMonthDayNano array

  ## Representation

 This type is stored as a single 128 bit integer, interpreted as three
 different signed integral fields:

 1. The number of months (32 bits)
 2. The number days (32 bits)
 2. The number of nanoseconds (64 bits).

 Nanoseconds does not allow for leap seconds.

 Each field is independent (e.g. there is no constraint that the quantity of
 nanoseconds represents less than a day's worth of time).

 ```text
 ┌───────────────┬─────────────┬─────────────────────────────┐
 │     Months    │     Days    │            Nanos            │
 │   (32 bits)   │  (32 bits)  │          (64 bits)          │
 └───────────────┴─────────────┴─────────────────────────────┘
 0            32             64                           128 bit offset
 ```
 Please see the [Arrow Spec](https://github.com/apache/arrow/blob/081b4022fe6f659d8765efc82b3f4787c5039e3c/format/Schema.fbs#L409-L415) for more details

## Note on Comparing and Ordering for Calendar Types

 Values of `IntervalMonthDayNano` are compared using their binary
 representation, which can lead to surprising results.

 Spans of time measured in calendar units are not fixed in absolute size (e.g.
 number of seconds) which makes defining comparisons and ordering non trivial.
 For example `1 month` is 28 days for February but `1 month` is 31 days
 in December.

 This makes the seemingly simple operation of comparing two intervals
 complicated in practice. For example is `1 month` more or less than `30
 days`? The answer depends on what month you are talking about.

 This crate defines comparisons for calendar types using their binary
 representation which is fast and efficient, but leads
 to potentially surprising results.

 For example a
 `IntervalMonthDayNano` of `1 month` will compare as **greater** than a
 `IntervalMonthDayNano` of `100 days` because the binary representation of `1 month`
 is larger than the binary representation of 100 days.

---
