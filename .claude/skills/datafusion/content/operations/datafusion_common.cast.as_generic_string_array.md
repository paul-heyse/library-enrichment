# `datafusion_common::cast::as_generic_string_array`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.cast.as_generic_string_array.json).

<a id="op-0c1033368d0f5d0a96065637"></a>
## as_generic_string_array

`function` · `datafusion_common::cast::as_generic_string_array` · datafusion-common 55.1.0

```rust
fn as_generic_string_array<T: OffsetSizeTrait>(array: &dyn Array) -> Result<&arrow::array::GenericStringArray<T>>
```

Source: `src/cast.rs:323`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
