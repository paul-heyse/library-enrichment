# `arrow_schema::datatype`

Crate `arrow-schema` · 15 public items · structured records in [`model/arrow_schema.datatype.json`](../model/arrow_schema.datatype.json)

## DECIMAL128_MAX_PRECISION

`constant` · `arrow_schema::datatype::DECIMAL128_MAX_PRECISION`

Also reachable as `arrow::datatypes::DECIMAL128_MAX_PRECISION`, `arrow_data::decimal::DECIMAL128_MAX_PRECISION`

```rust
const DECIMAL128_MAX_PRECISION: u8 = 38
```

The maximum precision for [DataType::Decimal128] values

---

## DECIMAL128_MAX_SCALE

`constant` · `arrow_schema::datatype::DECIMAL128_MAX_SCALE`

Also reachable as `arrow::datatypes::DECIMAL128_MAX_SCALE`, `arrow_data::decimal::DECIMAL128_MAX_SCALE`

```rust
const DECIMAL128_MAX_SCALE: i8 = 38
```

The maximum scale for [DataType::Decimal128] values

---

## DECIMAL256_MAX_PRECISION

`constant` · `arrow_schema::datatype::DECIMAL256_MAX_PRECISION`

Also reachable as `arrow::datatypes::DECIMAL256_MAX_PRECISION`, `arrow_data::decimal::DECIMAL256_MAX_PRECISION`

```rust
const DECIMAL256_MAX_PRECISION: u8 = 76
```

The maximum precision for [DataType::Decimal256] values

---

## DECIMAL256_MAX_SCALE

`constant` · `arrow_schema::datatype::DECIMAL256_MAX_SCALE`

Also reachable as `arrow::datatypes::DECIMAL256_MAX_SCALE`, `arrow_data::decimal::DECIMAL256_MAX_SCALE`

```rust
const DECIMAL256_MAX_SCALE: i8 = 76
```

The maximum scale for [DataType::Decimal256] values

---

## DECIMAL32_DEFAULT_SCALE

`constant` · `arrow_schema::datatype::DECIMAL32_DEFAULT_SCALE`

Also reachable as `arrow::datatypes::DECIMAL32_DEFAULT_SCALE`, `arrow_data::decimal::DECIMAL32_DEFAULT_SCALE`

```rust
const DECIMAL32_DEFAULT_SCALE: i8 = 2
```

The default scale for [DataType::Decimal32] values

---

## DECIMAL32_MAX_PRECISION

`constant` · `arrow_schema::datatype::DECIMAL32_MAX_PRECISION`

Also reachable as `arrow::datatypes::DECIMAL32_MAX_PRECISION`, `arrow_data::decimal::DECIMAL32_MAX_PRECISION`

```rust
const DECIMAL32_MAX_PRECISION: u8 = 9
```

The maximum precision for [DataType::Decimal32] values

---

## DECIMAL32_MAX_SCALE

`constant` · `arrow_schema::datatype::DECIMAL32_MAX_SCALE`

Also reachable as `arrow::datatypes::DECIMAL32_MAX_SCALE`, `arrow_data::decimal::DECIMAL32_MAX_SCALE`

```rust
const DECIMAL32_MAX_SCALE: i8 = 9
```

The maximum scale for [DataType::Decimal32] values

---

## DECIMAL64_DEFAULT_SCALE

`constant` · `arrow_schema::datatype::DECIMAL64_DEFAULT_SCALE`

Also reachable as `arrow::datatypes::DECIMAL64_DEFAULT_SCALE`, `arrow_data::decimal::DECIMAL64_DEFAULT_SCALE`

```rust
const DECIMAL64_DEFAULT_SCALE: i8 = 6
```

The default scale for [DataType::Decimal64] values

---

## DECIMAL64_MAX_PRECISION

`constant` · `arrow_schema::datatype::DECIMAL64_MAX_PRECISION`

Also reachable as `arrow::datatypes::DECIMAL64_MAX_PRECISION`, `arrow_data::decimal::DECIMAL64_MAX_PRECISION`

```rust
const DECIMAL64_MAX_PRECISION: u8 = 18
```

The maximum precision for [DataType::Decimal64] values

---

## DECIMAL64_MAX_SCALE

`constant` · `arrow_schema::datatype::DECIMAL64_MAX_SCALE`

Also reachable as `arrow::datatypes::DECIMAL64_MAX_SCALE`, `arrow_data::decimal::DECIMAL64_MAX_SCALE`

```rust
const DECIMAL64_MAX_SCALE: i8 = 18
```

The maximum scale for [DataType::Decimal64] values

---

## DECIMAL_DEFAULT_SCALE

`constant` · `arrow_schema::datatype::DECIMAL_DEFAULT_SCALE`

Also reachable as `arrow::datatypes::DECIMAL_DEFAULT_SCALE`, `arrow_data::decimal::DECIMAL_DEFAULT_SCALE`

```rust
const DECIMAL_DEFAULT_SCALE: i8 = 10
```

The default scale for [DataType::Decimal128] and [DataType::Decimal256]
values

---

## DataType

`enum` · `arrow_schema::datatype::DataType`

Also reachable as `arrow::datatypes::DataType`

```rust
enum DataType
```

**Variants**: `Null`, `Boolean`, `Int8`, `Int16`, `Int32`, `Int64`, `UInt8`, `UInt16`, `UInt32`, `UInt64`, `Float16`, `Float32`, `Float64`, `Timestamp`, `Date32`, `Date64`, `Time32`, `Time64`, `Duration`, `Interval`, `Binary`, `FixedSizeBinary`, `LargeBinary`, `BinaryView`, `Utf8`, `LargeUtf8`, `Utf8View`, `List`, `ListView`, `FixedSizeList`, `LargeList`, `LargeListView`, `Struct`, `Union`, `Dictionary`, `Decimal32`, `Decimal64`, `Decimal128`, `Decimal256`, `Map`, `RunEndEncoded`

