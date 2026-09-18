# `datafusion_execution::memory_pool`

Crate `datafusion-execution` · 4 public items · structured records in [`model/datafusion_execution.memory_pool.json`](../model/datafusion_execution.memory_pool.json)

## MemoryLimit

`enum` · `datafusion_execution::memory_pool::MemoryLimit`

```rust
enum MemoryLimit
```

**Variants**: `Infinite`, `Finite`, `Unknown`

[Full member, field, variant and typed contracts](../operations/datafusion_execution.memory_pool.MemoryLimit.md).


Memory limit of `MemoryPool`

---

## MemoryConsumer

`struct` · `datafusion_execution::memory_pool::MemoryConsumer`

```rust
struct MemoryConsumer
```

**Derives**: Debug, Eq, Hash, PartialEq

**Methods** (7)

```rust
fn can_spill(&self) -> bool
fn clone_with_new_id(&self) -> Self
fn id(&self) -> usize
fn name(&self) -> &str
fn new(name: impl Into<String>) -> Self
fn register(self, pool: &Arc<dyn MemoryPool>) -> MemoryReservation
fn with_can_spill(self, can_spill: bool) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.memory_pool.MemoryConsumer.md).


A memory consumer is a named allocation traced by a particular
[`MemoryReservation`] in a [`MemoryPool`]. All allocations are registered to
a particular `MemoryConsumer`;

Each `MemoryConsumer` is identifiable by a process-unique id, and is therefore not cloneable,
If you want a clone of a `MemoryConsumer`, you should look into [`MemoryConsumer::clone_with_new_id`],
but note that this `MemoryConsumer` may be treated as a separate entity based on the used pool,
and is only guaranteed to share the name and inner properties.

For help with allocation accounting, see the [`proxy`] module.

[proxy]: datafusion_common::utils::proxy

---

## MemoryReservation

`struct` · `datafusion_execution::memory_pool::MemoryReservation`

```rust
struct MemoryReservation
```

**Implements**: `arrow_buffer::pool::MemoryReservation`, `core::ops::drop::Drop`

**Derives**: Debug

**Methods** (12)

```rust
fn consumer(&self) -> &MemoryConsumer
fn free(&self) -> usize
fn grow(&self, capacity: usize)
fn new_empty(&self) -> Self
fn resize(&self, capacity: usize)
fn shrink(&self, capacity: usize)
fn size(&self) -> usize
fn split(&self, capacity: usize) -> MemoryReservation
fn take(&mut self) -> MemoryReservation
fn try_grow(&self, capacity: usize) -> Result<()>
fn try_resize(&self, capacity: usize) -> Result<()>
fn try_shrink(&self, capacity: usize) -> Result<usize>
```

**via `arrow_buffer::pool::MemoryReservation`**

```rust
fn resize(&mut self, new_size: usize)
fn size(&self) -> usize
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.memory_pool.MemoryReservation.md).


A [`MemoryReservation`] tracks an individual reservation of a
number of bytes of memory in a [`MemoryPool`] that is freed back
to the pool on drop.

The reservation can be grown or shrunk over time.

---

## MemoryPool

`trait` · `datafusion_execution::memory_pool::MemoryPool`

```rust
trait MemoryPool: Any + Send + Sync + std::fmt::Debug + Display
```

**Implementors** (5)

- `datafusion_execution::memory_pool::peak_recording::PeakRecordingPool`
- `datafusion_execution::memory_pool::pool::FairSpillPool`
- `datafusion_execution::memory_pool::pool::GreedyMemoryPool`
- `datafusion_execution::memory_pool::pool::TrackConsumersPool`
- `datafusion_execution::memory_pool::pool::UnboundedMemoryPool`

**Methods** (8)

