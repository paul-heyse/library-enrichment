# `arrow_array::cast::as_largestring_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.cast.as_largestring_array.json).

<a id="op-206a7f32cc20cbf11f3ee8a7"></a>
## as_largestring_array

`function` · `arrow_array::cast::as_largestring_array` · arrow-array 59.3.0

```rust
fn as_largestring_array(arr: &dyn Array) -> &LargeStringArray
```

Source: `src/cast.rs:783`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Force downcast of an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21), such as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) to 
[`LargeStringArray`](../operations/arrow_array.array.string_array.LargeStringArray.md#op-829f12b9a45fbfe752a7ee52), panicking on failure.
