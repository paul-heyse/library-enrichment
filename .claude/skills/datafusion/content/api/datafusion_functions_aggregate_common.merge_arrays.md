# `datafusion_functions_aggregate_common::merge_arrays`

Crate `datafusion-functions-aggregate-common` · 1 public items · structured records in [`model/datafusion_functions_aggregate_common.merge_arrays.json`](../model/datafusion_functions_aggregate_common.merge_arrays.json)

## merge_ordered_arrays

`function` · `datafusion_functions_aggregate_common::merge_arrays::merge_ordered_arrays`

```rust
fn merge_ordered_arrays(values: &mut [std::collections::VecDeque<datafusion_common::ScalarValue>], ordering_values: &mut [std::collections::VecDeque<Vec<datafusion_common::ScalarValue>>], sort_options: &[arrow::compute::SortOptions]) -> datafusion_common::Result<(Vec<datafusion_common::ScalarValue>, Vec<Vec<datafusion_common::ScalarValue>>)>
```

This functions merges `values` array (`&[Vec<ScalarValue>]`) into single array `Vec<ScalarValue>`
Merging done according to ordering values stored inside `ordering_values` (`&[Vec<Vec<ScalarValue>>]`)
Inner `Vec<ScalarValue>` in the `ordering_values` can be thought as ordering information for
each `ScalarValue` in the `values` array.
Desired ordering specified by `sort_options` argument (Should have same size with inner `Vec<ScalarValue>`
of the `ordering_values` array).

As an example
values can be \[
     \[1, 2, 3, 4, 5\],
     \[1, 2, 3, 4\],
     \[1, 2, 3, 4, 5, 6\],
\]
In this case we will be merging three arrays (doesn't have to be same size)
and produce a merged array with size 15 (sum of 5+4+6)
Merging will be done according to ordering at `ordering_values` vector.
As an example `ordering_values` can be [
     \[(1, a), (2, b), (3, b), (4, a), (5, b) \],
     \[(1, a), (2, b), (3, b), (4, a) \],
     \[(1, b), (2, c), (3, d), (4, e), (5, a), (6, b) \],
]
For each ScalarValue in the `values` we have a corresponding `Vec<ScalarValue>` (like timestamp of it)
for the example above `sort_options` will have size two, that defines ordering requirement of the merge.
Inner `Vec<ScalarValue>`s of the `ordering_values` will be compared according `sort_options` (Their sizes should match)

---
