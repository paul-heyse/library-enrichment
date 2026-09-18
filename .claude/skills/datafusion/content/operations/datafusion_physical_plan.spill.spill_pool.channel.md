# `datafusion_physical_plan::spill::spill_pool::channel`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.spill.spill_pool.channel.json).

<a id="op-61bffbfb1070ccb3cfc06f4a"></a>
## channel

`function` · `datafusion_physical_plan::spill::spill_pool::channel` · datafusion-physical-plan 55.1.0

```rust
fn channel(max_file_size_bytes: usize, spill_manager: std::sync::Arc<super::spill_manager::SpillManager>) -> (SpillPoolWriter, datafusion_execution::SendableRecordBatchStream)
```

Source: `src/spill/spill_pool.rs:480`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Alias for [`mpsc_channel`](../operations/datafusion_physical_plan.spill.spill_pool.mpsc_channel.md#op-03f7c46cd235c8bd43a84cfe).
