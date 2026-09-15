# `arrow_cast::cast`

Crate `arrow-cast` · 8 public items · structured records in [`model/arrow_cast.cast.json`](../model/arrow_cast.cast.json)

## can_cast_types

`function` · `arrow_cast::cast::can_cast_types`

Also reachable as `arrow::compute::can_cast_types`, `arrow_cast::can_cast_types`

```rust
fn can_cast_types(from_type: &DataType, to_type: &DataType) -> bool
```

Return true if a value of type `from_type` can be cast into a value of `to_type`.

See [`cast_with_options`] for more information

---

## cast

`function` · `arrow_cast::cast::cast`

Also reachable as `arrow::compute::cast`, `arrow::compute::kernels::cast`, `arrow_cast::cast`

```rust
fn cast(array: &dyn Array, to_type: &DataType) -> Result<ArrayRef, ArrowError>
```

Cast `array` to the provided data type and return a new Array with type `to_type`, if possible.

See [`cast_with_options`] for more information

---

## cast_num_to_bool

`function` · `arrow_cast::cast::cast_num_to_bool`

Also reachable as `arrow::compute::cast_num_to_bool`, `arrow_cast::cast_num_to_bool`

```rust
fn cast_num_to_bool<I>(value: I) -> bool where I: Default + PartialEq
```

Cast numeric types to boolean

---

## cast_with_options

`function` · `arrow_cast::cast::cast_with_options`

Also reachable as `arrow::compute::cast_with_options`, `arrow_cast::cast_with_options`

```rust
fn cast_with_options(array: &dyn Array, to_type: &DataType, cast_options: &CastOptions<'_>) -> Result<ArrayRef, ArrowError>
```

Try to cast `array` to `to_type` if possible.

Returns a new Array with type `to_type` if possible.

Accepts [`CastOptions`] to specify cast behavior. See also [`cast()`].

# Behavior
* `Boolean` to `Utf8`: `true` => '1', `false` => `0`
* `Utf8` to `Boolean`: `true`, `yes`, `on`, `1` => `true`, `false`, `no`, `off`, `0` => `false`,
  short variants are accepted, other strings return null or error
* `Utf8` to Numeric: strings that can't be parsed to numbers return null, float strings
  in integer casts return null
* Numeric to `Boolean`: 0 returns `false`, any other value returns `true`
* `List` to `List`: the underlying data type is cast
* `List` to `FixedSizeList`: the underlying data type is cast. If safe is true and a list element
  has the wrong length it will be replaced with NULL, otherwise an error will be returned
* Primitive to `List`: a list array with 1 value per slot is created
* `Date32` and `Date64`: precision lost when going to higher interval
* `Time32 and `Time64`: precision lost when going to higher interval
* `Timestamp` and `Date{32|64}`: precision lost when going to higher interval
* Temporal to/from backing Primitive: zero-copy with data type change
* `Float16/Float32/Float64` to `Decimal(precision, scale)` rounds to the `scale` decimals
  (i.e. casting `6.4999` to `Decimal(10, 1)` becomes `6.5`).
* `Decimal` to `Float16/Float32/Float64` is lossy and values outside the representable
  range become `INFINITY` or `-INFINITY` without error.

Unsupported Casts (check with `can_cast_types` before calling):
* To or from `StructArray`
* `List` to `Primitive`
* `Interval` and `Duration`

# Durations and Intervals

Casting integer types directly to interval types such as
[`IntervalMonthDayNano`] is not supported because the meaning of the integer
is ambiguous. For example, the integer  could represent either nanoseconds
or months.

To cast an integer type to an interval type, first convert to a Duration
type, and then cast that to the desired interval type.

For example, to convert an `Int64` representing nanoseconds to an
`IntervalMonthDayNano` you would first convert the `Int64` to a
`DurationNanoseconds`, and then cast that to `IntervalMonthDayNano`.

# Timestamps and Timezones

Timestamps are stored with an optional timezone in Arrow.

## Casting timestamps to a timestamp without timezone / UTC
```
# use arrow_array::Int64Array;
# use arrow_array::types::TimestampSecondType;
# use arrow_cast::{cast, display};
# use arrow_array::cast::AsArray;
# use arrow_schema::{DataType, TimeUnit};
// can use "UTC" if chrono-tz feature is enabled, here use offset based timezone
let data_type = DataType::Timestamp(TimeUnit::Second, None);
let a = Int64Array::from(vec![1_000_000_000, 2_000_000_000, 3_000_000_000]);
let b = cast(&a, &data_type).unwrap();
let b = b.as_primitive::<TimestampSecondType>(); // downcast to result type
assert_eq!(2_000_000_000, b.value(1)); // values are the same as the type has no timezone
// use display to show them (note has no trailing Z)
assert_eq!("2033-05-18T03:33:20", display::array_value_to_string(&b, 1).unwrap());
```

## Casting timestamps to a timestamp with timezone

Similarly to the previous example, if you cast numeric values to a timestamp
with timezone, the cast kernel will not change the underlying values
but display and other functions will interpret them as being in the provided timezone.

```
# use arrow_array::Int64Array;
# use arrow_array::types::TimestampSecondType;
# use arrow_cast::{cast, display};
# use arrow_array::cast::AsArray;
# use arrow_schema::{DataType, TimeUnit};
// can use "Americas/New_York" if chrono-tz feature is enabled, here use offset based timezone
let data_type = DataType::Timestamp(TimeUnit::Second, Some("-05:00".into()));
let a = Int64Array::from(vec![1_000_000_000, 2_000_000_000, 3_000_000_000]);
let b = cast(&a, &data_type).unwrap();
let b = b.as_primitive::<TimestampSecondType>(); // downcast to result type
assert_eq!(2_000_000_000, b.value(1)); // values are still the same
// displayed in the target timezone (note the offset -05:00)
assert_eq!("2033-05-17T22:33:20-05:00", display::array_value_to_string(&b, 1).unwrap());
```
# Casting timestamps without timezone to timestamps with timezone

When casting from a timestamp without timezone to a timestamp with
timezone, the cast kernel interprets the timestamp values as being in
the destination timezone and then adjusts the underlying value to UTC as required

However, note that when casting from a timestamp with timezone BACK to a
timestamp without timezone the cast kernel does not adjust the values.

Thus round trip casting a timestamp without timezone to a timestamp with
timezone and back to a timestamp without timezone results in different
values than the starting values.

```
# use arrow_array::Int64Array;
# use arrow_array::types::{TimestampSecondType};
# use arrow_cast::{cast, display};
# use arrow_array::cast::AsArray;
# use arrow_schema::{DataType, TimeUnit};
let data_type  = DataType::Timestamp(TimeUnit::Second, None);
let data_type_tz = DataType::Timestamp(TimeUnit::Second, Some("-05:00".into()));
let a = Int64Array::from(vec![1_000_000_000, 2_000_000_000, 3_000_000_000]);
let b = cast(&a, &data_type).unwrap(); // cast to timestamp without timezone
let b = b.as_primitive::<TimestampSecondType>(); // downcast to result type
assert_eq!(2_000_000_000, b.value(1)); // values are still the same
// displayed without a timezone (note lack of offset or Z)
assert_eq!("2033-05-18T03:33:20", display::array_value_to_string(&b, 1).unwrap());

