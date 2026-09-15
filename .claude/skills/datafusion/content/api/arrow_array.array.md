# `arrow_array::array`

Crate `arrow-array` · 9 public items · structured records in [`model/arrow_array.array.json`](../model/arrow_array.array.json)

## make_array

`function` · `arrow_array::array::make_array`

Also reachable as `arrow::array::make_array`, `arrow_array::make_array`

```rust
fn make_array(data: arrow_data::ArrayData) -> ArrayRef
```

Constructs an [`ArrayRef`] from an [`ArrayData`].

# Notes:

It is more efficient to directly construct the concrete array type rather
than using this function as creating an `ArrayData` requires at least one
additional allocation (the Vec of buffers).

# Example:
```
# use std::sync::Arc;
# use arrow_data::ArrayData;
# use arrow_array::{make_array, ArrayRef, Int32Array};
# use arrow_buffer::{Buffer, ScalarBuffer};
# use arrow_schema::DataType;
// Create an Int32Array with values [1, 2, 3]
let values_buffer = Buffer::from_slice_ref(&[1, 2, 3]);
// ArrayData can be constructed using ArrayDataBuilder
 let builder = ArrayData::builder(DataType::Int32)
   .len(3)
   .add_buffer(values_buffer.clone());
let array_data = builder.build().unwrap();
// Create the ArrayRef from the ArrayData
let array = make_array(array_data);

// It is equivalent to directly constructing the Int32Array
let scalar_buffer = ScalarBuffer::from(values_buffer);
let int32_array: ArrayRef = Arc::new(Int32Array::new(scalar_buffer, None));
assert_eq!(&array, &int32_array);
```

---

## new_empty_array

`function` · `arrow_array::array::new_empty_array`

Also reachable as `arrow::array::new_empty_array`, `arrow_array::new_empty_array`

```rust
fn new_empty_array(data_type: &arrow_schema::DataType) -> ArrayRef
```

Creates a new empty array

```
use std::sync::Arc;
use arrow_schema::DataType;
use arrow_array::{ArrayRef, Int32Array, new_empty_array};

let empty_array = new_empty_array(&DataType::Int32);
let array: ArrayRef = Arc::new(Int32Array::from(vec![] as Vec<i32>));

assert_eq!(&array, &empty_array);
```

---

## new_null_array

`function` · `arrow_array::array::new_null_array`

Also reachable as `arrow::array::new_null_array`, `arrow_array::new_null_array`

```rust
fn new_null_array(data_type: &arrow_schema::DataType, length: usize) -> ArrayRef
```

Creates a new array of `data_type` of length `length` filled
entirely of `NULL` values

```
use std::sync::Arc;
use arrow_schema::DataType;
use arrow_array::{ArrayRef, Int32Array, new_null_array};

let null_array = new_null_array(&DataType::Int32, 3);
let array: ArrayRef = Arc::new(Int32Array::from(vec![None, None, None]));

assert_eq!(&array, &null_array);
```

---

## Array

`trait` · `arrow_array::array::Array`

Also reachable as `arrow::array::Array`, `arrow_array::Array`

```rust
unsafe trait Array: std::fmt::Debug + Send + Sync
```

**Implementors** (17)

- `arrow_array::array::ArrayRef`
- `arrow_array::array::boolean_array::BooleanArray`
- `arrow_array::array::byte_array::GenericByteArray`
- `arrow_array::array::byte_view_array::GenericByteViewArray`
- `arrow_array::array::dictionary_array::DictionaryArray`
- `arrow_array::array::dictionary_array::TypedDictionaryArray`
- `arrow_array::array::fixed_size_binary_array::FixedSizeBinaryArray`
- `arrow_array::array::fixed_size_list_array::FixedSizeListArray`
- `arrow_array::array::list_array::GenericListArray`
- `arrow_array::array::list_view_array::GenericListViewArray`
- `arrow_array::array::map_array::MapArray`
- `arrow_array::array::null_array::NullArray`
- `arrow_array::array::primitive_array::PrimitiveArray`
- `arrow_array::array::run_array::RunArray`
- `arrow_array::array::run_array::TypedRunArray`
- `arrow_array::array::struct_array::StructArray`
- `arrow_array::array::union_array::UnionArray`

**Methods** (19)

```rust
fn as_any(&self) -> &dyn Any
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
fn data_type(&self) -> &DataType
fn get_array_memory_size(&self) -> usize
fn get_buffer_memory_size(&self) -> usize
fn into_data(self) -> ArrayData
fn is_empty(&self) -> bool
fn is_null(&self, index: usize) -> bool
fn is_nullable(&self) -> bool
fn is_valid(&self, index: usize) -> bool
fn len(&self) -> usize
fn logical_null_count(&self) -> usize
fn logical_nulls(&self) -> Option<NullBuffer>
fn null_count(&self) -> usize
fn nulls(&self) -> Option<&NullBuffer>
fn offset(&self) -> usize
fn shrink_to_fit(&mut self)
fn slice(&self, offset: usize, length: usize) -> ArrayRef
fn to_data(&self) -> ArrayData
```

