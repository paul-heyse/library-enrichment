# `arrow_array::builder::generic_bytes_dictionary_builder::StringDictionaryBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_bytes_dictionary_builder.StringDictionaryBuilder.json).

<a id="op-479447cadbe2fea59dfbce59"></a>
## StringDictionaryBuilder

`type_alias` · `arrow_array::builder::generic_bytes_dictionary_builder::StringDictionaryBuilder` · arrow-array 59.3.0

```rust
type StringDictionaryBuilder<K> = GenericByteDictionaryBuilder<K, types::GenericStringType<i32>>
```

Source: `src/builder/generic_bytes_dictionary_builder.rs:561`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) of [`StringArray`](crate::array::StringArray)

```
// Create a dictionary array indexed by bytes whose values are Strings.
// It can thus hold up to 256 distinct string values.

# use arrow_array::builder::StringDictionaryBuilder;
# use arrow_array::{Int8Array, StringArray};
# use arrow_array::types::Int8Type;

let mut builder = StringDictionaryBuilder::<Int8Type>::new();

// The builder builds the dictionary value by value
builder.append("abc").unwrap();
builder.append_null();
builder.append_n("def", 2).unwrap();  // appends "def" twice with a single lookup
builder.append("abc").unwrap();
let array = builder.finish();

assert_eq!(
  array.keys(),
  &Int8Array::from(vec![Some(0), None, Some(1), Some(1), Some(0)])
);

// Values are polymorphic and so require a downcast.
let av = array.values();
let ava: &StringArray = av.as_any().downcast_ref::<StringArray>().unwrap();

assert_eq!(ava.value(0), "abc");
assert_eq!(ava.value(1), "def");

```
