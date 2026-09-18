# `arrow_ord::sort::sort`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.sort.sort.json).

<a id="op-91e64eb35b158533f835c795"></a>
## sort

`function` · `arrow_ord::sort::sort` · arrow-ord 59.3.0

```rust
fn sort(values: &dyn Array, options: Option<SortOptions>) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/sort.rs:57`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Sort the `ArrayRef` using `SortOptions`.

Performs a sort on values and indices. Nulls are ordered according
to the `nulls_first` flag in `options`.  Floats are sorted using
IEEE 754 totalOrder

Returns an `ArrowError::ComputeError(String)` if the array type is
either unsupported by `sort_to_indices` or `take`.

Note: this is an unstable_sort, meaning it may not preserve the
order of equal elements.

# Example
```rust
# use std::sync::Arc;
# use arrow_array::Int32Array;
# use arrow_ord::sort::sort;
let array = Int32Array::from(vec![5, 4, 3, 2, 1]);
let sorted_array = sort(&array, None).unwrap();
assert_eq!(sorted_array.as_ref(), &Int32Array::from(vec![1, 2, 3, 4, 5]));
```
