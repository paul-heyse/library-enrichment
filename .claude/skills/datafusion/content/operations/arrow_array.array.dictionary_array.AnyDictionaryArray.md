# `arrow_array::array::dictionary_array::AnyDictionaryArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.dictionary_array.AnyDictionaryArray.json).

<a id="op-5955577f57f524b623a2af12"></a>
## AnyDictionaryArray

`trait` · `arrow_array::array::dictionary_array::AnyDictionaryArray` · arrow-array 59.3.0

```rust
trait AnyDictionaryArray: Array
```

Source: `src/array/dictionary_array.rs:1016`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) with the key type erased

This can be used to efficiently implement kernels for all possible dictionary
keys without needing to create specialized implementations for each key type

For example

```
# use arrow_array::*;
# use arrow_array::cast::AsArray;
# use arrow_array::builder::PrimitiveDictionaryBuilder;
# use arrow_array::types::*;
# use arrow_schema::ArrowError;
# use std::sync::Arc;

fn to_string(a: &dyn Array) -> Result<ArrayRef, ArrowError> {
    if let Some(d) = a.as_any_dictionary_opt() {
        // Recursively handle dictionary input
        let r = to_string(d.values().as_ref())?;
        return Ok(d.with_values(r));
    }
    downcast_primitive_array! {
        a => Ok(Arc::new(a.iter().map(|x| x.map(|x| format!("{x:?}"))).collect::<StringArray>())),
        d => Err(ArrowError::InvalidArgumentError(format!("{d:?} not supported")))
    }
}

let result = to_string(&Int32Array::from(vec![1, 2, 3])).unwrap();
let actual = result.as_string::<i32>().iter().map(Option::unwrap).collect::<Vec<_>>();
assert_eq!(actual, &["1", "2", "3"]);

let mut dict = PrimitiveDictionaryBuilder::<Int32Type, UInt16Type>::new();
dict.extend([Some(1), Some(1), Some(2), Some(3), Some(2)]);
let dict = dict.finish();

let r = to_string(&dict).unwrap();
let r = r.as_dictionary::<Int32Type>().downcast_dict::<StringArray>().unwrap();
assert_eq!(r.keys(), dict.keys()); // Keys are the same

let actual = r.into_iter().map(Option::unwrap).collect::<Vec<_>>();
assert_eq!(actual, &["1", "1", "2", "3", "2"]);
```

See [`AsArray::as_any_dictionary_opt`](../operations/arrow_array.cast.AsArray.md#op-ea0c096d64b7dccb3a293969) and [`AsArray::as_any_dictionary`](../operations/arrow_array.cast.AsArray.md#op-936d9f41d40b7be15147c47e)

<a id="op-452de658f9cbb8c489956037"></a>
## keys

`function` · `arrow_array::array::dictionary_array::AnyDictionaryArray::keys` · arrow-array 59.3.0

```rust
fn keys(&self) -> &dyn Array
```

Source: `src/array/dictionary_array.rs:1018`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the primitive keys of this dictionary as an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21)

<a id="op-c8e26bb608b4b944255fd4ea"></a>
## normalized_keys

`function` · `arrow_array::array::dictionary_array::AnyDictionaryArray::normalized_keys` · arrow-array 59.3.0

```rust
fn normalized_keys(&self) -> Vec<usize>
```

Source: `src/array/dictionary_array.rs:1031`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the keys of this dictionary as usize

The values for nulls will be arbitrary, but are guaranteed
to be in the range `0..self.values.len()`

# Panic

Panics if `values.len() == 0`

<a id="op-f15de9c6d4108e4e945b0ea4"></a>
## values

`function` · `arrow_array::array::dictionary_array::AnyDictionaryArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &ArrayRef
```

Source: `src/array/dictionary_array.rs:1021`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the values of this dictionary

<a id="op-f4f33a7a89d4905c14c46961"></a>
## with_values

`function` · `arrow_array::array::dictionary_array::AnyDictionaryArray::with_values` · arrow-array 59.3.0

```rust
fn with_values(&self, values: ArrayRef) -> ArrayRef
```

Source: `src/array/dictionary_array.rs:1036`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) replacing `values` with the new values

See [`DictionaryArray::with_values`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-0282e1ff97cb915672985c5e)
