# `arrow_array::cast::as_run_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.cast.as_run_array.json).

<a id="op-7c7c6222bec3874a764b655c"></a>
## as_run_array

`function` · `arrow_array::cast::as_run_array` · arrow-array 59.3.0

```rust
fn as_run_array<T>(arr: &dyn Array) -> &RunArray<T> where T: RunEndIndexType
```

Source: `src/cast.rs:612`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Force downcast of an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21), such as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) to
[`RunArray<T>`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f), panic'ing on failure.

# Example

```
# use arrow_array::{ArrayRef, RunArray};
# use arrow_array::cast::as_run_array;
# use arrow_array::types::Int32Type;

let arr: RunArray<Int32Type> = vec![Some("foo")].into_iter().collect();
let arr: ArrayRef = std::sync::Arc::new(arr);
let run_array: &RunArray<Int32Type> = as_run_array::<Int32Type>(&arr);
```
