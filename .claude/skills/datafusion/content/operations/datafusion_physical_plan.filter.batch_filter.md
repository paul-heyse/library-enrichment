# `datafusion_physical_plan::filter::batch_filter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.filter.batch_filter.json).

<a id="op-4013d0579384b00e04179a18"></a>
## batch_filter

`function` · `datafusion_physical_plan::filter::batch_filter` · datafusion-physical-plan 55.1.0

```rust
fn batch_filter(batch: &arrow::record_batch::RecordBatch, predicate: &std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::Result<arrow::record_batch::RecordBatch>
```

Source: `src/filter.rs:1229`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
