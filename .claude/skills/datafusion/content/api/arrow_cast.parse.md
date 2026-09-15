# `arrow_cast::parse`

Crate `arrow-cast` · 12 public items · structured records in [`model/arrow_cast.parse.json`](../model/arrow_cast.parse.json)

## IntervalUnit

`enum` · `arrow_cast::parse::IntervalUnit`

Also reachable as `arrow::compute::kernels::cast_utils::IntervalUnit`

```rust
enum IntervalUnit
```

**Variants**: `Century`, `Decade`, `Year`, `Month`, `Week`, `Day`, `Hour`, `Minute`, `Second`, `Millisecond`, `Microsecond`, `Nanosecond`

**Implements**: `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, ArrowError>
```

Represents the units of an interval, with each variant
corresponding to a bit in the interval's bitfield representation

---

## parse_decimal

`function` · `arrow_cast::parse::parse_decimal`

Also reachable as `arrow::compute::kernels::cast_utils::parse_decimal`

```rust
fn parse_decimal<T: DecimalType>(s: &str, precision: u8, scale: i8) -> Result<T::Native, arrow_schema::ArrowError>
```

Parse the string format decimal value to i128/i256 format and checking the precision and scale.
Expected behavior:
- The result value can't be out of bounds.
- When parsing a decimal with scale 0, all fractional digits will be discarded. The final
  fractional digits may be a subset or a superset of the digits after the decimal point when
  e-notation is used.

---

## parse_interval_day_time

`function` · `arrow_cast::parse::parse_interval_day_time`

Also reachable as `arrow::compute::kernels::cast_utils::parse_interval_day_time`

```rust
fn parse_interval_day_time(value: &str) -> Result<<IntervalDayTimeType as ArrowPrimitiveType>::Native, arrow_schema::ArrowError>
```

Parse human-readable interval string to Arrow [IntervalDayTimeType]

---

## parse_interval_month_day_nano

`function` · `arrow_cast::parse::parse_interval_month_day_nano`

Also reachable as `arrow::compute::kernels::cast_utils::parse_interval_month_day_nano`

```rust
fn parse_interval_month_day_nano(value: &str) -> Result<<IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, arrow_schema::ArrowError>
```

Parse human-readable interval string to Arrow [IntervalMonthDayNanoType]

---

## parse_interval_month_day_nano_config

`function` · `arrow_cast::parse::parse_interval_month_day_nano_config`

Also reachable as `arrow::compute::kernels::cast_utils::parse_interval_month_day_nano_config`

```rust
fn parse_interval_month_day_nano_config(value: &str, config: IntervalParseConfig) -> Result<<IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, arrow_schema::ArrowError>
```

Parse human-readable interval string to Arrow [IntervalMonthDayNanoType]

---

## parse_interval_year_month

`function` · `arrow_cast::parse::parse_interval_year_month`

Also reachable as `arrow::compute::kernels::cast_utils::parse_interval_year_month`

```rust
fn parse_interval_year_month(value: &str) -> Result<<IntervalYearMonthType as ArrowPrimitiveType>::Native, arrow_schema::ArrowError>
```

Parse human-readable interval string to Arrow [IntervalYearMonthType]

---

## string_to_datetime

`function` · `arrow_cast::parse::string_to_datetime`

Also reachable as `arrow::compute::kernels::cast_utils::string_to_datetime`

```rust
fn string_to_datetime<T: TimeZone>(timezone: &T, s: &str) -> Result<DateTime<T>, arrow_schema::ArrowError>
```

Accepts a string and parses it relative to the provided `timezone`

In addition to RFC3339 / ISO8601 standard timestamps, it also
accepts strings that use a space ` ` to separate the date and time
as well as strings that have no explicit timezone offset.

Examples of accepted inputs:
* `1997-01-31T09:26:56.123Z`        # RCF3339
* `1997-01-31T09:26:56.123-05:00`   # RCF3339
* `1997-01-31 09:26:56.123-05:00`   # close to RCF3339 but with a space rather than T
* `2023-01-01 04:05:06.789 -08`     # close to RCF3339, no fractional seconds or time separator
* `1997-01-31T09:26:56.123`         # close to RCF3339 but no timezone offset specified
* `1997-01-31 09:26:56.123`         # close to RCF3339 but uses a space and no timezone offset
* `1997-01-31 09:26:56`             # close to RCF3339, no fractional seconds
* `1997-01-31 092656`               # close to RCF3339, no fractional seconds
* `1997-01-31 092656+04:00`         # close to RCF3339, no fractional seconds or time separator
* `1997-01-31`                      # close to RCF3339, only date no time

[IANA timezones] are only supported if the `arrow-array/chrono-tz` feature is enabled

* `2023-01-01 040506 America/Los_Angeles`

If a timestamp is ambiguous, for example as a result of daylight-savings time, an error
will be returned

Some formats supported by PostgresSql <https://www.postgresql.org/docs/current/datatype-datetime.html#DATATYPE-DATETIME-TIME-TABLE>
are not supported, like

* "2023-01-01 04:05:06.789 +07:30:00",
* "2023-01-01 040506 +07:30:00",
* "2023-01-01 04:05:06.789 PST",

[IANA timezones]: https://www.iana.org/time-zones

---

## string_to_time_nanoseconds

