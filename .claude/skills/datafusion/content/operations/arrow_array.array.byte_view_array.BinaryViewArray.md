# `arrow_array::array::byte_view_array::BinaryViewArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.byte_view_array.BinaryViewArray.json).

<a id="op-3ee392743cc781de5833990c"></a>
## BinaryViewArray

`type_alias` · `arrow_array::array::byte_view_array::BinaryViewArray` · arrow-array 59.3.0

```rust
type BinaryViewArray = GenericByteViewArray<types::BinaryViewType>
```

Source: `src/array/byte_view_array.rs:1119`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`GenericByteViewArray`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-af313e332b96011fb72a0226) of `[u8]`

See [`GenericByteViewArray`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-af313e332b96011fb72a0226) for format and layout details.

# Example
```
use arrow_array::BinaryViewArray;
let array = BinaryViewArray::from_iter_values(vec![b"hello" as &[u8], b"world", b"lulu", b"large payload over 12 bytes"]);
assert_eq!(array.value(0), b"hello");
assert_eq!(array.value(3), b"large payload over 12 bytes");
```
