# `arrow_data::decimal`

Crate `arrow-data` · 17 public items · structured records in [`model/arrow_data.decimal.json`](../model/arrow_data.decimal.json)

## MAX_DECIMAL128_FOR_EACH_PRECISION

`constant` · `arrow_data::decimal::MAX_DECIMAL128_FOR_EACH_PRECISION`

Also reachable as `arrow::datatypes::MAX_DECIMAL128_FOR_EACH_PRECISION`

```rust
const MAX_DECIMAL128_FOR_EACH_PRECISION: [i128; 39] = _
```

[Full member, field, variant and typed contracts](../operations/arrow_data.decimal.MAX_DECIMAL128_FOR_EACH_PRECISION.md).


`MAX_DECIMAL128_FOR_EACH_PRECISION[p]` holds the maximum `i128` value that can
be stored in [`Decimal128`] value of precision `p`.

# Notes

The first element is unused and is inserted so that we can look up using
precision as the index without the need to subtract 1 first.

# Example
```
# use arrow_data::decimal::MAX_DECIMAL128_FOR_EACH_PRECISION;
assert_eq!(MAX_DECIMAL128_FOR_EACH_PRECISION[3], 999);
```

[`Decimal128`]: arrow_schema::DataType::Decimal128

---

## MAX_DECIMAL256_FOR_EACH_PRECISION

`constant` · `arrow_data::decimal::MAX_DECIMAL256_FOR_EACH_PRECISION`

Also reachable as `arrow::datatypes::MAX_DECIMAL256_FOR_EACH_PRECISION`

```rust
const MAX_DECIMAL256_FOR_EACH_PRECISION: [arrow_buffer::i256; 77] = _
```

[Full member, field, variant and typed contracts](../operations/arrow_data.decimal.MAX_DECIMAL256_FOR_EACH_PRECISION.md).


`MAX_DECIMAL256_FOR_EACH_PRECISION[p]` holds the maximum [`i256`] value that can
be stored in a [`Decimal256`] value of precision `p`.

# Notes

Each element is the max value of signed 256-bit integer for the specified
precision which is encoded to the 32-byte width format of little-endian.

The first element is unused and is inserted so that we can look up using
precision as the index without the need to subtract 1 first.

# Example
```
# use arrow_buffer::i256;
# use arrow_data::decimal::MAX_DECIMAL256_FOR_EACH_PRECISION;
assert_eq!(MAX_DECIMAL256_FOR_EACH_PRECISION[3], i256::from(999));
```

[`Decimal256`]: arrow_schema::DataType::Decimal256
[`i256`]: arrow_buffer::i256

---

## MAX_DECIMAL32_FOR_EACH_PRECISION

`constant` · `arrow_data::decimal::MAX_DECIMAL32_FOR_EACH_PRECISION`

Also reachable as `arrow::datatypes::MAX_DECIMAL32_FOR_EACH_PRECISION`

```rust
const MAX_DECIMAL32_FOR_EACH_PRECISION: [i32; 10] = _
```

[Full member, field, variant and typed contracts](../operations/arrow_data.decimal.MAX_DECIMAL32_FOR_EACH_PRECISION.md).


`MAX_DECIMAL32_FOR_EACH_PRECISION[p]` holds the maximum `i32` value that can
be stored in [`Decimal32`] value of precision `p`.

# Notes

The first element is unused and is inserted so that we can look up using
precision as the index without the need to subtract 1 first.

# Example
```
# use arrow_data::decimal::MAX_DECIMAL32_FOR_EACH_PRECISION;
assert_eq!(MAX_DECIMAL32_FOR_EACH_PRECISION[3], 999);
```

[`Decimal32`]: arrow_schema::DataType::Decimal32

---

## MAX_DECIMAL64_FOR_EACH_PRECISION

`constant` · `arrow_data::decimal::MAX_DECIMAL64_FOR_EACH_PRECISION`

Also reachable as `arrow::datatypes::MAX_DECIMAL64_FOR_EACH_PRECISION`

```rust
const MAX_DECIMAL64_FOR_EACH_PRECISION: [i64; 19] = _
```

[Full member, field, variant and typed contracts](../operations/arrow_data.decimal.MAX_DECIMAL64_FOR_EACH_PRECISION.md).


`MAX_DECIMAL64_FOR_EACH_PRECISION[p]` holds the maximum `i64` value that can
be stored in [`Decimal64`] value of precision `p`.

# Notes

The first element is unused and is inserted so that we can look up using
precision as the index without the need to subtract 1 first.

# Example
```
# use arrow_data::decimal::MAX_DECIMAL64_FOR_EACH_PRECISION;
assert_eq!(MAX_DECIMAL64_FOR_EACH_PRECISION[3], 999);
```