`function` · `arrow_cast::parse::string_to_time_nanoseconds`

Also reachable as `arrow::compute::kernels::cast_utils::string_to_time_nanoseconds`

```rust
fn string_to_time_nanoseconds(s: &str) -> Result<i64, arrow_schema::ArrowError>
```

Accepts a string in ISO8601 standard format and some
variants and converts it to nanoseconds since midnight.

Examples of accepted inputs:

* `09:26:56.123 AM`
* `23:59:59`
* `6:00 pm`

Internally, this function uses the `chrono` library for the time parsing

## Timezone / Offset Handling

This function does not support parsing strings with a timezone
or offset specified, as it considers only time since midnight.

---

## string_to_timestamp_nanos

`function` · `arrow_cast::parse::string_to_timestamp_nanos`

Also reachable as `arrow::compute::kernels::cast_utils::string_to_timestamp_nanos`

```rust
fn string_to_timestamp_nanos(s: &str) -> Result<i64, arrow_schema::ArrowError>
```

Accepts a string in RFC3339 / ISO8601 standard format and some
variants and converts it to a nanosecond precision timestamp.

See [`string_to_datetime`] for the full set of supported formats

Implements the `to_timestamp` function to convert a string to a
timestamp, following the model of spark SQL’s to_`timestamp`.

Internally, this function uses the `chrono` library for the
datetime parsing

We hope to extend this function in the future with a second
parameter to specifying the format string.

## Timestamp Precision

Function uses the maximum precision timestamps supported by
Arrow (nanoseconds stored as a 64-bit integer) timestamps. This
means the range of dates that timestamps can represent is ~1677 AD
to 2262 AM

## Timezone / Offset Handling

Numerical values of timestamps are stored compared to offset UTC.

This function interprets string without an explicit time zone as timestamps
relative to UTC, see [`string_to_datetime`] for alternative semantics

In particular:

```
# use arrow_cast::parse::string_to_timestamp_nanos;
// Note all three of these timestamps are parsed as the same value
let a = string_to_timestamp_nanos("1997-01-31 09:26:56.123Z").unwrap();
let b = string_to_timestamp_nanos("1997-01-31T09:26:56.123").unwrap();
let c = string_to_timestamp_nanos("1997-01-31T14:26:56.123+05:00").unwrap();

assert_eq!(a, b);
assert_eq!(b, c);
```

---

## IntervalParseConfig

`struct` · `arrow_cast::parse::IntervalParseConfig`

Also reachable as `arrow::compute::kernels::cast_utils::IntervalParseConfig`

```rust
struct IntervalParseConfig
```

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(default_unit: IntervalUnit) -> Self
```

Config to parse interval strings

Currently stores the `default_unit` to use if the string doesn't have one specified

---

## Parser

`trait` · `arrow_cast::parse::Parser`

Also reachable as `arrow::compute::kernels::cast_utils::Parser`

```rust
trait Parser: ArrowPrimitiveType
```

**Implementors** (25)

- `arrow_array::types::Date32Type`
- `arrow_array::types::Date64Type`
- `arrow_array::types::DurationMicrosecondType`
- `arrow_array::types::DurationMillisecondType`
- `arrow_array::types::DurationNanosecondType`
- `arrow_array::types::DurationSecondType`
- `arrow_array::types::Float16Type`
- `arrow_array::types::Float32Type`
- `arrow_array::types::Float64Type`
- `arrow_array::types::Int16Type`
- `arrow_array::types::Int32Type`
- `arrow_array::types::Int64Type`
- `arrow_array::types::Int8Type`
- `arrow_array::types::Time32MillisecondType`
- `arrow_array::types::Time32SecondType`
- `arrow_array::types::Time64MicrosecondType`
- `arrow_array::types::Time64NanosecondType`
- `arrow_array::types::TimestampMicrosecondType`
- `arrow_array::types::TimestampMillisecondType`
- `arrow_array::types::TimestampNanosecondType`
- `arrow_array::types::TimestampSecondType`
- `arrow_array::types::UInt16Type`
- `arrow_array::types::UInt32Type`
- `arrow_array::types::UInt64Type`
- `arrow_array::types::UInt8Type`

**Methods** (2)

```rust
fn parse(string: &str) -> Option<Self::Native>
fn parse_formatted(string: &str, _format: &str) -> Option<Self::Native>
```

Specialized parsing implementations to convert strings to Arrow types.

This is used by csv and json reader and can be used directly as well.

# Example

To parse a string to a [`Date32Type`]:

```
use arrow_cast::parse::Parser;
use arrow_array::types::Date32Type;
let date = Date32Type::parse("2021-01-01").unwrap();
assert_eq!(date, 18628);
```

To parse a string to a [`TimestampNanosecondType`]:

```
use arrow_cast::parse::Parser;
use arrow_array::types::TimestampNanosecondType;
let ts = TimestampNanosecondType::parse("2021-01-01T00:00:00.123456789Z").unwrap();
assert_eq!(ts, 1609459200123456789);
```

---

## MonthDayNano

`type_alias` · `arrow_cast::parse::MonthDayNano`

Also reachable as `arrow::compute::kernels::cast_utils::MonthDayNano`

```rust
type MonthDayNano = (i32, i32, i64)
```

A tuple representing (months, days, nanoseconds) in an interval

---
