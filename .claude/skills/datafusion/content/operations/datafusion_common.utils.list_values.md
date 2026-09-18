# `datafusion_common::utils::list_values`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.list_values.json).

<a id="op-275193e92ac6421c88a90e83"></a>
## list_values

`function` · `datafusion_common::utils::list_values` · datafusion-common 55.1.0

```rust
fn list_values(array: &dyn Array) -> Result<arrow::array::ArrayRef>
```

Source: `src/utils/mod.rs:1206`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the inner values of a list, or an error otherwise
For [`ListArray`](../operations/arrow_array.array.list_array.ListArray.md#op-fbfbbaae30c81192ec1e4456) and [`LargeListArray`](../operations/arrow_array.array.list_array.LargeListArray.md#op-e34a694617d66e91221407db), if it's sliced, it returns a
sliced array too. Therefore, too reconstruct a list using it,
you must adjust the offsets using [`adjust_offsets_for_slice`](../operations/datafusion_common.utils.adjust_offsets_for_slice.md#op-8443ee3fd14581f1634f4931)
