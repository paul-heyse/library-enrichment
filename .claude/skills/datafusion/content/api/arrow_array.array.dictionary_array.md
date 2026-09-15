# `arrow_array::array::dictionary_array`

Crate `arrow-array` · 11 public items · structured records in [`model/arrow_array.array.dictionary_array.json`](../model/arrow_array.array.dictionary_array.json)

## DictionaryArray

`struct` · `arrow_array::array::dictionary_array::DictionaryArray`

```rust
struct DictionaryArray<K: ArrowDictionaryKeyType>
```

**Implements**: `arrow_array::array::Array`, `arrow_array::array::dictionary_array::AnyDictionaryArray`, `core::convert::From`, `core::iter::traits::collect::FromIterator`

**Derives**: Clone, Debug, PartialEq

**Methods** (20)

```rust
fn downcast_dict<V: 'static>(&self) -> Option<TypedDictionaryArray<'_, K, V>>
fn into_parts(self) -> (PrimitiveArray<K>, ArrayRef)
fn into_primitive_dict_builder<V>(self) -> Result<PrimitiveDictionaryBuilder<K, V>, Self> where V: ArrowPrimitiveType
fn is_empty(&self) -> bool
fn is_ordered(&self) -> bool
fn key(&self, i: usize) -> Option<usize>
fn keys(&self) -> &PrimitiveArray<K>
fn keys_iter(&self) -> impl Iterator<Item = Option<usize>> + '_
fn len(&self) -> usize
fn lookup_key(&self, value: &str) -> Option<K::Native>
fn new(keys: PrimitiveArray<K>, values: ArrayRef) -> Self
fn new_scalar<T: Array + 'static>(value: Scalar<T>) -> Scalar<Self>
unsafe fn new_unchecked(keys: PrimitiveArray<K>, values: ArrayRef) -> Self
fn occupancy(&self) -> BooleanBuffer
fn slice(&self, offset: usize, length: usize) -> Self
fn try_new(keys: PrimitiveArray<K>, values: ArrayRef) -> Result<Self, ArrowError>
fn unary_mut<F, V>(self, op: F) -> Result<DictionaryArray<K>, DictionaryArray<K>> where V: ArrowPrimitiveType, F: Fn(V::Native) -> V::Native
fn value_type(&self) -> DataType
fn values(&self) -> &ArrayRef
fn with_values(&self, values: ArrayRef) -> Self
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
fn is_nullable(&self) -> bool
fn len(&self) -> usize
fn logical_null_count(&self) -> usize
fn logical_nulls(&self) -> Option<NullBuffer>
fn nulls(&self) -> Option<&NullBuffer>
fn offset(&self) -> usize
fn shrink_to_fit(&mut self)
fn slice(&self, offset: usize, length: usize) -> ArrayRef
fn to_data(&self) -> ArrayData
```

**via `arrow_array::array::dictionary_array::AnyDictionaryArray`**

```rust
fn keys(&self) -> &dyn Array
fn normalized_keys(&self) -> Vec<usize>
fn values(&self) -> &ArrayRef
fn with_values(&self, values: ArrayRef) -> ArrayRef
```

**via `core::convert::From`**

```rust
fn from(data: ArrayData) -> Self
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = Option<&'a str>>>(iter: I) -> Self
fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self
```

