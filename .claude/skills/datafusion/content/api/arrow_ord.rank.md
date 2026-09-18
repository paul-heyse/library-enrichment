# `arrow_ord::rank`

Crate `arrow-ord` · 1 public items · structured records in [`model/arrow_ord.rank.json`](../model/arrow_ord.rank.json)

## rank

`function` · `arrow_ord::rank::rank`

Also reachable as `arrow::compute::kernels::rank::rank`, `arrow::compute::rank`

```rust
fn rank(array: &dyn Array, options: Option<arrow_schema::SortOptions>) -> Result<Vec<u32>, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_ord.rank.rank.md).


Assigns a rank to each value in `array` based on its position in the sorted order

Where values are equal, they will be assigned the highest of their ranks,
leaving gaps in the overall rank assignment

```
# use arrow_array::StringArray;
# use arrow_ord::rank::rank;
let array = StringArray::from(vec![Some("foo"), None, Some("foo"), None, Some("bar")]);
let ranks = rank(&array, None).unwrap();
assert_eq!(ranks, &[5, 2, 5, 2, 3]);
```

---
