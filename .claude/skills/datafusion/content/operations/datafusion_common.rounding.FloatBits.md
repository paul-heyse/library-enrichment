# `datafusion_common::rounding::FloatBits`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.rounding.FloatBits.json).

<a id="op-80927c75f22f96544eb3fe34"></a>
## FloatBits

`trait` · `datafusion_common::rounding::FloatBits` · datafusion-common 55.1.0

```rust
trait FloatBits
```

Source: `src/rounding.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A trait to manipulate floating-point types with bitwise operations.
Provides functions to convert a floating-point value to/from its bitwise
representation as well as utility methods to handle special values.

<a id="op-65e9122e516f28c3d979d6df"></a>
## CLEAR_SIGN_MASK

`assoc_const` · `datafusion_common::rounding::FloatBits::CLEAR_SIGN_MASK` · datafusion-common 55.1.0

```rust
CLEAR_SIGN_MASK
```

Source: `src/rounding.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A mask to clear the sign bit of the floating-point value's bitwise representation.

<a id="op-c24004c67fab4891a23db52e"></a>
## Item

`assoc_type` · `datafusion_common::rounding::FloatBits::Item` · datafusion-common 55.1.0

```rust
Item
```

Source: `src/rounding.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The integer type used for bitwise operations.

<a id="op-26af2a8eae13f5d15f140108"></a>
## NEG_TINY_BITS

`assoc_const` · `datafusion_common::rounding::FloatBits::NEG_TINY_BITS` · datafusion-common 55.1.0

```rust
NEG_TINY_BITS
```

Source: `src/rounding.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The smallest (in magnitude) negative floating-point value representable by this type.

<a id="op-34ace6143110e0c7ed6fbc5d"></a>
## NEG_ZERO

`assoc_const` · `datafusion_common::rounding::FloatBits::NEG_ZERO` · datafusion-common 55.1.0

```rust
NEG_ZERO
```

Source: `src/rounding.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-593aae5a8be5603012b5dd1e"></a>
## ONE

`assoc_const` · `datafusion_common::rounding::FloatBits::ONE` · datafusion-common 55.1.0

```rust
ONE
```

Source: `src/rounding.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The integer value 1, used in bitwise operations.

<a id="op-7730c462a6444e9701c35610"></a>
## TINY_BITS

`assoc_const` · `datafusion_common::rounding::FloatBits::TINY_BITS` · datafusion-common 55.1.0

```rust
TINY_BITS
```

Source: `src/rounding.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The smallest positive floating-point value representable by this type.

<a id="op-b2878694308a8e3cb122ada6"></a>
## ZERO

`assoc_const` · `datafusion_common::rounding::FloatBits::ZERO` · datafusion-common 55.1.0

```rust
ZERO
```

Source: `src/rounding.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The integer value 0, used in bitwise operations.

<a id="op-479ccadc6b22d4e494cd1920"></a>
## float_is_nan

`function` · `datafusion_common::rounding::FloatBits::float_is_nan` · datafusion-common 55.1.0

```rust
fn float_is_nan(self) -> bool
```

Source: `src/rounding.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns true if the floating-point value is NaN (not a number).

<a id="op-beb25883fb7d1d7a67513392"></a>
## from_bits

`function` · `datafusion_common::rounding::FloatBits::from_bits` · datafusion-common 55.1.0

```rust
fn from_bits(bits: Self::Item) -> Self
```

Source: `src/rounding.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Converts the bitwise representation to the corresponding floating-point value.

<a id="op-622bfd88f2be2620ada5ef5a"></a>
## infinity

`function` · `datafusion_common::rounding::FloatBits::infinity` · datafusion-common 55.1.0

```rust
fn infinity() -> Self
```

Source: `src/rounding.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the positive infinity value for this floating-point type.

<a id="op-172a797eef3869287e48d9cb"></a>
## neg_infinity

`function` · `datafusion_common::rounding::FloatBits::neg_infinity` · datafusion-common 55.1.0

```rust
fn neg_infinity() -> Self
```

Source: `src/rounding.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the negative infinity value for this floating-point type.

<a id="op-52665e58d8eb9ae34615b3f9"></a>
## to_bits

`function` · `datafusion_common::rounding::FloatBits::to_bits` · datafusion-common 55.1.0

```rust
fn to_bits(self) -> Self::Item
```

Source: `src/rounding.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Converts the floating-point value to its bitwise representation.
