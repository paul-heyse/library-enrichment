# `datafusion_physical_plan::common::compute_record_batch_statistics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.common.compute_record_batch_statistics.json).

<a id="op-c60252a7f8183f48215f9197"></a>
## compute_record_batch_statistics

`function` · `datafusion_physical_plan::common::compute_record_batch_statistics` · datafusion-physical-plan 55.1.0

```rust
fn compute_record_batch_statistics(batches: &[Vec<arrow::record_batch::RecordBatch>], schema: &arrow::datatypes::Schema, projection: Option<Vec<usize>>) -> Statistics
```

Source: `src/common.rs:232`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Computes the statistics for an in-memory RecordBatch

Only computes statistics that are in arrows metadata (num rows, byte size and nulls)
and does not apply any kernel on the actual data.
