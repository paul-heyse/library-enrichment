# `datafusion_common::scalar`

Crate `datafusion-common` · 9 public items · structured records in [`model/datafusion_common.scalar.json`](../model/datafusion_common.scalar.json)

## ScalarValue

`enum` · `datafusion_common::scalar::ScalarValue`

Also reachable as `datafusion::common::ScalarValue`, `datafusion::scalar::ScalarValue`, `datafusion_common::ScalarValue`

```rust
enum ScalarValue
```

**Variants**: `Null`, `Boolean`, `Float16`, `Float32`, `Float64`, `Decimal32`, `Decimal64`, `Decimal128`, `Decimal256`, `Int8`, `Int16`, `Int32`, `Int64`, `UInt8`, `UInt16`, `UInt32`, `UInt64`, `Utf8`, `Utf8View`, `LargeUtf8`, `Binary`, `BinaryView`, `FixedSizeBinary`, `LargeBinary`, `FixedSizeList`, `List`, `LargeList`, `ListView`, `LargeListView`, `Struct`, `Map`, `Date32`, `Date64`, `Time32Second`, `Time32Millisecond`, `Time64Microsecond`, `Time64Nanosecond`, `TimestampSecond`, `TimestampMillisecond`, `TimestampMicrosecond`, `TimestampNanosecond`, `IntervalYearMonth`, `IntervalDayTime`, `IntervalMonthDayNano`, `DurationSecond`, `DurationMillisecond`, `DurationMicrosecond`, `DurationNanosecond`, `Union`, `Dictionary`, `RunEndEncoded`

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `core::fmt::Display`, `core::str::traits::FromStr`, `datafusion_common::heap_size::DFHeapSize`, `datafusion_expr::literal::Literal`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd

**Methods** (65)

