# `datafusion_physical_plan::spill::spill_pool::mpsc_channel`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.spill.spill_pool.mpsc_channel.json).

<a id="op-03f7c46cd235c8bd43a84cfe"></a>
## mpsc_channel

`function` · `datafusion_physical_plan::spill::spill_pool::mpsc_channel` · datafusion-physical-plan 55.1.0

```rust
fn mpsc_channel(max_file_size_bytes: usize, spill_manager: std::sync::Arc<super::spill_manager::SpillManager>) -> (SpillPoolWriter, datafusion_execution::SendableRecordBatchStream)
```

Source: `src/spill/spill_pool.rs:515`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Creates a paired writer and reader for a spill pool with MPSC (multi-producer,
single-consumer) semantics. See [`spsc_channel`](../operations/datafusion_physical_plan.spill.spill_pool.spsc_channel.md#op-334be7f1fdba1ab78f028f38) for the general architecture description
of the spill pool.

Additional writers can be created by cloning the returned [`SpillPoolWriter`](../operations/datafusion_physical_plan.spill.spill_pool.SpillPoolWriter.md#op-e6eb74ac5d1a8f5d6893d42a).

In contrast to [`spsc_channel`](../operations/datafusion_physical_plan.spill.spill_pool.spsc_channel.md#op-334be7f1fdba1ab78f028f38), this implementation provides no guarantees regarding
the read order of the returned [`SendableRecordBatchStream`](../operations/datafusion_execution.stream.SendableRecordBatchStream.md#op-7cf25e4c554567392af11cb6).

If you need strict end-to-end FIFO (a single writer whose batches are read back in exact
write order), use [`spsc_channel`](../operations/datafusion_physical_plan.spill.spill_pool.spsc_channel.md#op-334be7f1fdba1ab78f028f38) instead.

# File Management

The shared channel uses the same size-based rotation trigger as the [single producer channel](spsc_channel).
All writers share the same pool of write files and coordinate file rotation. The number of open
files is kept as small as possible. When more writes occur concurrently than there are open write
files an additional file will be opened to write to. This prevents multiple writers from blocking
each other.

When the last writer clone is dropped, it finalizes any remaining open write files so that all
written data can be accessed by the reader.

# Returns

A tuple of `(SpillPoolWriter, SendableRecordBatchStream)` that share the same
underlying pool. The reader is returned as a stream for immediate use with
async stream combinators. The writer can be cloned to create additional writers.
