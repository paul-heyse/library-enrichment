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

Tensor of primitive types

---

## BooleanTensor

`type_alias` · `arrow::tensor::BooleanTensor`

```rust
type BooleanTensor<'a> = Tensor<'a, BooleanType>
```

[Tensor] of type [BooleanType]

---

## Date32Tensor

`type_alias` · `arrow::tensor::Date32Tensor`

```rust
type Date32Tensor<'a> = Tensor<'a, Date32Type>
```

[Tensor] of type [Int8Type]

---

## Date64Tensor

`type_alias` · `arrow::tensor::Date64Tensor`

```rust
type Date64Tensor<'a> = Tensor<'a, Date64Type>
```

[Tensor] of type [Int16Type]

---

## Decimal128Tensor

`type_alias` · `arrow::tensor::Decimal128Tensor`

```rust
type Decimal128Tensor<'a> = Tensor<'a, Decimal128Type>
```

[Tensor] of type [Decimal128Type]

---

## Decimal256Tensor

`type_alias` · `arrow::tensor::Decimal256Tensor`

```rust
type Decimal256Tensor<'a> = Tensor<'a, Decimal256Type>
```

[Tensor] of type [Decimal256Type]

---

## Decimal32Tensor

`type_alias` · `arrow::tensor::Decimal32Tensor`

```rust
type Decimal32Tensor<'a> = Tensor<'a, Decimal32Type>
```

[Tensor] of type [Decimal32Type]

---

## Decimal64Tensor

`type_alias` · `arrow::tensor::Decimal64Tensor`

```rust
type Decimal64Tensor<'a> = Tensor<'a, Decimal64Type>
```

[Tensor] of type [Decimal64Type]

---

## DurationMicrosecondTensor

`type_alias` · `arrow::tensor::DurationMicrosecondTensor`

```rust
type DurationMicrosecondTensor<'a> = Tensor<'a, DurationMicrosecondType>
```

[Tensor] of type [DurationMicrosecondType]

---

## DurationMillisecondTensor

`type_alias` · `arrow::tensor::DurationMillisecondTensor`

```rust
type DurationMillisecondTensor<'a> = Tensor<'a, DurationMillisecondType>
```

[Tensor] of type [DurationMillisecondType]

---

## DurationNanosecondTensor

`type_alias` · `arrow::tensor::DurationNanosecondTensor`

```rust
type DurationNanosecondTensor<'a> = Tensor<'a, DurationNanosecondType>
```

[Tensor] of type [DurationNanosecondType]

---

## DurationSecondTensor

`type_alias` · `arrow::tensor::DurationSecondTensor`

```rust
type DurationSecondTensor<'a> = Tensor<'a, DurationSecondType>
```

[Tensor] of type [DurationSecondType]

---

## Float16Tensor

`type_alias` · `arrow::tensor::Float16Tensor`

```rust
type Float16Tensor<'a> = Tensor<'a, Float16Type>
```

[Tensor] of type [Float16Type]

---

## Float32Tensor

`type_alias` · `arrow::tensor::Float32Tensor`

```rust
type Float32Tensor<'a> = Tensor<'a, Float32Type>
```

[Tensor] of type [Float32Type]

---

## Float64Tensor

`type_alias` · `arrow::tensor::Float64Tensor`

```rust
type Float64Tensor<'a> = Tensor<'a, Float64Type>
```

[Tensor] of type [Float64Type]

---

## Int16Tensor

`type_alias` · `arrow::tensor::Int16Tensor`

```rust
type Int16Tensor<'a> = Tensor<'a, Int16Type>
```

[Tensor] of type [Int16Type]

---

## Int32Tensor

`type_alias` · `arrow::tensor::Int32Tensor`

```rust
type Int32Tensor<'a> = Tensor<'a, Int32Type>
```

[Tensor] of type [Int32Type]

---

## Int64Tensor

`type_alias` · `arrow::tensor::Int64Tensor`

```rust
type Int64Tensor<'a> = Tensor<'a, Int64Type>
```

[Tensor] of type [Int64Type]

---

## Int8Tensor

`type_alias` · `arrow::tensor::Int8Tensor`

