# `arrow_array::cast::as_primitive_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.cast.as_primitive_array.json).

<a id="op-7a92b517aa3330ce4b7cdef5"></a>
## as_primitive_array

`function` · `arrow_array::cast::as_primitive_array` · arrow-array 59.3.0

```rust
fn as_primitive_array<T>(arr: &dyn Array) -> &PrimitiveArray<T> where T: ArrowPrimitiveType
```

Source: `src/cast.rs:504`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Force downcast of an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21), such as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1), to
[`PrimitiveArray<T>`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814), panic'ing on failure.

# Example

```
# use std::sync::Arc;
# use arrow_array::{ArrayRef, Int32Array};
# use arrow_array::cast::as_primitive_array;
# use arrow_array::types::Int32Type;

let arr: ArrayRef = Arc::new(Int32Array::from(vec![Some(1)]));

// Downcast an `ArrayRef` to Int32Array / PrimitiveArray<Int32>:
let primitive_array: &Int32Array = as_primitive_array(&arr);

// Equivalently:
let primitive_array = as_primitive_array::<Int32Type>(&arr);

// This is the equivalent of:
let primitive_array = arr
    .as_any()
    .downcast_ref::<Int32Array>()
    .unwrap();
```
