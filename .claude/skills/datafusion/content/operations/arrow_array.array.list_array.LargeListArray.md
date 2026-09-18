# `arrow_array::array::list_array::LargeListArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.list_array.LargeListArray.json).

<a id="op-e34a694617d66e91221407db"></a>
## LargeListArray

`type_alias` · `arrow_array::array::list_array::LargeListArray` · arrow-array 59.3.0

```rust
type LargeListArray = GenericListArray<i64>
```

Source: `src/array/list_array.rs:702`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`GenericListArray`](../operations/arrow_array.array.list_array.GenericListArray.md#op-f5cb16d85a7337f2dafa403b) of variable size lists, storing offsets as `i64`.

See [`LargeListBuilder`](crate::builder::LargeListBuilder) for how to construct a [`LargeListArray`](../operations/arrow_array.array.list_array.LargeListArray.md#op-e34a694617d66e91221407db)