```rust
fn add<T: Borrow<ScalarValue>>(&self, other: T) -> Result<ScalarValue>
fn add_checked<T: Borrow<ScalarValue>>(&self, other: T) -> Result<ScalarValue>
fn arithmetic_negate(&self) -> Result<Self>
fn cast_to(&self, target_type: &DataType) -> Result<Self>
fn cast_to_with_options(&self, target_type: &DataType, cast_options: &CastOptions<'static>) -> Result<Self>
fn compact(&mut self)
fn compacted(self) -> Self
fn convert_array_to_scalar_vec(array: &dyn Array) -> Result<Vec<Option<Vec<Self>>>>
fn data_type(&self) -> DataType
fn distance(&self, other: &ScalarValue) -> Option<usize>
fn distance_u64(&self, other: &ScalarValue) -> Option<u64>
fn div<T: Borrow<ScalarValue>>(&self, other: T) -> Result<ScalarValue>
fn eq_array(&self, array: &ArrayRef, index: usize) -> Result<bool>
fn is_null(&self) -> bool
fn is_unsigned(&self) -> bool
fn iter_to_array(scalars: impl IntoIterator<Item = ScalarValue>) -> Result<ArrayRef>
fn max(datatype: &DataType) -> Option<ScalarValue>
fn min(datatype: &DataType) -> Option<ScalarValue>
fn mul<T: Borrow<ScalarValue>>(&self, other: T) -> Result<ScalarValue>
fn mul_checked<T: Borrow<ScalarValue>>(&self, other: T) -> Result<ScalarValue>
fn new_default(datatype: &DataType) -> Result<ScalarValue>
fn new_frac_pi_2(datatype: &DataType) -> Result<ScalarValue>
fn new_frac_pi_2_upper(datatype: &DataType) -> Result<ScalarValue>
fn new_infinity(datatype: &DataType) -> Result<ScalarValue>
fn new_interval_dt(days: i32, millis: i32) -> Self
fn new_interval_mdn(months: i32, days: i32, nanos: i64) -> Self
fn new_interval_ym(years: i32, months: i32) -> Self
fn new_large_list(values: &[ScalarValue], data_type: &DataType) -> Arc<LargeListArray>
fn new_list(values: &[ScalarValue], data_type: &DataType, nullable: bool) -> Arc<ListArray>
fn new_list_from_iter(values: impl IntoIterator<Item = ScalarValue> + ExactSizeIterator, data_type: &DataType, nullable: bool) -> Arc<ListArray>
fn new_list_nullable(values: &[ScalarValue], data_type: &DataType) -> Arc<ListArray>
fn new_neg_frac_pi_2(datatype: &DataType) -> Result<ScalarValue>
fn new_neg_frac_pi_2_lower(datatype: &DataType) -> Result<ScalarValue>
fn new_neg_infinity(datatype: &DataType) -> Result<ScalarValue>
fn new_negative_one(datatype: &DataType) -> Result<ScalarValue>
fn new_negative_pi(datatype: &DataType) -> Result<ScalarValue>
fn new_negative_pi_lower(datatype: &DataType) -> Result<ScalarValue>
fn new_null_list(data_type: DataType, nullable: bool, null_len: usize) -> Self
fn new_one(datatype: &DataType) -> Result<ScalarValue>
fn new_pi(datatype: &DataType) -> Result<ScalarValue>
fn new_pi_upper(datatype: &DataType) -> Result<ScalarValue>
fn new_primitive<T: ArrowPrimitiveType>(a: Option<T::Native>, d: &DataType) -> Result<Self>
fn new_ten(datatype: &DataType) -> Result<ScalarValue>
fn new_timestamp<T: ArrowTimestampType>(value: Option<i64>, tz_opt: Option<Arc<str>>) -> Self
fn new_utf8(val: impl Into<String>) -> Self
fn new_utf8view(val: impl Into<String>) -> Self
fn new_zero(datatype: &DataType) -> Result<ScalarValue>
fn raw_data(&self) -> Result<ArrayRef>
fn rem<T: Borrow<ScalarValue>>(&self, other: T) -> Result<ScalarValue>
fn size(&self) -> usize
fn size_of_hashmap<V, S>(map: &HashMap<Self, V, S>) -> usize
fn size_of_hashset<S>(set: &HashSet<Self, S>) -> usize
fn size_of_vec(vec: &Vec<Self>) -> usize
fn size_of_vec_deque(vec_deque: &VecDeque<Self>) -> usize
fn sub<T: Borrow<ScalarValue>>(&self, other: T) -> Result<ScalarValue>
fn sub_checked<T: Borrow<ScalarValue>>(&self, other: T) -> Result<ScalarValue>
fn to_array(&self) -> Result<ArrayRef>
fn to_array_of_size(&self, size: usize) -> Result<ArrayRef>
fn to_scalar(&self) -> Result<Scalar<ArrayRef>>
fn try_as_str(&self) -> Option<Option<&str>>
fn try_cmp(&self, other: &Self) -> Result<Ordering>
fn try_from_array(array: &dyn Array, index: usize) -> Result<Self>
fn try_from_string(value: String, target_type: &DataType) -> Result<Self>
fn try_new_decimal128(value: i128, precision: u8, scale: i8) -> Result<Self>
fn try_new_null(data_type: &DataType) -> Result<Self>
```

**via `core::convert::From`**

```rust
fn from(value: u16) -> Self
fn from(value: i32) -> Self
fn from(value: f32) -> Self
fn from(value: Option<u32>) -> Self
fn from(value: Option<i64>) -> Self
fn from(value: Option<f16>) -> Self
fn from(value: &str) -> Self
fn from(value: u8) -> Self
fn from(value: i16) -> Self
fn from(value: f64) -> Self
fn from(value: Option<u16>) -> Self
fn from(value: String) -> Self
fn from(value: Option<i32>) -> Self
fn from(value: Option<f32>) -> Self
fn from(value: u64) -> Self
fn from(value: bool) -> Self
fn from(value: i8) -> Self
fn from(value: Option<&str>) -> Self
fn from(value: Option<u8>) -> Self
fn from(value: Option<i16>) -> Self
fn from(value: Option<f64>) -> Self
fn from(value: u32) -> Self
fn from(value: Option<String>) -> Self
fn from(value: i64) -> Self
fn from(value: f16) -> Self
fn from(value: Option<u64>) -> Self
fn from(value: Option<bool>) -> Self
fn from(value: Option<i8>) -> Self
fn from(value: Vec<(&str, ScalarValue)>) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(datatype: DataType) -> Result<Self>
fn try_from(data_type: &DataType) -> Result<Self>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `datafusion_common::heap_size::DFHeapSize`**

```rust
fn heap_size(&self, ctx: &mut DFHeapSizeCtx) -> usize
```

A dynamically typed, nullable single value.

While an arrow  [`Array`]) stores one or more values of the same type, in a
single column, a `ScalarValue` stores a single value of a single type, the
equivalent of 1 row and one column.

```text
 ┌────────┐
 │ value1 │
 │ value2 │                  ┌────────┐
 │ value3 │                  │ value2 │
 │  ...   │                  └────────┘
 │ valueN │
 └────────┘

   Array                     ScalarValue

