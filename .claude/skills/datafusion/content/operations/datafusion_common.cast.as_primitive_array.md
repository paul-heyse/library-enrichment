# `datafusion_common::cast::as_primitive_array`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.cast.as_primitive_array.json).

<a id="op-fd4adbeee0f9ecb2e3bc6ee2"></a>
## as_primitive_array

`function` · `datafusion_common::cast::as_primitive_array` · datafusion-common 55.1.0

```rust
fn as_primitive_array<T: ArrowPrimitiveType>(array: &dyn Array) -> Result<&arrow::array::PrimitiveArray<T>>
```

Source: `src/cast.rs:189`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
