# `arrow_select::window`

Crate `arrow-select` · 1 public items · structured records in [`model/arrow_select.window.json`](../model/arrow_select.window.json)

## shift

`function` · `arrow_select::window::shift`

Also reachable as `arrow::compute::kernels::window::shift`, `arrow::compute::shift`

```rust
fn shift(array: &dyn Array, offset: i64) -> Result<arrow_array::ArrayRef, arrow_schema::ArrowError>
```

Shifts array by defined number of items (to left or right)
A positive value for `offset` shifts the array to the right
a negative value shifts the array to the left.
# Examples
```
# use arrow_array::Int32Array;
# use arrow_select::window::shift;
let a: Int32Array = vec![Some(1), None, Some(4)].into();

// shift array 1 element to the right
let res = shift(&a, 1).unwrap();
let expected: Int32Array = vec![None, Some(1), None].into();
assert_eq!(res.as_ref(), &expected);

// shift array 1 element to the left
let res = shift(&a, -1).unwrap();
let expected: Int32Array = vec![None, Some(4), None].into();
assert_eq!(res.as_ref(), &expected);

// shift array 0 element, although not recommended
let res = shift(&a, 0).unwrap();
let expected: Int32Array = vec![Some(1), None, Some(4)].into();
assert_eq!(res.as_ref(), &expected);

// shift array 3 element to the right
let res = shift(&a, 3).unwrap();
let expected: Int32Array = vec![None, None, None].into();
assert_eq!(res.as_ref(), &expected);
```

---
