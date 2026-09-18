# `arrow_ord`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.json).

<a id="op-1b38ba299ad923d1743e5e00"></a>
## arrow_ord

`module` · `arrow_ord` · arrow-ord 59.3.0

```rust
mod arrow_ord
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Arrow ordering kernels

# Sort RecordBatch

```
# use std::sync::Arc;
# use arrow_array::*;
# use arrow_array::cast::AsArray;
# use arrow_array::types::Int32Type;
# use arrow_ord::sort::sort_to_indices;
# use arrow_select::take::take;
#
let a: ArrayRef = Arc::new(Int32Array::from(vec![1, 2, 3, 4]));
let b: ArrayRef = Arc::new(StringArray::from(vec!["b", "a", "e", "d"]));
let batch = RecordBatch::try_from_iter(vec![("a", a), ("b", b)]).unwrap();

// Sort by column 1
let indices = sort_to_indices(batch.column(1), None, None).unwrap();

// Apply indices to batch columns
let columns = batch.columns().iter().map(|c| take(&*c, &indices, None).unwrap()).collect();
let sorted = RecordBatch::try_new(batch.schema(), columns).unwrap();

let col1 = sorted.column(0).as_primitive::<Int32Type>();
assert_eq!(col1.values(), &[2, 1, 4, 3]);
```

