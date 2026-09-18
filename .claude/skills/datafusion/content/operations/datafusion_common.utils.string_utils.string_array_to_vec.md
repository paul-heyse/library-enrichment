# `datafusion_common::utils::string_utils::string_array_to_vec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.string_utils.string_array_to_vec.json).

<a id="op-dddc31239d03077a03a7e391"></a>
## string_array_to_vec

`function` · `datafusion_common::utils::string_utils::string_array_to_vec` · datafusion-common 55.1.0

```rust
fn string_array_to_vec(array: &dyn Array) -> Vec<Option<&str>>
```

Source: `src/utils/string_utils.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Convenient function to convert an Arrow string array to a vector of strings
