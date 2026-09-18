# `arrow_array::array::run_array::Int64RunArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.run_array.Int64RunArray.json).

<a id="op-594ebb85c020aa0e82f9da7c"></a>
## Int64RunArray

`type_alias` · `arrow_array::array::run_array::Int64RunArray` · arrow-array 59.3.0

```rust
type Int64RunArray = RunArray<types::Int64Type>
```

Source: `src/array/run_array.rs:627`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).


A [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) with `i64` run ends

# Example: Using `collect`
```
# use arrow_array::{Array, Int64RunArray, Int64Array, StringArray};
# use std::sync::Arc;

let array: Int64RunArray = vec!["a", "a", "b", "c", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.run_ends().values(), &[2, 3, 5]);
assert_eq!(array.values(), &values);
```
