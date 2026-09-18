# `datafusion_common::utils::coerced_type_with_base_type_only`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.coerced_type_with_base_type_only.json).

<a id="op-bf9876f3946e0aafc01c388e"></a>
## coerced_type_with_base_type_only

`function` · `datafusion_common::utils::coerced_type_with_base_type_only` · datafusion-common 55.1.0

```rust
fn coerced_type_with_base_type_only(data_type: &arrow::datatypes::DataType, base_type: &arrow::datatypes::DataType, array_coercion: Option<&ListCoercion>) -> arrow::datatypes::DataType
```

Source: `src/utils/mod.rs:770`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A helper function to coerce base type in List.

Example
```
use arrow::datatypes::{DataType, Field};
use datafusion_common::utils::coerced_type_with_base_type_only;
use std::sync::Arc;

let data_type = DataType::List(Arc::new(Field::new_list_field(DataType::Int32, true)));
let base_type = DataType::Float64;
let coerced_type = coerced_type_with_base_type_only(&data_type, &base_type, None);
assert_eq!(coerced_type, DataType::List(Arc::new(Field::new_list_field(DataType::Float64, true))));
```
