# `arrow_array::array::list_view_array::LargeListViewArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.list_view_array.LargeListViewArray.json).

<a id="op-184699cca31c6f96df389081"></a>
## LargeListViewArray

`type_alias` · `arrow_array::array::list_view_array::LargeListViewArray` · arrow-array 59.3.0

```rust
type LargeListViewArray = GenericListViewArray<i64>
```

Source: `src/array/list_view_array.rs:37`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`GenericListViewArray`](../operations/arrow_array.array.list_view_array.GenericListViewArray.md#op-e77399dcef65864e81be3a9a) of variable size lists, storing offsets as `i64`.
