# `arrow_array::cast::as_large_list_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.cast.as_large_list_array.json).

<a id="op-d9e5e87ceefcc7f25bd2538a"></a>
## as_large_list_array

`function` · `arrow_array::cast::as_large_list_array` · arrow-array 59.3.0

```rust
fn as_large_list_array(arr: &dyn Array) -> &LargeListArray
```

Source: `src/cast.rs:710`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Force downcast of an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21), such as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) to
[`LargeListArray`](../operations/arrow_array.array.list_array.LargeListArray.md#op-e34a694617d66e91221407db), panicking on failure.
