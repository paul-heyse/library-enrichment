# `datafusion_common::cast::as_generic_binary_array`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.cast.as_generic_binary_array.json).

<a id="op-3b57977cf8fd56af4f7830bc"></a>
## as_generic_binary_array

`function` · `datafusion_common::cast::as_generic_binary_array` · datafusion-common 55.1.0

```rust
fn as_generic_binary_array<T: OffsetSizeTrait>(array: &dyn Array) -> Result<&arrow::array::GenericBinaryArray<T>>
```

Source: `src/cast.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
