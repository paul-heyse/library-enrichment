# `arrow_array::array::new_null_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.new_null_array.json).

<a id="op-355c0c90e6c6820ca7b82959"></a>
## new_null_array

`function` · `arrow_array::array::new_null_array` · arrow-array 59.3.0

```rust
fn new_null_array(data_type: &arrow_schema::DataType, length: usize) -> ArrayRef
```

Source: `src/array/mod.rs:1020`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new array of `data_type` of length `length` filled
entirely of `NULL` values

```
use std::sync::Arc;
use arrow_schema::DataType;
use arrow_array::{ArrayRef, Int32Array, new_null_array};

let null_array = new_null_array(&DataType::Int32, 3);
let array: ArrayRef = Arc::new(Int32Array::from(vec![None, None, None]));

assert_eq!(&array, &null_array);
```
