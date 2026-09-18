# `arrow_ord::partition::partition`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.partition.partition.json).

<a id="op-bb0253e80df493c54b1c48d6"></a>
## partition

`function` · `arrow_ord::partition::partition` · arrow-ord 59.3.0

```rust
fn partition(columns: &[arrow_array::ArrayRef]) -> Result<Partitions, arrow_schema::ArrowError>
```

Source: `src/partition.rs:127`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Given a list of lexicographically sorted columns, computes the [`Partitions`](../operations/arrow_ord.partition.Partitions.md#op-9548b71e532931bce2ff0343),
where a partition consists of the set of consecutive rows with equal values

Returns an error if no columns are specified or all columns do not
have the same number of rows.

# Example:

For example, given columns `x`, `y` and `z`, calling
[`partition`](../operations/arrow_ord.partition.partition.md#op-bb0253e80df493c54b1c48d6)`(values, (x, y))` will divide the
rows into ranges where the values of `(x, y)` are equal:

```text
┌ ─ ┬───┬ ─ ─┌───┐─ ─ ┬───┬ ─ ─ ┐
    │ 1 │    │ 1 │    │ A │        Range: 0..1 (x=1, y=1)
├ ─ ┼───┼ ─ ─├───┤─ ─ ┼───┼ ─ ─ ┤
    │ 1 │    │ 2 │    │ B │
│   ├───┤    ├───┤    ├───┤     │
    │ 1 │    │ 2 │    │ C │        Range: 1..4 (x=1, y=2)
│   ├───┤    ├───┤    ├───┤     │
    │ 1 │    │ 2 │    │ D │
├ ─ ┼───┼ ─ ─├───┤─ ─ ┼───┼ ─ ─ ┤
    │ 2 │    │ 1 │    │ E │        Range: 4..5 (x=2, y=1)
├ ─ ┼───┼ ─ ─├───┤─ ─ ┼───┼ ─ ─ ┤
    │ 3 │    │ 1 │    │ F │        Range: 5..6 (x=3, y=1)
└ ─ ┴───┴ ─ ─└───┘─ ─ ┴───┴ ─ ─ ┘

      x        y        z     partition(&[x, y])
```

# Example Code

```
# use std::{sync::Arc, ops::Range};
# use arrow_array::{RecordBatch, Int64Array, StringArray, ArrayRef};
# use arrow_ord::sort::{SortColumn, SortOptions};
# use arrow_ord::partition::partition;
let batch = RecordBatch::try_from_iter(vec![
    ("x", Arc::new(Int64Array::from(vec![1, 1, 1, 1, 2, 3])) as ArrayRef),
    ("y", Arc::new(Int64Array::from(vec![1, 2, 2, 2, 1, 1])) as ArrayRef),
    ("z", Arc::new(StringArray::from(vec!["A", "B", "C", "D", "E", "F"])) as ArrayRef),
]).unwrap();

// Partition on first two columns
let ranges = partition(&batch.columns()[..2]).unwrap().ranges();

let expected = vec![
    (0..1),
    (1..4),
    (4..5),
    (5..6),
];

assert_eq!(ranges, expected);
```
