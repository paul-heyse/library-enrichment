# `arrow_array::types`

Crate `arrow-array` · 50 public items · structured records in [`model/arrow_array.types.json`](../model/arrow_array.types.json)

## validate_decimal_precision_and_scale

`function` · `arrow_array::types::validate_decimal_precision_and_scale`

Also reachable as `arrow::datatypes::validate_decimal_precision_and_scale`

```rust
fn validate_decimal_precision_and_scale<T: DecimalType>(precision: u8, scale: i8) -> Result<(), arrow_schema::ArrowError>
```

Validate that `precision` and `scale` are valid for `T`

Returns an Error if:
- `precision` is zero
- `precision` is larger than `T:MAX_PRECISION`
- `scale` is larger than `T::MAX_SCALE`
- `scale` is > `precision`

---

## BinaryViewType

`struct` · `arrow_array::types::BinaryViewType`

Also reachable as `arrow::datatypes::BinaryViewType`

```rust
struct BinaryViewType
```

**Implements**: `arrow_array::types::ByteViewType`

**Derives**: PartialEq, StructuralPartialEq

**via `arrow_array::types::ByteViewType`**

```rust
fn validate(views: &[u128], buffers: &[Buffer]) -> Result<(), ArrowError>
```

[`BinaryViewType`] for string arrays

---

## BooleanType

`struct` · `arrow_array::types::BooleanType`

Also reachable as `arrow::datatypes::BooleanType`

```rust
struct BooleanType
```

**Derives**: Debug

A boolean datatype

---

## Date32Type

`struct` · `arrow_array::types::Date32Type`

Also reachable as `arrow::datatypes::Date32Type`

```rust
struct Date32Type
```

**Implements**: `arrow::util::data_gen::RandomTemporalValue`, `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::ArrowTemporalType`, `arrow_cast::parse::Parser`, `datafusion_common::scalar::ScalarType`

**Derives**: Debug

**Methods** (15)

```rust
fn add_day_time(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native) -> <Date32Type as ArrowPrimitiveType>::Native
fn add_day_time_opt(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native) -> Option<<Date32Type as ArrowPrimitiveType>::Native>
fn add_month_day_nano(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native) -> <Date32Type as ArrowPrimitiveType>::Native
fn add_month_day_nano_opt(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native) -> Option<<Date32Type as ArrowPrimitiveType>::Native>
fn add_year_months(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native) -> <Date32Type as ArrowPrimitiveType>::Native
fn add_year_months_opt(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native) -> Option<<Date32Type as ArrowPrimitiveType>::Native>
fn from_naive_date(d: NaiveDate) -> <Date32Type as ArrowPrimitiveType>::Native
fn subtract_day_time(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native) -> <Date32Type as ArrowPrimitiveType>::Native
fn subtract_day_time_opt(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native) -> Option<<Date32Type as ArrowPrimitiveType>::Native>
fn subtract_month_day_nano(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native) -> <Date32Type as ArrowPrimitiveType>::Native
fn subtract_month_day_nano_opt(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native) -> Option<<Date32Type as ArrowPrimitiveType>::Native>
fn subtract_year_months(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native) -> <Date32Type as ArrowPrimitiveType>::Native
fn subtract_year_months_opt(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native) -> Option<<Date32Type as ArrowPrimitiveType>::Native>
fn to_naive_date(i: <Date32Type as ArrowPrimitiveType>::Native) -> NaiveDate
fn to_naive_date_opt(i: <Date32Type as ArrowPrimitiveType>::Native) -> Option<NaiveDate>
```

32-bit date type: the elapsed time since UNIX epoch in days (32 bits).

---

## Date64Type

`struct` · `arrow_array::types::Date64Type`

Also reachable as `arrow::datatypes::Date64Type`

```rust
struct Date64Type
```

**Implements**: `arrow::util::data_gen::RandomTemporalValue`, `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::ArrowTemporalType`, `arrow_cast::parse::Parser`

**Derives**: Debug

**Methods** (8)

