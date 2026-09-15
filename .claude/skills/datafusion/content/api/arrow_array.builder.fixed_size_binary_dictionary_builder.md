# `arrow_array::builder::fixed_size_binary_dictionary_builder`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.builder.fixed_size_binary_dictionary_builder.json`](../model/arrow_array.builder.fixed_size_binary_dictionary_builder.json)

## FixedSizeBinaryDictionaryBuilder

`struct` · `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder`

```rust
struct FixedSizeBinaryDictionaryBuilder<K> where K: ArrowDictionaryKeyType
```

**Implements**: `arrow_array::builder::ArrayBuilder`

**Derives**: Debug

**Methods** (11)

```rust
fn append(&mut self, value: impl AsRef<[u8]>) -> Result<K::Native, ArrowError>
fn append_n(&mut self, value: impl AsRef<[u8]>, count: usize) -> Result<K::Native, ArrowError>
fn append_null(&mut self)
fn append_nulls(&mut self, n: usize)
fn append_value(&mut self, value: impl AsRef<[u8]>)
fn finish(&mut self) -> DictionaryArray<K>
fn finish_cloned(&self) -> DictionaryArray<K>
fn finish_preserve_values(&mut self) -> DictionaryArray<K>
fn new(byte_width: i32) -> Self
fn try_new_from_builder<K2>(source: FixedSizeBinaryDictionaryBuilder<K2>) -> Result<Self, ArrowError> where K::Native: NumCast, K2: ArrowDictionaryKeyType, K2::Native: NumCast
fn with_capacity(keys_capacity: usize, value_capacity: usize, byte_width: i32) -> Self
```

**via `arrow_array::builder::ArrayBuilder`**

```rust
fn as_any(&self) -> &dyn Any
fn as_any_mut(&mut self) -> &mut dyn Any
fn finish(&mut self) -> ArrayRef
fn finish_cloned(&self) -> ArrayRef
fn finish_preserve_values(&mut self) -> ArrayRef
fn into_box_any(Box<self>) -> Box<dyn Any>
fn len(&self) -> usize
```

Builder for [`DictionaryArray`] of [`FixedSizeBinaryArray`]

The output array has a dictionary of unique, fixed-size binary values. The
builder handles deduplication.

# Example
```
# use arrow_array::builder::{FixedSizeBinaryDictionaryBuilder};
# use arrow_array::array::{Array, FixedSizeBinaryArray};
# use arrow_array::DictionaryArray;
# use arrow_array::types::Int8Type;
// Build 3 byte FixedBinaryArrays
let byte_width = 3;
let mut builder = FixedSizeBinaryDictionaryBuilder::<Int8Type>::new(3);
builder.append("abc").unwrap();
builder.append_null();
builder.append(b"def").unwrap();
builder.append(b"def").unwrap(); // duplicate value
// Result is a Dictionary Array
let array = builder.finish();
let dict_array = array.as_any().downcast_ref::<DictionaryArray<Int8Type>>().unwrap();
// The array represents "abc", null, "def", "def"
assert_eq!(array.keys().len(), 4);
// but there are only 2 unique values
assert_eq!(array.values().len(), 2);
let values = dict_array.values().as_any().downcast_ref::<FixedSizeBinaryArray>().unwrap();
assert_eq!(values.value(0), "abc".as_bytes());
assert_eq!(values.value(1), "def".as_bytes());
```

[`FixedSizeBinaryArray`]: crate::FixedSizeBinaryArray

---
