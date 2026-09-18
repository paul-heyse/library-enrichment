# `arrow_array::types::DecimalType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.types.DecimalType.json).

<a id="op-a2cd942431b75f4c4899d6fe"></a>
## DecimalType

`trait` · `arrow_array::types::DecimalType` · arrow-array 59.3.0

```rust
trait DecimalType: 'static + Send + Sync + ArrowPrimitiveType + decimal::DecimalTypeSealed
```

Source: `src/types.rs:1372`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A trait over the decimal types, used by [`PrimitiveArray`] to provide a generic
implementation across the various decimal types

Implemented by [`Decimal32Type`](../operations/arrow_array.types.Decimal32Type.md#op-7185fc8560257c1b45d40d1b), [`Decimal64Type`](../operations/arrow_array.types.Decimal64Type.md#op-231fc2e5875c21547edf7271), [`Decimal128Type`](../operations/arrow_array.types.Decimal128Type.md#op-592f3a4cb5ae89447c626364) and [`Decimal256Type`](../operations/arrow_array.types.Decimal256Type.md#op-0f78dc76aab096b6ad3b4725)
for [`Decimal32Array`], [`Decimal64Array`], [`Decimal128Array`] and [`Decimal256Array`] respectively

[`PrimitiveArray`]: crate::array::PrimitiveArray
[`Decimal32Array`]: crate::array::Decimal32Array
[`Decimal64Array`]: crate::array::Decimal64Array
[`Decimal128Array`]: crate::array::Decimal128Array
[`Decimal256Array`]: crate::array::Decimal256Array

<a id="op-379ce6e9df6b694310cee11e"></a>
## BYTE_LENGTH

`assoc_const` · `arrow_array::types::DecimalType::BYTE_LENGTH` · arrow-array 59.3.0

```rust
BYTE_LENGTH
```

Source: `src/types.rs:1376`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Width of the type

<a id="op-3c016aa58f8b8ff1e12a6e76"></a>
## DEFAULT_TYPE

`assoc_const` · `arrow_array::types::DecimalType::DEFAULT_TYPE` · arrow-array 59.3.0

```rust
DEFAULT_TYPE
```

Source: `src/types.rs:1386`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Default values for [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c)

<a id="op-70518df31df80026595faed0"></a>
## MAX_FOR_EACH_PRECISION

`assoc_const` · `arrow_array::types::DecimalType::MAX_FOR_EACH_PRECISION` · arrow-array 59.3.0

```rust
MAX_FOR_EACH_PRECISION
```

Source: `src/types.rs:1382`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

The maximum value for each precision in `0..=MAX_PRECISION`: [0, 9, 99, ...]

<a id="op-bf43967f9880aa10d53440dd"></a>
## MAX_PRECISION

`assoc_const` · `arrow_array::types::DecimalType::MAX_PRECISION` · arrow-array 59.3.0

```rust
MAX_PRECISION
```

Source: `src/types.rs:1378`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Maximum number of significant digits

<a id="op-e8ce85ee86a77b004f88a9bc"></a>
## MAX_SCALE

`assoc_const` · `arrow_array::types::DecimalType::MAX_SCALE` · arrow-array 59.3.0

```rust
MAX_SCALE
```

Source: `src/types.rs:1380`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Maximum no of digits after the decimal point (note the scale can be negative)

<a id="op-c950b731249df073f750341a"></a>
## PREFIX

`assoc_const` · `arrow_array::types::DecimalType::PREFIX` · arrow-array 59.3.0

```rust
PREFIX
```

Source: `src/types.rs:1389`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

"Decimal32", "Decimal64", "Decimal128" or "Decimal256", for use in error messages

<a id="op-2d434e9b1db6cf5144a11b6b"></a>
## TYPE_CONSTRUCTOR

`assoc_const` · `arrow_array::types::DecimalType::TYPE_CONSTRUCTOR` · arrow-array 59.3.0

```rust
TYPE_CONSTRUCTOR
```

Source: `src/types.rs:1384`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

fn to create its [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c)

<a id="op-791038aeea67a3f388396892"></a>
## format_decimal

`function` · `arrow_array::types::DecimalType::format_decimal` · arrow-array 59.3.0

```rust
fn format_decimal(value: Self::Native, precision: u8, scale: i8) -> String
```

Source: `src/types.rs:1392`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Formats the decimal value with the provided precision and scale

<a id="op-0da9f697e1ae1d4fb5f2fc6d"></a>
## is_valid_decimal_precision

`function` · `arrow_array::types::DecimalType::is_valid_decimal_precision` · arrow-array 59.3.0

```rust
fn is_valid_decimal_precision(value: Self::Native, precision: u8) -> bool
```

Source: `src/types.rs:1402`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Determines whether `value` contains no more than `precision` decimal digits

<a id="op-9a1e8c213497dc3583477cbd"></a>
## validate_decimal_precision

`function` · `arrow_array::types::DecimalType::validate_decimal_precision` · arrow-array 59.3.0

```rust
fn validate_decimal_precision(value: Self::Native, precision: u8, scale: i8) -> Result<(), ArrowError>
```

Source: `src/types.rs:1395`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Validates that `value` contains no more than `precision` decimal digits
