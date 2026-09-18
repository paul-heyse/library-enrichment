# `arrow_ord::sort::sort_limit`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.sort.sort_limit.json).

<a id="op-3d08e3155b529e334d766c8b"></a>
## sort_limit

`function` · `arrow_ord::sort::sort_limit` · arrow-ord 59.3.0

```rust
fn sort_limit(values: &dyn Array, options: Option<SortOptions>, limit: Option<usize>) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/sort.rs:159`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Sort the `ArrayRef` partially.

If `limit` is specified, the resulting array will contain only
first `limit` in the sort order. Any data data after the limit
will be discarded.

Note: this is an unstable_sort, meaning it may not preserve the
order of equal elements.

# Example
```rust
# use std::sync::Arc;
# use arrow_array::Int32Array;
# use arrow_ord::sort::{sort_limit, SortOptions};
let array = Int32Array::from(vec![5, 4, 3, 2, 1]);

// Find the the top 2 items
let sorted_array = sort_limit(&array, None, Some(2)).unwrap();
assert_eq!(sorted_array.as_ref(), &Int32Array::from(vec![1, 2]));

// Find the bottom top 2 items
let options = Some(SortOptions {
                 descending: true,
                 ..Default::default()
              });
let sorted_array = sort_limit(&array, options, Some(2)).unwrap();
assert_eq!(sorted_array.as_ref(), &Int32Array::from(vec![5, 4]));
```
