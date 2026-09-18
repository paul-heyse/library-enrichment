# `arrow_select::filter::filter_record_batch`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.filter.filter_record_batch.json).

<a id="op-50a068de42a854681b747251"></a>
## filter_record_batch

`function` · `arrow_select::filter::filter_record_batch` · arrow-select 59.3.0

```rust
fn filter_record_batch(record_batch: &RecordBatch, predicate: &BooleanArray) -> Result<RecordBatch, ArrowError>
```

Source: `src/filter.rs:225`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Returns a filtered [RecordBatch](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) where the corresponding elements of
`predicate` are true.

This is the equivalent of calling [filter](../operations/arrow_select.filter.filter.md#op-81498df4434a7e4fa4ec4f9e) on each column of the [RecordBatch](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34).

If multiple record batches (or arrays) need to be filtered using the same predicate array,
consider using [FilterBuilder](../operations/arrow_select.filter.FilterBuilder.md#op-cffeb561fc559f04206cfdc3) to create a single [FilterPredicate](../operations/arrow_select.filter.FilterPredicate.md#op-13d555f716d9c3db0d29bdbd) and then
calling [FilterPredicate::filter_record_batch](../operations/arrow_select.filter.FilterPredicate.md#op-71633f5acf7bee57fadf5cb7).
In contrast to this function, it is then the responsibility of the caller
to use [FilterBuilder::optimize](../operations/arrow_select.filter.FilterBuilder.md#op-58a30c42976fa26d4bc93e1e) if appropriate.
