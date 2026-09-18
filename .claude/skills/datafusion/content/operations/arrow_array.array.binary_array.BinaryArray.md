# `arrow_array::array::binary_array::BinaryArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.binary_array.BinaryArray.json).

<a id="op-7129d9dc68ce402f881c540c"></a>
## BinaryArray

`type_alias` · `arrow_array::array::binary_array::BinaryArray` · arrow-array 59.3.0

```rust
type BinaryArray = GenericBinaryArray<i32>
```

Source: `src/array/binary_array.rs:170`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`GenericBinaryArray`](../operations/arrow_array.array.binary_array.GenericBinaryArray.md#op-faf606314f632d89aa2152d6) of `[u8]` using `i32` offsets

The byte length of each element is represented by an i32.

# Examples

Create a BinaryArray from a vector of byte slices.

```
use arrow_array::{Array, BinaryArray};
let values: Vec<&[u8]> =
    vec![b"one", b"two", b"", b"three"];
let array = BinaryArray::from_vec(values);
assert_eq!(4, array.len());
assert_eq!(b"one", array.value(0));
assert_eq!(b"two", array.value(1));
assert_eq!(b"", array.value(2));
assert_eq!(b"three", array.value(3));
```

Create a BinaryArray from a vector of Optional (null) byte slices.

```
use arrow_array::{Array, BinaryArray};
let values: Vec<Option<&[u8]>> =
    vec![Some(b"one"), Some(b"two"), None, Some(b""), Some(b"three")];
let array = BinaryArray::from_opt_vec(values);
assert_eq!(5, array.len());
assert_eq!(b"one", array.value(0));
assert_eq!(b"two", array.value(1));
assert_eq!(b"", array.value(3));
assert_eq!(b"three", array.value(4));
assert!(!array.is_null(0));
assert!(!array.is_null(1));
assert!(array.is_null(2));
assert!(!array.is_null(3));
assert!(!array.is_null(4));
```

See [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443) for more information and examples
