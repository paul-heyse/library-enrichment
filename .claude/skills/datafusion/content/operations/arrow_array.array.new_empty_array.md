# `arrow_array::array::new_empty_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.new_empty_array.json).

<a id="op-394a32caa15fce90a598bb41"></a>
## new_empty_array

`function` · `arrow_array::array::new_empty_array` · arrow-array 59.3.0

```rust
fn new_empty_array(data_type: &arrow_schema::DataType) -> ArrayRef
```

Source: `src/array/mod.rs:1002`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new empty array

```
use std::sync::Arc;
use arrow_schema::DataType;
use arrow_array::{ArrayRef, Int32Array, new_empty_array};

let empty_array = new_empty_array(&DataType::Int32);
let array: ArrayRef = Arc::new(Int32Array::from(vec![] as Vec<i32>));

assert_eq!(&array, &empty_array);
```
