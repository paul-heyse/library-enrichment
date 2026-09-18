# `arrow_array::builder::primitive_builder`

Crate `arrow-array` · 33 public items · structured records in [`model/arrow_array.builder.primitive_builder.json`](../model/arrow_array.builder.primitive_builder.json)

## PrimitiveBuilder

`struct` · `arrow_array::builder::primitive_builder::PrimitiveBuilder`

```rust
struct PrimitiveBuilder<T: ArrowPrimitiveType>
```

**Implements**: `arrow_array::builder::ArrayBuilder`, `core::iter::traits::collect::Extend`

**Derives**: Debug, Default

**Methods** (26)

```rust
fn append_array(&mut self, array: &PrimitiveArray<T>)
fn append_null(&mut self)
fn append_nulls(&mut self, n: usize)
fn append_option(&mut self, v: Option<T::Native>)
fn append_slice(&mut self, v: &[T::Native])
unsafe fn append_trusted_len_iter(&mut self, iter: impl IntoIterator<Item = T::Native>)
fn append_value(&mut self, v: T::Native)
fn append_value_n(&mut self, v: T::Native, n: usize)
fn append_values(&mut self, values: &[T::Native], is_valid: &[bool])
fn capacity(&self) -> usize
fn extend_from_iter_option<I: IntoIterator<Item = Option<T::Native>>>(&mut self, iter: I)
fn finish(&mut self) -> PrimitiveArray<T>
fn finish_cloned(&self) -> PrimitiveArray<T>
fn new() -> Self
fn new_from_buffer(values_buffer: MutableBuffer, null_buffer: Option<MutableBuffer>) -> Self
fn slices_mut(&mut self) -> (&mut [T::Native], Option<&mut [u8]>)
fn validity_capacity(&self) -> usize
fn validity_slice(&self) -> Option<&[u8]>
fn validity_slice_mut(&mut self) -> Option<&mut [u8]>
fn values_slice(&self) -> &[T::Native]
fn values_slice_mut(&mut self) -> &mut [T::Native]
fn with_capacity(capacity: usize) -> Self
fn with_data_type(self, data_type: DataType) -> Self
fn with_precision_and_scale(self, precision: u8, scale: i8) -> Result<Self, ArrowError>
fn with_timezone(self, timezone: impl Into<Arc<str>>) -> Self
fn with_timezone_opt<S: Into<Arc<str>>>(self, timezone: Option<S>) -> Self
```

**via `arrow_array::builder::ArrayBuilder`**

```rust
fn as_any(&self) -> &dyn Any
fn as_any_mut(&mut self) -> &mut dyn Any
fn finish(&mut self) -> ArrayRef
fn finish_cloned(&self) -> ArrayRef
fn into_box_any(Box<self>) -> Box<dyn Any>
fn len(&self) -> usize
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: IntoIterator<Item = Option<P::Native>>>(&mut self, iter: T)
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.PrimitiveBuilder.md).


Builder for [`PrimitiveArray`]

---

## Date32Builder

`type_alias` · `arrow_array::builder::primitive_builder::Date32Builder`

```rust
type Date32Builder = PrimitiveBuilder<Date32Type>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.Date32Builder.md).


A 32-bit date array builder.

---

## Date64Builder

`type_alias` · `arrow_array::builder::primitive_builder::Date64Builder`

```rust
type Date64Builder = PrimitiveBuilder<Date64Type>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.Date64Builder.md).


A 64-bit date array builder.

---

## Decimal128Builder

`type_alias` · `arrow_array::builder::primitive_builder::Decimal128Builder`

```rust
type Decimal128Builder = PrimitiveBuilder<Decimal128Type>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.Decimal128Builder.md).


A decimal 128 array builder

---

## Decimal256Builder

`type_alias` · `arrow_array::builder::primitive_builder::Decimal256Builder`

```rust
type Decimal256Builder = PrimitiveBuilder<Decimal256Type>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.Decimal256Builder.md).


A decimal 256 array builder

---

## Decimal32Builder

`type_alias` · `arrow_array::builder::primitive_builder::Decimal32Builder`

```rust
type Decimal32Builder = PrimitiveBuilder<Decimal32Type>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.Decimal32Builder.md).


