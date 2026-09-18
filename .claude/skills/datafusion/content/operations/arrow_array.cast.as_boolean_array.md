# `arrow_array::cast::as_boolean_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.cast.as_boolean_array.json).

<a id="op-0d4acf3fd7ede32dceed9acd"></a>
## as_boolean_array

`function` · `arrow_array::cast::as_boolean_array` · arrow-array 59.3.0

```rust
fn as_boolean_array(arr: &dyn Array) -> &BooleanArray
```

Source: `src/cast.rs:755`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Force downcast of an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21), such as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) to
[`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505), panicking on failure.

# Example

```
# use std::sync::Arc;
# use arrow_array::{ArrayRef, BooleanArray};
# use arrow_array::cast::as_boolean_array;

let arr: ArrayRef = Arc::new(BooleanArray::from_iter(vec![Some(true)]));
let boolean_array = as_boolean_array(&arr);
```
