# `arrow_array::cast::as_struct_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.cast.as_struct_array.json).

<a id="op-e16b1c4db0a43b79b1a3dee3"></a>
## as_struct_array

`function` · `arrow_array::cast::as_struct_array` · arrow-array 59.3.0

```rust
fn as_struct_array(arr: &dyn Array) -> &StructArray
```

Source: `src/cast.rs:785`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Force downcast of an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21), such as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) to 
[`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207), panicking on failure.
