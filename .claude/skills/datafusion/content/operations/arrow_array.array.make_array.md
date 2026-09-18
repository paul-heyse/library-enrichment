# `arrow_array::array::make_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.make_array.json).

<a id="op-1e7d6b70f3cd028f2f0440b2"></a>
## make_array

`function` · `arrow_array::array::make_array` · arrow-array 59.3.0

```rust
fn make_array(data: arrow_data::ArrayData) -> ArrayRef
```

Source: `src/array/mod.rs:890`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Constructs an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) from an [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78).

# Notes:

It is more efficient to directly construct the concrete array type rather
than using this function as creating an `ArrayData` requires at least one
additional allocation (the Vec of buffers).

# Example:
```
# use std::sync::Arc;
# use arrow_data::ArrayData;
# use arrow_array::{make_array, ArrayRef, Int32Array};
# use arrow_buffer::{Buffer, ScalarBuffer};
# use arrow_schema::DataType;
// Create an Int32Array with values [1, 2, 3]
let values_buffer = Buffer::from_slice_ref(&[1, 2, 3]);
// ArrayData can be constructed using ArrayDataBuilder
 let builder = ArrayData::builder(DataType::Int32)
   .len(3)
   .add_buffer(values_buffer.clone());
let array_data = builder.build().unwrap();
// Create the ArrayRef from the ArrayData
let array = make_array(array_data);

// It is equivalent to directly constructing the Int32Array
let scalar_buffer = ScalarBuffer::from(values_buffer);
let int32_array: ArrayRef = Arc::new(Int32Array::new(scalar_buffer, None));
assert_eq!(&array, &int32_array);
```