**Implements**: `arrow_pyarrow::FromPyArrow`, `arrow_pyarrow::ToPyArrow`, `core::convert::TryFrom`, `core::fmt::Display`, `core::str::traits::FromStr`, `datafusion_common::datatype::DataTypeExt`, `datafusion_common::heap_size::DFHeapSize`, `parquet_variant_compute::shred_variant::IntoShreddingField`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (22)

```rust
fn contains(&self, other: &DataType) -> bool
fn equals_datatype(&self, other: &DataType) -> bool
fn is_binary(&self) -> bool
fn is_decimal(&self) -> bool
fn is_dictionary_key_type(&self) -> bool
fn is_floating(&self) -> bool
fn is_integer(&self) -> bool
fn is_list(&self) -> bool
fn is_nested(&self) -> bool
fn is_null(&self) -> bool
fn is_numeric(&self) -> bool
fn is_primitive(&self) -> bool
fn is_run_ends_type(&self) -> bool
fn is_signed_integer(&self) -> bool
fn is_string(&self) -> bool
fn is_temporal(&self) -> bool
fn is_unsigned_integer(&self) -> bool
fn new_fixed_size_list(data_type: DataType, size: i32, nullable: bool) -> Self
fn new_large_list(data_type: DataType, nullable: bool) -> Self
fn new_list(data_type: DataType, nullable: bool) -> Self
fn primitive_width(&self) -> Option<usize>
fn size(&self) -> usize
```

**via `core::convert::TryFrom`**

```rust
fn try_from(c_schema: &FFI_ArrowSchema) -> Result<Self, ArrowError>
fn try_from(value: &str) -> Result<Self, Self::Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

 Datatypes supported by this implementation of Apache Arrow.

 The variants of this enum include primitive fixed size types as well as
 parametric or nested types. See [`Schema.fbs`] for Arrow's specification.

 # Examples

 Primitive types
 ```
 # use arrow_schema::DataType;
 // create a new 32-bit signed integer
 let data_type = DataType::Int32;
 ```

 Nested Types
 ```
 # use arrow_schema::{DataType, Field};
 # use std::sync::Arc;
 // create a new list of 32-bit signed integers directly
 let list_data_type = DataType::List(Arc::new(Field::new_list_field(DataType::Int32, true)));
 // Create the same list type with constructor
 let list_data_type2 = DataType::new_list(DataType::Int32, true);
 assert_eq!(list_data_type, list_data_type2);
 ```

 Dictionary Types
 ```
 # use arrow_schema::{DataType};
 // String Dictionary (key type Int32 and value type Utf8)
 let data_type = DataType::Dictionary(Box::new(DataType::Int32), Box::new(DataType::Utf8));
 ```

 Timestamp Types
 ```
 # use arrow_schema::{DataType, TimeUnit};
 // timestamp with millisecond precision without timezone specified
 let data_type = DataType::Timestamp(TimeUnit::Millisecond, None);
 // timestamp with nanosecond precision in UTC timezone
 let data_type = DataType::Timestamp(TimeUnit::Nanosecond, Some("UTC".into()));
```

 # Display and FromStr

 The `Display` and `FromStr` implementations for `DataType` are
 human-readable, parseable, and reversible.

 ```
 # use arrow_schema::DataType;
 let data_type = DataType::Dictionary(Box::new(DataType::Int32), Box::new(DataType::Utf8));
 let data_type_string = data_type.to_string();
 assert_eq!(data_type_string, "Dictionary(Int32, Utf8)");
 // display can be parsed back into the original type
 let parsed_data_type: DataType = data_type.to_string().parse().unwrap();
 assert_eq!(data_type, parsed_data_type);
 ```

 # Nested Support
 Currently, the Rust implementation supports the following nested types:
  - `List<T>`
  - `LargeList<T>`
  - `FixedSizeList<T>`
  - `Struct<T, U, V, ...>`
  - `Union<T, U, V, ...>`
  - `Map<K, V>`

 Nested types can themselves be nested within other arrays.
 For more information on these types please see
 [the physical memory layout of Apache Arrow]

 [`Schema.fbs`]: https://github.com/apache/arrow/blob/main/format/Schema.fbs
 [the physical memory layout of Apache Arrow]: https://arrow.apache.org/docs/format/Columnar.html#physical-memory-layout

---

## IntervalUnit

`enum` · `arrow_schema::datatype::IntervalUnit`

Also reachable as `arrow::datatypes::IntervalUnit`

```rust
enum IntervalUnit
```

**Variants**: `YearMonth`, `DayTime`, `MonthDayNano`

**Implements**: `core::convert::From`, `datafusion_common::heap_size::DFHeapSize`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

YEAR_MONTH, DAY_TIME, MONTH_DAY_NANO interval in SQL style.

---

## TimeUnit

`enum` · `arrow_schema::datatype::TimeUnit`

Also reachable as `arrow::datatypes::TimeUnit`

```rust
enum TimeUnit
```

**Variants**: `Second`, `Millisecond`, `Microsecond`, `Nanosecond`

**Implements**: `core::convert::From`, `core::fmt::Display`, `datafusion_common::heap_size::DFHeapSize`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

An absolute length of time in seconds, milliseconds, microseconds or nanoseconds.

---

## UnionMode

`enum` · `arrow_schema::datatype::UnionMode`

Also reachable as `arrow::datatypes::UnionMode`

```rust
enum UnionMode
```

**Variants**: `Sparse`, `Dense`

**Implements**: `datafusion_common::heap_size::DFHeapSize`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Sparse or Dense union layouts

---