A decimal 32 array builder

---

## Decimal64Builder

`type_alias` · `arrow_array::builder::primitive_builder::Decimal64Builder`

```rust
type Decimal64Builder = PrimitiveBuilder<Decimal64Type>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.Decimal64Builder.md).


A decimal 64 array builder

---

## DurationMicrosecondBuilder

`type_alias` · `arrow_array::builder::primitive_builder::DurationMicrosecondBuilder`

```rust
type DurationMicrosecondBuilder = PrimitiveBuilder<DurationMicrosecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.DurationMicrosecondBuilder.md).


An elapsed time in microseconds array builder.

---

## DurationMillisecondBuilder

`type_alias` · `arrow_array::builder::primitive_builder::DurationMillisecondBuilder`

```rust
type DurationMillisecondBuilder = PrimitiveBuilder<DurationMillisecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.DurationMillisecondBuilder.md).


An elapsed time in milliseconds array builder.

---

## DurationNanosecondBuilder

`type_alias` · `arrow_array::builder::primitive_builder::DurationNanosecondBuilder`

```rust
type DurationNanosecondBuilder = PrimitiveBuilder<DurationNanosecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.DurationNanosecondBuilder.md).


An elapsed time in nanoseconds array builder.

---

## DurationSecondBuilder

`type_alias` · `arrow_array::builder::primitive_builder::DurationSecondBuilder`

```rust
type DurationSecondBuilder = PrimitiveBuilder<DurationSecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.DurationSecondBuilder.md).


An elapsed time in seconds array builder.

---

## Float16Builder

`type_alias` · `arrow_array::builder::primitive_builder::Float16Builder`

```rust
type Float16Builder = PrimitiveBuilder<Float16Type>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.Float16Builder.md).


A 16-bit floating point array builder.

---

## Float32Builder

`type_alias` · `arrow_array::builder::primitive_builder::Float32Builder`

```rust
type Float32Builder = PrimitiveBuilder<Float32Type>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.Float32Builder.md).


A 32-bit floating point array builder.

---

## Float64Builder

`type_alias` · `arrow_array::builder::primitive_builder::Float64Builder`

```rust
type Float64Builder = PrimitiveBuilder<Float64Type>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.Float64Builder.md).


A 64-bit floating point array builder.

---

## Int16Builder

`type_alias` · `arrow_array::builder::primitive_builder::Int16Builder`

```rust
type Int16Builder = PrimitiveBuilder<Int16Type>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.Int16Builder.md).


A signed 16-bit integer array builder.

---

## Int32Builder

`type_alias` · `arrow_array::builder::primitive_builder::Int32Builder`

```rust
type Int32Builder = PrimitiveBuilder<Int32Type>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.Int32Builder.md).


A signed 32-bit integer array builder.

---

## Int64Builder

`type_alias` · `arrow_array::builder::primitive_builder::Int64Builder`

```rust
type Int64Builder = PrimitiveBuilder<Int64Type>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.Int64Builder.md).


A signed 64-bit integer array builder.

---

## Int8Builder

`type_alias` · `arrow_array::builder::primitive_builder::Int8Builder`

```rust
type Int8Builder = PrimitiveBuilder<Int8Type>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.Int8Builder.md).


A signed 8-bit integer array builder.

---

## IntervalDayTimeBuilder

`type_alias` · `arrow_array::builder::primitive_builder::IntervalDayTimeBuilder`

```rust
type IntervalDayTimeBuilder = PrimitiveBuilder<IntervalDayTimeType>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.IntervalDayTimeBuilder.md).


A “calendar” interval in days and milliseconds array builder.

---

## IntervalMonthDayNanoBuilder

`type_alias` · `arrow_array::builder::primitive_builder::IntervalMonthDayNanoBuilder`

```rust
type IntervalMonthDayNanoBuilder = PrimitiveBuilder<IntervalMonthDayNanoType>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.IntervalMonthDayNanoBuilder.md).


