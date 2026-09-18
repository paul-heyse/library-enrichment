# `arrow_select::filter::prep_null_mask_filter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.filter.prep_null_mask_filter.json).

<a id="op-ccf02e93f22567e0bf544fb1"></a>
## prep_null_mask_filter

`function` · `arrow_select::filter::prep_null_mask_filter` · arrow-select 59.3.0

```rust
fn prep_null_mask_filter(filter: &BooleanArray) -> BooleanArray
```

Source: `src/filter.rs:167`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Convert all null values in `BooleanArray` to `false`

This is useful for filter-like operations which select only `true`
values, but not `false` or `NULL` values

Internally this is implemented as a bitwise `AND` operation with null bits
and the boolean bits.

# Example
```
# use arrow_array::{Array, BooleanArray};
# use arrow_select::filter::prep_null_mask_filter;
let filter = BooleanArray::from(vec![
  Some(true),
  Some(false),
  None
]);
// convert Boolean array to a filter mask
let null_mask = prep_null_mask_filter(&filter);
// there are no nulls in the output mask
assert!(null_mask.nulls().is_none());
assert_eq!(null_mask, BooleanArray::from(vec![
 true,
 false,
 false, // Null is converted to false
]));
```
