# `arrow_select::nullif::nullif`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.nullif.nullif.json).

<a id="op-6f7af12f30b5294798fce7ec"></a>
## nullif

`function` · `arrow_select::nullif::nullif` · arrow-select 59.3.0

```rust
fn nullif(left: &dyn Array, right: &arrow_array::BooleanArray) -> Result<arrow_array::ArrayRef, arrow_schema::ArrowError>
```

Source: `src/nullif.rs:44`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Returns a new array with the same values and the validity bit to false where
the corresponding element of `right` is true.

This can be used to implement SQL `NULLIF`

# Example
```
# use arrow_array::{Int32Array, BooleanArray};
# use arrow_array::cast::AsArray;
# use arrow_array::types::Int32Type;
# use arrow_select::nullif::nullif;
// input is [null, 8, 1, 9]
let a = Int32Array::from(vec![None, Some(8), Some(1), Some(9)]);
// use nullif to set index 1 to null
let bool_array = BooleanArray::from(vec![Some(false), Some(true), Some(false), None]);
let nulled = nullif(&a, &bool_array).unwrap();
// The resulting array is [null, null, 1, 9]
assert_eq!(nulled.as_primitive(), &Int32Array::from(vec![None, None, Some(1), Some(9)]));
```
