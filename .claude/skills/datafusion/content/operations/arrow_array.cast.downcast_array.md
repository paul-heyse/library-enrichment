# `arrow_array::cast::downcast_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.cast.downcast_array.json).

<a id="op-dab8fd19a0598954fbf380e2"></a>
## downcast_array

`function` · `arrow_array::cast::downcast_array` · arrow-array 59.3.0

```rust
fn downcast_array<T>(array: &dyn Array) -> T where T: From<arrow_data::ArrayData>
```

Source: `src/cast.rs:814`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Downcasts a `dyn Array` to a concrete type

```
# use arrow_array::{BooleanArray, Int32Array, RecordBatch, StringArray};
# use arrow_array::cast::downcast_array;
struct ConcreteBatch {
    col1: Int32Array,
    col2: BooleanArray,
    col3: StringArray,
}

impl ConcreteBatch {
    fn new(batch: &RecordBatch) -> Self {
        Self {
            col1: downcast_array(batch.column(0).as_ref()),
            col2: downcast_array(batch.column(1).as_ref()),
            col3: downcast_array(batch.column(2).as_ref()),
        }
    }
}
```

# Panics

Panics if array is not of the correct data type
