# `arrow_ord::cmp::compare_byte_view`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.cmp.compare_byte_view.json).

<a id="op-43fecd027b90a19dd1c2304b"></a>
## compare_byte_view

`function` · `arrow_ord::cmp::compare_byte_view` · arrow-ord 59.3.0

```rust
fn compare_byte_view<T: ByteViewType>(left: &arrow_array::GenericByteViewArray<T>, left_idx: usize, right: &arrow_array::GenericByteViewArray<T>, right_idx: usize) -> std::cmp::Ordering
```

Source: `src/cmp.rs:864`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Compares two [`GenericByteViewArray`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-af313e332b96011fb72a0226) at index `left_idx` and `right_idx`