A “calendar” interval in months, days, and nanoseconds array builder.

---

## IntervalYearMonthBuilder

`type_alias` · `arrow_array::builder::primitive_builder::IntervalYearMonthBuilder`

```rust
type IntervalYearMonthBuilder = PrimitiveBuilder<IntervalYearMonthType>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.IntervalYearMonthBuilder.md).


A “calendar” interval in months array builder.

---

## Time32MillisecondBuilder

`type_alias` · `arrow_array::builder::primitive_builder::Time32MillisecondBuilder`

```rust
type Time32MillisecondBuilder = PrimitiveBuilder<Time32MillisecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.Time32MillisecondBuilder.md).


A 32-bit elaspsed time in milliseconds array builder.

---

## Time32SecondBuilder

`type_alias` · `arrow_array::builder::primitive_builder::Time32SecondBuilder`

```rust
type Time32SecondBuilder = PrimitiveBuilder<Time32SecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.Time32SecondBuilder.md).


A 32-bit elaspsed time in seconds array builder.

---

## Time64MicrosecondBuilder

`type_alias` · `arrow_array::builder::primitive_builder::Time64MicrosecondBuilder`

```rust
type Time64MicrosecondBuilder = PrimitiveBuilder<Time64MicrosecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.Time64MicrosecondBuilder.md).


A 64-bit elaspsed time in microseconds array builder.

---

## Time64NanosecondBuilder

`type_alias` · `arrow_array::builder::primitive_builder::Time64NanosecondBuilder`

```rust
type Time64NanosecondBuilder = PrimitiveBuilder<Time64NanosecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.Time64NanosecondBuilder.md).


A 64-bit elaspsed time in nanoseconds array builder.

---

## TimestampMicrosecondBuilder

`type_alias` · `arrow_array::builder::primitive_builder::TimestampMicrosecondBuilder`

```rust
type TimestampMicrosecondBuilder = PrimitiveBuilder<TimestampMicrosecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.TimestampMicrosecondBuilder.md).


A timestamp microsecond array builder.

---

## TimestampMillisecondBuilder

`type_alias` · `arrow_array::builder::primitive_builder::TimestampMillisecondBuilder`

```rust
type TimestampMillisecondBuilder = PrimitiveBuilder<TimestampMillisecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.TimestampMillisecondBuilder.md).


A timestamp millisecond array builder.

---

## TimestampNanosecondBuilder

`type_alias` · `arrow_array::builder::primitive_builder::TimestampNanosecondBuilder`

```rust
type TimestampNanosecondBuilder = PrimitiveBuilder<TimestampNanosecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.TimestampNanosecondBuilder.md).


A timestamp nanosecond array builder.

---

## TimestampSecondBuilder

`type_alias` · `arrow_array::builder::primitive_builder::TimestampSecondBuilder`

```rust
type TimestampSecondBuilder = PrimitiveBuilder<TimestampSecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.TimestampSecondBuilder.md).


A timestamp second array builder.

---

## UInt16Builder

`type_alias` · `arrow_array::builder::primitive_builder::UInt16Builder`

```rust
type UInt16Builder = PrimitiveBuilder<UInt16Type>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.UInt16Builder.md).


An usigned 16-bit integer array builder.

---

## UInt32Builder

`type_alias` · `arrow_array::builder::primitive_builder::UInt32Builder`

```rust
type UInt32Builder = PrimitiveBuilder<UInt32Type>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.UInt32Builder.md).


An usigned 32-bit integer array builder.

---

## UInt64Builder

`type_alias` · `arrow_array::builder::primitive_builder::UInt64Builder`

```rust
type UInt64Builder = PrimitiveBuilder<UInt64Type>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.UInt64Builder.md).


An usigned 64-bit integer array builder.

---

## UInt8Builder

`type_alias` · `arrow_array::builder::primitive_builder::UInt8Builder`

```rust
type UInt8Builder = PrimitiveBuilder<UInt8Type>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.primitive_builder.UInt8Builder.md).


An usigned 8-bit integer array builder.

---
