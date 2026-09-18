# `datafusion_common::utils::list_ndims`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.list_ndims.json).

<a id="op-fbff5daf5dddb6a62c044ccc"></a>
## list_ndims

`function` · `datafusion_common::utils::list_ndims` · datafusion-common 55.1.0

```rust
fn list_ndims(data_type: &arrow::datatypes::DataType) -> u64
```

Source: `src/utils/mod.rs:892`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Compute the number of dimensions in a list data type.
