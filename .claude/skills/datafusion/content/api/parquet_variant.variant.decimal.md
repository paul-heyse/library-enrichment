# `parquet_variant::variant::decimal`

Crate `parquet-variant` · 4 public items · structured records in [`model/parquet_variant.variant.decimal.json`](../model/parquet_variant.variant.decimal.json)

## VariantDecimal16

`struct` · `parquet_variant::variant::decimal::VariantDecimal16`

```rust
struct VariantDecimal16
```

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `core::fmt::Display`, `parquet_variant::variant::decimal::VariantDecimalType`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn integer(&self) -> i128
fn scale(&self) -> u8
fn try_new(integer: i128, scale: u8) -> Result<Self, ArrowError>
```

**via `core::convert::From`**

```rust
fn from(decimal: VariantDecimal8) -> Self
fn from(decimal: VariantDecimal4) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(integer: i128) -> Result<Self, ArrowError>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `parquet_variant::variant::decimal::VariantDecimalType`**

```rust
fn integer(&self) -> i128
fn scale(&self) -> u8
fn try_new(integer: i128, scale: u8) -> Result<Self, ArrowError>
fn try_new_with_signed_scale(integer: i128, scale: i8) -> Result<Self, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/parquet_variant.variant.decimal.VariantDecimal16.md).


Represents an 16-byte decimal value in the Variant format.

This struct stores a decimal number using a 128-bit signed integer for the coefficient
and an 8-bit unsigned integer for the scale (number of decimal places). Its precision is between 19 and 38 digits.

For valid precision and scale values, see the Variant specification:

<https://github.com/apache/parquet-format/blob/87f2c8bf77eefb4c43d0ebaeea1778bd28ac3609/VariantEncoding.md?plain=1#L418-L420>

# Example: Create a VariantDecimal16
```
# use parquet_variant::VariantDecimal16;
// Create a value representing the decimal 12345678901234567.890
let decimal = VariantDecimal16::try_new(12345678901234567890, 3).unwrap();
```

---

## VariantDecimal4

`struct` · `parquet_variant::variant::decimal::VariantDecimal4`

```rust
struct VariantDecimal4
```

**Implements**: `core::convert::TryFrom`, `core::fmt::Display`, `parquet_variant::variant::decimal::VariantDecimalType`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn integer(&self) -> i32
fn scale(&self) -> u8
fn try_new(integer: i32, scale: u8) -> Result<Self, ArrowError>
```

**via `core::convert::TryFrom`**

```rust
fn try_from(decimal: VariantDecimal16) -> Result<Self, ArrowError>
fn try_from(integer: i32) -> Result<Self, ArrowError>
fn try_from(decimal: VariantDecimal8) -> Result<Self, ArrowError>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `parquet_variant::variant::decimal::VariantDecimalType`**

```rust
fn integer(&self) -> i32
fn scale(&self) -> u8
fn try_new(integer: i32, scale: u8) -> Result<Self, ArrowError>
fn try_new_with_signed_scale(integer: i32, scale: i8) -> Result<Self, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/parquet_variant.variant.decimal.VariantDecimal4.md).


Represents a 4-byte decimal value in the Variant format.

This struct stores a decimal number using a 32-bit signed integer for the coefficient
and an 8-bit unsigned integer for the scale (number of decimal places). Its precision is limited to 9 digits.

For valid precision and scale values, see the Variant specification:
<https://github.com/apache/parquet-format/blob/87f2c8bf77eefb4c43d0ebaeea1778bd28ac3609/VariantEncoding.md?plain=1#L418-L420>

# Example: Create a VariantDecimal4
```
# use parquet_variant::VariantDecimal4;
// Create a value representing the decimal 123.4567
let decimal = VariantDecimal4::try_new(1234567, 4).expect("Failed to create decimal");
```

---

## VariantDecimal8

`struct` · `parquet_variant::variant::decimal::VariantDecimal8`

```rust
struct VariantDecimal8
```

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `core::fmt::Display`, `parquet_variant::variant::decimal::VariantDecimalType`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn integer(&self) -> i64
fn scale(&self) -> u8
fn try_new(integer: i64, scale: u8) -> Result<Self, ArrowError>
```

**via `core::convert::From`**

```rust
fn from(decimal: VariantDecimal4) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(decimal: VariantDecimal16) -> Result<Self, ArrowError>
fn try_from(integer: i64) -> Result<Self, ArrowError>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `parquet_variant::variant::decimal::VariantDecimalType`**

```rust
fn integer(&self) -> i64
fn scale(&self) -> u8
fn try_new(integer: i64, scale: u8) -> Result<Self, ArrowError>
fn try_new_with_signed_scale(integer: i64, scale: i8) -> Result<Self, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/parquet_variant.variant.decimal.VariantDecimal8.md).


Represents an 8-byte decimal value in the Variant format.

This struct stores a decimal number using a 64-bit signed integer for the coefficient
and an 8-bit unsigned integer for the scale (number of decimal places). Its precision is between 10 and 18 digits.

For valid precision and scale values, see the Variant specification:

<https://github.com/apache/parquet-format/blob/87f2c8bf77eefb4c43d0ebaeea1778bd28ac3609/VariantEncoding.md?plain=1#L418-L420>

# Example: Create a VariantDecimal8
```
# use parquet_variant::VariantDecimal8;
// Create a value representing the decimal 123456.78
let decimal = VariantDecimal8::try_new(12345678, 2).expect("Failed to create decimal");
```

---

## VariantDecimalType

`trait` · `parquet_variant::variant::decimal::VariantDecimalType`

```rust
trait VariantDecimalType: Into<super::Variant<'static, 'static>>
```

**Implementors** (3)

- `parquet_variant::variant::decimal::VariantDecimal16`
- `parquet_variant::variant::decimal::VariantDecimal4`
- `parquet_variant::variant::decimal::VariantDecimal8`

**Methods** (5)

```rust
fn integer(&self) -> Self::Native
fn is_valid_precision_and_scale(precision: &u8, scale: &i8) -> bool
fn scale(&self) -> u8
fn try_new(integer: Self::Native, scale: u8) -> Result<Self, ArrowError>
fn try_new_with_signed_scale(integer: Self::Native, scale: i8) -> Result<Self, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/parquet_variant.variant.decimal.VariantDecimalType.md).


Trait for variant decimal types, enabling generic code across Decimal4/8/16

This trait provides a common interface for the three variant decimal types,
allowing generic functions and data structures to work with any decimal width.
It is modeled after Arrow's `DecimalType` trait but adapted for variant semantics.

# Example

```
# use parquet_variant::{VariantDecimal4, VariantDecimal8, VariantDecimalType};
#
fn extract_scale<D: VariantDecimalType>(decimal: D) -> u8 {
    decimal.scale()
}

let dec4 = VariantDecimal4::try_new(12345, 2).unwrap();
let dec8 = VariantDecimal8::try_new(67890, 3).unwrap();

assert_eq!(extract_scale(dec4), 2);
assert_eq!(extract_scale(dec8), 3);
```

---
