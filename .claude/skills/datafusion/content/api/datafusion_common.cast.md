# `datafusion_common::cast`

Crate `datafusion-common` · 55 public items · structured records in [`model/datafusion_common.cast.json`](../model/datafusion_common.cast.json)

## as_binary_array

`function` · `datafusion_common::cast::as_binary_array`

```rust
fn as_binary_array(array: &dyn Array) -> Result<&arrow::array::BinaryArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_binary_array.md).


---

## as_binary_view_array

`function` · `datafusion_common::cast::as_binary_view_array`

```rust
fn as_binary_view_array(array: &dyn Array) -> Result<&arrow::array::BinaryViewArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_binary_view_array.md).


---

## as_boolean_array

`function` · `datafusion_common::cast::as_boolean_array`

```rust
fn as_boolean_array(array: &dyn Array) -> Result<&arrow::array::BooleanArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_boolean_array.md).


---

## as_date32_array

`function` · `datafusion_common::cast::as_date32_array`

```rust
fn as_date32_array(array: &dyn Array) -> Result<&arrow::array::Date32Array>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_date32_array.md).


---

## as_date64_array

`function` · `datafusion_common::cast::as_date64_array`

```rust
fn as_date64_array(array: &dyn Array) -> Result<&arrow::array::Date64Array>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_date64_array.md).


---

## as_decimal128_array

`function` · `datafusion_common::cast::as_decimal128_array`

```rust
fn as_decimal128_array(array: &dyn Array) -> Result<&arrow::array::Decimal128Array>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_decimal128_array.md).


---

## as_decimal256_array

`function` · `datafusion_common::cast::as_decimal256_array`

```rust
fn as_decimal256_array(array: &dyn Array) -> Result<&arrow::array::Decimal256Array>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_decimal256_array.md).


---

## as_decimal32_array

`function` · `datafusion_common::cast::as_decimal32_array`

```rust
fn as_decimal32_array(array: &dyn Array) -> Result<&arrow::array::Decimal32Array>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_decimal32_array.md).


---

## as_decimal64_array

`function` · `datafusion_common::cast::as_decimal64_array`

```rust
fn as_decimal64_array(array: &dyn Array) -> Result<&arrow::array::Decimal64Array>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_decimal64_array.md).


---

## as_dictionary_array

`function` · `datafusion_common::cast::as_dictionary_array`

```rust
fn as_dictionary_array<T: ArrowDictionaryKeyType>(array: &dyn Array) -> Result<&arrow::array::DictionaryArray<T>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_dictionary_array.md).


---

## as_duration_microsecond_array

`function` · `datafusion_common::cast::as_duration_microsecond_array`

```rust
fn as_duration_microsecond_array(array: &dyn Array) -> Result<&arrow::array::DurationMicrosecondArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_duration_microsecond_array.md).


---

## as_duration_millisecond_array

`function` · `datafusion_common::cast::as_duration_millisecond_array`

```rust
fn as_duration_millisecond_array(array: &dyn Array) -> Result<&arrow::array::DurationMillisecondArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_duration_millisecond_array.md).


---

## as_duration_nanosecond_array

`function` · `datafusion_common::cast::as_duration_nanosecond_array`

```rust
fn as_duration_nanosecond_array(array: &dyn Array) -> Result<&arrow::array::DurationNanosecondArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_duration_nanosecond_array.md).


---

## as_duration_second_array

`function` · `datafusion_common::cast::as_duration_second_array`

```rust
fn as_duration_second_array(array: &dyn Array) -> Result<&arrow::array::DurationSecondArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_duration_second_array.md).


---

## as_fixed_size_binary_array

`function` · `datafusion_common::cast::as_fixed_size_binary_array`

```rust
fn as_fixed_size_binary_array(array: &dyn Array) -> Result<&arrow::array::FixedSizeBinaryArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_fixed_size_binary_array.md).


---

## as_fixed_size_list_array

`function` · `datafusion_common::cast::as_fixed_size_list_array`

```rust
fn as_fixed_size_list_array(array: &dyn Array) -> Result<&arrow::array::FixedSizeListArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_fixed_size_list_array.md).


---

## as_float16_array

`function` · `datafusion_common::cast::as_float16_array`

```rust
fn as_float16_array(array: &dyn Array) -> Result<&arrow::array::Float16Array>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_float16_array.md).


---

## as_float32_array

`function` · `datafusion_common::cast::as_float32_array`

```rust
fn as_float32_array(array: &dyn Array) -> Result<&arrow::array::Float32Array>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_float32_array.md).


