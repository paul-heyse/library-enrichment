# `arrow_array::array::byte_view_array::StringViewArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.byte_view_array.StringViewArray.json).

<a id="op-f468d0a8f2aaecdb58e1f8c0"></a>
## StringViewArray

`type_alias` · `arrow_array::array::byte_view_array::StringViewArray` · arrow-array 59.3.0

```rust
type StringViewArray = GenericByteViewArray<types::StringViewType>
```

Source: `src/array/byte_view_array.rs:1160`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`GenericByteViewArray`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-af313e332b96011fb72a0226) that stores utf8 data

See [`GenericByteViewArray`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-af313e332b96011fb72a0226) for format and layout details.

# Example
```
use arrow_array::StringViewArray;
let array = StringViewArray::from_iter_values(vec!["hello", "world", "lulu", "large payload over 12 bytes"]);
assert_eq!(array.value(0), "hello");
assert_eq!(array.value(3), "large payload over 12 bytes");
```