stores multiple,             stores a single,
possibly null, values of     possible null, value
the same type
```

# Performance

In general, performance will be better using arrow [`Array`]s rather than
[`ScalarValue`], as it is far more efficient to process multiple values at
once (vectorized processing).

# Example
```
# use datafusion_common::ScalarValue;
// Create single scalar value for an Int32 value
let s1 = ScalarValue::Int32(Some(10));

// You can also create values using the From impl:
let s2 = ScalarValue::from(10i32);
assert_eq!(s1, s2);
```

# Null Handling

`ScalarValue` represents null values in the same way as Arrow. Nulls are
"typed" in the sense that a null value in an [`Int32Array`] is different
from a null value in a [`Float64Array`], and is different from the values in
a [`NullArray`].

```
# fn main() -> datafusion_common::Result<()> {
# use std::collections::hash_set::Difference;
# use datafusion_common::ScalarValue;
# use arrow::datatypes::DataType;
// You can create a 'null' Int32 value directly:
let s1 = ScalarValue::Int32(None);

// You can also create a null value for a given datatype:
let s2 = ScalarValue::try_from(&DataType::Int32)?;
assert_eq!(s1, s2);

// Note that this is DIFFERENT than a `ScalarValue::Null`
let s3 = ScalarValue::Null;
assert_ne!(s1, s3);
# Ok(())
# }
```

# Nested Types

`List` / `LargeList` / `FixedSizeList` / `ListView` / `LargeListView` / `Struct` / `Map`
are represented as a single element array of the corresponding type.

## Example: Creating [`ScalarValue::Struct`] using [`ScalarStructBuilder`]
```
# use std::sync::Arc;
# use arrow::datatypes::{DataType, Field};
# use datafusion_common::{ScalarValue, scalar::ScalarStructBuilder};
// Build a struct like: {a: 1, b: "foo"}
let field_a = Field::new("a", DataType::Int32, false);
let field_b = Field::new("b", DataType::Utf8, false);

let s1 = ScalarStructBuilder::new()
    .with_scalar(field_a, ScalarValue::from(1i32))
    .with_scalar(field_b, ScalarValue::from("foo"))
    .build();
```

## Example: Creating a null [`ScalarValue::Struct`] using [`ScalarStructBuilder`]
```
# use std::sync::Arc;
# use arrow::datatypes::{DataType, Field};
# use datafusion_common::{ScalarValue, scalar::ScalarStructBuilder};
// Build a struct representing a NULL value
let fields = vec![
    Field::new("a", DataType::Int32, false),
    Field::new("b", DataType::Utf8, false),
];

let s1 = ScalarStructBuilder::new_null(fields);
```

## Example: Creating [`ScalarValue::Struct`] directly
```
# use std::sync::Arc;
# use arrow::datatypes::{DataType, Field, Fields};
# use arrow::array::{ArrayRef, Int32Array, StructArray, StringArray};
# use datafusion_common::ScalarValue;
// Build a struct like: {a: 1, b: "foo"}
// Field description
let fields = Fields::from(vec![
    Field::new("a", DataType::Int32, false),
    Field::new("b", DataType::Utf8, false),
]);
// one row arrays for each field
let arrays: Vec<ArrayRef> = vec![
    Arc::new(Int32Array::from(vec![1])),
    Arc::new(StringArray::from(vec!["foo"])),
];
// no nulls for this array
let nulls = None;
let arr = StructArray::new(fields, arrays, nulls);

