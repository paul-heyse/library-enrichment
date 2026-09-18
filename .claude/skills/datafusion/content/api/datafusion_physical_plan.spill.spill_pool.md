# `datafusion_physical_plan::spill::spill_pool`

Crate `datafusion-physical-plan` · 6 public items · structured records in [`model/datafusion_physical_plan.spill.spill_pool.json`](../model/datafusion_physical_plan.spill.spill_pool.json)

## channel

`function` · `datafusion_physical_plan::spill::spill_pool::channel`

> **Deprecated** — Use mpsc_channel instead

```rust
fn channel(max_file_size_bytes: usize, spill_manager: std::sync::Arc<super::spill_manager::SpillManager>) -> (SpillPoolWriter, datafusion_execution::SendableRecordBatchStream)
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.spill.spill_pool.channel.md).


Alias for [`mpsc_channel`].

---

## mpsc_channel

`function` · `datafusion_physical_plan::spill::spill_pool::mpsc_channel`

```rust
fn mpsc_channel(max_file_size_bytes: usize, spill_manager: std::sync::Arc<super::spill_manager::SpillManager>) -> (SpillPoolWriter, datafusion_execution::SendableRecordBatchStream)
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.spill.spill_pool.mpsc_channel.md).


Creates a paired writer and reader for a spill pool with MPSC (multi-producer,
single-consumer) semantics. See [`spsc_channel`] for the general architecture description
of the spill pool.

Additional writers can be created by cloning the returned [`SpillPoolWriter`].

In contrast to [`spsc_channel`], this implementation provides no guarantees regarding
the read order of the returned [`SendableRecordBatchStream`].

If you need strict end-to-end FIFO (a single writer whose batches are read back in exact
write order), use [`spsc_channel`] instead.

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

---

## spsc_channel

`function` · `datafusion_physical_plan::spill::spill_pool::spsc_channel`

```rust
fn spsc_channel(max_file_size_bytes: usize, spill_manager: std::sync::Arc<super::spill_manager::SpillManager>) -> (SpillPoolSink, datafusion_execution::SendableRecordBatchStream)
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.spill.spill_pool.spsc_channel.md).


Creates a paired writer and reader for a spill pool with SPSC (single-producer,
single-consumer) semantics and strict FIFO ordering.

If you need a spill pool that supports several producers, use [`mpsc_channel`] instead.

The reader can start reading immediately after the writer appends a batch
to the spill file, without waiting for the file to be sealed, while the writer continues to
write more data.

Internally this coordinates rotating spill files based on size limits, and
handles asynchronous notification between the writer and reader using wakers.
This ensures that we manage disk usage efficiently while allowing concurrent
I/O between the writer and reader.

# Data Flow Overview

1. Writer write batch `B0` to F1
2. Writer write batch `B1` to F1, notices the size limit exceeded, finishes F1.
3. Reader read `B0` from F1
4. Reader read `B1`, no more batch to read -> wait on the waker
5. Writer write batch `B2` to a new file `F2`, wake up the waiting reader.
6. Reader read `B2` from F2.
7. Repeat until writer is dropped.

# Architecture

```text
┌─────────────────────────────────────────────────────────────────────────┐
│                            SpillPool                                    │
│                                                                         │
│  Writer Side              Shared State              Reader Side         │
│  ───────────              ────────────              ───────────         │
│                                                                         │
│  SpillPoolSink      ┌────────────────────┐    RecordBatchStream         │
│       │             │  VecDeque<File>    │          │                   │
│       │             │  ┌────┐┌────┐      │          │                   │
│  push_batch()       │  │ F1 ││ F2 │ ...  │      next().await            │
│       │             │  └────┘└────┘      │          │                   │
│       ▼             │                    │          ▼                   │
│  ┌─────────┐        │                    │    ┌──────────┐              │
│  │Current  │───────▶│ Coordination:      │◀───│ Current  │              │
│  │Write    │        │ - Wakers           │    │ Read     │              │
│  │File     │        │ - Batch counts     │    │ File     │              │
│  └─────────┘        │ - Writer status    │    └──────────┘              │
│       │             └────────────────────┘           │                  │
│       │                                              │                  │
│  Size > limit?                                Read all batches?         │
│       │                                              │                  │
│       ▼                                              ▼                  │
│  Rotate to new file                            Pop from queue           │
└─────────────────────────────────────────────────────────────────────────┘

Writer produces → Shared queue → Reader consumes
```

# File State Machine

Each file in the pool coordinates between writer and reader:

```text
               Writer View              Reader View
               ───────────              ───────────

Created        writer: Some(..)         batches_read: 0
               batches_written: 0       (waiting for data)
                      │
                      ▼
Writing        append_batch()           Can read if:
               batches_written++        batches_read < batches_written
               wake readers
                      │                        │
                      │                        ▼
               ┌──────┴──────┐          poll_next() → batch
               │             │          batches_read++
               ▼             ▼
         Size > limit?  More data?
               │             │
               │             └─▶ Yes ──▶ Continue writing
               ▼
         finish()                   Reader catches up:
         writer_finished = true     batches_read == batches_written
         wake readers                       │
               │                            ▼
               └─────────────────────▶ Returns Poll::Ready(None)
                                      File complete, pop from queue
