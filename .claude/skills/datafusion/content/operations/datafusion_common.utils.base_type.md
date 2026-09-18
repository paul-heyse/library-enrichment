# `datafusion_common::utils::base_type`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.base_type.json).

<a id="op-8e15032924c03a12fe95b52b"></a>
## base_type

`function` · `datafusion_common::utils::base_type` · datafusion-common 55.1.0

```rust
fn base_type(data_type: &arrow::datatypes::DataType) -> arrow::datatypes::DataType
```

Source: `src/utils/mod.rs:735`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Get the base type of a data type.

Example
```
use arrow::datatypes::{DataType, Field};
use datafusion_common::utils::base_type;
use std::sync::Arc;

let data_type =
    DataType::List(Arc::new(Field::new_list_field(DataType::Int32, true)));
assert_eq!(base_type(&data_type), DataType::Int32);

let data_type = DataType::Int32;
assert_eq!(base_type(&data_type), DataType::Int32);
```
