# `arrow_array::cast::as_list_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.cast.as_list_array.json).

<a id="op-23d4fcd0b3dcdeabf59012a8"></a>
## as_list_array

`function` · `arrow_array::cast::as_list_array` · arrow-array 59.3.0

```rust
fn as_list_array(arr: &dyn Array) -> &ListArray
```

Source: `src/cast.rs:694`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Force downcast of an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21), such as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) to
[`ListArray`](../operations/arrow_array.array.list_array.ListArray.md#op-fbfbbaae30c81192ec1e4456), panicking on failure.
