# `datafusion_common::cast::as_run_array`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.cast.as_run_array.json).

<a id="op-bdc4e1c34d7f1e884a7a8c2c"></a>
## as_run_array

`function` · `datafusion_common::cast::as_run_array` · datafusion-common 55.1.0

```rust
fn as_run_array<T: RunEndIndexType>(array: &dyn Array) -> Result<&arrow::array::RunArray<T>>
```

Source: `src/cast.rs:340`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
