# `datafusion_common::utils::arrays_into_list_array`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.arrays_into_list_array.json).

<a id="op-cb6f2b1cdada6ae1de3e6fe5"></a>
## arrays_into_list_array

`function` · `datafusion_common::utils::arrays_into_list_array` · datafusion-common 55.1.0

```rust
fn arrays_into_list_array(arr: impl IntoIterator<Item = arrow::array::ArrayRef>) -> Result<arrow::array::ListArray>
```

Source: `src/utils/mod.rs:692`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Wrap arrays into a single element `ListArray`.

Example:
```
use arrow::array::{Int32Array, ListArray, ArrayRef};
use arrow::datatypes::{Int32Type, Field};
use std::sync::Arc;

let arr1 = Arc::new(Int32Array::from(vec![1, 2, 3])) as ArrayRef;
let arr2 = Arc::new(Int32Array::from(vec![4, 5, 6])) as ArrayRef;

let list_arr = datafusion_common::utils::arrays_into_list_array([arr1, arr2]).unwrap();

let expected = ListArray::from_iter_primitive::<Int32Type, _, _>(
   vec![
    Some(vec![Some(1), Some(2), Some(3)]),
    Some(vec![Some(4), Some(5), Some(6)]),
   ]
);

assert_eq!(list_arr, expected);
```