---

## as_float64_array

`function` · `datafusion_common::cast::as_float64_array`

```rust
fn as_float64_array(array: &dyn Array) -> Result<&arrow::array::Float64Array>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_float64_array.md).


---

## as_generic_binary_array

`function` · `datafusion_common::cast::as_generic_binary_array`

```rust
fn as_generic_binary_array<T: OffsetSizeTrait>(array: &dyn Array) -> Result<&arrow::array::GenericBinaryArray<T>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_generic_binary_array.md).


---

## as_generic_list_array

`function` · `datafusion_common::cast::as_generic_list_array`

```rust
fn as_generic_list_array<T: OffsetSizeTrait>(array: &dyn Array) -> Result<&arrow::array::GenericListArray<T>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_generic_list_array.md).


---

## as_generic_string_array

`function` · `datafusion_common::cast::as_generic_string_array`

```rust
fn as_generic_string_array<T: OffsetSizeTrait>(array: &dyn Array) -> Result<&arrow::array::GenericStringArray<T>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_generic_string_array.md).


---

## as_int16_array

`function` · `datafusion_common::cast::as_int16_array`

```rust
fn as_int16_array(array: &dyn Array) -> Result<&arrow::array::Int16Array>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_int16_array.md).


---

## as_int32_array

`function` · `datafusion_common::cast::as_int32_array`

```rust
fn as_int32_array(array: &dyn Array) -> Result<&arrow::array::Int32Array>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_int32_array.md).


---

## as_int64_array

`function` · `datafusion_common::cast::as_int64_array`

```rust
fn as_int64_array(array: &dyn Array) -> Result<&arrow::array::Int64Array>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_int64_array.md).


---

## as_int8_array

`function` · `datafusion_common::cast::as_int8_array`

```rust
fn as_int8_array(array: &dyn Array) -> Result<&arrow::array::Int8Array>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_int8_array.md).


---

## as_interval_dt_array

`function` · `datafusion_common::cast::as_interval_dt_array`

```rust
fn as_interval_dt_array(array: &dyn Array) -> Result<&arrow::array::IntervalDayTimeArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_interval_dt_array.md).


---

## as_interval_mdn_array

`function` · `datafusion_common::cast::as_interval_mdn_array`

```rust
fn as_interval_mdn_array(array: &dyn Array) -> Result<&arrow::array::IntervalMonthDayNanoArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_interval_mdn_array.md).


---

## as_interval_ym_array

`function` · `datafusion_common::cast::as_interval_ym_array`

```rust
fn as_interval_ym_array(array: &dyn Array) -> Result<&arrow::array::IntervalYearMonthArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_interval_ym_array.md).


---

## as_large_binary_array

`function` · `datafusion_common::cast::as_large_binary_array`

```rust
fn as_large_binary_array(array: &dyn Array) -> Result<&arrow::array::LargeBinaryArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_large_binary_array.md).


---

## as_large_list_array

`function` · `datafusion_common::cast::as_large_list_array`

```rust
fn as_large_list_array(array: &dyn Array) -> Result<&arrow::array::LargeListArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_large_list_array.md).


---

## as_large_list_view_array

`function` · `datafusion_common::cast::as_large_list_view_array`

```rust
fn as_large_list_view_array(array: &dyn Array) -> Result<&arrow::array::LargeListViewArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_large_list_view_array.md).


---

## as_large_string_array

`function` · `datafusion_common::cast::as_large_string_array`

```rust
fn as_large_string_array(array: &dyn Array) -> Result<&arrow::array::LargeStringArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_large_string_array.md).


---

## as_list_array

`function` · `datafusion_common::cast::as_list_array`

```rust
fn as_list_array(array: &dyn Array) -> Result<&arrow::array::ListArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_list_array.md).


---

## as_list_view_array

`function` · `datafusion_common::cast::as_list_view_array`

```rust
fn as_list_view_array(array: &dyn Array) -> Result<&arrow::array::ListViewArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_list_view_array.md).


---

## as_map_array

`function` · `datafusion_common::cast::as_map_array`

```rust
fn as_map_array(array: &dyn Array) -> Result<&arrow::array::MapArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_map_array.md).


---

## as_null_array

`function` · `datafusion_common::cast::as_null_array`

