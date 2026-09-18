# `arrow_select::zip::zip`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.zip.zip.json).

<a id="op-c40a0b0a379f198055f80c56"></a>
## zip

`function` · `arrow_select::zip::zip` · arrow-select 59.3.0

```rust
fn zip(mask: &BooleanArray, truthy: &dyn Datum, falsy: &dyn Datum) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/zip.rs:99`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Zip two arrays by some boolean mask.

- Where `mask` is `true`, values of `truthy` are taken
- Where `mask` is `false` or `NULL`, values of `falsy` are taken

# Example: `zip` two arrays
```
# use std::sync::Arc;
# use arrow_array::{ArrayRef, BooleanArray, Int32Array};
# use arrow_select::zip::zip;
// mask: [true, true, false, NULL, true]
let mask = BooleanArray::from(vec![
  Some(true), Some(true), Some(false), None, Some(true)
]);
// truthy array: [1, NULL, 3, 4, 5]
let truthy = Int32Array::from(vec![
  Some(1), None, Some(3), Some(4), Some(5)
]);
// falsy array: [10, 20, 30, 40, 50]
let falsy = Int32Array::from(vec![
  Some(10), Some(20), Some(30), Some(40), Some(50)
]);
// zip with this mask select the first, second and last value from `truthy`
// and the third and fourth value from `falsy`
let result = zip(&mask, &truthy, &falsy).unwrap();
// Expected: [1, NULL, 30, 40, 5]
let expected: ArrayRef = Arc::new(Int32Array::from(vec![
  Some(1), None, Some(30), Some(40), Some(5)
]));
assert_eq!(&result, &expected);
```

# Example: `zip` and array with a scalar

Use `zip` to replace certain values in an array with a scalar

```
# use std::sync::Arc;
# use arrow_array::{ArrayRef, BooleanArray, Int32Array};
# use arrow_select::zip::zip;
// mask: [true, true, false, NULL, true]
let mask = BooleanArray::from(vec![
  Some(true), Some(true), Some(false), None, Some(true)
]);
//  array: [1, NULL, 3, 4, 5]
let arr = Int32Array::from(vec![
  Some(1), None, Some(3), Some(4), Some(5)
]);
// scalar: 42
let scalar = Int32Array::new_scalar(42);
// zip the array with the  mask select the first, second and last value from `arr`
// and fill the third and fourth value with the scalar 42
let result = zip(&mask, &arr, &scalar).unwrap();
// Expected: [1, NULL, 42, 42, 5]
let expected: ArrayRef = Arc::new(Int32Array::from(vec![
  Some(1), None, Some(42), Some(42), Some(5)
]));
assert_eq!(&result, &expected);
```
