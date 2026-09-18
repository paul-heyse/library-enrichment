# `datafusion_common::cast::as_generic_list_array`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.cast.as_generic_list_array.json).

<a id="op-a3f4ea3f92ce9428a8d09de3"></a>
## as_generic_list_array

`function` · `datafusion_common::cast::as_generic_list_array` · datafusion-common 55.1.0

```rust
fn as_generic_list_array<T: OffsetSizeTrait>(array: &dyn Array) -> Result<&arrow::array::GenericListArray<T>>
```

Source: `src/cast.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
