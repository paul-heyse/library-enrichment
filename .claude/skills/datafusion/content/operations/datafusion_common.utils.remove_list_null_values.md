# `datafusion_common::utils::remove_list_null_values`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.remove_list_null_values.json).

<a id="op-53e5ca5c623127932370f7bd"></a>
## remove_list_null_values

`function` · `datafusion_common::utils::remove_list_null_values` · datafusion-common 55.1.0

```rust
fn remove_list_null_values(array: &arrow::array::ArrayRef) -> Result<arrow::array::ArrayRef>
```

Source: `src/utils/mod.rs:1245`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

For lists and large lists, truncates the sublist of null values
Otherwise returns an error
