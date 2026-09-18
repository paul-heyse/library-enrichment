# `arrow_array::array::run_array::Int16RunArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.run_array.Int16RunArray.json).

<a id="op-9b92dce30871dd856e0da679"></a>
## Int16RunArray

`type_alias` · `arrow_array::array::run_array::Int16RunArray` · arrow-array 59.3.0

```rust
type Int16RunArray = RunArray<types::Int16Type>
```

Source: `src/array/run_array.rs:597`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).


A [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) with `i16` run ends

# Example: Using `collect`
```
# use arrow_array::{Array, Int16RunArray, Int16Array, StringArray};
# use std::sync::Arc;

let array: Int16RunArray = vec!["a", "a", "b", "c", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.run_ends().values(), &[2, 3, 5]);
assert_eq!(array.values(), &values);
```
