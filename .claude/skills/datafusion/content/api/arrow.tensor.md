# `arrow::tensor`

Crate `arrow` · 34 public items · structured records in [`model/arrow.tensor.json`](../model/arrow.tensor.json)

## Tensor

`struct` · `arrow::tensor::Tensor`

```rust
struct Tensor<'a, T: ArrowPrimitiveType>
```

**Derives**: Debug

**Methods** (14)

```rust
fn data(&self) -> &Buffer
fn data_type(&self) -> &DataType
fn dim_name(&self, i: usize) -> Option<&'a str>
fn is_column_major(&self) -> Result<bool>
fn is_contiguous(&self) -> Result<bool>
fn is_row_major(&self) -> Result<bool>
fn names(&self) -> Option<&Vec<&'a str>>
fn ndim(&self) -> usize
fn new_column_major(buffer: Buffer, shape: Option<Vec<usize>>, names: Option<Vec<&'a str>>) -> Result<Self>
fn new_row_major(buffer: Buffer, shape: Option<Vec<usize>>, names: Option<Vec<&'a str>>) -> Result<Self>
fn shape(&self) -> Option<&Vec<usize>>
fn size(&self) -> usize
fn strides(&self) -> Option<&Vec<usize>>
fn try_new(buffer: Buffer, shape: Option<Vec<usize>>, strides: Option<Vec<usize>>, names: Option<Vec<&'a str>>) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.Tensor.md).


Tensor of primitive types

---

## BooleanTensor

`type_alias` · `arrow::tensor::BooleanTensor`

```rust
type BooleanTensor<'a> = Tensor<'a, BooleanType>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.BooleanTensor.md).


[Tensor] of type [BooleanType]

---

## Date32Tensor

`type_alias` · `arrow::tensor::Date32Tensor`

```rust
type Date32Tensor<'a> = Tensor<'a, Date32Type>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.Date32Tensor.md).


[Tensor] of type [Int8Type]

---

## Date64Tensor

`type_alias` · `arrow::tensor::Date64Tensor`

```rust
type Date64Tensor<'a> = Tensor<'a, Date64Type>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.Date64Tensor.md).


[Tensor] of type [Int16Type]

---

## Decimal128Tensor

`type_alias` · `arrow::tensor::Decimal128Tensor`

```rust
type Decimal128Tensor<'a> = Tensor<'a, Decimal128Type>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.Decimal128Tensor.md).


[Tensor] of type [Decimal128Type]

---

## Decimal256Tensor

`type_alias` · `arrow::tensor::Decimal256Tensor`

```rust
type Decimal256Tensor<'a> = Tensor<'a, Decimal256Type>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.Decimal256Tensor.md).


[Tensor] of type [Decimal256Type]

---

## Decimal32Tensor

`type_alias` · `arrow::tensor::Decimal32Tensor`

```rust
type Decimal32Tensor<'a> = Tensor<'a, Decimal32Type>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.Decimal32Tensor.md).


[Tensor] of type [Decimal32Type]

---

## Decimal64Tensor

`type_alias` · `arrow::tensor::Decimal64Tensor`

```rust
type Decimal64Tensor<'a> = Tensor<'a, Decimal64Type>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.Decimal64Tensor.md).


[Tensor] of type [Decimal64Type]

---

## DurationMicrosecondTensor

`type_alias` · `arrow::tensor::DurationMicrosecondTensor`

```rust
type DurationMicrosecondTensor<'a> = Tensor<'a, DurationMicrosecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.DurationMicrosecondTensor.md).


[Tensor] of type [DurationMicrosecondType]

---

## DurationMillisecondTensor

`type_alias` · `arrow::tensor::DurationMillisecondTensor`

```rust
type DurationMillisecondTensor<'a> = Tensor<'a, DurationMillisecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.DurationMillisecondTensor.md).


[Tensor] of type [DurationMillisecondType]

---

## DurationNanosecondTensor

`type_alias` · `arrow::tensor::DurationNanosecondTensor`

```rust
type DurationNanosecondTensor<'a> = Tensor<'a, DurationNanosecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.DurationNanosecondTensor.md).


[Tensor] of type [DurationNanosecondType]

---

## DurationSecondTensor

`type_alias` · `arrow::tensor::DurationSecondTensor`

```rust
type DurationSecondTensor<'a> = Tensor<'a, DurationSecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.DurationSecondTensor.md).


[Tensor] of type [DurationSecondType]

---

## Float16Tensor

`type_alias` · `arrow::tensor::Float16Tensor`

```rust
type Float16Tensor<'a> = Tensor<'a, Float16Type>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.Float16Tensor.md).


[Tensor] of type [Float16Type]

---

## Float32Tensor

`type_alias` · `arrow::tensor::Float32Tensor`

```rust
type Float32Tensor<'a> = Tensor<'a, Float32Type>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.Float32Tensor.md).


[Tensor] of type [Float32Type]

---

## Float64Tensor

`type_alias` · `arrow::tensor::Float64Tensor`

```rust
type Float64Tensor<'a> = Tensor<'a, Float64Type>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.Float64Tensor.md).


[Tensor] of type [Float64Type]

---

## Int16Tensor

`type_alias` · `arrow::tensor::Int16Tensor`

```rust
type Int16Tensor<'a> = Tensor<'a, Int16Type>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.Int16Tensor.md).


