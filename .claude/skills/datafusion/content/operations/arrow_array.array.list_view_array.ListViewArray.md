# `arrow_array::array::list_view_array::ListViewArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.list_view_array.ListViewArray.json).

<a id="op-f804ab17e2d4b64e9d2e4b55"></a>
## ListViewArray

`type_alias` · `arrow_array::array::list_view_array::ListViewArray` · arrow-array 59.3.0

```rust
type ListViewArray = GenericListViewArray<i32>
```

Source: `src/array/list_view_array.rs:34`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`GenericListViewArray`](../operations/arrow_array.array.list_view_array.GenericListViewArray.md#op-e77399dcef65864e81be3a9a) of variable size lists, storing offsets as `i32`.
