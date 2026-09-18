# `arrow_array::cast::as_null_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.cast.as_null_array.json).

<a id="op-6715b50ffcb512486d4792c9"></a>
## as_null_array

`function` · `arrow_array::cast::as_null_array` · arrow-array 59.3.0

```rust
fn as_null_array(arr: &dyn Array) -> &NullArray
```

Source: `src/cast.rs:784`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Force downcast of an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21), such as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) to 
[`NullArray`](../operations/arrow_array.array.null_array.NullArray.md#op-eccd805771d0b7ea8e20fabd), panicking on failure.
