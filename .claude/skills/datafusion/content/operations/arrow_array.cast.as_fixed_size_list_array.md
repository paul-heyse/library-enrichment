# `arrow_array::cast::as_fixed_size_list_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.cast.as_fixed_size_list_array.json).

<a id="op-7e9a02789b1ee4a5dfe7d22b"></a>
## as_fixed_size_list_array

`function` · `arrow_array::cast::as_fixed_size_list_array` · arrow-array 59.3.0

```rust
fn as_fixed_size_list_array(arr: &dyn Array) -> &FixedSizeListArray
```

Source: `src/cast.rs:701`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Force downcast of an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21), such as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) to
[`FixedSizeListArray`](../operations/arrow_array.array.fixed_size_list_array.FixedSizeListArray.md#op-4772cb4965341d875c1269ee), panicking on failure.
