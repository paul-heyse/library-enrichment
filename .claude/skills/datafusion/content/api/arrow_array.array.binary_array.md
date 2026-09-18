# `arrow_array::array::binary_array`

Crate `arrow-array` · 3 public items · structured records in [`model/arrow_array.array.binary_array.json`](../model/arrow_array.array.binary_array.json)

## BinaryArray

`type_alias` · `arrow_array::array::binary_array::BinaryArray`

```rust
type BinaryArray = GenericBinaryArray<i32>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.array.binary_array.BinaryArray.md).


A [`GenericBinaryArray`] of `[u8]` using `i32` offsets

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

See [`GenericByteArray`] for more information and examples

---

## GenericBinaryArray

`type_alias` · `arrow_array::array::binary_array::GenericBinaryArray`

```rust
type GenericBinaryArray<OffsetSize> = GenericByteArray<types::GenericBinaryType<OffsetSize>>
```

**Implements**: `core::convert::From`

**via `core::convert::From`**

```rust
fn from(v: GenericListArray<T>) -> Self
fn from(value: GenericStringArray<OffsetSize>) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_array.array.binary_array.GenericBinaryArray.md).


A [`GenericByteArray`] for storing `[u8]`

---

## LargeBinaryArray

`type_alias` · `arrow_array::array::binary_array::LargeBinaryArray`

```rust
type LargeBinaryArray = GenericBinaryArray<i64>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.array.binary_array.LargeBinaryArray.md).


A [`GenericBinaryArray`] of `[u8]` using `i64` offsets

# Examples

Create a LargeBinaryArray from a vector of byte slices.

```
use arrow_array::{Array, LargeBinaryArray};
let values: Vec<&[u8]> =
    vec![b"one", b"two", b"", b"three"];
let array = LargeBinaryArray::from_vec(values);
assert_eq!(4, array.len());
assert_eq!(b"one", array.value(0));
assert_eq!(b"two", array.value(1));
assert_eq!(b"", array.value(2));
assert_eq!(b"three", array.value(3));
```

Create a LargeBinaryArray from a vector of Optional (null) byte slices.

```
use arrow_array::{Array, LargeBinaryArray};
let values: Vec<Option<&[u8]>> =
    vec![Some(b"one"), Some(b"two"), None, Some(b""), Some(b"three")];
let array = LargeBinaryArray::from_opt_vec(values);
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

See [`GenericByteArray`] for more information and examples

---