```rust
type Int8Tensor<'a> = Tensor<'a, Int8Type>
```

[Tensor] of type [Int8Type]

---

## IntervalDayTimeTensor

`type_alias` · `arrow::tensor::IntervalDayTimeTensor`

```rust
type IntervalDayTimeTensor<'a> = Tensor<'a, IntervalDayTimeType>
```

[Tensor] of type [IntervalDayTimeType]

---

## IntervalMonthDayNanoTensor

`type_alias` · `arrow::tensor::IntervalMonthDayNanoTensor`

```rust
type IntervalMonthDayNanoTensor<'a> = Tensor<'a, IntervalMonthDayNanoType>
```

[Tensor] of type [IntervalMonthDayNanoType]

---

## IntervalYearMonthTensor

`type_alias` · `arrow::tensor::IntervalYearMonthTensor`

```rust
type IntervalYearMonthTensor<'a> = Tensor<'a, IntervalYearMonthType>
```

[Tensor] of type [IntervalYearMonthType]

---

## Time32MillisecondTensor

`type_alias` · `arrow::tensor::Time32MillisecondTensor`

```rust
type Time32MillisecondTensor<'a> = Tensor<'a, Time32MillisecondType>
```

[Tensor] of type [Time32MillisecondType]

---

## Time32SecondTensor

`type_alias` · `arrow::tensor::Time32SecondTensor`

```rust
type Time32SecondTensor<'a> = Tensor<'a, Time32SecondType>
```

[Tensor] of type [Time32SecondType]

---

## Time64MicrosecondTensor

`type_alias` · `arrow::tensor::Time64MicrosecondTensor`

```rust
type Time64MicrosecondTensor<'a> = Tensor<'a, Time64MicrosecondType>
```

[Tensor] of type [Time64MicrosecondType]

---

## Time64NanosecondTensor

`type_alias` · `arrow::tensor::Time64NanosecondTensor`

```rust
type Time64NanosecondTensor<'a> = Tensor<'a, Time64NanosecondType>
```

[Tensor] of type [Time64NanosecondType]

---

## TimestampMicrosecondTensor

`type_alias` · `arrow::tensor::TimestampMicrosecondTensor`

```rust
type TimestampMicrosecondTensor<'a> = Tensor<'a, TimestampMicrosecondType>
```

[Tensor] of type [TimestampMicrosecondType]

---

## TimestampMillisecondTensor

`type_alias` · `arrow::tensor::TimestampMillisecondTensor`

```rust
type TimestampMillisecondTensor<'a> = Tensor<'a, TimestampMillisecondType>
```

[Tensor] of type [TimestampMillisecondType]

---

## TimestampNanosecondTensor

`type_alias` · `arrow::tensor::TimestampNanosecondTensor`

```rust
type TimestampNanosecondTensor<'a> = Tensor<'a, TimestampNanosecondType>
```

[Tensor] of type [TimestampNanosecondType]

---

## TimestampSecondTensor

`type_alias` · `arrow::tensor::TimestampSecondTensor`

```rust
type TimestampSecondTensor<'a> = Tensor<'a, TimestampSecondType>
```

[Tensor] of type [TimestampSecondType]

---

## UInt16Tensor

`type_alias` · `arrow::tensor::UInt16Tensor`

```rust
type UInt16Tensor<'a> = Tensor<'a, UInt16Type>
```

[Tensor] of type [UInt16Type]

---

## UInt32Tensor

`type_alias` · `arrow::tensor::UInt32Tensor`

```rust
type UInt32Tensor<'a> = Tensor<'a, UInt32Type>
```

[Tensor] of type [UInt32Type]

---

## UInt64Tensor

`type_alias` · `arrow::tensor::UInt64Tensor`

```rust
type UInt64Tensor<'a> = Tensor<'a, UInt64Type>
```

[Tensor] of type [UInt64Type]

---

## UInt8Tensor

`type_alias` · `arrow::tensor::UInt8Tensor`

```rust
type UInt8Tensor<'a> = Tensor<'a, UInt8Type>
```

[Tensor] of type [UInt8Type]

---