[`Decimal64`]: arrow_schema::DataType::Decimal64

---

## MIN_DECIMAL128_FOR_EACH_PRECISION

`constant` · `arrow_data::decimal::MIN_DECIMAL128_FOR_EACH_PRECISION`

Also reachable as `arrow::datatypes::MIN_DECIMAL128_FOR_EACH_PRECISION`

```rust
const MIN_DECIMAL128_FOR_EACH_PRECISION: [i128; 39] = _
```

[Full member, field, variant and typed contracts](../operations/arrow_data.decimal.MIN_DECIMAL128_FOR_EACH_PRECISION.md).


`MIN_DECIMAL_FOR_EACH_PRECISION[p]` holds the minimum `i128` value that can
be stored in a [`Decimal128`] value of precision `p`.

# Notes

The first element is unused and is inserted so that we can look up using
precision as the index without the need to subtract 1 first.

# Example
```
# use arrow_data::decimal::MIN_DECIMAL128_FOR_EACH_PRECISION;
assert_eq!(MIN_DECIMAL128_FOR_EACH_PRECISION[3], -999);
```

[`Decimal128`]: arrow_schema::DataType::Decimal128

---

## MIN_DECIMAL256_FOR_EACH_PRECISION

`constant` · `arrow_data::decimal::MIN_DECIMAL256_FOR_EACH_PRECISION`

Also reachable as `arrow::datatypes::MIN_DECIMAL256_FOR_EACH_PRECISION`

```rust
const MIN_DECIMAL256_FOR_EACH_PRECISION: [arrow_buffer::i256; 77] = _
```

[Full member, field, variant and typed contracts](../operations/arrow_data.decimal.MIN_DECIMAL256_FOR_EACH_PRECISION.md).


`MIN_DECIMAL256_FOR_EACH_PRECISION[p]` holds the minimum [`i256`] value that can
be stored in a [`Decimal256`] value of precision `p`.

# Notes

Each element is the min value of signed 256-bit integer for the specified precision which
is encoded to the 76-byte width format of little-endian.

The first element is unused and is inserted so that we can look up using
precision as the index without the need to subtract 1 first.
# Example
```
# use arrow_buffer::i256;
# use arrow_data::decimal::MIN_DECIMAL256_FOR_EACH_PRECISION;
assert_eq!(MIN_DECIMAL256_FOR_EACH_PRECISION[3], i256::from(-999));
```

[`i256`]: arrow_buffer::i256
[`Decimal256`]: arrow_schema::DataType::Decimal256

---

## MIN_DECIMAL32_FOR_EACH_PRECISION

`constant` · `arrow_data::decimal::MIN_DECIMAL32_FOR_EACH_PRECISION`

Also reachable as `arrow::datatypes::MIN_DECIMAL32_FOR_EACH_PRECISION`

```rust
const MIN_DECIMAL32_FOR_EACH_PRECISION: [i32; 10] = _
```

[Full member, field, variant and typed contracts](../operations/arrow_data.decimal.MIN_DECIMAL32_FOR_EACH_PRECISION.md).


