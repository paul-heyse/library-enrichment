# `arrow_array::cast::as_map_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.cast.as_map_array.json).

<a id="op-3d7ebb0a652d853c0fe7405b"></a>
## as_map_array

`function` · `arrow_array::cast::as_map_array` · arrow-array 59.3.0

```rust
fn as_map_array(arr: &dyn Array) -> &MapArray
```

Source: `src/cast.rs:787`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Force downcast of an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21), such as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) to 
[`MapArray`](../operations/arrow_array.array.map_array.MapArray.md#op-2c9f2f57a7578a4beb5adb8f), panicking on failure.
