# `datafusion_physical_plan::common::collect`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.common.collect.json).

<a id="op-f496f48951d639a94d1d803a"></a>
## collect

`function` · `datafusion_physical_plan::common::collect` · datafusion-physical-plan 55.1.0

```rust
async fn collect(stream: super::SendableRecordBatchStream) -> datafusion_common::Result<Vec<arrow::record_batch::RecordBatch>>
```

Source: `src/common.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a vector of record batches from a stream
