# `datafusion_common::utils::coerced_fixed_size_list_to_list`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.coerced_fixed_size_list_to_list.json).

<a id="op-d4b007b34f967054876cc0c4"></a>
## coerced_fixed_size_list_to_list

`function` · `datafusion_common::utils::coerced_fixed_size_list_to_list` · datafusion-common 55.1.0

```rust
fn coerced_fixed_size_list_to_list(data_type: &arrow::datatypes::DataType) -> arrow::datatypes::DataType
```

Source: `src/utils/mod.rs:848`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Recursively coerce and `FixedSizeList` elements to `List`