```rust
fn add_day_time_opt(date: <Date64Type as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native) -> Option<<Date64Type as ArrowPrimitiveType>::Native>
fn add_month_day_nano_opt(date: <Date64Type as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native) -> Option<<Date64Type as ArrowPrimitiveType>::Native>
fn add_year_months_opt(date: <Date64Type as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native) -> Option<<Date64Type as ArrowPrimitiveType>::Native>
fn from_naive_date(d: NaiveDate) -> <Date64Type as ArrowPrimitiveType>::Native
fn subtract_day_time_opt(date: <Date64Type as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native) -> Option<<Date64Type as ArrowPrimitiveType>::Native>
fn subtract_month_day_nano_opt(date: <Date64Type as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native) -> Option<<Date64Type as ArrowPrimitiveType>::Native>
fn subtract_year_months_opt(date: <Date64Type as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native) -> Option<<Date64Type as ArrowPrimitiveType>::Native>
fn to_naive_date_opt(i: <Date64Type as ArrowPrimitiveType>::Native) -> Option<NaiveDate>
```

64-bit date type: the elapsed time since UNIX epoch in milliseconds (64 bits). Values must be divisible by `86_400_000`. See [`DataType::Date64`] for more details.

---

## Decimal128Type

`struct` · `arrow_array::types::Decimal128Type`

Also reachable as `arrow::datatypes::Decimal128Type`

```rust
struct Decimal128Type
```

**Implements**: `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::DecimalType`

**Derives**: Debug

**via `arrow_array::types::DecimalType`**

```rust
fn format_decimal(value: Self::Native, precision: u8, scale: i8) -> String
fn is_valid_decimal_precision(value: Self::Native, precision: u8) -> bool
fn validate_decimal_precision(num: i128, precision: u8, scale: i8) -> Result<(), ArrowError>
```

The decimal type for a Decimal128Array

---

## Decimal256Type

`struct` · `arrow_array::types::Decimal256Type`

Also reachable as `arrow::datatypes::Decimal256Type`

```rust
struct Decimal256Type
```

**Implements**: `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::DecimalType`

**Derives**: Debug

**via `arrow_array::types::DecimalType`**

```rust
fn format_decimal(value: Self::Native, precision: u8, scale: i8) -> String
fn is_valid_decimal_precision(value: Self::Native, precision: u8) -> bool
fn validate_decimal_precision(num: i256, precision: u8, scale: i8) -> Result<(), ArrowError>
```

The decimal type for a Decimal256Array

---

## Decimal32Type

`struct` · `arrow_array::types::Decimal32Type`

Also reachable as `arrow::datatypes::Decimal32Type`

```rust
struct Decimal32Type
```

**Implements**: `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::DecimalType`

**Derives**: Debug

**via `arrow_array::types::DecimalType`**

```rust
fn format_decimal(value: Self::Native, precision: u8, scale: i8) -> String
fn is_valid_decimal_precision(value: Self::Native, precision: u8) -> bool
fn validate_decimal_precision(num: i32, precision: u8, scale: i8) -> Result<(), ArrowError>
```

The decimal type for a Decimal32Array

---

## Decimal64Type

`struct` · `arrow_array::types::Decimal64Type`

Also reachable as `arrow::datatypes::Decimal64Type`

```rust
struct Decimal64Type
```

**Implements**: `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::DecimalType`

**Derives**: Debug

**via `arrow_array::types::DecimalType`**

```rust
fn format_decimal(value: Self::Native, precision: u8, scale: i8) -> String
fn is_valid_decimal_precision(value: Self::Native, precision: u8) -> bool
fn validate_decimal_precision(num: i64, precision: u8, scale: i8) -> Result<(), ArrowError>
```

The decimal type for a Decimal64Array

---

## DurationMicrosecondType

`struct` · `arrow_array::types::DurationMicrosecondType`

Also reachable as `arrow::datatypes::DurationMicrosecondType`

```rust
struct DurationMicrosecondType
```

**Implements**: `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::ArrowTemporalType`, `arrow_cast::parse::Parser`

**Derives**: Debug

Elapsed time type: microseconds.

---

## DurationMillisecondType

`struct` · `arrow_array::types::DurationMillisecondType`

Also reachable as `arrow::datatypes::DurationMillisecondType`

```rust
struct DurationMillisecondType
```

**Implements**: `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::ArrowTemporalType`, `arrow_cast::parse::Parser`

**Derives**: Debug

Elapsed time type: milliseconds.

---

## DurationNanosecondType

`struct` · `arrow_array::types::DurationNanosecondType`

Also reachable as `arrow::datatypes::DurationNanosecondType`

```rust
struct DurationNanosecondType
```

**Implements**: `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::ArrowTemporalType`, `arrow_cast::parse::Parser`

**Derives**: Debug

