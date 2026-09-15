# `arrow_ord::partition`

Crate `arrow-ord` · 2 public items · structured records in [`model/arrow_ord.partition.json`](../model/arrow_ord.partition.json)

## partition

`function` · `arrow_ord::partition::partition`

Also reachable as `arrow::compute::kernels::partition::partition`, `arrow::compute::partition`

```rust
fn partition(columns: &[arrow_array::ArrayRef]) -> Result<Partitions, arrow_schema::ArrowError>
```

Given a list of lexicographically sorted columns, computes the [`Partitions`],
where a partition consists of the set of consecutive rows with equal values

Returns an error if no columns are specified or all columns do not
have the same number of rows.

# Example:

For example, given columns `x`, `y` and `z`, calling
[`partition`]`(values, (x, y))` will divide the
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

---

## Partitions

`struct` · `arrow_ord::partition::Partitions`

Also reachable as `arrow::compute::Partitions`, `arrow::compute::kernels::partition::Partitions`

```rust
struct Partitions
```

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn ranges(&self) -> Vec<Range<usize>>
```

A computed set of partitions, see [`partition`]

---