```rust
fn grow(&self, reservation: &MemoryReservation, additional: usize)
fn memory_limit(&self) -> MemoryLimit
fn name(&self) -> &str
fn register(&self, _consumer: &MemoryConsumer)
fn reserved(&self) -> usize
fn shrink(&self, reservation: &MemoryReservation, shrink: usize)
fn try_grow(&self, reservation: &MemoryReservation, additional: usize) -> Result<()>
fn unregister(&self, _consumer: &MemoryConsumer)
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.memory_pool.MemoryPool.md).


Tracks and potentially limits memory use across operators during execution.

# Memory Management Overview

DataFusion is a streaming query engine, processing most queries without
buffering the entire input. Most operators require a fixed amount of memory
based on the schema and target batch size. However, certain operations such
as sorting and grouping/joining, require buffering intermediate results,
which can require memory proportional to the number of input rows.

Rather than tracking all allocations, DataFusion takes a pragmatic approach:
Intermediate memory used as data streams through the system is not accounted
(it assumed to be "small") but the large consumers of memory must register
and constrain their use. This design trades off the additional code
complexity of memory tracking with limiting resource usage.

When limiting memory with a `MemoryPool` you should typically reserve some
overhead (e.g. 10%) for the "small" memory allocations that are not tracked.

# Memory Management Design

As explained above, DataFusion's design ONLY limits operators that require
"large" amounts of memory (proportional to number of input rows), such as
`GroupByHashExec`. It does NOT track and limit memory used internally by
other operators such as `DataSourceExec` or the `RecordBatch`es that flow
between operators. Furthermore, operators should not reserve memory for the
batches they produce. Instead, if a consumer operator needs to hold batches
from its producers in memory for an extended period, it is the consumer
operator's responsibility to reserve the necessary memory for those batches.

In order to avoid allocating memory until the OS or the container system
kills the process, DataFusion `ExecutionPlan`s (operators) that consume
large amounts of memory must first request their desired allocation from a
[`MemoryPool`] before allocating more.  The request is typically managed via
a  [`MemoryReservation`] and [`MemoryConsumer`].

If the allocation is successful, the operator should proceed and allocate
the desired memory. If the allocation fails, the operator must either first
free memory (e.g. by spilling to local disk) and try again, or error.

Note that a `MemoryPool` can be shared by concurrently executing plans,
which can be used to control memory usage in a multi-tenant system.

# How MemoryPool works by example

Scenario 1:
For `Filter` operator, `RecordBatch`es will stream through it, so it
don't have to keep track of memory usage through [`MemoryPool`].

Scenario 2:
For `CrossJoin` operator, if the input size gets larger, the intermediate
state will also grow. So `CrossJoin` operator will use [`MemoryPool`] to
limit the memory usage.
2.1 `CrossJoin` operator has read a new batch, asked memory pool for
additional memory. Memory pool updates the usage and returns success.
2.2 `CrossJoin` has read another batch, and tries to reserve more memory
again, memory pool does not have enough memory. Since `CrossJoin` operator
has not implemented spilling, it will stop execution and return an error.

Scenario 3:
For `Aggregate` operator, its intermediate states will also accumulate as
the input size gets larger, but with spilling capability. When it tries to
reserve more memory from the memory pool, and the memory pool has already
reached the memory limit, it will return an error. Then, `Aggregate`
operator will spill the intermediate buffers to disk, and release memory
from the memory pool, and continue to retry memory reservation.

# Related Structs

To better understand memory management in DataFusion, here are the key structs
and their relationships:

- [`MemoryConsumer`]: A named allocation traced by a particular operator. If an
  execution is parallelized, and there are multiple partitions of the same
  operator, each partition will have a separate `MemoryConsumer`.
- `SharedRegistration`: A registration of a `MemoryConsumer` with a `MemoryPool`.
  `SharedRegistration` and `MemoryPool` have a many-to-one relationship. `MemoryPool`
  implementation can decide how to allocate memory based on the registered consumers.
  (e.g. `FairSpillPool` will try to share available memory evenly among all registered
  consumers)
- [`MemoryReservation`]: Each `MemoryConsumer`/operator can have multiple
  `MemoryReservation`s for different internal data structures. The relationship
  between `MemoryConsumer` and `MemoryReservation` is one-to-many. This design
  enables cleaner operator implementations:
  - Different `MemoryReservation`s can be used for different purposes
  - `MemoryReservation` follows RAII principles - to release a reservation,
    simply drop the `MemoryReservation` object. When all `MemoryReservation`s
    for a `SharedRegistration` are dropped, the `SharedRegistration` is dropped
    when its reference count reaches zero, automatically unregistering the
    `MemoryConsumer` from the `MemoryPool`.

## Relationship Diagram

```text
┌──────────────────┐     ┌──────────────────┐
│MemoryReservation │     │MemoryReservation │
└───┬──────────────┘     └──────────────────┘ ......
    │belongs to                    │
    │      ┌───────────────────────┘           │  │
    │      │                                   │  │
    ▼      ▼                                   ▼  ▼
┌────────────────────────┐       ┌────────────────────────┐
│   SharedRegistration   │       │   SharedRegistration   │
│   ┌────────────────┐   │       │   ┌────────────────┐   │
│   │                │   │       │   │                │   │
│   │ MemoryConsumer │   │       │   │ MemoryConsumer │   │
│   │                │   │       │   │                │   │
│   └────────────────┘   │       │   └────────────────┘   │
└────────────┬───────────┘       └────────────┬───────────┘
             │                                │
             │                        register│into
             │                                │
             └─────────────┐   ┌──────────────┘
                           │   │
                           ▼   ▼
   ╔═══════════════════════════════════════════════════╗
   ║                                                   ║
   ║                    MemoryPool                     ║
   ║                                                   ║
   ╚═══════════════════════════════════════════════════╝
```

For example, there are two parallel partitions of an operator X: each partition
corresponds to a `MemoryConsumer` in the above diagram. Inside each partition of
operator X, there are typically several `MemoryReservation`s - one for each
internal data structure that needs memory tracking (e.g., 1 reservation for the hash
table, and 1 reservation for buffered input, etc.).

# Implementing `MemoryPool`

You can implement a custom allocation policy by implementing the
[`MemoryPool`] trait and configuring a `SessionContext` appropriately.
However, DataFusion comes with the following simple memory pool implementations that
handle many common cases:

* [`UnboundedMemoryPool`]: no memory limits (the default)

* [`GreedyMemoryPool`]: Limits memory usage to a fixed size using a "first
  come first served" policy

* [`FairSpillPool`]: Limits memory usage to a fixed size, allocating memory
  to all spilling operators fairly

* [`TrackConsumersPool`]: Wraps another [`MemoryPool`] and tracks consumers,
  providing better error messages on the largest memory users.

---
