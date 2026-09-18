# `arrow_array::cast::as_generic_binary_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.cast.as_generic_binary_array.json).

<a id="op-ba7e104a7923d53b3a234734"></a>
## as_generic_binary_array

`function` · `arrow_array::cast::as_generic_binary_array` · arrow-array 59.3.0

```rust
fn as_generic_binary_array<S: OffsetSizeTrait>(arr: &dyn Array) -> &GenericBinaryArray<S>
```

Source: `src/cast.rs:717`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Force downcast of an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21), such as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) to
[`GenericBinaryArray<S>`](../operations/arrow_array.array.binary_array.GenericBinaryArray.md#op-faf606314f632d89aa2152d6), panicking on failure.
