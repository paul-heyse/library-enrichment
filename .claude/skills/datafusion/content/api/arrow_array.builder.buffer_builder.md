# `arrow_array::builder::buffer_builder`

Crate `arrow-array` · 32 public items · structured records in [`model/arrow_array.builder.buffer_builder.json`](../model/arrow_array.builder.buffer_builder.json)

## Date32BufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::Date32BufferBuilder`

```rust
type Date32BufferBuilder = BufferBuilder<<Date32Type as ArrowPrimitiveType>::Native>
```

Buffer builder for 32-bit date type.

---

## Date64BufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::Date64BufferBuilder`

```rust
type Date64BufferBuilder = BufferBuilder<<Date64Type as ArrowPrimitiveType>::Native>
```

Buffer builder for 64-bit date type.

---

## Decimal128BufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::Decimal128BufferBuilder`

```rust
type Decimal128BufferBuilder = BufferBuilder<<Decimal128Type as ArrowPrimitiveType>::Native>
```

Buffer builder for 128-bit decimal type.

---

## Decimal256BufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::Decimal256BufferBuilder`

```rust
type Decimal256BufferBuilder = BufferBuilder<<Decimal256Type as ArrowPrimitiveType>::Native>
```

Buffer builder for 256-bit decimal type.

---

## Decimal32BufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::Decimal32BufferBuilder`

```rust
type Decimal32BufferBuilder = BufferBuilder<<Decimal32Type as ArrowPrimitiveType>::Native>
```

Buffer builder for 32-bit decimal type.

---

## Decimal64BufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::Decimal64BufferBuilder`

```rust
type Decimal64BufferBuilder = BufferBuilder<<Decimal64Type as ArrowPrimitiveType>::Native>
```

Buffer builder for 64-bit decimal type.

---

## DurationMicrosecondBufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::DurationMicrosecondBufferBuilder`

```rust
type DurationMicrosecondBufferBuilder = BufferBuilder<<DurationMicrosecondType as ArrowPrimitiveType>::Native>
```

Buffer builder for elaspsed time of microseconds unit.

---

## DurationMillisecondBufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::DurationMillisecondBufferBuilder`

```rust
type DurationMillisecondBufferBuilder = BufferBuilder<<DurationMillisecondType as ArrowPrimitiveType>::Native>
```

Buffer builder for elaspsed time of milliseconds unit.

---

## DurationNanosecondBufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::DurationNanosecondBufferBuilder`

```rust
type DurationNanosecondBufferBuilder = BufferBuilder<<DurationNanosecondType as ArrowPrimitiveType>::Native>
```

Buffer builder for elaspsed time of nanoseconds unit.

---

## DurationSecondBufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::DurationSecondBufferBuilder`

```rust
type DurationSecondBufferBuilder = BufferBuilder<<DurationSecondType as ArrowPrimitiveType>::Native>
```

Buffer builder for elaspsed time of second unit.

---

## Float16BufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::Float16BufferBuilder`

```rust
type Float16BufferBuilder = BufferBuilder<half::f16>
```

Buffer builder for 16-bit floating point type.

---

## Float32BufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::Float32BufferBuilder`

```rust
type Float32BufferBuilder = BufferBuilder<f32>
```

Buffer builder for 32-bit floating point type.

---

## Float64BufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::Float64BufferBuilder`

```rust
type Float64BufferBuilder = BufferBuilder<f64>
```

Buffer builder for 64-bit floating point type.

---

## Int16BufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::Int16BufferBuilder`

```rust
type Int16BufferBuilder = BufferBuilder<i16>
```

Buffer builder for signed 16-bit integer type.

---

## Int32BufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::Int32BufferBuilder`

```rust
type Int32BufferBuilder = BufferBuilder<i32>
```

Buffer builder for signed 32-bit integer type.

---

## Int64BufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::Int64BufferBuilder`

```rust
type Int64BufferBuilder = BufferBuilder<i64>
```

Buffer builder for signed 64-bit integer type.

---

## Int8BufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::Int8BufferBuilder`

```rust
type Int8BufferBuilder = BufferBuilder<i8>
```

