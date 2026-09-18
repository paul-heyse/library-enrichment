# `arrow_array::cast::as_string_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.cast.as_string_array.json).

<a id="op-849079efc41162f95d6c5dfa"></a>
## as_string_array

`function` · `arrow_array::cast::as_string_array` · arrow-array 59.3.0

```rust
fn as_string_array(arr: &dyn Array) -> &StringArray
```

Source: `src/cast.rs:736`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Force downcast of an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21), such as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) to
[`StringArray`](../operations/arrow_array.array.string_array.StringArray.md#op-5d32f770159d415652a55945), panicking on failure.

# Example

```
# use std::sync::Arc;
# use arrow_array::cast::as_string_array;
# use arrow_array::{ArrayRef, StringArray};

let arr: ArrayRef = Arc::new(StringArray::from_iter(vec![Some("foo")]));
let string_array = as_string_array(&arr);
```
