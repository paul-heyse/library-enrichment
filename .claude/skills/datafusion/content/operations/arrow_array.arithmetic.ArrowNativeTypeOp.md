# `arrow_array::arithmetic::ArrowNativeTypeOp`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.arithmetic.ArrowNativeTypeOp.json).

<a id="op-b7cc92a560825113ff25ce61"></a>
## ArrowNativeTypeOp

`trait` · `arrow_array::arithmetic::ArrowNativeTypeOp` · arrow-array 59.3.0

```rust
trait ArrowNativeTypeOp: ArrowNativeType
```

Source: `src/arithmetic.rs:41`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Trait for [`ArrowNativeType`](../operations/arrow_buffer.native.ArrowNativeType.md#op-5f78ae59d708cee9753802c9) that adds checked and unchecked arithmetic operations,
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


Unresolved upstream links (retained, not inferred): ``f32::total_cmp``.

<a id="op-f1fe41f9ea32f07e875ab1ae"></a>
## MAX_TOTAL_ORDER

`assoc_const` · `arrow_array::arithmetic::ArrowNativeTypeOp::MAX_TOTAL_ORDER` · arrow-array 59.3.0

```rust
MAX_TOTAL_ORDER
```

Source: `src/arithmetic.rs:56`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

The maximum value and identity for the `min` aggregation.
Note that the aggregation uses the total order predicate for floating point values,
which means that this value is a positive NaN.

<a id="op-f3c1e7061541e04517deff9c"></a>
## MIN_TOTAL_ORDER

`assoc_const` · `arrow_array::arithmetic::ArrowNativeTypeOp::MIN_TOTAL_ORDER` · arrow-array 59.3.0

```rust
MIN_TOTAL_ORDER
```

Source: `src/arithmetic.rs:51`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

The minimum value and identity for the `max` aggregation.
Note that the aggregation uses the total order predicate for floating point values,
which means that this value is a negative NaN.

<a id="op-4520698ce05d4df6ccb3b0c5"></a>
## ONE

`assoc_const` · `arrow_array::arithmetic::ArrowNativeTypeOp::ONE` · arrow-array 59.3.0

```rust
ONE
```

Source: `src/arithmetic.rs:46`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

The multiplicative identity

<a id="op-9d8f9fbc13da3310cdd57d54"></a>
## ZERO

`assoc_const` · `arrow_array::arithmetic::ArrowNativeTypeOp::ZERO` · arrow-array 59.3.0

```rust
ZERO
```

Source: `src/arithmetic.rs:43`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

The additive identity

<a id="op-2c244ebcb6babe864b698142"></a>
## add_checked

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::add_checked` · arrow-array 59.3.0

```rust
fn add_checked(self, rhs: Self) -> Result<Self, ArrowError>
```

Source: `src/arithmetic.rs:59`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Checked addition operation

<a id="op-700c14e579343af378c5ae90"></a>
## add_wrapping

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::add_wrapping` · arrow-array 59.3.0

```rust
fn add_wrapping(self, rhs: Self) -> Self
```

Source: `src/arithmetic.rs:62`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Wrapping addition operation

<a id="op-2e5f8399e368d88d0924288b"></a>
## compare

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::compare` · arrow-array 59.3.0

```rust
fn compare(self, rhs: Self) -> Ordering
```

Source: `src/arithmetic.rs:104`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Compare operation

<a id="op-db8735f0c543ec0590c6d5c8"></a>
## div_checked

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::div_checked` · arrow-array 59.3.0

```rust
fn div_checked(self, rhs: Self) -> Result<Self, ArrowError>
```

Source: `src/arithmetic.rs:77`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Checked division operation

<a id="op-b62a48eac4cf5b48726acc02"></a>
## div_wrapping

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::div_wrapping` · arrow-array 59.3.0

```rust
fn div_wrapping(self, rhs: Self) -> Self
```

Source: `src/arithmetic.rs:80`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Wrapping division operation

<a id="op-ec54768d61282ad4bde2e9e5"></a>
## is_eq

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::is_eq` · arrow-array 59.3.0

```rust
fn is_eq(self, rhs: Self) -> bool
```

Source: `src/arithmetic.rs:107`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Equality operation

<a id="op-172ca9fc1d2fb0d55e367b17"></a>
## is_ge

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::is_ge` · arrow-array 59.3.0

```rust
fn is_ge(self, rhs: Self) -> bool
```

Source: `src/arithmetic.rs:135`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Greater than equals operation

<a id="op-a2f728129b3145eb6b516c8e"></a>
## is_gt

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::is_gt` · arrow-array 59.3.0

```rust
fn is_gt(self, rhs: Self) -> bool
```

Source: `src/arithmetic.rs:129`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Greater than operation

<a id="op-d65f7567d2fe51833ef65c02"></a>
## is_le

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::is_le` · arrow-array 59.3.0

```rust
fn is_le(self, rhs: Self) -> bool
```

Source: `src/arithmetic.rs:123`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Less than equals operation

<a id="op-a8d015744e59cf1a62068240"></a>
## is_lt

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::is_lt` · arrow-array 59.3.0

```rust
fn is_lt(self, rhs: Self) -> bool
```

Source: `src/arithmetic.rs:117`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Less than operation

<a id="op-2f8e12eeab968cf8e6a06a4c"></a>
## is_ne

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::is_ne` · arrow-array 59.3.0

```rust
fn is_ne(self, rhs: Self) -> bool
```

Source: `src/arithmetic.rs:111`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Not equal operation

<a id="op-289fb1d103cd4e0d070e215c"></a>
## is_zero

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::is_zero` · arrow-array 59.3.0

```rust
fn is_zero(self) -> bool
```

Source: `src/arithmetic.rs:101`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns true if zero else false

<a id="op-4c63c81c59bdcb01b21696ff"></a>
## mod_checked

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::mod_checked` · arrow-array 59.3.0

```rust
fn mod_checked(self, rhs: Self) -> Result<Self, ArrowError>
```

Source: `src/arithmetic.rs:83`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Checked remainder operation

<a id="op-8f07bbf6e0d4cb6613106da0"></a>
## mod_wrapping

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::mod_wrapping` · arrow-array 59.3.0

```rust
fn mod_wrapping(self, rhs: Self) -> Self
```

Source: `src/arithmetic.rs:86`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Wrapping remainder operation

<a id="op-c115d528e6730a504e32859c"></a>
## mul_checked

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::mul_checked` · arrow-array 59.3.0

```rust
fn mul_checked(self, rhs: Self) -> Result<Self, ArrowError>
```

Source: `src/arithmetic.rs:71`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Checked multiplication operation

<a id="op-3ce73ed16d07322ecbc5aca1"></a>
## mul_wrapping

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::mul_wrapping` · arrow-array 59.3.0

```rust
fn mul_wrapping(self, rhs: Self) -> Self
```

Source: `src/arithmetic.rs:74`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Wrapping multiplication operation

<a id="op-aaa1138f262bdd4391196106"></a>
## neg_checked

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::neg_checked` · arrow-array 59.3.0

```rust
fn neg_checked(self) -> Result<Self, ArrowError>
```

Source: `src/arithmetic.rs:89`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Checked negation operation

<a id="op-61ad8c1368c04caaf3a78b44"></a>
## neg_wrapping

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::neg_wrapping` · arrow-array 59.3.0

```rust
fn neg_wrapping(self) -> Self
```

Source: `src/arithmetic.rs:92`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Wrapping negation operation

<a id="op-d6fbb718a47684a9bae09b25"></a>
## pow_checked

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::pow_checked` · arrow-array 59.3.0

```rust
fn pow_checked(self, exp: u32) -> Result<Self, ArrowError>
```

Source: `src/arithmetic.rs:95`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Checked exponentiation operation

<a id="op-0195c0089321e8dd14de63df"></a>
## pow_wrapping

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::pow_wrapping` · arrow-array 59.3.0

```rust
fn pow_wrapping(self, exp: u32) -> Self
```

Source: `src/arithmetic.rs:98`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Wrapping exponentiation operation

<a id="op-2fc3615093bf734cab2c6e53"></a>
## sub_checked

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::sub_checked` · arrow-array 59.3.0

```rust
fn sub_checked(self, rhs: Self) -> Result<Self, ArrowError>
```

Source: `src/arithmetic.rs:65`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Checked subtraction operation

<a id="op-fec79ad95237c34101aa52c1"></a>
## sub_wrapping

`function` · `arrow_array::arithmetic::ArrowNativeTypeOp::sub_wrapping` · arrow-array 59.3.0

```rust
fn sub_wrapping(self, rhs: Self) -> Self
```

Source: `src/arithmetic.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Wrapping subtraction operation