Buffer builder for signed 8-bit integer type.

---

## IntervalDayTimeBufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::IntervalDayTimeBufferBuilder`

```rust
type IntervalDayTimeBufferBuilder = BufferBuilder<<IntervalDayTimeType as ArrowPrimitiveType>::Native>
```

Buffer builder for “calendar” interval in days and milliseconds.

---

## IntervalMonthDayNanoBufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::IntervalMonthDayNanoBufferBuilder`

```rust
type IntervalMonthDayNanoBufferBuilder = BufferBuilder<<IntervalMonthDayNanoType as ArrowPrimitiveType>::Native>
```

Buffer builder “calendar” interval in months, days, and nanoseconds.

---

## IntervalYearMonthBufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::IntervalYearMonthBufferBuilder`

```rust
type IntervalYearMonthBufferBuilder = BufferBuilder<<IntervalYearMonthType as ArrowPrimitiveType>::Native>
```

Buffer builder for “calendar” interval in months.

---

## Time32MillisecondBufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::Time32MillisecondBufferBuilder`

```rust
type Time32MillisecondBufferBuilder = BufferBuilder<<Time32MillisecondType as ArrowPrimitiveType>::Native>
```

Buffer builder for 32-bit elaspsed time since midnight of millisecond unit.

---

## Time32SecondBufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::Time32SecondBufferBuilder`

```rust
type Time32SecondBufferBuilder = BufferBuilder<<Time32SecondType as ArrowPrimitiveType>::Native>
```

Buffer builder for 32-bit elaspsed time since midnight of second unit.

---

## Time64MicrosecondBufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::Time64MicrosecondBufferBuilder`

```rust
type Time64MicrosecondBufferBuilder = BufferBuilder<<Time64MicrosecondType as ArrowPrimitiveType>::Native>
```

Buffer builder for 64-bit elaspsed time since midnight of microsecond unit.

---

## Time64NanosecondBufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::Time64NanosecondBufferBuilder`

```rust
type Time64NanosecondBufferBuilder = BufferBuilder<<Time64NanosecondType as ArrowPrimitiveType>::Native>
```

Buffer builder for 64-bit elaspsed time since midnight of nanosecond unit.

---

## TimestampMicrosecondBufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::TimestampMicrosecondBufferBuilder`

```rust
type TimestampMicrosecondBufferBuilder = BufferBuilder<<TimestampMicrosecondType as ArrowPrimitiveType>::Native>
```

Buffer builder for timestamp type of microsecond unit.

---

## TimestampMillisecondBufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::TimestampMillisecondBufferBuilder`

```rust
type TimestampMillisecondBufferBuilder = BufferBuilder<<TimestampMillisecondType as ArrowPrimitiveType>::Native>
```

Buffer builder for timestamp type of millisecond unit.

---

## TimestampNanosecondBufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::TimestampNanosecondBufferBuilder`

```rust
type TimestampNanosecondBufferBuilder = BufferBuilder<<TimestampNanosecondType as ArrowPrimitiveType>::Native>
```

Buffer builder for timestamp type of nanosecond unit.

---

## TimestampSecondBufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::TimestampSecondBufferBuilder`

```rust
type TimestampSecondBufferBuilder = BufferBuilder<<TimestampSecondType as ArrowPrimitiveType>::Native>
```

Buffer builder for timestamp type of second unit.

---

## UInt16BufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::UInt16BufferBuilder`

```rust
type UInt16BufferBuilder = BufferBuilder<u16>
```

Buffer builder for usigned 16-bit integer type.

---

## UInt32BufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::UInt32BufferBuilder`

```rust
type UInt32BufferBuilder = BufferBuilder<u32>
```

Buffer builder for usigned 32-bit integer type.

---

## UInt64BufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::UInt64BufferBuilder`

```rust
type UInt64BufferBuilder = BufferBuilder<u64>
```

Buffer builder for usigned 64-bit integer type.

---

## UInt8BufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::UInt8BufferBuilder`

```rust
type UInt8BufferBuilder = BufferBuilder<u8>
```

Buffer builder for usigned 8-bit integer type.

---
