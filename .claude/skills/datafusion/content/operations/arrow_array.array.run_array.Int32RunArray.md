# `arrow_array::array::run_array::Int32RunArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.run_array.Int32RunArray.json).

<a id="op-6fe3890abcca1661390ae0ba"></a>
## Int32RunArray

`type_alias` · `arrow_array::array::run_array::Int32RunArray` · arrow-array 59.3.0

```rust
type Int32RunArray = RunArray<types::Int32Type>
```

Source: `src/array/run_array.rs:612`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).


A [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) with `i32` run ends

# Example: Using `collect`
```
# use arrow_array::{Array, Int32RunArray, Int32Array, StringArray};
# use std::sync::Arc;

let array: Int32RunArray = vec!["a", "a", "b", "c", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.run_ends().values(), &[2, 3, 5]);
assert_eq!(array.values(), &values);
```
