# `datafusion_common::utils::string_utils`

Crate `datafusion-common` · 1 public items · structured records in [`model/datafusion_common.utils.string_utils.json`](../model/datafusion_common.utils.string_utils.json)

## string_array_to_vec

`function` · `datafusion_common::utils::string_utils::string_array_to_vec`

```rust
fn string_array_to_vec(array: &dyn Array) -> Vec<Option<&str>>
```

Convenient function to convert an Arrow string array to a vector of strings

---
