# `arrow_select::filter::filter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.filter.filter.json).

<a id="op-81498df4434a7e4fa4ec4f9e"></a>
## filter

`function` · `arrow_select::filter::filter` · arrow-select 59.3.0

```rust
fn filter(values: &dyn Array, predicate: &BooleanArray) -> Result<ArrayRef, ArrowError>
```

Source: `src/filter.rs:201`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Returns a filtered `values` [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21) where the corresponding elements of
`predicate` are `true`.

If multiple arrays (or record batches) need to be filtered using the same predicate array,
consider using [FilterBuilder](../operations/arrow_select.filter.FilterBuilder.md#op-cffeb561fc559f04206cfdc3) to create a single [FilterPredicate](../operations/arrow_select.filter.FilterPredicate.md#op-13d555f716d9c3db0d29bdbd) and then
calling [FilterPredicate::filter_record_batch](../operations/arrow_select.filter.FilterPredicate.md#op-71633f5acf7bee57fadf5cb7).

In contrast to this function, it is then the responsibility of the caller
to use [FilterBuilder::optimize](../operations/arrow_select.filter.FilterBuilder.md#op-58a30c42976fa26d4bc93e1e) if appropriate.

# See also
* [`FilterBuilder`](../operations/arrow_select.filter.FilterBuilder.md#op-cffeb561fc559f04206cfdc3) for more control over the filtering process.
* [`filter_record_batch`](../operations/arrow_select.filter.filter_record_batch.md#op-50a068de42a854681b747251) to filter a [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)
* [`BatchCoalescer`]: to filter multiple [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) and coalesce
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
