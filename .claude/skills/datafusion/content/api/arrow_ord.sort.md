# `arrow_ord::sort`

Crate `arrow-ord` · 10 public items · structured records in [`model/arrow_ord.sort.json`](../model/arrow_ord.sort.json)

## lexsort

`function` · `arrow_ord::sort::lexsort`

Also reachable as `arrow::compute::kernels::sort::lexsort`, `arrow::compute::lexsort`

```rust
fn lexsort(columns: &[SortColumn], limit: Option<usize>) -> Result<Vec<ArrayRef>, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_ord.sort.lexsort.md).


Sort a list of `ArrayRef` using `SortOptions` provided for each array.

Performs an unstable lexicographical sort on values and indices.

Returns an `ArrowError::ComputeError(String)` if any of the array type is either unsupported by
`lexsort_to_indices` or `take`.

# Example:

```
# use std::convert::From;
# use std::sync::Arc;
# use arrow_array::{ArrayRef, StringArray, PrimitiveArray};
# use arrow_array::types::Int64Type;
# use arrow_array::cast::AsArray;
# use arrow_ord::sort::{SortColumn, SortOptions, lexsort};
let sorted_columns = lexsort(&vec![
    SortColumn {
        values: Arc::new(PrimitiveArray::<Int64Type>::from(vec![
            None,
            Some(-2),
            Some(89),
            Some(-64),
            Some(101),
        ])) as ArrayRef,
        options: None,
    },
    SortColumn {
        values: Arc::new(StringArray::from(vec![
            Some("hello"),
            Some("world"),
            Some(","),
            Some("foobar"),
            Some("!"),
        ])) as ArrayRef,
        options: Some(SortOptions {
            descending: true,
            nulls_first: false,
        }),
    },
], None).unwrap();

assert_eq!(sorted_columns[0].as_primitive::<Int64Type>().value(1), -64);
assert!(sorted_columns[0].is_null(0));
```

Note: for multi-column sorts without a limit, using the [row format](https://docs.rs/arrow-row/latest/arrow_row/)
may be significantly faster

---

## lexsort_to_indices

`function` · `arrow_ord::sort::lexsort_to_indices`

Also reachable as `arrow::compute::kernels::sort::lexsort_to_indices`, `arrow::compute::lexsort_to_indices`

```rust
fn lexsort_to_indices(columns: &[SortColumn], limit: Option<usize>) -> Result<UInt32Array, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_ord.sort.lexsort_to_indices.md).


Sort elements lexicographically from a list of `ArrayRef` into an unsigned integer
(`UInt32Array`) of indices.

Note: for multi-column sorts without a limit, using the [row format](https://docs.rs/arrow-row/latest/arrow_row/)
may be significantly faster

---

## partial_sort

`function` · `arrow_ord::sort::partial_sort`

Also reachable as `arrow::compute::kernels::sort::partial_sort`, `arrow::compute::partial_sort`

```rust
fn partial_sort<T, F>(v: &mut [T], limit: usize, is_less: F) where F: FnMut(&T, &T) -> std::cmp::Ordering
```

[Full member, field, variant and typed contracts](../operations/arrow_ord.sort.partial_sort.md).


It's unstable_sort, may not preserve the order of equal elements

---

## partition_validity

`function` · `arrow_ord::sort::partition_validity`

Also reachable as `arrow::compute::kernels::sort::partition_validity`, `arrow::compute::partition_validity`

```rust
fn partition_validity(array: &dyn Array) -> (Vec<u32>, Vec<u32>)
```

[Full member, field, variant and typed contracts](../operations/arrow_ord.sort.partition_validity.md).


Partition indices of an Arrow array into two categories:
- `valid`: indices of non-null elements
- `nulls`: indices of null elements

Optimized for performance with fast-path for all-valid arrays
and bit-parallel scan for null-containing arrays.

---

## sort

`function` · `arrow_ord::sort::sort`

Also reachable as `arrow::compute::kernels::sort::sort`, `arrow::compute::sort`

```rust
fn sort(values: &dyn Array, options: Option<SortOptions>) -> Result<ArrayRef, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_ord.sort.sort.md).


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

---

## sort_limit

`function` · `arrow_ord::sort::sort_limit`

Also reachable as `arrow::compute::kernels::sort::sort_limit`, `arrow::compute::sort_limit`

```rust
fn sort_limit(values: &dyn Array, options: Option<SortOptions>, limit: Option<usize>) -> Result<ArrayRef, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_ord.sort.sort_limit.md).


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

---

## sort_to_indices

`function` · `arrow_ord::sort::sort_to_indices`

Also reachable as `arrow::compute::kernels::sort::sort_to_indices`, `arrow::compute::sort_to_indices`

```rust
fn sort_to_indices(array: &dyn Array, options: Option<SortOptions>, limit: Option<usize>) -> Result<UInt32Array, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_ord.sort.sort_to_indices.md).


Sort elements from `ArrayRef` into an unsigned integer (`UInt32Array`) of indices.
Floats are sorted using IEEE 754 totalOrder.  `limit` is an option for [partial_sort].

---

## FixedLexicographicalComparator

`struct` · `arrow_ord::sort::FixedLexicographicalComparator`

Also reachable as `arrow::compute::FixedLexicographicalComparator`, `arrow::compute::kernels::sort::FixedLexicographicalComparator`

```rust
struct FixedLexicographicalComparator<const N: usize>
```

**Methods** (2)

```rust
fn compare(&self, a_idx: usize, b_idx: usize) -> Ordering
fn try_new(columns: &[SortColumn]) -> Result<FixedLexicographicalComparator<N>, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_ord.sort.FixedLexicographicalComparator.md).


A lexicographical comparator that wraps given array data (columns) and can lexicographically compare data
at given two indices. This version of the comparator is for compile-time constant number of columns.
The lifetime is the same at the data wrapped.

---

## LexicographicalComparator

`struct` · `arrow_ord::sort::LexicographicalComparator`

Also reachable as `arrow::compute::LexicographicalComparator`, `arrow::compute::kernels::sort::LexicographicalComparator`

```rust
struct LexicographicalComparator
```

**Methods** (2)

```rust
fn compare(&self, a_idx: usize, b_idx: usize) -> Ordering
fn try_new(columns: &[SortColumn]) -> Result<LexicographicalComparator, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_ord.sort.LexicographicalComparator.md).


A lexicographical comparator that wraps given array data (columns) and can lexicographically compare data
at given two indices. The lifetime is the same at the data wrapped.

---

## SortColumn

`struct` · `arrow_ord::sort::SortColumn`

Also reachable as `arrow::compute::SortColumn`, `arrow::compute::kernels::sort::SortColumn`

```rust
struct SortColumn
```

**Fields**: `values`, `options`

**Derives**: Clone, Debug

[Full member, field, variant and typed contracts](../operations/arrow_ord.sort.SortColumn.md).


One column to be used in lexicographical sort

---
