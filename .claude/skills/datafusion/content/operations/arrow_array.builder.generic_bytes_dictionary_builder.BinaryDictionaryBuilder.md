# `arrow_array::builder::generic_bytes_dictionary_builder::BinaryDictionaryBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_bytes_dictionary_builder.BinaryDictionaryBuilder.json).

<a id="op-69a71f51ea683be5187b8f12"></a>
## BinaryDictionaryBuilder

`type_alias` · `arrow_array::builder::generic_bytes_dictionary_builder::BinaryDictionaryBuilder` · arrow-array 59.3.0

```rust
type BinaryDictionaryBuilder<K> = GenericByteDictionaryBuilder<K, types::GenericBinaryType<i32>>
```

Source: `src/builder/generic_bytes_dictionary_builder.rs:599`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) of [`BinaryArray`](crate::array::BinaryArray)

```
// Create a dictionary array indexed by bytes whose values are binary.
// It can thus hold up to 256 distinct binary values.

# use arrow_array::builder::BinaryDictionaryBuilder;
# use arrow_array::{BinaryArray, Int8Array};
# use arrow_array::types::Int8Type;

let mut builder = BinaryDictionaryBuilder::<Int8Type>::new();

// The builder builds the dictionary value by value
builder.append(b"abc").unwrap();
builder.append_null();
builder.append(b"def").unwrap();
builder.append(b"def").unwrap();
builder.append(b"abc").unwrap();
let array = builder.finish();

assert_eq!(
  array.keys(),
  &Int8Array::from(vec![Some(0), None, Some(1), Some(1), Some(0)])
);

// Values are polymorphic and so require a downcast.
let av = array.values();
let ava: &BinaryArray = av.as_any().downcast_ref::<BinaryArray>().unwrap();

assert_eq!(ava.value(0), b"abc");
assert_eq!(ava.value(1), b"def");

```
