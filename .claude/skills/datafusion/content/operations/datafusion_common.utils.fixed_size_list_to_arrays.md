# `datafusion_common::utils::fixed_size_list_to_arrays`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.fixed_size_list_to_arrays.json).

<a id="op-cddd0baf6cd728943b7cdcdb"></a>
## fixed_size_list_to_arrays

`function` · `datafusion_common::utils::fixed_size_list_to_arrays` · datafusion-common 55.1.0

```rust
fn fixed_size_list_to_arrays(a: &arrow::array::ArrayRef) -> Vec<arrow::array::ArrayRef>
```

Source: `src/utils/mod.rs:716`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Helper function to convert a FixedSizeListArray into a vector of ArrayRefs.
