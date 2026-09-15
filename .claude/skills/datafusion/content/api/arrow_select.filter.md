# `arrow_select::filter`

Crate `arrow-select` · 6 public items · structured records in [`model/arrow_select.filter.json`](../model/arrow_select.filter.json)

## filter

`function` · `arrow_select::filter::filter`

Also reachable as `arrow::compute::filter`, `arrow::compute::kernels::filter::filter`

```rust
fn filter(values: &dyn Array, predicate: &BooleanArray) -> Result<ArrayRef, ArrowError>
```

Returns a filtered `values` [`Array`] where the corresponding elements of
`predicate` are `true`.

If multiple arrays (or record batches) need to be filtered using the same predicate array,
consider using [FilterBuilder] to create a single [FilterPredicate] and then
calling [FilterPredicate::filter_record_batch].

In contrast to this function, it is then the responsibility of the caller
to use [FilterBuilder::optimize] if appropriate.

# See also
* [`FilterBuilder`] for more control over the filtering process.
* [`filter_record_batch`] to filter a [`RecordBatch`]
* [`BatchCoalescer`]: to filter multiple [`RecordBatch`] and coalesce
  the results into a single array.

[`BatchCoalescer`]: crate::coalesce::BatchCoalescer

# Example
```rust
# use arrow_array::{Int32Array, BooleanArray};
# use arrow_select::filter::filter;
let array = Int32Array::from(vec![5, 6, 7, 8, 9]);
let filter_array = BooleanArray::from(vec![true, false, false, true, false]);
let c = filter(&array, &filter_array).unwrap();
let c = c.as_any().downcast_ref::<Int32Array>().unwrap();
assert_eq!(c, &Int32Array::from(vec![5, 8]));
```

---

## filter_record_batch

`function` · `arrow_select::filter::filter_record_batch`

Also reachable as `arrow::compute::filter_record_batch`, `arrow::compute::kernels::filter::filter_record_batch`

```rust
fn filter_record_batch(record_batch: &RecordBatch, predicate: &BooleanArray) -> Result<RecordBatch, ArrowError>
```

Returns a filtered [RecordBatch] where the corresponding elements of
`predicate` are true.

This is the equivalent of calling [filter] on each column of the [RecordBatch].

If multiple record batches (or arrays) need to be filtered using the same predicate array,
consider using [FilterBuilder] to create a single [FilterPredicate] and then
calling [FilterPredicate::filter_record_batch].
In contrast to this function, it is then the responsibility of the caller
to use [FilterBuilder::optimize] if appropriate.

---

## prep_null_mask_filter

`function` · `arrow_select::filter::prep_null_mask_filter`

Also reachable as `arrow::compute::kernels::filter::prep_null_mask_filter`, `arrow::compute::prep_null_mask_filter`

```rust
fn prep_null_mask_filter(filter: &BooleanArray) -> BooleanArray
```

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

---

## FilterBuilder

`struct` · `arrow_select::filter::FilterBuilder`

Also reachable as `arrow::compute::FilterBuilder`, `arrow::compute::kernels::filter::FilterBuilder`

```rust
struct FilterBuilder
```

**Derives**: Debug

**Methods** (4)

```rust
fn build(self) -> FilterPredicate
fn is_optimize_beneficial(data_type: &DataType) -> bool
fn new(filter: &BooleanArray) -> Self
fn optimize(self) -> Self
```

A builder to construct [`FilterPredicate`]

---

## FilterPredicate

`struct` · `arrow_select::filter::FilterPredicate`

Also reachable as `arrow::compute::FilterPredicate`, `arrow::compute::kernels::filter::FilterPredicate`

```rust
struct FilterPredicate
```

**Derives**: Debug

**Methods** (4)

```rust
fn count(&self) -> usize
fn filter(&self, values: &dyn Array) -> Result<ArrayRef, ArrowError>
fn filter_nulls(&self, nulls: Option<&NullBuffer>) -> Option<NullBuffer>
fn filter_record_batch(&self, record_batch: &RecordBatch) -> Result<RecordBatch, ArrowError>
```

A filtering predicate that can be applied to an [`Array`]

---

## SlicesIterator

`struct` · `arrow_select::filter::SlicesIterator`

Also reachable as `arrow::compute::SlicesIterator`, `arrow::compute::kernels::filter::SlicesIterator`

```rust
struct SlicesIterator<'a>
```

**Implements**: `core::convert::From`, `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**Methods** (1)

```rust
fn new(filter: &'a BooleanArray) -> Self
```

**via `core::convert::From`**

```rust
fn from(filter: &'a BooleanBuffer) -> Self
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

An iterator of `(usize, usize)` each representing an interval
`[start, end)` whose slots of a bitmap [Buffer] are true.

Each interval corresponds to a contiguous region of memory to be
"taken" from an array to be filtered.

## Notes:

1. Ignores the validity bitmap (ignores nulls)

2. Only performant for filters that copy across long contiguous runs

---
