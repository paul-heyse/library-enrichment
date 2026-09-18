# `datafusion_execution::memory_pool::peak_recording`

Crate `datafusion-execution` · 1 public items · structured records in [`model/datafusion_execution.memory_pool.peak_recording.json`](../model/datafusion_execution.memory_pool.peak_recording.json)

## PeakRecordingPool

`struct` · `datafusion_execution::memory_pool::peak_recording::PeakRecordingPool`

```rust
struct PeakRecordingPool
```

**Implements**: `core::fmt::Display`, `datafusion_execution::memory_pool::MemoryPool`

**Derives**: Debug

**Methods** (5)

```rust
fn from_pool(pool: &dyn MemoryPool) -> Option<&Self>
fn max_reserved(&self) -> usize
fn new(inner: Arc<dyn MemoryPool>) -> Self
fn peak_reserved(&self) -> usize
fn reset_peak(&self)
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_execution::memory_pool::MemoryPool`**

```rust
fn grow(&self, reservation: &MemoryReservation, additional: usize)
fn memory_limit(&self) -> MemoryLimit
fn name(&self) -> &str
fn register(&self, consumer: &MemoryConsumer)
fn reserved(&self) -> usize
fn shrink(&self, reservation: &MemoryReservation, shrink: usize)
fn try_grow(&self, reservation: &MemoryReservation, additional: usize) -> Result<()>
fn unregister(&self, consumer: &MemoryConsumer)
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.memory_pool.peak_recording.PeakRecordingPool.md).


Wraps a [`MemoryPool`], recording the high-water mark of
[`MemoryPool::reserved`] as reservations come and go.

Every method delegates to the wrapped pool, so wrapping does not change how
memory is granted, limited, or reported. The one thing it does change is
downcasting: `rt.memory_pool.downcast_ref::<FairSpillPool>()` now finds this
wrapper instead of the pool it wraps. Nothing in the benchmarks relies on
that, and [`Self::from_pool`] uses the same mechanism to find the recorder.

Both high-water marks are held per instance, so a benchmark that builds a
fresh runtime per query gets a reading scoped to that query without any
coordination.

# Example

```
# use std::sync::Arc;
# use datafusion_execution::memory_pool::{GreedyMemoryPool, MemoryConsumer, MemoryPool, PeakRecordingPool};
let recording = Arc::new(PeakRecordingPool::new(Arc::new(GreedyMemoryPool::new(1024))));
let pool: Arc<dyn MemoryPool> = Arc::clone(&recording) as _;

let reservation = MemoryConsumer::new("example").register(&pool);
reservation.try_grow(512)?;
reservation.shrink(512);

// The pool is back to empty, but the high-water mark is retained.
assert_eq!(pool.reserved(), 0);
assert_eq!(recording.peak_reserved(), 512);

// The recorder can also be recovered from the pool it was installed as.
assert_eq!(PeakRecordingPool::from_pool(&*pool).unwrap().peak_reserved(), 512);
# Ok::<(), datafusion_common::DataFusionError>(())
```

---