[Tensor] of type [Int16Type]

---

## Int32Tensor

`type_alias` · `arrow::tensor::Int32Tensor`

```rust
type Int32Tensor<'a> = Tensor<'a, Int32Type>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.Int32Tensor.md).


[Tensor] of type [Int32Type]

---

## Int64Tensor

`type_alias` · `arrow::tensor::Int64Tensor`

```rust
type Int64Tensor<'a> = Tensor<'a, Int64Type>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.Int64Tensor.md).


[Tensor] of type [Int64Type]

---

## Int8Tensor

`type_alias` · `arrow::tensor::Int8Tensor`

```rust
type Int8Tensor<'a> = Tensor<'a, Int8Type>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.Int8Tensor.md).


[Tensor] of type [Int8Type]

---

## IntervalDayTimeTensor

`type_alias` · `arrow::tensor::IntervalDayTimeTensor`

```rust
type IntervalDayTimeTensor<'a> = Tensor<'a, IntervalDayTimeType>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.IntervalDayTimeTensor.md).


[Tensor] of type [IntervalDayTimeType]

---

## IntervalMonthDayNanoTensor

`type_alias` · `arrow::tensor::IntervalMonthDayNanoTensor`

```rust
type IntervalMonthDayNanoTensor<'a> = Tensor<'a, IntervalMonthDayNanoType>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.IntervalMonthDayNanoTensor.md).


[Tensor] of type [IntervalMonthDayNanoType]

---

## IntervalYearMonthTensor

`type_alias` · `arrow::tensor::IntervalYearMonthTensor`

```rust
type IntervalYearMonthTensor<'a> = Tensor<'a, IntervalYearMonthType>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.IntervalYearMonthTensor.md).


[Tensor] of type [IntervalYearMonthType]

---

## Time32MillisecondTensor

`type_alias` · `arrow::tensor::Time32MillisecondTensor`

```rust
type Time32MillisecondTensor<'a> = Tensor<'a, Time32MillisecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.Time32MillisecondTensor.md).


[Tensor] of type [Time32MillisecondType]

---

## Time32SecondTensor

`type_alias` · `arrow::tensor::Time32SecondTensor`

```rust
type Time32SecondTensor<'a> = Tensor<'a, Time32SecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.Time32SecondTensor.md).


[Tensor] of type [Time32SecondType]

---

## Time64MicrosecondTensor

`type_alias` · `arrow::tensor::Time64MicrosecondTensor`

```rust
type Time64MicrosecondTensor<'a> = Tensor<'a, Time64MicrosecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.Time64MicrosecondTensor.md).


[Tensor] of type [Time64MicrosecondType]

---

## Time64NanosecondTensor

`type_alias` · `arrow::tensor::Time64NanosecondTensor`

```rust
type Time64NanosecondTensor<'a> = Tensor<'a, Time64NanosecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.Time64NanosecondTensor.md).


[Tensor] of type [Time64NanosecondType]

---

## TimestampMicrosecondTensor

`type_alias` · `arrow::tensor::TimestampMicrosecondTensor`

```rust
type TimestampMicrosecondTensor<'a> = Tensor<'a, TimestampMicrosecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.TimestampMicrosecondTensor.md).


[Tensor] of type [TimestampMicrosecondType]

---

## TimestampMillisecondTensor

`type_alias` · `arrow::tensor::TimestampMillisecondTensor`

```rust
type TimestampMillisecondTensor<'a> = Tensor<'a, TimestampMillisecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.TimestampMillisecondTensor.md).


[Tensor] of type [TimestampMillisecondType]

---

## TimestampNanosecondTensor

`type_alias` · `arrow::tensor::TimestampNanosecondTensor`

```rust
type TimestampNanosecondTensor<'a> = Tensor<'a, TimestampNanosecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.TimestampNanosecondTensor.md).


[Tensor] of type [TimestampNanosecondType]

---

## TimestampSecondTensor

`type_alias` · `arrow::tensor::TimestampSecondTensor`

```rust
type TimestampSecondTensor<'a> = Tensor<'a, TimestampSecondType>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.TimestampSecondTensor.md).


[Tensor] of type [TimestampSecondType]

---

## UInt16Tensor

`type_alias` · `arrow::tensor::UInt16Tensor`

```rust
type UInt16Tensor<'a> = Tensor<'a, UInt16Type>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.UInt16Tensor.md).


[Tensor] of type [UInt16Type]

---

## UInt32Tensor

`type_alias` · `arrow::tensor::UInt32Tensor`

```rust
type UInt32Tensor<'a> = Tensor<'a, UInt32Type>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.UInt32Tensor.md).


[Tensor] of type [UInt32Type]

---

## UInt64Tensor

`type_alias` · `arrow::tensor::UInt64Tensor`

```rust
type UInt64Tensor<'a> = Tensor<'a, UInt64Type>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.UInt64Tensor.md).


[Tensor] of type [UInt64Type]

---

## UInt8Tensor

`type_alias` · `arrow::tensor::UInt8Tensor`

```rust
type UInt8Tensor<'a> = Tensor<'a, UInt8Type>
```

[Full member, field, variant and typed contracts](../operations/arrow.tensor.UInt8Tensor.md).


[Tensor] of type [UInt8Type]

---
