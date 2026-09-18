# `arrow_array::builder::generic_byte_run_builder::StringRunBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_byte_run_builder.StringRunBuilder.json).

<a id="op-389151ed6e3224962afd2337"></a>
## StringRunBuilder

`type_alias` · `arrow_array::builder::generic_byte_run_builder::StringRunBuilder` · arrow-array 59.3.0

```rust
type StringRunBuilder<K> = GenericByteRunBuilder<K, types::Utf8Type>
```

Source: `src/builder/generic_byte_run_builder.rs:333`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) of [`StringArray`](crate::array::StringArray)

```
// Create a run-end encoded array with run-end indexes data type as `i16`.
// The encoded values are Strings.

# use arrow_array::builder::StringRunBuilder;
# use arrow_array::{Int16Array, StringArray};
# use arrow_array::types::Int16Type;
# use arrow_array::cast::AsArray;
#
let mut builder = StringRunBuilder::<Int16Type>::new();

// The builder builds the dictionary value by value
builder.append_value("abc");
builder.append_null();
builder.extend([Some("def"), Some("def"), Some("abc")]);
let array = builder.finish();

assert_eq!(array.run_ends().values(), &[1, 2, 4, 5]);

// Values are polymorphic and so require a downcast.
let av = array.values();
let ava: &StringArray = av.as_string::<i32>();

assert_eq!(ava.value(0), "abc");
assert!(av.is_null(1));
assert_eq!(ava.value(2), "def");
assert_eq!(ava.value(3), "abc");

```