`MIN_DECIMAL32_FOR_EACH_PRECISION[p]` holds the minimum `ialue that can
be stored in a [`Decimal32`] value of precision `p`.

# Notes

The first element is unused and is inserted so that we can look up using
precision as the index without the need to subtract 1 first.

# Example
```
# use arrow_data::decimal::MIN_DECIMAL32_FOR_EACH_PRECISION;
assert_eq!(MIN_DECIMAL32_FOR_EACH_PRECISION[3], -999);
```

[`Decimal32`]: arrow_schema::DataType::Decimal32

---

## MIN_DECIMAL64_FOR_EACH_PRECISION

`constant` · `arrow_data::decimal::MIN_DECIMAL64_FOR_EACH_PRECISION`

Also reachable as `arrow::datatypes::MIN_DECIMAL64_FOR_EACH_PRECISION`

```rust
const MIN_DECIMAL64_FOR_EACH_PRECISION: [i64; 19] = _
```

[Full member, field, variant and typed contracts](../operations/arrow_data.decimal.MIN_DECIMAL64_FOR_EACH_PRECISION.md).


`MIN_DECIMAL64_FOR_EACH_PRECISION[p]` holds the minimum `i64` value that can
be stored in a [`Decimal64`] value of precision `p`.

# Notes

The first element is unused and is inserted so that we can look up using
precision as the index without the need to subtract 1 first.

# Example
```
# use arrow_data::decimal::MIN_DECIMAL64_FOR_EACH_PRECISION;
assert_eq!(MIN_DECIMAL64_FOR_EACH_PRECISION[3], -999);
```

[`Decimal64`]: arrow_schema::DataType::Decimal64

---

## format_decimal_str

`function` · `arrow_data::decimal::format_decimal_str`

Also reachable as `arrow::datatypes::format_decimal_str`

```rust
fn format_decimal_str(value_str: &str, precision: usize, scale: i8) -> String
```

[Full member, field, variant and typed contracts](../operations/arrow_data.decimal.format_decimal_str.md).


Formats a decimal string given the precision and scale.

---

## is_validate_decimal256_precision

`function` · `arrow_data::decimal::is_validate_decimal256_precision`

Also reachable as `arrow::datatypes::is_validate_decimal256_precision`

```rust
fn is_validate_decimal256_precision(value: arrow_buffer::i256, precision: u8) -> bool
```

[Full member, field, variant and typed contracts](../operations/arrow_data.decimal.is_validate_decimal256_precision.md).


Return true if the specified `i256` value can be properly
interpreted as a [`Decimal256`] number with precision `precision`

[`Decimal256`]: arrow_schema::DataType::Decimal256

---

## is_validate_decimal32_precision

`function` · `arrow_data::decimal::is_validate_decimal32_precision`

Also reachable as `arrow::datatypes::is_validate_decimal32_precision`

```rust
fn is_validate_decimal32_precision(value: i32, precision: u8) -> bool
```

[Full member, field, variant and typed contracts](../operations/arrow_data.decimal.is_validate_decimal32_precision.md).


Returns true if the specified `i32` value can be properly
interpreted as a [`Decimal32`] number with precision `precision`

[`Decimal32`]: arrow_schema::DataType::Decimal32

---

## is_validate_decimal64_precision

`function` · `arrow_data::decimal::is_validate_decimal64_precision`

Also reachable as `arrow::datatypes::is_validate_decimal64_precision`

```rust
fn is_validate_decimal64_precision(value: i64, precision: u8) -> bool
```

[Full member, field, variant and typed contracts](../operations/arrow_data.decimal.is_validate_decimal64_precision.md).


Returns true if the specified `i64` value can be properly
interpreted as a [`Decimal64`] number with precision `precision`

[`Decimal64`]: arrow_schema::DataType::Decimal64

---

## is_validate_decimal_precision

`function` · `arrow_data::decimal::is_validate_decimal_precision`

Also reachable as `arrow::datatypes::is_validate_decimal_precision`

```rust
fn is_validate_decimal_precision(value: i128, precision: u8) -> bool
```

[Full member, field, variant and typed contracts](../operations/arrow_data.decimal.is_validate_decimal_precision.md).


Returns true if the specified `i128` value can be properly
interpreted as a [`Decimal128`] number with precision `precision`

[`Decimal128`]: arrow_schema::DataType::Decimal128

---

## validate_decimal256_precision

`function` · `arrow_data::decimal::validate_decimal256_precision`

Also reachable as `arrow::datatypes::validate_decimal256_precision`

```rust
fn validate_decimal256_precision(value: arrow_buffer::i256, precision: u8, scale: i8) -> Result<(), arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_data.decimal.validate_decimal256_precision.md).


Validates that the specified `i256` of value can be properly
interpreted as a [`Decimal256`] number with precision `precision`

[`Decimal256`]: arrow_schema::DataType::Decimal256

---

## validate_decimal32_precision

`function` · `arrow_data::decimal::validate_decimal32_precision`

Also reachable as `arrow::datatypes::validate_decimal32_precision`

```rust
fn validate_decimal32_precision(value: i32, precision: u8, scale: i8) -> Result<(), arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_data.decimal.validate_decimal32_precision.md).


Validates that the specified `i32` value can be properly
interpreted as a [`Decimal32`] number with precision `precision`

[`Decimal32`]: arrow_schema::DataType::Decimal32

---

## validate_decimal64_precision

`function` · `arrow_data::decimal::validate_decimal64_precision`

Also reachable as `arrow::datatypes::validate_decimal64_precision`

```rust
fn validate_decimal64_precision(value: i64, precision: u8, scale: i8) -> Result<(), arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_data.decimal.validate_decimal64_precision.md).


Validates that the specified `i64` value can be properly
interpreted as a [`Decimal64`] number with precision `precision`

[`Decimal64`]: arrow_schema::DataType::Decimal64

---

## validate_decimal_precision

`function` · `arrow_data::decimal::validate_decimal_precision`

Also reachable as `arrow::datatypes::validate_decimal_precision`

```rust
fn validate_decimal_precision(value: i128, precision: u8, scale: i8) -> Result<(), arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_data.decimal.validate_decimal_precision.md).


Validates that the specified `i128` value can be properly
interpreted as a [`Decimal128`] number with precision `precision`

[`Decimal128`]: arrow_schema::DataType::Decimal128

---