An array of [dictionary encoded values](https://arrow.apache.org/docs/format/Columnar.html#dictionary-encoded-layout)

This is mostly used to represent strings or a limited set of primitive types as integers,
for example when doing NLP analysis or representing chromosomes by name.

[`DictionaryArray`] are represented using a `keys` array and a
`values` array, which may be different lengths. The `keys` array
stores indexes in the `values` array which holds
the corresponding logical value, as shown here:

```text
┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
  ┌─────────────────┐  ┌─────────┐ │     ┌─────────────────┐
│ │        A        │  │    0    │       │        A        │     values[keys[0]]
  ├─────────────────┤  ├─────────┤ │     ├─────────────────┤
│ │        D        │  │    2    │       │        B        │     values[keys[1]]
  ├─────────────────┤  ├─────────┤ │     ├─────────────────┤
│ │        B        │  │    2    │       │        B        │     values[keys[2]]
  └─────────────────┘  ├─────────┤ │     ├─────────────────┤
│                      │    1    │       │        D        │     values[keys[3]]
                       ├─────────┤ │     ├─────────────────┤
│                      │    1    │       │        D        │     values[keys[4]]
                       ├─────────┤ │     ├─────────────────┤
│                      │    0    │       │        A        │     values[keys[5]]
                       └─────────┘ │     └─────────────────┘
│       values            keys
 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘
                                            Logical array
                                               Contents
          DictionaryArray
             length = 6
```

# Example: From Nullable Data

```
# use arrow_array::{DictionaryArray, Int8Array, types::Int8Type};
let test = vec!["a", "a", "b", "c"];
let array : DictionaryArray<Int8Type> = test.iter().map(|&x| if x == "b" {None} else {Some(x)}).collect();
assert_eq!(array.keys(), &Int8Array::from(vec![Some(0), Some(0), None, Some(1)]));
```

# Example: From Non-Nullable Data

```
# use arrow_array::{DictionaryArray, Int8Array, types::Int8Type};
let test = vec!["a", "a", "b", "c"];
let array : DictionaryArray<Int8Type> = test.into_iter().collect();
assert_eq!(array.keys(), &Int8Array::from(vec![0, 0, 1, 2]));
```

# Example: From Existing Arrays

```
# use std::sync::Arc;
# use arrow_array::{DictionaryArray, Int8Array, StringArray, types::Int8Type};
// You can form your own DictionaryArray by providing the
// values (dictionary) and keys (indexes into the dictionary):
let values = StringArray::from_iter_values(["a", "b", "c"]);
let keys = Int8Array::from_iter_values([0, 0, 1, 2]);
let array = DictionaryArray::<Int8Type>::try_new(keys, Arc::new(values)).unwrap();
let expected: DictionaryArray::<Int8Type> = vec!["a", "a", "b", "c"].into_iter().collect();
assert_eq!(&array, &expected);
```

# Example: Using Builder

```
# use arrow_array::{Array, StringArray};
# use arrow_array::builder::StringDictionaryBuilder;
# use arrow_array::types::Int32Type;
let mut builder = StringDictionaryBuilder::<Int32Type>::new();
builder.append_value("a");
builder.append_null();
builder.append_value("a");
builder.append_value("b");
let array = builder.finish();

let values: Vec<_> = array.downcast_dict::<StringArray>().unwrap().into_iter().collect();
assert_eq!(&values, &[Some("a"), None, Some("a"), Some("b")]);
```

---

## TypedDictionaryArray

`struct` · `arrow_array::array::dictionary_array::TypedDictionaryArray`

```rust
struct TypedDictionaryArray<'a, K: ArrowDictionaryKeyType, V>
```

**Implements**: `arrow_array::array::Array`, `arrow_array::array::ArrayAccessor`, `core::iter::traits::collect::IntoIterator`

**Derives**: Clone, Copy, Debug

**Methods** (2)

```rust
fn keys(&self) -> &'a PrimitiveArray<K>
fn values(&self) -> &'a V
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
fn is_nullable(&self) -> bool
fn len(&self) -> usize
fn logical_null_count(&self) -> usize
fn logical_nulls(&self) -> Option<NullBuffer>
fn nulls(&self) -> Option<&NullBuffer>
fn offset(&self) -> usize
fn slice(&self, offset: usize, length: usize) -> ArrayRef
fn to_data(&self) -> ArrayData
```

**via `arrow_array::array::ArrayAccessor`**

```rust
fn value(&self, index: usize) -> Self::Item
unsafe fn value_unchecked(&self, index: usize) -> Self::Item
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

A [`DictionaryArray`] typed on its child values array

Implements [`ArrayAccessor`] allowing fast access to its elements

```
use arrow_array::{DictionaryArray, StringArray, types::Int32Type};

let orig = ["a", "b", "a", "b"];
let dictionary = DictionaryArray::<Int32Type>::from_iter(orig);

// `TypedDictionaryArray` allows you to access the values directly
let typed = dictionary.downcast_dict::<StringArray>().unwrap();

for (maybe_val, orig) in typed.into_iter().zip(orig) {
    assert_eq!(maybe_val.unwrap(), orig)
}
```

---

## AnyDictionaryArray

`trait` · `arrow_array::array::dictionary_array::AnyDictionaryArray`

```rust
trait AnyDictionaryArray: Array
```

**Implementors** (1)

- `arrow_array::array::dictionary_array::DictionaryArray`

**Methods** (4)

```rust
fn keys(&self) -> &dyn Array
fn normalized_keys(&self) -> Vec<usize>
fn values(&self) -> &ArrayRef
fn with_values(&self, values: ArrayRef) -> ArrayRef
```

A [`DictionaryArray`] with the key type erased

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

See [`AsArray::as_any_dictionary_opt`] and [`AsArray::as_any_dictionary`]

---

## Int16DictionaryArray

`type_alias` · `arrow_array::array::dictionary_array::Int16DictionaryArray`

```rust
type Int16DictionaryArray = DictionaryArray<Int16Type>
```

A [`DictionaryArray`] indexed by `i16`

# Example: Using `collect`
```
# use arrow_array::{Array, Int16DictionaryArray, Int16Array, StringArray};
# use std::sync::Arc;

let array: Int16DictionaryArray = vec!["a", "a", "b", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.keys(), &Int16Array::from(vec![0, 0, 1, 2]));
assert_eq!(array.values(), &values);
```

See [`DictionaryArray`] for more information and examples

---

## Int32DictionaryArray

`type_alias` · `arrow_array::array::dictionary_array::Int32DictionaryArray`

```rust
type Int32DictionaryArray = DictionaryArray<Int32Type>
```

A [`DictionaryArray`] indexed by `i32`

# Example: Using `collect`
```
# use arrow_array::{Array, Int32DictionaryArray, Int32Array, StringArray};
# use std::sync::Arc;

let array: Int32DictionaryArray = vec!["a", "a", "b", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.keys(), &Int32Array::from(vec![0, 0, 1, 2]));
assert_eq!(array.values(), &values);
```

See [`DictionaryArray`] for more information and examples

---

## Int64DictionaryArray

`type_alias` · `arrow_array::array::dictionary_array::Int64DictionaryArray`

```rust
type Int64DictionaryArray = DictionaryArray<Int64Type>
```

A [`DictionaryArray`] indexed by `i64`

# Example: Using `collect`
```
# use arrow_array::{Array, Int64DictionaryArray, Int64Array, StringArray};
# use std::sync::Arc;

let array: Int64DictionaryArray = vec!["a", "a", "b", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.keys(), &Int64Array::from(vec![0, 0, 1, 2]));
assert_eq!(array.values(), &values);
```

See [`DictionaryArray`] for more information and examples

---

## Int8DictionaryArray

`type_alias` · `arrow_array::array::dictionary_array::Int8DictionaryArray`

```rust
type Int8DictionaryArray = DictionaryArray<Int8Type>
```

A [`DictionaryArray`] indexed by `i8`

# Example: Using `collect`
```
# use arrow_array::{Array, Int8DictionaryArray, Int8Array, StringArray};
# use std::sync::Arc;

let array: Int8DictionaryArray = vec!["a", "a", "b", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.keys(), &Int8Array::from(vec![0, 0, 1, 2]));
assert_eq!(array.values(), &values);
```

See [`DictionaryArray`] for more information and examples

---

## UInt16DictionaryArray

`type_alias` · `arrow_array::array::dictionary_array::UInt16DictionaryArray`

```rust
type UInt16DictionaryArray = DictionaryArray<UInt16Type>
```

A [`DictionaryArray`] indexed by `u16`

# Example: Using `collect`
```
# use arrow_array::{Array, UInt16DictionaryArray, UInt16Array, StringArray};
# use std::sync::Arc;

let array: UInt16DictionaryArray = vec!["a", "a", "b", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.keys(), &UInt16Array::from(vec![0, 0, 1, 2]));
assert_eq!(array.values(), &values);
```

See [`DictionaryArray`] for more information and examples

---

## UInt32DictionaryArray

`type_alias` · `arrow_array::array::dictionary_array::UInt32DictionaryArray`

```rust
type UInt32DictionaryArray = DictionaryArray<UInt32Type>
```

A [`DictionaryArray`] indexed by `u32`

# Example: Using `collect`
```
# use arrow_array::{Array, UInt32DictionaryArray, UInt32Array, StringArray};
# use std::sync::Arc;

let array: UInt32DictionaryArray = vec!["a", "a", "b", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.keys(), &UInt32Array::from(vec![0, 0, 1, 2]));
assert_eq!(array.values(), &values);
```

See [`DictionaryArray`] for more information and examples

---

## UInt64DictionaryArray

`type_alias` · `arrow_array::array::dictionary_array::UInt64DictionaryArray`

```rust
type UInt64DictionaryArray = DictionaryArray<UInt64Type>
```

A [`DictionaryArray`] indexed by `u64`

# Example: Using `collect`
```
# use arrow_array::{Array, UInt64DictionaryArray, UInt64Array, StringArray};
# use std::sync::Arc;

let array: UInt64DictionaryArray = vec!["a", "a", "b", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.keys(), &UInt64Array::from(vec![0, 0, 1, 2]));
assert_eq!(array.values(), &values);
```

See [`DictionaryArray`] for more information and examples

---

## UInt8DictionaryArray

`type_alias` · `arrow_array::array::dictionary_array::UInt8DictionaryArray`

```rust
type UInt8DictionaryArray = DictionaryArray<UInt8Type>
```

A [`DictionaryArray`] indexed by `u8`

# Example: Using `collect`
```
# use arrow_array::{Array, UInt8DictionaryArray, UInt8Array, StringArray};
# use std::sync::Arc;

let array: UInt8DictionaryArray = vec!["a", "a", "b", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.keys(), &UInt8Array::from(vec![0, 0, 1, 2]));
assert_eq!(array.values(), &values);
```

See [`DictionaryArray`] for more information and examples

---
