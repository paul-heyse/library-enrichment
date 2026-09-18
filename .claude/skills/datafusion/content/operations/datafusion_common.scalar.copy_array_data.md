# `datafusion_common::scalar::copy_array_data`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.scalar.copy_array_data.json).

<a id="op-f012a05c7c7ef232520171af"></a>
## copy_array_data

`function` · `datafusion_common::scalar::copy_array_data` · datafusion-common 55.1.0

```rust
fn copy_array_data(src_data: &arrow::array::ArrayData) -> arrow::array::ArrayData
```

Source: `src/scalar/mod.rs:5318`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Compacts the data of an `ArrayData` into a new `ArrayData`.

This is useful when you want to minimize the memory footprint of an
`ArrayData`. For example, the value returned by [`Array::slice`](../operations/arrow_array.array.Array.md#op-de5c5ee117de6e0858673954) still
points at the same underlying data buffers as the original array, which may
hold many more values. Calling `copy_array_data` on the sliced array will
create a new, smaller, `ArrayData` that only contains the data for the
sliced array.

# Example
```
# use arrow::array::{make_array, Array, Int32Array};
use datafusion_common::scalar::copy_array_data;
let array = Int32Array::from_iter_values(0..8192);
// Take only the first 2 elements
let sliced_array = array.slice(0, 2);
// The memory footprint of `sliced_array` is close to 8192 * 4 bytes
assert_eq!(32864, sliced_array.get_array_memory_size());
// however, we can copy the data to a new `ArrayData`
let new_array = make_array(copy_array_data(&sliced_array.into_data()));
// The memory footprint of `new_array` is now only 2 * 4 bytes
// and overhead:
assert_eq!(160, new_array.get_array_memory_size());
```

See also [`ScalarValue::compact`](../operations/datafusion_common.scalar.ScalarValue.md#op-26119b497e462f00f8475563) which applies to `ScalarValue` instances
as necessary.
