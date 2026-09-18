# `arrow_ord::rank::rank`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.rank.rank.json).

<a id="op-aebc0fd77fb1e4160dc1c741"></a>
## rank

`function` · `arrow_ord::rank::rank` · arrow-ord 59.3.0

```rust
fn rank(array: &dyn Array, options: Option<arrow_schema::SortOptions>) -> Result<Vec<u32>, arrow_schema::ArrowError>
```

Source: `src/rank.rs:54`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

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