Elapsed time type: nanoseconds.

---

## DurationSecondType

`struct` · `arrow_array::types::DurationSecondType`

Also reachable as `arrow::datatypes::DurationSecondType`

```rust
struct DurationSecondType
```

**Implements**: `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::ArrowTemporalType`, `arrow_cast::parse::Parser`

**Derives**: Debug

Elapsed time type: seconds.

---

## Float16Type

`struct` · `arrow_array::types::Float16Type`

Also reachable as `arrow::datatypes::Float16Type`

```rust
struct Float16Type
```

**Implements**: `arrow_array::types::ArrowPrimitiveType`, `arrow_cast::parse::Parser`

**Derives**: Debug

16-bit floating point number type.

---

## Float32Type

`struct` · `arrow_array::types::Float32Type`

Also reachable as `arrow::datatypes::Float32Type`

```rust
struct Float32Type
```

**Implements**: `arrow_array::types::ArrowPrimitiveType`, `arrow_cast::parse::Parser`, `datafusion_common::scalar::ScalarType`

**Derives**: Debug

32-bit floating point number type.

---

## Float64Type

`struct` · `arrow_array::types::Float64Type`

Also reachable as `arrow::datatypes::Float64Type`

```rust
struct Float64Type
```

**Implements**: `arrow_array::types::ArrowPrimitiveType`, `arrow_cast::parse::Parser`

**Derives**: Debug

64-bit floating point number type.

---

## GenericBinaryType

`struct` · `arrow_array::types::GenericBinaryType`

Also reachable as `arrow::datatypes::GenericBinaryType`

```rust
struct GenericBinaryType<O: OffsetSizeTrait>
```

**Implements**: `arrow_array::types::ByteArrayType`

**via `arrow_array::types::ByteArrayType`**

```rust
fn validate(offsets: &OffsetBuffer<Self::Offset>, values: &Buffer) -> Result<(), ArrowError>
```

[`ByteArrayType`] for binary arrays

---

## GenericStringType

`struct` · `arrow_array::types::GenericStringType`

Also reachable as `arrow::datatypes::GenericStringType`

```rust
struct GenericStringType<O: OffsetSizeTrait>
```

**Implements**: `arrow_array::types::ByteArrayType`

**via `arrow_array::types::ByteArrayType`**

```rust
fn validate(offsets: &OffsetBuffer<Self::Offset>, values: &Buffer) -> Result<(), ArrowError>
```

[`ByteArrayType`] for string arrays

---

## Int16Type

`struct` · `arrow_array::types::Int16Type`

Also reachable as `arrow::datatypes::Int16Type`

```rust
struct Int16Type
```

**Implements**: `arrow_array::types::ArrowDictionaryKeyType`, `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::RunEndIndexType`, `arrow_cast::parse::Parser`

**Derives**: Debug

Signed 16-bit integer type.

---

## Int32Type

`struct` · `arrow_array::types::Int32Type`

Also reachable as `arrow::datatypes::Int32Type`

```rust
struct Int32Type
```

**Implements**: `arrow_array::types::ArrowDictionaryKeyType`, `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::RunEndIndexType`, `arrow_cast::parse::Parser`

**Derives**: Debug

Signed 32-bit integer type.

---

## Int64Type

`struct` · `arrow_array::types::Int64Type`

Also reachable as `arrow::datatypes::Int64Type`

```rust
struct Int64Type
```

**Implements**: `arrow_array::types::ArrowDictionaryKeyType`, `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::RunEndIndexType`, `arrow_cast::parse::Parser`

**Derives**: Debug

Signed 64-bit integer type.

---

## Int8Type

`struct` · `arrow_array::types::Int8Type`

Also reachable as `arrow::datatypes::Int8Type`

```rust
struct Int8Type
```

**Implements**: `arrow_array::types::ArrowDictionaryKeyType`, `arrow_array::types::ArrowPrimitiveType`, `arrow_cast::parse::Parser`

**Derives**: Debug

A signed 8-bit integer type.

---

## IntervalDayTimeType

`struct` · `arrow_array::types::IntervalDayTimeType`

Also reachable as `arrow::datatypes::IntervalDayTimeType`

```rust
struct IntervalDayTimeType
```

**Implements**: `arrow_array::types::ArrowPrimitiveType`

**Derives**: Debug

**Methods** (2)

