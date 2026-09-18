# `arrow_ord::ord::make_comparator`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.ord.make_comparator.json).

<a id="op-81ddd97bea600d32278eaff7"></a>
## make_comparator

`function` · `arrow_ord::ord::make_comparator` · arrow-ord 59.3.0

```rust
fn make_comparator(left: &dyn Array, right: &dyn Array, opts: arrow_schema::SortOptions) -> Result<DynComparator, arrow_schema::ArrowError>
```

Source: `src/ord.rs:495`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Returns a comparison function that compares two values at two different positions
between the two arrays.

For comparing arrays element-wise, see also the vectorised kernels in [`crate::cmp`](../modules/arrow_ord.cmp.md#op-4a319ff8353ad1130d47722a).

If `nulls_first` is true `NULL` values will be considered less than any non-null value,
otherwise they will be considered greater.

# Basic Usage

```
# use std::cmp::Ordering;
# use arrow_array::Int32Array;
# use arrow_ord::ord::make_comparator;
# use arrow_schema::SortOptions;
#
let array1 = Int32Array::from(vec![1, 2]);
let array2 = Int32Array::from(vec![3, 4]);

let cmp = make_comparator(&array1, &array2, SortOptions::default()).unwrap();
// 1 (index 0 of array1) is smaller than 4 (index 1 of array2)
assert_eq!(cmp(0, 1), Ordering::Less);

let array1 = Int32Array::from(vec![Some(1), None]);
let array2 = Int32Array::from(vec![None, Some(2)]);
let cmp = make_comparator(&array1, &array2, SortOptions::default()).unwrap();

assert_eq!(cmp(0, 1), Ordering::Less); // Some(1) vs Some(2)
assert_eq!(cmp(1, 1), Ordering::Less); // None vs Some(2)
assert_eq!(cmp(1, 0), Ordering::Equal); // None vs None
assert_eq!(cmp(0, 0), Ordering::Greater); // Some(1) vs None
```

# Postgres-compatible Nested Comparison

Whilst SQL prescribes ternary logic for nulls, that is comparing a value against a NULL yields
a NULL, many systems, including postgres, instead apply a total ordering to comparison of
nested nulls. That is nulls within nested types are either greater than any value (postgres),
or less than any value (Spark).

In particular

```ignore
{ a: 1, b: null } == { a: 1, b: null } => true
{ a: 1, b: null } == { a: 1, b: 1 } => false
{ a: 1, b: null } == null => null
null == null => null
```

This could be implemented as below

```
# use arrow_array::{Array, BooleanArray};
# use arrow_buffer::NullBuffer;
# use arrow_ord::cmp;
# use arrow_ord::ord::make_comparator;
# use arrow_schema::{ArrowError, SortOptions};
fn eq(a: &dyn Array, b: &dyn Array) -> Result<BooleanArray, ArrowError> {
    if !a.data_type().is_nested() {
        return cmp::eq(&a, &b); // Use faster vectorised kernel
    }

    let cmp = make_comparator(a, b, SortOptions::default())?;
    let len = a.len().min(b.len());
    let values = (0..len).map(|i| cmp(i, i).is_eq()).collect();
    let nulls = NullBuffer::union(a.nulls(), b.nulls());
    Ok(BooleanArray::new(values, nulls))
}
````
