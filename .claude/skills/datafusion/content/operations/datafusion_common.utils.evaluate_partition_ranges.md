# `datafusion_common::utils::evaluate_partition_ranges`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.evaluate_partition_ranges.json).

<a id="op-572754d50b19c8692251fed9"></a>
## evaluate_partition_ranges

`function` · `datafusion_common::utils::evaluate_partition_ranges` · datafusion-common 55.1.0

```rust
fn evaluate_partition_ranges(num_rows: usize, partition_columns: &[arrow::compute::SortColumn]) -> Result<Vec<std::ops::Range<usize>>>
```

Source: `src/utils/mod.rs:246`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Given a list of 0 or more already sorted columns, finds the
partition ranges that would partition equally across columns.

See [`partition`](../operations/arrow_ord.partition.partition.md#op-bb0253e80df493c54b1c48d6) for more details.