```rust
fn make_value(days: i32, milliseconds: i32) -> IntervalDayTime
fn to_parts(i: IntervalDayTime) -> (i32, i32)
```

“Calendar” interval type: days and milliseconds. See [`IntervalDayTime`] for more details.

---

## IntervalMonthDayNanoType

`struct` · `arrow_array::types::IntervalMonthDayNanoType`

Also reachable as `arrow::datatypes::IntervalMonthDayNanoType`

```rust
struct IntervalMonthDayNanoType
```

**Implements**: `arrow_array::types::ArrowPrimitiveType`

**Derives**: Debug

**Methods** (2)

```rust
fn make_value(months: i32, days: i32, nanoseconds: i64) -> IntervalMonthDayNano
fn to_parts(i: IntervalMonthDayNano) -> (i32, i32, i64)
```

“Calendar” interval type: months, days, and nanoseconds. See [`IntervalMonthDayNano`] for more details.

---

## IntervalYearMonthType

`struct` · `arrow_array::types::IntervalYearMonthType`

Also reachable as `arrow::datatypes::IntervalYearMonthType`

```rust
struct IntervalYearMonthType
```

**Implements**: `arrow_array::types::ArrowPrimitiveType`

**Derives**: Debug

**Methods** (2)

```rust
fn make_value(years: i32, months: i32) -> <IntervalYearMonthType as ArrowPrimitiveType>::Native
fn to_months(i: <IntervalYearMonthType as ArrowPrimitiveType>::Native) -> i32
```

32-bit “calendar” interval type: the number of whole months.

---

## StringViewType

`struct` · `arrow_array::types::StringViewType`

Also reachable as `arrow::datatypes::StringViewType`

```rust
struct StringViewType
```

**Implements**: `arrow_array::types::ByteViewType`

**Derives**: PartialEq, StructuralPartialEq

**via `arrow_array::types::ByteViewType`**

```rust
fn validate(views: &[u128], buffers: &[Buffer]) -> Result<(), ArrowError>
```

[`ByteViewType`] for string arrays

---

## Time32MillisecondType

`struct` · `arrow_array::types::Time32MillisecondType`

Also reachable as `arrow::datatypes::Time32MillisecondType`

```rust
struct Time32MillisecondType
```

**Implements**: `arrow::util::data_gen::RandomTemporalValue`, `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::ArrowTemporalType`, `arrow_cast::parse::Parser`

**Derives**: Debug

32-bit time type: the elapsed time since midnight in milliseconds.

---

## Time32SecondType

`struct` · `arrow_array::types::Time32SecondType`

Also reachable as `arrow::datatypes::Time32SecondType`

```rust
struct Time32SecondType
```

**Implements**: `arrow::util::data_gen::RandomTemporalValue`, `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::ArrowTemporalType`, `arrow_cast::parse::Parser`

**Derives**: Debug

32-bit time type: the elapsed time since midnight in seconds.

---

## Time64MicrosecondType

`struct` · `arrow_array::types::Time64MicrosecondType`

Also reachable as `arrow::datatypes::Time64MicrosecondType`

```rust
struct Time64MicrosecondType
```

**Implements**: `arrow::util::data_gen::RandomTemporalValue`, `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::ArrowTemporalType`, `arrow_cast::parse::Parser`

**Derives**: Debug

64-bit time type: the elapsed time since midnight in microseconds.

---

## Time64NanosecondType

`struct` · `arrow_array::types::Time64NanosecondType`

Also reachable as `arrow::datatypes::Time64NanosecondType`

```rust
struct Time64NanosecondType
```

**Implements**: `arrow::util::data_gen::RandomTemporalValue`, `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::ArrowTemporalType`, `arrow_cast::parse::Parser`

**Derives**: Debug

64-bit time type: the elapsed time since midnight in nanoseconds.

---

## TimestampMicrosecondType

`struct` · `arrow_array::types::TimestampMicrosecondType`

Also reachable as `arrow::datatypes::TimestampMicrosecondType`

```rust
struct TimestampMicrosecondType
```

**Implements**: `arrow::util::data_gen::RandomTemporalValue`, `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::ArrowTemporalType`, `arrow_array::types::ArrowTimestampType`, `arrow_cast::parse::Parser`, `datafusion_common::scalar::ScalarType`

**Derives**: Debug

**Methods** (6)

