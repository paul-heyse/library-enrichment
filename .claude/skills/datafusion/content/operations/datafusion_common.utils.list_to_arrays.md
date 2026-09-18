# `datafusion_common::utils::list_to_arrays`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.list_to_arrays.json).

<a id="op-75037df7d50dd1ad7c94f7c3"></a>
## list_to_arrays

`function` · `datafusion_common::utils::list_to_arrays` · datafusion-common 55.1.0

```rust
fn list_to_arrays<O: OffsetSizeTrait>(a: &arrow::array::ArrayRef) -> Vec<arrow::array::ArrayRef>
```

Source: `src/utils/mod.rs:711`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Helper function to convert a ListArray into a vector of ArrayRefs.