```rust
fn as_null_array(array: &dyn Array) -> Result<&arrow::array::NullArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_null_array.md).


---

## as_primitive_array

`function` · `datafusion_common::cast::as_primitive_array`

```rust
fn as_primitive_array<T: ArrowPrimitiveType>(array: &dyn Array) -> Result<&arrow::array::PrimitiveArray<T>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_primitive_array.md).


---

## as_run_array

`function` · `datafusion_common::cast::as_run_array`

```rust
fn as_run_array<T: RunEndIndexType>(array: &dyn Array) -> Result<&arrow::array::RunArray<T>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_run_array.md).


---

## as_string_array

`function` · `datafusion_common::cast::as_string_array`

```rust
fn as_string_array(array: &dyn Array) -> Result<&arrow::array::StringArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_string_array.md).


---

## as_string_view_array

`function` · `datafusion_common::cast::as_string_view_array`

```rust
fn as_string_view_array(array: &dyn Array) -> Result<&arrow::array::StringViewArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_string_view_array.md).


---

## as_struct_array

`function` · `datafusion_common::cast::as_struct_array`

```rust
fn as_struct_array(array: &dyn Array) -> Result<&arrow::array::StructArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_struct_array.md).


---

## as_time32_millisecond_array

`function` · `datafusion_common::cast::as_time32_millisecond_array`

```rust
fn as_time32_millisecond_array(array: &dyn Array) -> Result<&arrow::array::Time32MillisecondArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_time32_millisecond_array.md).


---

## as_time32_second_array

`function` · `datafusion_common::cast::as_time32_second_array`

```rust
fn as_time32_second_array(array: &dyn Array) -> Result<&arrow::array::Time32SecondArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_time32_second_array.md).


---

## as_time64_microsecond_array

`function` · `datafusion_common::cast::as_time64_microsecond_array`

```rust
fn as_time64_microsecond_array(array: &dyn Array) -> Result<&arrow::array::Time64MicrosecondArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_time64_microsecond_array.md).


---

## as_time64_nanosecond_array

`function` · `datafusion_common::cast::as_time64_nanosecond_array`

```rust
fn as_time64_nanosecond_array(array: &dyn Array) -> Result<&arrow::array::Time64NanosecondArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_time64_nanosecond_array.md).


---

## as_timestamp_microsecond_array

`function` · `datafusion_common::cast::as_timestamp_microsecond_array`

```rust
fn as_timestamp_microsecond_array(array: &dyn Array) -> Result<&arrow::array::TimestampMicrosecondArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_timestamp_microsecond_array.md).


---

## as_timestamp_millisecond_array

`function` · `datafusion_common::cast::as_timestamp_millisecond_array`

```rust
fn as_timestamp_millisecond_array(array: &dyn Array) -> Result<&arrow::array::TimestampMillisecondArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_timestamp_millisecond_array.md).


---

## as_timestamp_nanosecond_array

`function` · `datafusion_common::cast::as_timestamp_nanosecond_array`

```rust
fn as_timestamp_nanosecond_array(array: &dyn Array) -> Result<&arrow::array::TimestampNanosecondArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_timestamp_nanosecond_array.md).


---

## as_timestamp_second_array

`function` · `datafusion_common::cast::as_timestamp_second_array`

```rust
fn as_timestamp_second_array(array: &dyn Array) -> Result<&arrow::array::TimestampSecondArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_timestamp_second_array.md).


---

## as_uint16_array

`function` · `datafusion_common::cast::as_uint16_array`

```rust
fn as_uint16_array(array: &dyn Array) -> Result<&arrow::array::UInt16Array>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_uint16_array.md).


---

## as_uint32_array

`function` · `datafusion_common::cast::as_uint32_array`

```rust
fn as_uint32_array(array: &dyn Array) -> Result<&arrow::array::UInt32Array>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_uint32_array.md).


---

## as_uint64_array

`function` · `datafusion_common::cast::as_uint64_array`

```rust
fn as_uint64_array(array: &dyn Array) -> Result<&arrow::array::UInt64Array>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_uint64_array.md).


---

## as_uint8_array

`function` · `datafusion_common::cast::as_uint8_array`

```rust
fn as_uint8_array(array: &dyn Array) -> Result<&arrow::array::UInt8Array>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_uint8_array.md).


---

## as_union_array

`function` · `datafusion_common::cast::as_union_array`

```rust
fn as_union_array(array: &dyn Array) -> Result<&arrow::array::UnionArray>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.cast.as_union_array.md).


---