// Convert timestamps without a timezone to timestamps with a timezone
let c = cast(&b, &data_type_tz).unwrap();
let c = c.as_primitive::<TimestampSecondType>(); // downcast to result type
assert_eq!(2_000_018_000, c.value(1)); // value has been adjusted by offset
// displayed with the target timezone offset (-05:00)
assert_eq!("2033-05-18T03:33:20-05:00", display::array_value_to_string(&c, 1).unwrap());

// Convert from timestamp with timezone back to timestamp without timezone
let d = cast(&c, &data_type).unwrap();
let d = d.as_primitive::<TimestampSecondType>(); // downcast to result type
assert_eq!(2_000_018_000, d.value(1)); // value has not been adjusted
// NOTE: the timestamp is adjusted (08:33:20 instead of 03:33:20 as in previous example)
assert_eq!("2033-05-18T08:33:20", display::array_value_to_string(&d, 1).unwrap());
```

---

## num_cast

`function` · `arrow_cast::cast::num_cast`

Also reachable as `arrow::compute::num_cast`, `arrow_cast::num_cast`

```rust
fn num_cast<I, O>(value: I) -> Option<O> where I: NumCast, O: NumCast
```

Natural cast between numeric types
Return None if the input `value` can't be casted to type `O`.

---

## single_bool_to_numeric

`function` · `arrow_cast::cast::single_bool_to_numeric`

Also reachable as `arrow::compute::single_bool_to_numeric`, `arrow_cast::single_bool_to_numeric`

```rust
fn single_bool_to_numeric<O>(value: bool) -> Option<O> where O: num_traits::NumCast + Default
```

Cast single bool value to numeric value.

---

## single_decimal_to_float_lossy

`function` · `arrow_cast::cast::single_decimal_to_float_lossy`

Also reachable as `arrow::compute::single_decimal_to_float_lossy`, `arrow_cast::single_decimal_to_float_lossy`

```rust
fn single_decimal_to_float_lossy<D, F>(f: &F, x: D::Native, scale: i32) -> f64 where D: DecimalType, F: Fn(D::Native) -> f64
```

Lossy conversion from decimal to float.

Conversion is lossy and follows standard floating point semantics. Values
that exceed the representable range become `INFINITY` or `-INFINITY` without
returning an error.

---

## CastOptions

`struct` · `arrow_cast::cast::CastOptions`

Also reachable as `arrow::compute::CastOptions`, `arrow_cast::CastOptions`

```rust
struct CastOptions<'a>
```

**Fields**: `safe`, `format_options`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

CastOptions provides a way to override the default cast behaviors

---
