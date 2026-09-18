# `arrow_array::builder::generic_byte_run_builder::BinaryRunBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_byte_run_builder.BinaryRunBuilder.json).

<a id="op-dd759d6a2a03380b41f71136"></a>
## BinaryRunBuilder

`type_alias` · `arrow_array::builder::generic_byte_run_builder::BinaryRunBuilder` · arrow-array 59.3.0

```rust
type BinaryRunBuilder<K> = GenericByteRunBuilder<K, types::BinaryType>
```

Source: `src/builder/generic_byte_run_builder.rs:369`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) of [`BinaryArray`](crate::array::BinaryArray)

```
// Create a run-end encoded array with run-end indexes data type as `i16`.
// The encoded data is binary values.

# use arrow_array::builder::BinaryRunBuilder;
# use arrow_array::{BinaryArray, Int16Array};
# use arrow_array::cast::AsArray;
# use arrow_array::types::Int16Type;

let mut builder = BinaryRunBuilder::<Int16Type>::new();

// The builder builds the dictionary value by value
builder.append_value(b"abc");
builder.append_null();
builder.extend([Some(b"def"), Some(b"def"), Some(b"abc")]);
let array = builder.finish();

assert_eq!(array.run_ends().values(), &[1, 2, 4, 5]);

// Values are polymorphic and so require a downcast.
let av = array.values();
let ava: &BinaryArray = av.as_binary();

assert_eq!(ava.value(0), b"abc");
assert!(av.is_null(1));
assert_eq!(ava.value(2), b"def");
assert_eq!(ava.value(3), b"abc");

```
