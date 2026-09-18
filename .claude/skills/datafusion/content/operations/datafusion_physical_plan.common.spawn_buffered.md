# `datafusion_physical_plan::common::spawn_buffered`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.common.spawn_buffered.json).

<a id="op-90173e4c01f18101e5967e02"></a>
## spawn_buffered

`function` · `datafusion_physical_plan::common::spawn_buffered` · datafusion-physical-plan 55.1.0

```rust
fn spawn_buffered(input: super::SendableRecordBatchStream, buffer: usize) -> super::SendableRecordBatchStream
```

Source: `src/common.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

If running in a tokio context spawns the execution of `stream` to a separate task
allowing it to execute in parallel with an intermediate buffer of size `buffer`.
At most `buffer` record batches will be produced ahead of the consumer.