// Create a ScalarValue::Struct directly
let s1 = ScalarValue::Struct(Arc::new(arr));
```


# Further Reading
See [datatypes](https://arrow.apache.org/docs/python/api/datatypes.html) for
details on datatypes and the [format](https://github.com/apache/arrow/blob/master/format/Schema.fbs#L354-L375)
for the definitive reference.

[`NullArray`]: arrow::array::NullArray

---

## copy_array_data

`function` · `datafusion_common::scalar::copy_array_data`

```rust
fn copy_array_data(src_data: &arrow::array::ArrayData) -> arrow::array::ArrayData
```

Compacts the data of an `ArrayData` into a new `ArrayData`.

This is useful when you want to minimize the memory footprint of an
`ArrayData`. For example, the value returned by [`Array::slice`] still
points at the same underlying data buffers as the original array, which may
hold many more values. Calling `copy_array_data` on the sliced array will
create a new, smaller, `ArrayData` that only contains the data for the
sliced array.

# Example
```
# use arrow::array::{make_array, Array, Int32Array};
use datafusion_common::scalar::copy_array_data;
let array = Int32Array::from_iter_values(0..8192);
// Take only the first 2 elements
let sliced_array = array.slice(0, 2);
// The memory footprint of `sliced_array` is close to 8192 * 4 bytes
assert_eq!(32864, sliced_array.get_array_memory_size());
// however, we can copy the data to a new `ArrayData`
let new_array = make_array(copy_array_data(&sliced_array.into_data()));
// The memory footprint of `new_array` is now only 2 * 4 bytes
// and overhead:
assert_eq!(160, new_array.get_array_memory_size());
```

See also [`ScalarValue::compact`] which applies to `ScalarValue` instances
as necessary.

---

## date_to_timestamp_multiplier

`function` · `datafusion_common::scalar::date_to_timestamp_multiplier`

```rust
fn date_to_timestamp_multiplier(source_type: &arrow::datatypes::DataType, target_type: &arrow::datatypes::DataType) -> Option<i64>
```

Returns the multiplier that converts the input date representation into the
desired timestamp unit, if the conversion requires a multiplication that can
overflow an `i64`.

---

## dict_from_values

`function` · `datafusion_common::scalar::dict_from_values`

```rust
fn dict_from_values<K: ArrowDictionaryKeyType>(values_array: arrow::array::ArrayRef) -> error::Result<arrow::array::ArrayRef>
```

Create a `DictionaryArray` from the provided values array.

Each element gets a unique key (`0..N-1`), without deduplication.
Useful for wrapping arrays in dictionary form.

# Input
["alice", "bob", "alice", null, "carol"]

# Output
`DictionaryArray<Int32>`
{
  keys:   [0, 1, 2, 3, 4],
  values: ["alice", "bob", "alice", null, "carol"]
}

---

## ensure_timestamp_in_bounds

`function` · `datafusion_common::scalar::ensure_timestamp_in_bounds`

```rust
fn ensure_timestamp_in_bounds(value: i64, multiplier: i64, source_type: &arrow::datatypes::DataType, target_type: &arrow::datatypes::DataType) -> error::Result<()>
```

Ensures the provided value can be represented as a timestamp with the given
multiplier. Returns an [`DataFusionError::Execution`] when the converted
value would overflow the timestamp range.

---

## get_dict_value

`function` · `datafusion_common::scalar::get_dict_value`

```rust
fn get_dict_value<K: ArrowDictionaryKeyType>(array: &dyn Array, index: usize) -> error::Result<(&arrow::array::ArrayRef, Option<usize>)>
```

Return a reference to the values array and the index into it for a
dictionary array

# Errors

Errors if the array cannot be downcasted to DictionaryArray

---

## partial_cmp_struct

`function` · `datafusion_common::scalar::partial_cmp_struct`

```rust
fn partial_cmp_struct(s1: &arrow::array::StructArray, s2: &arrow::array::StructArray) -> Option<std::cmp::Ordering>
```

---

## timestamp_to_timestamp_multiplier

`function` · `datafusion_common::scalar::timestamp_to_timestamp_multiplier`

```rust
fn timestamp_to_timestamp_multiplier(source_type: &arrow::datatypes::DataType, target_type: &arrow::datatypes::DataType) -> Option<i64>
```

Returns the multiplier that converts the input timestamp representation into
the desired timestamp unit, if the conversion requires a multiplication that
can overflow an `i64`.

---

## ScalarType

`trait` · `datafusion_common::scalar::ScalarType`

Also reachable as `datafusion::common::ScalarType`, `datafusion::scalar::ScalarType`, `datafusion_common::ScalarType`

```rust
trait ScalarType<T: ArrowNativeType>
```

**Implementors** (6)

- `arrow_array::types::Date32Type`
- `arrow_array::types::Float32Type`
- `arrow_array::types::TimestampMicrosecondType`
- `arrow_array::types::TimestampMillisecondType`
- `arrow_array::types::TimestampNanosecondType`
- `arrow_array::types::TimestampSecondType`

**Methods** (1)

```rust
fn scalar(r: Option<T>) -> ScalarValue
```

Trait used to map a NativeType to a ScalarValue

---
