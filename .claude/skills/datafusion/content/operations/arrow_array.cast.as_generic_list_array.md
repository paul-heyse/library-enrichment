# `arrow_array::cast::as_generic_list_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.cast.as_generic_list_array.json).

<a id="op-75f6dce0c055c99a2c7e555a"></a>
## as_generic_list_array

`function` · `arrow_array::cast::as_generic_list_array` · arrow-array 59.3.0

```rust
fn as_generic_list_array<S: OffsetSizeTrait>(arr: &dyn Array) -> &GenericListArray<S>
```

Source: `src/cast.rs:685`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Force downcast of an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21), such as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) to
[`GenericListArray<T>`](../operations/arrow_array.array.list_array.GenericListArray.md#op-f5cb16d85a7337f2dafa403b), panicking on failure.
