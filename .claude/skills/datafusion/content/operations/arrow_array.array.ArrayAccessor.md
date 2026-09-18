# `arrow_array::array::ArrayAccessor`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.ArrayAccessor.json).

<a id="op-0f7f2e64730382bf2e8d3393"></a>
## ArrayAccessor

`trait` · `arrow_array::array::ArrayAccessor` · arrow-array 59.3.0

```rust
trait ArrayAccessor: Array
```

Source: `src/array/mod.rs:671`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A generic trait for accessing the values of an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21)

This trait helps write specialized implementations of algorithms for
different array types. Specialized implementations allow the compiler
to optimize the code for the specific array type, which can lead to
significant performance improvements.

# Example
For example, to write three different implementations of a string length function
for [`StringArray`](../operations/arrow_array.array.string_array.StringArray.md#op-5d32f770159d415652a55945), [`LargeStringArray`](../operations/arrow_array.array.string_array.LargeStringArray.md#op-829f12b9a45fbfe752a7ee52), and [`StringViewArray`](../operations/arrow_array.array.byte_view_array.StringViewArray.md#op-f468d0a8f2aaecdb58e1f8c0), you can write

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

An [`ArrayAccessor`](../operations/arrow_array.array.ArrayAccessor.md#op-0f7f2e64730382bf2e8d3393) must always return a well-defined value for an index
that is within the bounds `0..Array::len`, including for null indexes where
[`Array::is_null`](../operations/arrow_array.array.Array.md#op-fc6aff0e0247c3255575b6e1) is true.

The value at null indexes is unspecified, and implementations must not rely
on a specific value such as [`Default::default`] being returned, however, it
must not be undefined

Unresolved upstream links (retained, not inferred): ``Default::default``.

<a id="op-793e0167132c7b9b97b3a96c"></a>
## Item

`assoc_type` · `arrow_array::array::ArrayAccessor::Item` · arrow-array 59.3.0

```rust
Item
```

Source: `src/array/mod.rs:673`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

The Arrow type of the element being accessed.

<a id="op-2067e59793bedd52ef0a79e7"></a>
## value

`function` · `arrow_array::array::ArrayAccessor::value` · arrow-array 59.3.0

```rust
fn value(&self, index: usize) -> Self::Item
```

Source: `src/array/mod.rs:678`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the element at index `i`
# Panics
Panics if the value is outside the bounds of the array

<a id="op-dd722747ff758f5cb4fccda1"></a>
## value_unchecked

`function` · `arrow_array::array::ArrayAccessor::value_unchecked` · arrow-array 59.3.0

```rust
unsafe fn value_unchecked(&self, index: usize) -> Self::Item
```

Source: `src/array/mod.rs:683`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the element at index `i`
# Safety
Caller is responsible for ensuring that the index is within the bounds of the array
