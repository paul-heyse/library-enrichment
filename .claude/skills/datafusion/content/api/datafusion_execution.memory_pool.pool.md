# `datafusion_execution::memory_pool::pool`

Crate `datafusion-execution` · 5 public items · structured records in [`model/datafusion_execution.memory_pool.pool.json`](../model/datafusion_execution.memory_pool.pool.json)

## FairSpillPool

`struct` · `datafusion_execution::memory_pool::pool::FairSpillPool`

```rust
struct FairSpillPool
```

**Implements**: `core::fmt::Display`, `datafusion_execution::memory_pool::MemoryPool`

**Derives**: Debug

**Methods** (1)

```rust
fn new(pool_size: usize) -> Self
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

[Full member, field, variant and typed contracts](../operations/datafusion_execution.memory_pool.pool.FairSpillPool.md).


A [`MemoryPool`] that prevents spillable reservations from using more than
an even fraction of the available memory sans any unspillable reservations
(i.e. `(pool_size - unspillable_memory) / num_spillable_reservations`)

This pool works best when you know beforehand the query has
multiple spillable operators that will likely all need to
spill. Sometimes it will cause spills even when there was
sufficient memory (reserved for other operators) to avoid doing
so.

```text
   ┌───────────────────────z──────────────────────z───────────────┐
   │                       z                      z               │
   │                       z                      z               │
   │       Spillable       z       Unspillable    z     Free      │
   │        Memory         z        Memory        z    Memory     │
   │                       z                      z               │
   │                       z                      z               │
   └───────────────────────z──────────────────────z───────────────┘
```

Unspillable memory is allocated in a first-come, first-serve fashion

---

## GreedyMemoryPool

`struct` · `datafusion_execution::memory_pool::pool::GreedyMemoryPool`

```rust
struct GreedyMemoryPool
```

**Implements**: `core::fmt::Display`, `datafusion_execution::memory_pool::MemoryPool`

**Derives**: Debug

**Methods** (1)

```rust
fn new(pool_size: usize) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_execution::memory_pool::MemoryPool`**

```rust
fn grow(&self, _reservation: &MemoryReservation, additional: usize)
fn memory_limit(&self) -> MemoryLimit
fn name(&self) -> &str
fn reserved(&self) -> usize
fn shrink(&self, _reservation: &MemoryReservation, shrink: usize)
fn try_grow(&self, reservation: &MemoryReservation, additional: usize) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.memory_pool.pool.GreedyMemoryPool.md).


A [`MemoryPool`] that implements a greedy first-come first-serve limit.

This pool works well for queries that do not need to spill or have
a single spillable operator. See [`FairSpillPool`] if there are
multiple spillable operators that all will spill.

---

## MemoryConsumerMetrics

`struct` · `datafusion_execution::memory_pool::pool::MemoryConsumerMetrics`

```rust
struct MemoryConsumerMetrics
```

**Fields**: `name`, `can_spill`, `reserved`, `peak`

**Derives**: Clone, Debug

[Full member, field, variant and typed contracts](../operations/datafusion_execution.memory_pool.pool.MemoryConsumerMetrics.md).


A point-in-time snapshot of a tracked memory consumer's state.

Returned by [`TrackConsumersPool::metrics()`].

---

## TrackConsumersPool

`struct` · `datafusion_execution::memory_pool::pool::TrackConsumersPool`

```rust
struct TrackConsumersPool<I>
```

**Implements**: `core::fmt::Display`, `datafusion_execution::memory_pool::MemoryPool`

**Derives**: Debug

**Methods** (4)

```rust
fn inner(&self) -> &I
fn metrics(&self) -> Vec<MemoryConsumerMetrics>
fn new(inner: I, top: NonZeroUsize) -> Self
fn report_top(&self, top: usize) -> String
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

[Full member, field, variant and typed contracts](../operations/datafusion_execution.memory_pool.pool.TrackConsumersPool.md).


A [`MemoryPool`] that tracks the consumers that have
reserved memory within the inner memory pool.

By tracking memory reservations more carefully this pool
can provide better error messages on the largest memory users
when memory allocation fails.

Tracking is per hashed [`MemoryConsumer`], not per [`MemoryReservation`].
The same consumer can have multiple reservations.

# Automatic Usage via [`RuntimeEnvBuilder`]

The easiest way to use `TrackConsumersPool` is via
[`RuntimeEnvBuilder::with_memory_limit()`].

[`RuntimeEnvBuilder`]: crate::runtime_env::RuntimeEnvBuilder
[`RuntimeEnvBuilder::with_memory_limit()`]: crate::runtime_env::RuntimeEnvBuilder::with_memory_limit

# Usage Examples

For more examples of using `TrackConsumersPool`, see the [memory_pool_tracking.rs] example

[memory_pool_tracking.rs]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/execution_monitoring/memory_pool_tracking.rs
[memory_pool_execution_plan.rs]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/execution_monitoring/memory_pool_execution_plan.rs

---

## UnboundedMemoryPool

`struct` · `datafusion_execution::memory_pool::pool::UnboundedMemoryPool`

```rust
struct UnboundedMemoryPool
```

**Implements**: `core::fmt::Display`, `datafusion_execution::memory_pool::MemoryPool`

**Derives**: Debug, Default

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_execution::memory_pool::MemoryPool`**

```rust
fn grow(&self, _reservation: &MemoryReservation, additional: usize)
fn memory_limit(&self) -> MemoryLimit
fn name(&self) -> &str
fn reserved(&self) -> usize
fn shrink(&self, _reservation: &MemoryReservation, shrink: usize)
fn try_grow(&self, reservation: &MemoryReservation, additional: usize) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.memory_pool.pool.UnboundedMemoryPool.md).


A [`MemoryPool`] that enforces no limit

---
