# `arrow_array::array::primitive_array`

Crate `arrow-array` · 34 public items · structured records in [`model/arrow_array.array.primitive_array.json`](../model/arrow_array.array.primitive_array.json)

## NativeAdapter

`struct` · `arrow_array::array::primitive_array::NativeAdapter`

```rust
struct NativeAdapter<T: ArrowPrimitiveType>
```

**Fields**: `native`

**Implements**: `core::convert::From`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(value: u64) -> Self
fn from(value: f16) -> Self
fn from(value: i32) -> Self
fn from(value: i32) -> Self
fn from(value: u16) -> Self
fn from(value: Option<<T as ArrowPrimitiveType>::Native>) -> Self
fn from(value: i8) -> Self
fn from(value: f32) -> Self
fn from(value: i64) -> Self
fn from(value: i64) -> Self
fn from(value: u32) -> Self
fn from(value: &Option<<T as ArrowPrimitiveType>::Native>) -> Self
fn from(value: i16) -> Self
fn from(value: f64) -> Self
fn from(value: u8) -> Self
fn from(value: i128) -> Self
fn from(value: i256) -> Self
```

An optional primitive value

This struct is used as an adapter when creating `PrimitiveArray` from an iterator.
`FromIterator` for `PrimitiveArray` takes an iterator where the elements can be `into`
this struct. So once implementing `From` or `Into` trait for a type, an iterator of
the type can be collected to `PrimitiveArray`.

---

## PrimitiveArray

`struct` · `arrow_array::array::primitive_array::PrimitiveArray`

```rust
struct PrimitiveArray<T: ArrowPrimitiveType>
```

**Implements**: `arrow_array::array::Array`, `core::convert::From`, `core::iter::traits::collect::FromIterator`

**Derives**: Clone, Debug, PartialEq

**Methods** (45)

```rust
fn builder(capacity: usize) -> PrimitiveBuilder<T>
fn from_iter_values<I: IntoIterator<Item = T::Native>>(iter: I) -> Self
fn from_iter_values_with_nulls<I: IntoIterator<Item = T::Native>>(iter: I, nulls: Option<NullBuffer>) -> Self
unsafe fn from_trusted_len_iter<I, P>(iter: I) -> Self where P: std::borrow::Borrow<Option<<T as ArrowPrimitiveType>::Native>>, I: IntoIterator<Item = P>
fn from_unary<U: ArrayAccessor, F>(left: U, op: F) -> Self where F: FnMut(U::Item) -> T::Native
fn from_value(value: T::Native, count: usize) -> Self
fn into_builder(self) -> Result<PrimitiveBuilder<T>, Self>
fn into_parts(self) -> (DataType, ScalarBuffer<T::Native>, Option<NullBuffer>)
fn is_compatible(data_type: &DataType) -> bool
fn is_empty(&self) -> bool
fn iter(&'a self) -> PrimitiveIter<'a, T>
fn len(&self) -> usize
fn new(values: ScalarBuffer<T::Native>, nulls: Option<NullBuffer>) -> Self
fn new_null(length: usize) -> Self
fn new_scalar(value: T::Native) -> Scalar<Self>
unsafe fn new_unchecked(values: ScalarBuffer<T::Native>, nulls: Option<NullBuffer>) -> Self
fn null_if_overflow_precision(&self, precision: u8) -> Self
fn precision(&self) -> u8
fn reinterpret_cast<K>(&self) -> PrimitiveArray<K> where K: ArrowPrimitiveType<Native = T::Native>
fn scale(&self) -> i8
fn slice(&self, offset: usize, length: usize) -> Self
fn take_iter<'a>(&'a self, indexes: impl Iterator<Item = Option<usize>> + 'a) -> impl Iterator<Item = Option<T::Native>> + 'a
unsafe fn take_iter_unchecked<'a>(&'a self, indexes: impl Iterator<Item = Option<usize>> + 'a) -> impl Iterator<Item = Option<T::Native>> + 'a
fn timezone(&self) -> Option<&str>
fn try_new(values: ScalarBuffer<T::Native>, nulls: Option<NullBuffer>) -> Result<Self, ArrowError>
fn try_unary<F, O, E>(&self, op: F) -> Result<PrimitiveArray<O>, E> where O: ArrowPrimitiveType, F: Fn(T::Native) -> Result<O::Native, E>
fn try_unary_mut<F, E>(self, op: F) -> Result<Result<PrimitiveArray<T>, E>, PrimitiveArray<T>> where F: Fn(T::Native) -> Result<T::Native, E>
fn unary<F, O>(&self, op: F) -> PrimitiveArray<O> where O: ArrowPrimitiveType, F: Fn(T::Native) -> O::Native
fn unary_mut<F>(self, op: F) -> Result<PrimitiveArray<T>, PrimitiveArray<T>> where F: Fn(T::Native) -> T::Native
fn unary_opt<F, O>(&self, op: F) -> PrimitiveArray<O> where O: ArrowPrimitiveType, F: Fn(T::Native) -> Option<O::Native>
fn validate_decimal_precision(&self, precision: u8) -> Result<(), ArrowError>
fn value(&self, i: usize) -> T::Native
fn value_as_date(&self, i: usize) -> Option<NaiveDate>
fn value_as_datetime(&self, i: usize) -> Option<NaiveDateTime>
fn value_as_datetime_with_tz(&self, i: usize, tz: Tz) -> Option<DateTime<Tz>>
fn value_as_duration(&self, i: usize) -> Option<Duration>
fn value_as_string(&self, row: usize) -> String
fn value_as_time(&self, i: usize) -> Option<NaiveTime>
unsafe fn value_unchecked(&self, i: usize) -> T::Native
fn values(&self) -> &ScalarBuffer<T::Native>
fn with_data_type(self, data_type: DataType) -> Self
fn with_precision_and_scale(self, precision: u8, scale: i8) -> Result<Self, ArrowError>
fn with_timezone(self, timezone: impl Into<Arc<str>>) -> Self
fn with_timezone_opt<S: Into<Arc<str>>>(self, timezone: Option<S>) -> Self
fn with_timezone_utc(self) -> Self
```

**via `arrow_array::array::Array`**

```rust
fn as_any(&self) -> &dyn Any
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
fn data_type(&self) -> &DataType
fn get_array_memory_size(&self) -> usize
fn get_buffer_memory_size(&self) -> usize
fn into_data(self) -> ArrayData
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn logical_null_count(&self) -> usize
fn nulls(&self) -> Option<&NullBuffer>
fn offset(&self) -> usize
fn shrink_to_fit(&mut self)
fn slice(&self, offset: usize, length: usize) -> ArrayRef
fn to_data(&self) -> ArrayData
```

**via `core::convert::From`**

```rust
fn from(data: Vec<Option<<UInt8Type as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<Option<<DurationNanosecondType as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<Decimal256Type as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<Int8Type as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<Option<<IntervalMonthDayNanoType as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<Float64Type as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<Time64MicrosecondType as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<UInt32Type as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<<TimestampMillisecondType as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<Date32Type as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<Int32Type as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<<DurationMillisecondType as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<Decimal32Type as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<IntervalYearMonthType as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<UInt64Type as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<Option<<TimestampMicrosecondType as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<Time32SecondType as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<Int64Type as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<Option<<DurationMicrosecondType as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<Decimal128Type as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<IntervalDayTimeType as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<Float32Type as ArrowPrimitiveType>::Native>) -> Self
fn from(data: ArrayData) -> Self
fn from(data: Vec<Option<<Time32MillisecondType as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<UInt16Type as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<<TimestampSecondType as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<Decimal256Type as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<Int16Type as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<<DurationSecondType as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<Float64Type as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<Time64NanosecondType as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<UInt32Type as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<Option<<TimestampMillisecondType as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<Date64Type as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<Int32Type as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<Option<<DurationMillisecondType as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<Decimal64Type as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<IntervalYearMonthType as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<Float16Type as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<<TimestampNanosecondType as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<Time32SecondType as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<UInt8Type as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<<DurationNanosecondType as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<Decimal128Type as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<Int8Type as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<<IntervalMonthDayNanoType as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<Float32Type as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<Time64MicrosecondType as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<UInt16Type as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<Option<<TimestampSecondType as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<Date32Type as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<Int16Type as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<Option<<DurationSecondType as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<Decimal32Type as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<Time64NanosecondType as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<UInt64Type as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<<TimestampMicrosecondType as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<Date64Type as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<Int64Type as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<<DurationMicrosecondType as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<Decimal64Type as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<IntervalDayTimeType as ArrowPrimitiveType>::Native>) -> Self
fn from(data: Vec<Option<<Float16Type as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<Option<<TimestampNanosecondType as ArrowPrimitiveType>::Native>>) -> Self
fn from(data: Vec<<Time32MillisecondType as ArrowPrimitiveType>::Native>) -> Self
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = Ptr>>(iter: I) -> Self
```

An array of primitive values, of type [`ArrowPrimitiveType`]

# Example: From a Vec

*Note*: Converting a `Vec` to a `PrimitiveArray` does not copy the data.
The new `PrimitiveArray` uses the same underlying allocation from the `Vec`.

```
# use arrow_array::{Array, PrimitiveArray, types::Int32Type};
let arr: PrimitiveArray<Int32Type> = vec![1, 2, 3, 4].into();
assert_eq!(4, arr.len());
assert_eq!(0, arr.null_count());
assert_eq!(arr.values(), &[1, 2, 3, 4])
```

# Example: To a `Vec<T>`

*Note*: In some cases, converting `PrimitiveArray` to a `Vec` is zero-copy
and does not copy the data (see [`Buffer::into_vec`] for conditions). In
such cases, the `Vec` will use the same underlying memory allocation from
the `PrimitiveArray`.

The Rust compiler generates highly optimized code for operations on
Vec, so using a Vec can often be faster than using a PrimitiveArray directly.

```
# use arrow_array::{Array, PrimitiveArray, types::Int32Type};
let arr = PrimitiveArray::<Int32Type>::from(vec![1, 2, 3, 4]);
let starting_ptr = arr.values().as_ptr();
// split into its parts
let (datatype, buffer, nulls) = arr.into_parts();
// Convert the buffer to a Vec<i32> (zero copy)
// (note this requires that there are no other references)
let mut vec: Vec<i32> = buffer.into();
vec[2] = 300;
// put the parts back together
let arr = PrimitiveArray::<Int32Type>::try_new(vec.into(), nulls).unwrap();
assert_eq!(arr.values(), &[1, 2, 300, 4]);
// The same allocation was used
assert_eq!(starting_ptr, arr.values().as_ptr());
```

# Example: From an optional Vec

```
# use arrow_array::{Array, PrimitiveArray, types::Int32Type};
let arr: PrimitiveArray<Int32Type> = vec![Some(1), None, Some(3), None].into();
assert_eq!(4, arr.len());
assert_eq!(2, arr.null_count());
// Note: values for null indexes are arbitrary
assert_eq!(arr.values(), &[1, 0, 3, 0])
```

# Example: From an iterator of values

```
# use arrow_array::{Array, PrimitiveArray, types::Int32Type};
let arr: PrimitiveArray<Int32Type> = (0..10).map(|x| x + 1).collect();
assert_eq!(10, arr.len());
assert_eq!(0, arr.null_count());
for i in 0..10i32 {
    assert_eq!(i + 1, arr.value(i as usize));
}
```

# Example: From an iterator of option

```
# use arrow_array::{Array, PrimitiveArray, types::Int32Type};
let arr: PrimitiveArray<Int32Type> = (0..10).map(|x| (x % 2 == 0).then_some(x)).collect();
assert_eq!(10, arr.len());
assert_eq!(5, arr.null_count());
// Note: values for null indexes are arbitrary
assert_eq!(arr.values(), &[0, 0, 2, 0, 4, 0, 6, 0, 8, 0])
```

# Example: Using Builder

```
# use arrow_array::Array;
# use arrow_array::builder::PrimitiveBuilder;
# use arrow_array::types::Int32Type;
let mut builder = PrimitiveBuilder::<Int32Type>::new();
builder.append_value(1);
builder.append_null();
builder.append_value(2);
let array = builder.finish();
// Note: values for null indexes are arbitrary
assert_eq!(array.values(), &[1, 0, 2]);
assert!(array.is_null(1));
```

# Example: Get a `PrimitiveArray` from an [`ArrayRef`]
```
# use std::sync::Arc;
# use arrow_array::{Array, cast::AsArray, ArrayRef, Float32Array, PrimitiveArray};
# use arrow_array::types::{Float32Type};
# use arrow_schema::DataType;
# let array: ArrayRef =  Arc::new(Float32Array::from(vec![1.2, 2.3]));
// will panic if the array is not a Float32Array
assert_eq!(&DataType::Float32, array.data_type());
let f32_array: Float32Array  = array.as_primitive().clone();
assert_eq!(f32_array, Float32Array::from(vec![1.2, 2.3]));
```

---

## Date32Array

`type_alias` · `arrow_array::array::primitive_array::Date32Array`

```rust
type Date32Array = PrimitiveArray<Date32Type>
```

A [`PrimitiveArray`] of days since UNIX epoch stored as `i32`

This type is similar to the [`chrono::NaiveDate`] type and can hold
values such as `2018-11-13`

---

## Date64Array

`type_alias` · `arrow_array::array::primitive_array::Date64Array`

```rust
type Date64Array = PrimitiveArray<Date64Type>
```

A [`PrimitiveArray`] of milliseconds since UNIX epoch stored as `i64`

This type is similar to the [`chrono::NaiveDate`] type and can hold
values such as `2018-11-13`

---

## Decimal128Array

`type_alias` · `arrow_array::array::primitive_array::Decimal128Array`

```rust
type Decimal128Array = PrimitiveArray<Decimal128Type>
```

A [`PrimitiveArray`] of 128-bit fixed point decimals

# Examples

Construction

```
# use arrow_array::Decimal128Array;
// Create from Vec<Option<i128>>
let arr = Decimal128Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<i128>
let arr = Decimal128Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: Decimal128Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`] for more information and examples

---

## Decimal256Array

`type_alias` · `arrow_array::array::primitive_array::Decimal256Array`

```rust
type Decimal256Array = PrimitiveArray<Decimal256Type>
```

A [`PrimitiveArray`] of 256-bit fixed point decimals

# Examples

Construction

```
# use arrow_array::Decimal256Array;
use arrow_buffer::i256;
// Create from Vec<Option<i256>>
let arr = Decimal256Array::from(vec![Some(i256::from(1)), None, Some(i256::from(2))]);
// Create from Vec<i256>
let arr = Decimal256Array::from(vec![i256::from(1), i256::from(2), i256::from(3)]);
// Create iter/collect
let arr: Decimal256Array = std::iter::repeat(i256::from(42)).take(10).collect();
```

See [`PrimitiveArray`] for more information and examples

---

## Decimal32Array

`type_alias` · `arrow_array::array::primitive_array::Decimal32Array`

```rust
type Decimal32Array = PrimitiveArray<Decimal32Type>
```

A [`PrimitiveArray`] of 32-bit fixed point decimals

# Examples

Construction

```
# use arrow_array::Decimal32Array;
// Create from Vec<Option<i32>>
let arr = Decimal32Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<i32>
let arr = Decimal32Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: Decimal32Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`] for more information and examples

---

## Decimal64Array

`type_alias` · `arrow_array::array::primitive_array::Decimal64Array`

```rust
type Decimal64Array = PrimitiveArray<Decimal64Type>
```

A [`PrimitiveArray`] of 64-bit fixed point decimals

# Examples

Construction

```
# use arrow_array::Decimal64Array;
// Create from Vec<Option<i64>>
let arr = Decimal64Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<i64>
let arr = Decimal64Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: Decimal64Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`] for more information and examples

---

## DurationMicrosecondArray

`type_alias` · `arrow_array::array::primitive_array::DurationMicrosecondArray`

```rust
type DurationMicrosecondArray = PrimitiveArray<DurationMicrosecondType>
```

A [`PrimitiveArray`] of elapsed durations in microseconds

---

## DurationMillisecondArray

`type_alias` · `arrow_array::array::primitive_array::DurationMillisecondArray`

```rust
type DurationMillisecondArray = PrimitiveArray<DurationMillisecondType>
```

A [`PrimitiveArray`] of elapsed durations in milliseconds

---

## DurationNanosecondArray

`type_alias` · `arrow_array::array::primitive_array::DurationNanosecondArray`

```rust
type DurationNanosecondArray = PrimitiveArray<DurationNanosecondType>
```

A [`PrimitiveArray`] of elapsed durations in nanoseconds

---

## DurationSecondArray

`type_alias` · `arrow_array::array::primitive_array::DurationSecondArray`

```rust
type DurationSecondArray = PrimitiveArray<DurationSecondType>
```

A [`PrimitiveArray`] of elapsed durations in seconds

---

## Float16Array

`type_alias` · `arrow_array::array::primitive_array::Float16Array`

```rust
type Float16Array = PrimitiveArray<Float16Type>
```

A [`PrimitiveArray`] of `f16`

# Examples

Construction

```
# use arrow_array::Float16Array;
use half::f16;
// Create from Vec<Option<f16>>
let arr = Float16Array::from(vec![Some(f16::from_f64(1.0)), Some(f16::from_f64(2.0))]);
// Create from Vec<i8>
let arr = Float16Array::from(vec![f16::from_f64(1.0), f16::from_f64(2.0), f16::from_f64(3.0)]);
// Create iter/collect
let arr: Float16Array = std::iter::repeat(f16::from_f64(1.0)).take(10).collect();
```

# Example: Using `collect`
```
# use arrow_array::Float16Array;
use half::f16;
let arr : Float16Array = [Some(f16::from_f64(1.0)), Some(f16::from_f64(2.0))].into_iter().collect();
```

See [`PrimitiveArray`] for more information and examples

---

## Float32Array

`type_alias` · `arrow_array::array::primitive_array::Float32Array`

```rust
type Float32Array = PrimitiveArray<Float32Type>
```

A [`PrimitiveArray`] of `f32`

# Examples

Construction

```
# use arrow_array::Float32Array;
// Create from Vec<Option<f32>>
let arr = Float32Array::from(vec![Some(1.0), None, Some(2.0)]);
// Create from Vec<f32>
let arr = Float32Array::from(vec![1.0, 2.0, 3.0]);
// Create iter/collect
let arr: Float32Array = std::iter::repeat(42.0).take(10).collect();
```

See [`PrimitiveArray`] for more information and examples

---

## Float64Array

`type_alias` · `arrow_array::array::primitive_array::Float64Array`

```rust
type Float64Array = PrimitiveArray<Float64Type>
```

A [`PrimitiveArray`] of `f64`

# Examples

Construction

```
# use arrow_array::Float64Array;
// Create from Vec<Option<f32>>
let arr = Float64Array::from(vec![Some(1.0), None, Some(2.0)]);
// Create from Vec<f32>
let arr = Float64Array::from(vec![1.0, 2.0, 3.0]);
// Create iter/collect
let arr: Float64Array = std::iter::repeat(42.0).take(10).collect();
```

See [`PrimitiveArray`] for more information and examples

---

## Int16Array

`type_alias` · `arrow_array::array::primitive_array::Int16Array`

```rust
type Int16Array = PrimitiveArray<Int16Type>
```

A [`PrimitiveArray`] of `i16`

# Examples

Construction

```
# use arrow_array::Int16Array;
// Create from Vec<Option<i16>>
let arr = Int16Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<i16>
let arr = Int16Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: Int16Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`] for more information and examples

---

## Int32Array

`type_alias` · `arrow_array::array::primitive_array::Int32Array`

```rust
type Int32Array = PrimitiveArray<Int32Type>
```

A [`PrimitiveArray`] of `i32`

# Examples

Construction

```
# use arrow_array::Int32Array;
// Create from Vec<Option<i32>>
let arr = Int32Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<i32>
let arr = Int32Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: Int32Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`] for more information and examples

---

## Int64Array

`type_alias` · `arrow_array::array::primitive_array::Int64Array`

```rust
type Int64Array = PrimitiveArray<Int64Type>
```

A [`PrimitiveArray`] of `i64`

# Examples

Construction

```
# use arrow_array::Int64Array;
// Create from Vec<Option<i64>>
let arr = Int64Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<i64>
let arr = Int64Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: Int64Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`] for more information and examples

---

## Int8Array

`type_alias` · `arrow_array::array::primitive_array::Int8Array`

```rust
type Int8Array = PrimitiveArray<Int8Type>
```

A [`PrimitiveArray`] of `i8`

# Examples

Construction

```
# use arrow_array::Int8Array;
// Create from Vec<Option<i8>>
let arr = Int8Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<i8>
let arr = Int8Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: Int8Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`] for more information and examples

---

## IntervalDayTimeArray

`type_alias` · `arrow_array::array::primitive_array::IntervalDayTimeArray`

```rust
type IntervalDayTimeArray = PrimitiveArray<IntervalDayTimeType>
```

A [`PrimitiveArray`] of “calendar” intervals in days and milliseconds

See [`IntervalDayTime`] for details on representation and caveats.

# Example
```
# use arrow_array::IntervalDayTimeArray;
use arrow_array::types::IntervalDayTime;
let array = IntervalDayTimeArray::from(vec![
  IntervalDayTime::new(1, 1000),                 // 1 day, 1000 milliseconds
  IntervalDayTime::new(33, 0),                  // 33 days, 0 milliseconds
  IntervalDayTime::new(0, 12 * 60 * 60 * 1000), // 0 days, 12 hours
]);
```

---

## IntervalMonthDayNanoArray

`type_alias` · `arrow_array::array::primitive_array::IntervalMonthDayNanoArray`

```rust
type IntervalMonthDayNanoArray = PrimitiveArray<IntervalMonthDayNanoType>
```

A [`PrimitiveArray`] of “calendar” intervals in  months, days, and nanoseconds.

See [`IntervalMonthDayNano`] for details on representation and caveats.

# Example
```
# use arrow_array::IntervalMonthDayNanoArray;
use arrow_array::types::IntervalMonthDayNano;
let array = IntervalMonthDayNanoArray::from(vec![
  IntervalMonthDayNano::new(1, 2, 1000),             // 1 month, 2 days, 1 nanosecond
  IntervalMonthDayNano::new(12, 1, 0),               // 12 months, 1 days, 0 nanoseconds
  IntervalMonthDayNano::new(0, 0, 12 * 1000 * 1000), // 0 days, 12 milliseconds
]);
```

---

## IntervalYearMonthArray

`type_alias` · `arrow_array::array::primitive_array::IntervalYearMonthArray`

```rust
type IntervalYearMonthArray = PrimitiveArray<IntervalYearMonthType>
```

A [`PrimitiveArray`] of “calendar” intervals in whole months

See [`IntervalYearMonthType`] for details on representation and caveats.

# Example
```
# use arrow_array::IntervalYearMonthArray;
let array = IntervalYearMonthArray::from(vec![
  2,  // 2 months
  25, // 2 years and 1 month
  -1  // -1 months
]);
```

---

## Time32MillisecondArray

`type_alias` · `arrow_array::array::primitive_array::Time32MillisecondArray`

```rust
type Time32MillisecondArray = PrimitiveArray<Time32MillisecondType>
```

A [`PrimitiveArray`] of milliseconds since midnight stored as `i32`

This type is similar to the [`chrono::NaiveTime`] type and can
hold values such as `00:02:00.123`

---

## Time32SecondArray

`type_alias` · `arrow_array::array::primitive_array::Time32SecondArray`

```rust
type Time32SecondArray = PrimitiveArray<Time32SecondType>
```

A [`PrimitiveArray`] of seconds since midnight stored as `i32`

This type is similar to the [`chrono::NaiveTime`] type and can
hold values such as `00:02:00`

---

## Time64MicrosecondArray

`type_alias` · `arrow_array::array::primitive_array::Time64MicrosecondArray`

```rust
type Time64MicrosecondArray = PrimitiveArray<Time64MicrosecondType>
```

A [`PrimitiveArray`] of microseconds since midnight stored as `i64`

This type is similar to the [`chrono::NaiveTime`] type and can
hold values such as `00:02:00.123456`

---

## Time64NanosecondArray

`type_alias` · `arrow_array::array::primitive_array::Time64NanosecondArray`

```rust
type Time64NanosecondArray = PrimitiveArray<Time64NanosecondType>
```

A [`PrimitiveArray`] of nanoseconds since midnight stored as `i64`

This type is similar to the [`chrono::NaiveTime`] type and can
hold values such as `00:02:00.123456789`

---

## TimestampMicrosecondArray

`type_alias` · `arrow_array::array::primitive_array::TimestampMicrosecondArray`

```rust
type TimestampMicrosecondArray = PrimitiveArray<TimestampMicrosecondType>
```

A [`PrimitiveArray`] of microseconds since UNIX epoch stored as `i64`

See examples for [`TimestampSecondArray`]

---

## TimestampMillisecondArray

`type_alias` · `arrow_array::array::primitive_array::TimestampMillisecondArray`

```rust
type TimestampMillisecondArray = PrimitiveArray<TimestampMillisecondType>
```

A [`PrimitiveArray`] of milliseconds since UNIX epoch stored as `i64`

See examples for [`TimestampSecondArray`]

---

## TimestampNanosecondArray

`type_alias` · `arrow_array::array::primitive_array::TimestampNanosecondArray`

```rust
type TimestampNanosecondArray = PrimitiveArray<TimestampNanosecondType>
```

A [`PrimitiveArray`] of nanoseconds since UNIX epoch stored as `i64`

See examples for [`TimestampSecondArray`]

---

## TimestampSecondArray

`type_alias` · `arrow_array::array::primitive_array::TimestampSecondArray`

```rust
type TimestampSecondArray = PrimitiveArray<TimestampSecondType>
```

A [`PrimitiveArray`] of seconds since UNIX epoch stored as `i64`

This type is similar to the [`chrono::DateTime`] type and can hold
values such as `1970-05-09 14:25:11 +01:00`

See also [`Timestamp`](arrow_schema::DataType::Timestamp).

# Example: UTC timestamps post epoch
```
# use arrow_array::TimestampSecondArray;
use arrow_array::timezone::Tz;
// Corresponds to single element array with entry 1970-05-09T14:25:11+0:00
let arr = TimestampSecondArray::from(vec![11111111]);
// OR
let arr = TimestampSecondArray::from(vec![Some(11111111)]);
let utc_tz: Tz = "+00:00".parse().unwrap();

assert_eq!(arr.value_as_datetime_with_tz(0, utc_tz).map(|v| v.to_string()).unwrap(), "1970-05-09 14:25:11 +00:00")
```

# Example: UTC timestamps pre epoch
```
# use arrow_array::TimestampSecondArray;
use arrow_array::timezone::Tz;
// Corresponds to single element array with entry 1969-08-25T09:34:49+0:00
let arr = TimestampSecondArray::from(vec![-11111111]);
// OR
let arr = TimestampSecondArray::from(vec![Some(-11111111)]);
let utc_tz: Tz = "+00:00".parse().unwrap();

assert_eq!(arr.value_as_datetime_with_tz(0, utc_tz).map(|v| v.to_string()).unwrap(), "1969-08-25 09:34:49 +00:00")
```

# Example: With timezone specified
```
# use arrow_array::TimestampSecondArray;
use arrow_array::timezone::Tz;
// Corresponds to single element array with entry 1970-05-10T00:25:11+10:00
let arr = TimestampSecondArray::from(vec![11111111]).with_timezone("+10:00".to_string());
// OR
let arr = TimestampSecondArray::from(vec![Some(11111111)]).with_timezone("+10:00".to_string());
let sydney_tz: Tz = "+10:00".parse().unwrap();

assert_eq!(arr.value_as_datetime_with_tz(0, sydney_tz).map(|v| v.to_string()).unwrap(), "1970-05-10 00:25:11 +10:00")
```

See [`PrimitiveArray`] for more information and examples

---

## UInt16Array

`type_alias` · `arrow_array::array::primitive_array::UInt16Array`

```rust
type UInt16Array = PrimitiveArray<UInt16Type>
```

A [`PrimitiveArray`] of `u16`

# Examples

Construction

```
# use arrow_array::UInt16Array;
// Create from Vec<Option<u16>>
let arr = UInt16Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<u16>
let arr = UInt16Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: UInt16Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`] for more information and examples

---

## UInt32Array

`type_alias` · `arrow_array::array::primitive_array::UInt32Array`

```rust
type UInt32Array = PrimitiveArray<UInt32Type>
```

A [`PrimitiveArray`] of `u32`

# Examples

Construction

```
# use arrow_array::UInt32Array;
// Create from Vec<Option<u32>>
let arr = UInt32Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<u32>
let arr = UInt32Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: UInt32Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`] for more information and examples

---

## UInt64Array

`type_alias` · `arrow_array::array::primitive_array::UInt64Array`

```rust
type UInt64Array = PrimitiveArray<UInt64Type>
```

A [`PrimitiveArray`] of `u64`

# Examples

Construction

```
# use arrow_array::UInt64Array;
// Create from Vec<Option<u64>>
let arr = UInt64Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<u64>
let arr = UInt64Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: UInt64Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`] for more information and examples

---

## UInt8Array

`type_alias` · `arrow_array::array::primitive_array::UInt8Array`

```rust
type UInt8Array = PrimitiveArray<UInt8Type>
```

A [`PrimitiveArray`] of `u8`

# Examples

Construction

```
# use arrow_array::UInt8Array;
// Create from Vec<Option<u8>>
let arr = UInt8Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<u8>
let arr = UInt8Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: UInt8Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`] for more information and examples

---
