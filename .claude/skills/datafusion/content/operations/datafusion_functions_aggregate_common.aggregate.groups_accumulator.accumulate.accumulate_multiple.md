# `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::accumulate_multiple`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.groups_accumulator.accumulate.accumulate_multiple.json).

<a id="op-a649bbeb0b1cce5d8d06b69a"></a>
## accumulate_multiple

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::accumulate_multiple` · datafusion-functions-aggregate-common 55.1.0

```rust
fn accumulate_multiple<T, F>(group_indices: &[usize], value_columns: &[&arrow::array::PrimitiveArray<T>], opt_filter: Option<&arrow::array::BooleanArray>, value_fn: F) where T: ArrowPrimitiveType + Send, F: FnMut(usize, usize, &[&arrow::array::PrimitiveArray<T>]) + Send
```

Source: `src/aggregate/groups_accumulator/accumulate.rs:486`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Accumulates with multiple accumulate(value) columns. (e.g. `corr(c1, c2)`)

This method assumes that for any input record index, if any of the value column
is null, or it's filtered out by `opt_filter`, then the record would be ignored.
(Won't be accumulated by `value_fn`)

# Arguments

* `group_indices` - To which groups do the rows in `value_columns` belong
* `value_columns` - The input arrays to accumulate
* `opt_filter` - Optional filter array. If present, only rows where filter is `Some(true)` are included
* `value_fn` - Callback function for each valid row, with parameters:
    * `group_idx`: The group index for the current row
    * `batch_idx`: The index of the current row in the input arrays
    * `columns`: Reference to all input arrays for accessing values