```rust
fn add_day_time(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn add_month_day_nano(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn add_year_months(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn subtract_day_time(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn subtract_month_day_nano(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn subtract_year_months(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

**via `arrow_array::types::ArrowTimestampType`**

```rust
fn from_datetime<Tz: TimeZone>(datetime: DateTime<Tz>) -> Option<i64>
fn make_value(naive: NaiveDateTime) -> Option<i64>
```

Timestamp microsecond type with an optional timezone.

---

## TimestampMillisecondType

`struct` · `arrow_array::types::TimestampMillisecondType`

Also reachable as `arrow::datatypes::TimestampMillisecondType`

```rust
struct TimestampMillisecondType
```

**Implements**: `arrow::util::data_gen::RandomTemporalValue`, `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::ArrowTemporalType`, `arrow_array::types::ArrowTimestampType`, `arrow_cast::parse::Parser`, `datafusion_common::scalar::ScalarType`

**Derives**: Debug

**Methods** (6)

```rust
fn add_day_time(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn add_month_day_nano(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn add_year_months(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn subtract_day_time(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn subtract_month_day_nano(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn subtract_year_months(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

**via `arrow_array::types::ArrowTimestampType`**

```rust
fn from_datetime<Tz: TimeZone>(datetime: DateTime<Tz>) -> Option<i64>
fn make_value(naive: NaiveDateTime) -> Option<i64>
```

Timestamp millisecond type with an optional timezone.

---

## TimestampNanosecondType

`struct` · `arrow_array::types::TimestampNanosecondType`

Also reachable as `arrow::datatypes::TimestampNanosecondType`

```rust
struct TimestampNanosecondType
```

**Implements**: `arrow::util::data_gen::RandomTemporalValue`, `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::ArrowTemporalType`, `arrow_array::types::ArrowTimestampType`, `arrow_cast::parse::Parser`, `datafusion_common::scalar::ScalarType`

**Derives**: Debug

**Methods** (6)

```rust
fn add_day_time(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn add_month_day_nano(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn add_year_months(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn subtract_day_time(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn subtract_month_day_nano(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn subtract_year_months(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

**via `arrow_array::types::ArrowTimestampType`**

```rust
fn from_datetime<Tz: TimeZone>(datetime: DateTime<Tz>) -> Option<i64>
fn make_value(naive: NaiveDateTime) -> Option<i64>
```

Timestamp nanosecond type with an optional timezone.

---

## TimestampSecondType

`struct` · `arrow_array::types::TimestampSecondType`

Also reachable as `arrow::datatypes::TimestampSecondType`

```rust
struct TimestampSecondType
```

**Implements**: `arrow::util::data_gen::RandomTemporalValue`, `arrow_array::types::ArrowPrimitiveType`, `arrow_array::types::ArrowTemporalType`, `arrow_array::types::ArrowTimestampType`, `arrow_cast::parse::Parser`, `datafusion_common::scalar::ScalarType`

**Derives**: Debug

**Methods** (6)

```rust
fn add_day_time(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn add_month_day_nano(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn add_year_months(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn subtract_day_time(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn subtract_month_day_nano(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
fn subtract_year_months(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

**via `arrow_array::types::ArrowTimestampType`**

```rust
fn from_datetime<Tz: TimeZone>(datetime: DateTime<Tz>) -> Option<i64>
fn make_value(naive: NaiveDateTime) -> Option<i64>
```

Timestamp second type with an optional timezone.

---

## UInt16Type

`struct` · `arrow_array::types::UInt16Type`

Also reachable as `arrow::datatypes::UInt16Type`

```rust
struct UInt16Type
```

**Implements**: `arrow_array::types::ArrowDictionaryKeyType`, `arrow_array::types::ArrowPrimitiveType`, `arrow_cast::parse::Parser`

**Derives**: Debug

Unsigned 16-bit integer type.

---

## UInt32Type

`struct` · `arrow_array::types::UInt32Type`

Also reachable as `arrow::datatypes::UInt32Type`

```rust
struct UInt32Type
```

**Implements**: `arrow_array::types::ArrowDictionaryKeyType`, `arrow_array::types::ArrowPrimitiveType`, `arrow_cast::parse::Parser`

**Derives**: Debug

Unsigned 32-bit integer type.

---

## UInt64Type

`struct` · `arrow_array::types::UInt64Type`

Also reachable as `arrow::datatypes::UInt64Type`

```rust
struct UInt64Type
```

**Implements**: `arrow_array::types::ArrowDictionaryKeyType`, `arrow_array::types::ArrowPrimitiveType`, `arrow_cast::parse::Parser`

**Derives**: Debug

Unsigned 64-bit integer type.

---

## UInt8Type

`struct` · `arrow_array::types::UInt8Type`

Also reachable as `arrow::datatypes::UInt8Type`

```rust
struct UInt8Type
```

**Implements**: `arrow_array::types::ArrowDictionaryKeyType`, `arrow_array::types::ArrowPrimitiveType`, `arrow_cast::parse::Parser`

**Derives**: Debug

Unsigned 8-bit integer type.

---

## ArrowDictionaryKeyType

`trait` · `arrow_array::types::ArrowDictionaryKeyType`

Also reachable as `arrow::datatypes::ArrowDictionaryKeyType`

```rust
trait ArrowDictionaryKeyType: ArrowPrimitiveType
```

**Implementors** (8)

- `arrow_array::types::Int16Type`
- `arrow_array::types::Int32Type`
- `arrow_array::types::Int64Type`
- `arrow_array::types::Int8Type`
- `arrow_array::types::UInt16Type`
- `arrow_array::types::UInt32Type`
- `arrow_array::types::UInt64Type`
- `arrow_array::types::UInt8Type`

A subtype of primitive type that represents legal dictionary keys.
See <https://arrow.apache.org/docs/format/Columnar.html>

---

## ArrowPrimitiveType

`trait` · `arrow_array::types::ArrowPrimitiveType`

Also reachable as `arrow::datatypes::ArrowPrimitiveType`

```rust
trait ArrowPrimitiveType: primitive::PrimitiveTypeSealed + 'static
```

**Implementors** (32)

- `arrow_array::types::Date32Type`
- `arrow_array::types::Date64Type`
- `arrow_array::types::Decimal128Type`
- `arrow_array::types::Decimal256Type`
- `arrow_array::types::Decimal32Type`
- `arrow_array::types::Decimal64Type`
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
- `arrow_array::types::IntervalDayTimeType`
- `arrow_array::types::IntervalMonthDayNanoType`
- `arrow_array::types::IntervalYearMonthType`
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

**Methods** (1)

```rust
fn default_value() -> Self::Native
```

Trait for [primitive values].

This trait bridges the dynamic-typed nature of Arrow
(via [`DataType`]) with the static-typed nature of rust types
([`ArrowNativeType`]) for all types that implement [`ArrowNativeType`].

[primitive values]: https://arrow.apache.org/docs/format/Columnar.html#fixed-size-primitive-layout
[`ArrowNativeType`]: arrow_buffer::ArrowNativeType

---

## ArrowTemporalType

`trait` · `arrow_array::types::ArrowTemporalType`

Also reachable as `arrow::datatypes::ArrowTemporalType`

```rust
trait ArrowTemporalType: ArrowPrimitiveType
```

**Implementors** (14)

- `arrow_array::types::Date32Type`
- `arrow_array::types::Date64Type`
- `arrow_array::types::DurationMicrosecondType`
- `arrow_array::types::DurationMillisecondType`
- `arrow_array::types::DurationNanosecondType`
- `arrow_array::types::DurationSecondType`
- `arrow_array::types::Time32MillisecondType`
- `arrow_array::types::Time32SecondType`
- `arrow_array::types::Time64MicrosecondType`
- `arrow_array::types::Time64NanosecondType`
- `arrow_array::types::TimestampMicrosecondType`
- `arrow_array::types::TimestampMillisecondType`
- `arrow_array::types::TimestampNanosecondType`
- `arrow_array::types::TimestampSecondType`

A subtype of primitive type that represents temporal values.

---

## ArrowTimestampType

`trait` · `arrow_array::types::ArrowTimestampType`

Also reachable as `arrow::datatypes::ArrowTimestampType`

```rust
trait ArrowTimestampType: ArrowTemporalType<Native = i64>
```

**Implementors** (4)

- `arrow_array::types::TimestampMicrosecondType`
- `arrow_array::types::TimestampMillisecondType`
- `arrow_array::types::TimestampNanosecondType`
- `arrow_array::types::TimestampSecondType`

**Methods** (3)

```rust
fn from_datetime<Tz: TimeZone>(datetime: DateTime<Tz>) -> Option<i64>
fn from_naive_datetime(naive: NaiveDateTime, tz: Option<&Tz>) -> Option<i64>
fn make_value(naive: NaiveDateTime) -> Option<i64>
```

A timestamp type allows us to create array builders that take a timestamp.

---

## ByteArrayType

`trait` · `arrow_array::types::ByteArrayType`

Also reachable as `arrow::datatypes::ByteArrayType`

```rust
trait ByteArrayType: 'static + Send + Sync + bytes::ByteArrayTypeSealed
```

**Implementors** (2)

- `arrow_array::types::GenericBinaryType`
- `arrow_array::types::GenericStringType`

**Methods** (1)

```rust
fn validate(offsets: &OffsetBuffer<Self::Offset>, values: &Buffer) -> Result<(), ArrowError>
```

A trait over the variable-size byte array types

See [Variable Size Binary Layout](https://arrow.apache.org/docs/format/Columnar.html#variable-size-binary-layout)

---

## ByteViewType

`trait` · `arrow_array::types::ByteViewType`

Also reachable as `arrow::datatypes::ByteViewType`

```rust
trait ByteViewType: byte_view::Sealed + 'static + PartialEq + Send + Sync
```

**Implementors** (2)

- `arrow_array::types::BinaryViewType`
- `arrow_array::types::StringViewType`

**Methods** (1)

```rust
fn validate(views: &[u128], buffers: &[Buffer]) -> Result<(), ArrowError>
```

A trait over the variable length bytes view array types

---

## DecimalType

`trait` · `arrow_array::types::DecimalType`

Also reachable as `arrow::datatypes::DecimalType`

```rust
trait DecimalType: 'static + Send + Sync + ArrowPrimitiveType + decimal::DecimalTypeSealed
```

**Implementors** (4)

- `arrow_array::types::Decimal128Type`
- `arrow_array::types::Decimal256Type`
- `arrow_array::types::Decimal32Type`
- `arrow_array::types::Decimal64Type`

**Methods** (3)

```rust
fn format_decimal(value: Self::Native, precision: u8, scale: i8) -> String
fn is_valid_decimal_precision(value: Self::Native, precision: u8) -> bool
fn validate_decimal_precision(value: Self::Native, precision: u8, scale: i8) -> Result<(), ArrowError>
```

A trait over the decimal types, used by [`PrimitiveArray`] to provide a generic
implementation across the various decimal types

Implemented by [`Decimal32Type`], [`Decimal64Type`], [`Decimal128Type`] and [`Decimal256Type`]
for [`Decimal32Array`], [`Decimal64Array`], [`Decimal128Array`] and [`Decimal256Array`] respectively

[`PrimitiveArray`]: crate::array::PrimitiveArray
[`Decimal32Array`]: crate::array::Decimal32Array
[`Decimal64Array`]: crate::array::Decimal64Array
[`Decimal128Array`]: crate::array::Decimal128Array
[`Decimal256Array`]: crate::array::Decimal256Array

---

## RunEndIndexType

`trait` · `arrow_array::types::RunEndIndexType`

Also reachable as `arrow::datatypes::RunEndIndexType`

```rust
trait RunEndIndexType: ArrowPrimitiveType
```

**Implementors** (3)

- `arrow_array::types::Int16Type`
- `arrow_array::types::Int32Type`
- `arrow_array::types::Int64Type`

A subtype of primitive type that is used as run-ends index
in `RunArray`.
See <https://arrow.apache.org/docs/format/Columnar.html>

---

## BinaryType

`type_alias` · `arrow_array::types::BinaryType`

Also reachable as `arrow::datatypes::BinaryType`

```rust
type BinaryType = GenericBinaryType<i32>
```

An arrow binary array with i32 offsets

---

## LargeBinaryType

`type_alias` · `arrow_array::types::LargeBinaryType`

Also reachable as `arrow::datatypes::LargeBinaryType`

```rust
type LargeBinaryType = GenericBinaryType<i64>
```

An arrow binary array with i64 offsets

---

## LargeUtf8Type

`type_alias` · `arrow_array::types::LargeUtf8Type`

Also reachable as `arrow::datatypes::LargeUtf8Type`

```rust
type LargeUtf8Type = GenericStringType<i64>
```

An arrow utf8 array with i64 offsets

---

## Utf8Type

`type_alias` · `arrow_array::types::Utf8Type`

Also reachable as `arrow::datatypes::Utf8Type`

```rust
type Utf8Type = GenericStringType<i32>
```

An arrow utf8 array with i32 offsets

---