An array in the [Arrow Columnar Format](https://arrow.apache.org/docs/format/Columnar.html)

# Safety

Implementations of this trait must ensure that all methods implementations comply with
the Arrow specification. No safety guards are placed and failing to comply with it can
translate into panics or undefined behavior. For example, a value computed based on `len`
may be used as a direct index into memory regions without checks.

Note that it is likely impossible to correctly implement the trait for a
third party type, as substantial arrow-rs functionality is based on the
return values of [`Array::data_type`] and third party types cannot extend
the [`DataType`] enum. So any code that attempts casting based on data type
(including internal arrow library code) risks a panic or undefined behavior.
See [this discussion] for more details.

This trait might be sealed in the future. Use at your own risk.

[this discussion]: https://github.com/apache/arrow-rs/pull/9234#pullrequestreview-3708950936

---

## ArrayAccessor

`trait` · `arrow_array::array::ArrayAccessor`

Also reachable as `arrow::array::ArrayAccessor`, `arrow_array::ArrayAccessor`

```rust
trait ArrayAccessor: Array
```

**Implementors** (3)

- `arrow_array::array::dictionary_array::TypedDictionaryArray`
- `arrow_array::array::fixed_size_list_array::FixedSizeListArray`
- `arrow_array::array::run_array::TypedRunArray`

**Methods** (2)

```rust
fn value(&self, index: usize) -> Self::Item
unsafe fn value_unchecked(&self, index: usize) -> Self::Item
```

A generic trait for accessing the values of an [`Array`]

This trait helps write specialized implementations of algorithms for
different array types. Specialized implementations allow the compiler
to optimize the code for the specific array type, which can lead to
significant performance improvements.

# Example
For example, to write three different implementations of a string length function
for [`StringArray`], [`LargeStringArray`], and [`StringViewArray`], you can write

```
# use std::sync::Arc;
# use arrow_array::{ArrayAccessor, ArrayRef, ArrowPrimitiveType, OffsetSizeTrait, PrimitiveArray};
# use arrow_buffer::ArrowNativeType;
# use arrow_array::cast::AsArray;
# use arrow_array::iterator::ArrayIter;
# use arrow_array::types::{Int32Type, Int64Type};
# use arrow_schema::{ArrowError, DataType};
/// This function takes a dynamically typed `ArrayRef` and calls
/// calls one of three specialized implementations
fn character_length(arg: ArrayRef) -> Result<ArrayRef, ArrowError> {
    match arg.data_type() {
        DataType::Utf8 => {
            // downcast the ArrayRef to a StringArray and call the specialized implementation
            let string_array = arg.as_string::<i32>();
            character_length_general::<Int32Type, _>(string_array)
        }
        DataType::LargeUtf8 => {
            character_length_general::<Int64Type, _>(arg.as_string::<i64>())
        }
        DataType::Utf8View => {
            character_length_general::<Int32Type, _>(arg.as_string_view())
        }
        _ => Err(ArrowError::InvalidArgumentError("Unsupported data type".to_string())),
    }
}

/// A generic implementation of the character_length function
/// This function uses the `ArrayAccessor` trait to access the values of the array
/// so the compiler can generated specialized implementations for different array types
///
/// Returns a new array with the length of each string in the input array
/// * Int32Array for Utf8 and Utf8View arrays (lengths are 32-bit integers)
/// * Int64Array for LargeUtf8 arrays (lengths are 64-bit integers)
///
/// This is generic on the type of the primitive array (different string arrays have
/// different lengths) and the type of the array accessor (different string arrays
/// have different ways to access the values)
fn character_length_general<'a, T: ArrowPrimitiveType, V: ArrayAccessor<Item = &'a str>>(
    array: V,
) -> Result<ArrayRef, ArrowError>
where
    T::Native: OffsetSizeTrait,
{
    let iter = ArrayIter::new(array);
    // Create a Int32Array / Int64Array with the length of each string
    let result = iter
        .map(|string| {
            string.map(|string: &str| {
                T::Native::from_usize(string.chars().count())
                    .expect("should not fail as string.chars will always return integer")
            })
        })
        .collect::<PrimitiveArray<T>>();

    /// Return the result as a new ArrayRef (dynamically typed)
    Ok(Arc::new(result) as ArrayRef)
}
```

# Validity

An [`ArrayAccessor`] must always return a well-defined value for an index
that is within the bounds `0..Array::len`, including for null indexes where
[`Array::is_null`] is true.

The value at null indexes is unspecified, and implementations must not rely
on a specific value such as [`Default::default`] being returned, however, it
must not be undefined

---

## BinaryArrayType

`trait` · `arrow_array::array::BinaryArrayType`

Also reachable as `arrow::array::BinaryArrayType`, `arrow_array::BinaryArrayType`

```rust
trait BinaryArrayType<'a>: ArrayAccessor<Item = &'a [u8]> + Sized
```

**Methods** (1)

```rust
fn iter(&self) -> ArrayIter<Self>
```

A trait for Arrow Binary Arrays, currently four types are supported:
- `BinaryArray`
- `LargeBinaryArray`
- `BinaryViewArray`
- `FixedSizeBinaryArray`

This trait helps to abstract over the different types of binary arrays
so that we don't need to duplicate the implementation for each type.

---

## ListLikeArray

`trait` · `arrow_array::array::ListLikeArray`

Also reachable as `arrow::array::ListLikeArray`, `arrow_array::ListLikeArray`

```rust
trait ListLikeArray: Array
```

**Implementors** (3)

- `arrow_array::array::fixed_size_list_array::FixedSizeListArray`
- `arrow_array::array::list_array::GenericListArray`
- `arrow_array::array::list_view_array::GenericListViewArray`

**Methods** (2)

```rust
fn element_range(&self, index: usize) -> std::ops::Range<usize>
fn values(&self) -> &ArrayRef
```

A trait for Arrow list-like arrays, abstracting over
[`GenericListArray`], [`GenericListViewArray`], and [`FixedSizeListArray`].

This trait provides a uniform interface for accessing the child values and
computing the element range for a given index, regardless of the underlying
list layout (offsets, offsets+sizes, or fixed-size).

---

## StringArrayType

`trait` · `arrow_array::array::StringArrayType`

Also reachable as `arrow::array::StringArrayType`, `arrow::compute::StringArrayType`, `arrow::compute::kernels::comparison::StringArrayType`, `arrow_array::StringArrayType`, `arrow_string::like::StringArrayType`

```rust
trait StringArrayType<'a>: ArrayAccessor<Item = &'a str> + Sized
```

**Methods** (2)

```rust
fn is_ascii(&self) -> bool
fn iter(&self) -> ArrayIter<Self>
```

A trait for Arrow String Arrays, currently three types are supported:
- `StringArray`
- `LargeStringArray`
- `StringViewArray`

This trait helps to abstract over the different types of string arrays
so that we don't need to duplicate the implementation for each type.

---

## ArrayRef

`type_alias` · `arrow_array::array::ArrayRef`

Also reachable as `arrow::array::ArrayRef`, `arrow_array::ArrayRef`

```rust
type ArrayRef = std::sync::Arc<dyn Array>
```

**Implements**: `arrow_array::array::Array`, `arrow_array::cast::AsArray`, `core::convert::From`, `core::convert::TryFrom`, `datafusion_common::hash_utils::AsDynArray`

**via `arrow_array::array::Array`**

```rust
fn as_any(&self) -> &dyn Any
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
fn data_type(&self) -> &DataType
fn get_array_memory_size(&self) -> usize
fn get_buffer_memory_size(&self) -> usize
fn into_data(self) -> ArrayData
fn is_empty(&self) -> bool
fn is_null(&self, index: usize) -> bool
fn is_nullable(&self) -> bool
fn is_valid(&self, index: usize) -> bool
fn len(&self) -> usize
fn logical_null_count(&self) -> usize
fn logical_nulls(&self) -> Option<NullBuffer>
fn null_count(&self) -> usize
fn nulls(&self) -> Option<&NullBuffer>
fn offset(&self) -> usize
fn shrink_to_fit(&mut self)
fn slice(&self, offset: usize, length: usize) -> ArrayRef
fn to_data(&self) -> ArrayData
```

**via `arrow_array::cast::AsArray`**

```rust
fn as_any_dictionary_opt(&self) -> Option<&dyn AnyDictionaryArray>
fn as_any_ree_opt(&self) -> Option<&dyn AnyRunEndArray>
fn as_boolean_opt(&self) -> Option<&BooleanArray>
fn as_byte_view_opt<T: ByteViewType>(&self) -> Option<&GenericByteViewArray<T>>
fn as_bytes_opt<T: ByteArrayType>(&self) -> Option<&GenericByteArray<T>>
fn as_dictionary_opt<K: ArrowDictionaryKeyType>(&self) -> Option<&DictionaryArray<K>>
fn as_fixed_size_binary_opt(&self) -> Option<&FixedSizeBinaryArray>
fn as_fixed_size_list_opt(&self) -> Option<&FixedSizeListArray>
fn as_list_opt<O: OffsetSizeTrait>(&self) -> Option<&GenericListArray<O>>
fn as_list_view_opt<O: OffsetSizeTrait>(&self) -> Option<&GenericListViewArray<O>>
fn as_map_opt(&self) -> Option<&MapArray>
fn as_primitive_opt<T: ArrowPrimitiveType>(&self) -> Option<&PrimitiveArray<T>>
fn as_run_opt<K: RunEndIndexType>(&self) -> Option<&RunArray<K>>
fn as_string_opt<O: OffsetSizeTrait>(&self) -> Option<&GenericStringArray<O>>
fn as_struct_opt(&self) -> Option<&StructArray>
fn as_union_opt(&self) -> Option<&UnionArray>
```

A reference-counted reference to a generic `Array`

---