```

# Arguments

* `max_file_size_bytes` - Maximum size per file before rotation. When a file
  exceeds this size, the writer automatically rotates to a new file.
* `spill_manager` - Manager for file creation and metrics tracking

# Returns

A tuple of `(SpillPoolSink, SendableRecordBatchStream)` that share the same
underlying pool. The reader is returned as a stream for immediate use with
async stream combinators.

# Example

```
use std::sync::Arc;
use arrow::array::{ArrayRef, Int32Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use datafusion_execution::runtime_env::RuntimeEnv;
use futures::StreamExt;

# use datafusion_physical_plan::spill::spill_pool;
# use datafusion_physical_plan::spill::SpillManager; // Re-exported for doctests
# use datafusion_physical_plan::metrics::{ExecutionPlanMetricsSet, SpillMetrics};
#
# #[tokio::main]
# async fn main() -> datafusion_common::Result<()> {
# // Setup for the example (typically comes from TaskContext in production)
# let env = Arc::new(RuntimeEnv::default());
# let metrics = SpillMetrics::new(&ExecutionPlanMetricsSet::new(), 0);
# let schema = Arc::new(Schema::new(vec![Field::new("a", DataType::Int32, false)]));
# let spill_manager = Arc::new(SpillManager::new(env, metrics, schema.clone()));
#
// Create channel with 1MB file size limit
let (writer, mut reader) = spill_pool::spsc_channel(1024 * 1024, spill_manager);

// Spawn writer and reader concurrently; writer wakes reader via wakers
let writer_task = tokio::spawn(async move {
    for i in 0..5 {
        let array: ArrayRef = Arc::new(Int32Array::from(vec![i; 100]));
        let batch = RecordBatch::try_new(schema.clone(), vec![array]).unwrap();
        writer.push_batch(&batch)?;
    }
    // Explicitly drop writer to finalize the spill file and wake the reader
    drop(writer);
    datafusion_common::Result::<()>::Ok(())
});

let reader_task = tokio::spawn(async move {
    let mut batches_read = 0;
    while let Some(result) = reader.next().await {
        let _batch = result?;
        batches_read += 1;
    }
    datafusion_common::Result::<usize>::Ok(batches_read)
});

let (writer_res, reader_res) = tokio::join!(writer_task, reader_task);
writer_res
    .map_err(|e| datafusion_common::DataFusionError::Execution(e.to_string()))??;
let batches_read = reader_res
    .map_err(|e| datafusion_common::DataFusionError::Execution(e.to_string()))??;

assert_eq!(batches_read, 5);
# Ok(())
# }
```

# Why rotate files?

File rotation ensures we don't end up with unreferenced disk usage.
If we used a single file for all spilled data, we would end up with
unreferenced data at the beginning of the file that has already been read
by readers but we can't delete because you can't truncate from the start of a file.

Consider the case of a query like `SELECT * FROM large_table WHERE false`.
Obviously this query produces no output rows, but if we had a spilling operator
in the middle of this query between the scan and the filter it would see the entire
`large_table` flow through it and thus would spill all of that data to disk.
So we'd end up using up to `size(large_table)` bytes of disk space.
If instead we use file rotation, and as long as the readers can keep up with the writer,
then we can ensure that once a file is fully read by all readers it can be deleted,
thus bounding the maximum disk usage to roughly `max_file_size_bytes`.

---

## SpillPoolReader

`struct` · `datafusion_physical_plan::spill::spill_pool::SpillPoolReader`

```rust
struct SpillPoolReader
```

**Implements**: `datafusion_execution::stream::RecordBatchStream`, `futures_core::stream::Stream`

**via `datafusion_execution::stream::RecordBatchStream`**

```rust
fn schema(&self) -> SchemaRef
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(std::pin::Pin<&mut self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.spill.spill_pool.SpillPoolReader.md).


A stream that reads from a SpillPool. The reader guarantees FIFO order if a single writer is used.

Created by [`spsc_channel`]. See that function for architecture diagrams and usage examples.

The stream automatically handles file rotation and reads from completed files.
When no data is available, it returns `Poll::Pending` and registers a waker to
be notified when the writer produces more data.

# Infinite Stream Semantics

This stream never returns `None` (`Poll::Ready(None)`) on its own - it will keep
waiting for the writer to produce more data. The stream ends only when:
- The reader is dropped
- The writer is dropped AND all queued data has been consumed

This makes it suitable for continuous streaming scenarios where the writer may
produce data intermittently.

---

## SpillPoolSink

`struct` · `datafusion_physical_plan::spill::spill_pool::SpillPoolSink`

```rust
struct SpillPoolSink
```

**Implements**: `core::ops::drop::Drop`

**Methods** (1)

```rust
fn push_batch(&self, batch: &RecordBatch) -> Result<()>
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.spill.spill_pool.SpillPoolSink.md).


Single writer for a spill pool that cannot be cloned.

Created by [`spsc_channel`] and [`SpillPoolWriter::new_sink`].

---

## SpillPoolWriter

`struct` · `datafusion_physical_plan::spill::spill_pool::SpillPoolWriter`

```rust
struct SpillPoolWriter
```

**Derives**: Clone

**Methods** (2)

```rust
fn new_sink(&self) -> SpillPoolSink
fn push_batch(&self, batch: &RecordBatch) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.spill.spill_pool.SpillPoolWriter.md).


Writer for a spill pool that can be cloned to produce additional writers.

Created by [`mpsc_channel`]. See that function for architecture diagrams and usage
examples.

---
