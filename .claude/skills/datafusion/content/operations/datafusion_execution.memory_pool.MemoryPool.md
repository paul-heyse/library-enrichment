# `datafusion_execution::memory_pool::MemoryPool`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.memory_pool.MemoryPool.json).

<a id="op-05e08c54432abb360395d4af"></a>
## MemoryPool

`trait` · `datafusion_execution::memory_pool::MemoryPool` · datafusion-execution 55.1.0

```rust
trait MemoryPool: Any + Send + Sync + std::fmt::Debug + Display
```

Source: `src/memory_pool/mod.rs:188`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

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
[`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af) before allocating more.  The request is typically managed via
a  [`MemoryReservation`](../operations/datafusion_execution.memory_pool.MemoryReservation.md#op-a0611bd048c178da7e0d05df) and [`MemoryConsumer`](../operations/datafusion_execution.memory_pool.MemoryConsumer.md#op-a6a1cc9789177d5167596e41).

If the allocation is successful, the operator should proceed and allocate
the desired memory. If the allocation fails, the operator must either first
free memory (e.g. by spilling to local disk) and try again, or error.

Note that a `MemoryPool` can be shared by concurrently executing plans,
which can be used to control memory usage in a multi-tenant system.

# How MemoryPool works by example

Scenario 1:
For `Filter` operator, `RecordBatch`es will stream through it, so it
don't have to keep track of memory usage through [`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af).

Scenario 2:
For `CrossJoin` operator, if the input size gets larger, the intermediate
state will also grow. So `CrossJoin` operator will use [`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af) to
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

- [`MemoryConsumer`](../operations/datafusion_execution.memory_pool.MemoryConsumer.md#op-a6a1cc9789177d5167596e41): A named allocation traced by a particular operator. If an
  execution is parallelized, and there are multiple partitions of the same
  operator, each partition will have a separate `MemoryConsumer`.
- `SharedRegistration`: A registration of a `MemoryConsumer` with a `MemoryPool`.
  `SharedRegistration` and `MemoryPool` have a many-to-one relationship. `MemoryPool`
  implementation can decide how to allocate memory based on the registered consumers.
  (e.g. `FairSpillPool` will try to share available memory evenly among all registered
  consumers)
- [`MemoryReservation`](../operations/datafusion_execution.memory_pool.MemoryReservation.md#op-a0611bd048c178da7e0d05df): Each `MemoryConsumer`/operator can have multiple
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
[`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af) trait and configuring a `SessionContext` appropriately.
However, DataFusion comes with the following simple memory pool implementations that
handle many common cases:

* [`UnboundedMemoryPool`](../operations/datafusion_execution.memory_pool.pool.UnboundedMemoryPool.md#op-bc210b8fe79074b67e0fb694): no memory limits (the default)

* [`GreedyMemoryPool`](../operations/datafusion_execution.memory_pool.pool.GreedyMemoryPool.md#op-2f1ce34e98ac037b2abc0c26): Limits memory usage to a fixed size using a "first
  come first served" policy

* [`FairSpillPool`](../operations/datafusion_execution.memory_pool.pool.FairSpillPool.md#op-beb0cc08c58aa417e2866734): Limits memory usage to a fixed size, allocating memory
  to all spilling operators fairly

* [`TrackConsumersPool`](../operations/datafusion_execution.memory_pool.pool.TrackConsumersPool.md#op-68ccf7cd2608b07636f02ae9): Wraps another [`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af) and tracks consumers,
  providing better error messages on the largest memory users.

<a id="op-901665d129617381f78ddd6c"></a>
## grow

`function` · `datafusion_execution::memory_pool::MemoryPool::grow` · datafusion-execution 55.1.0

```rust
fn grow(&self, reservation: &MemoryReservation, additional: usize)
```

Source: `src/memory_pool/mod.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Infallibly grow the provided `reservation` by `additional` bytes

This must always succeed

<a id="op-17fe84af23b4f41e7f01b41e"></a>
## memory_limit

`function` · `datafusion_execution::memory_pool::MemoryPool::memory_limit` · datafusion-execution 55.1.0

```rust
fn memory_limit(&self) -> MemoryLimit
```

Source: `src/memory_pool/mod.rs:225`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Return the memory limit of the pool

The default implementation of `MemoryPool::memory_limit`
will return `MemoryLimit::Unknown`.
If you are using your custom memory pool, but have the requirement to
know the memory usage limit of the pool, please implement this method
to return it(`Memory::Finite(limit)`).

<a id="op-0f0e8f533cb6517f9ca299fa"></a>
## name

`function` · `datafusion_execution::memory_pool::MemoryPool::name` · datafusion-execution 55.1.0

```rust
fn name(&self) -> &str
```

Source: `src/memory_pool/mod.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Return pool name

<a id="op-1314b1707a7bd206ae235f18"></a>
## register

`function` · `datafusion_execution::memory_pool::MemoryPool::register` · datafusion-execution 55.1.0

```rust
fn register(&self, _consumer: &MemoryConsumer)
```

Source: `src/memory_pool/mod.rs:195`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Registers a new [`MemoryConsumer`](../operations/datafusion_execution.memory_pool.MemoryConsumer.md#op-a6a1cc9789177d5167596e41)

Note: Subsequent calls to [`Self::grow`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-901665d129617381f78ddd6c) must be made to reserve memory

<a id="op-b2c68b53e8e4ef3cae11576a"></a>
## reserved

`function` · `datafusion_execution::memory_pool::MemoryPool::reserved` · datafusion-execution 55.1.0

```rust
fn reserved(&self) -> usize
```

Source: `src/memory_pool/mod.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Return the total amount of memory reserved

<a id="op-2f7bc0a4f5fb725a09daa959"></a>
## shrink

`function` · `datafusion_execution::memory_pool::MemoryPool::shrink` · datafusion-execution 55.1.0

```rust
fn shrink(&self, reservation: &MemoryReservation, shrink: usize)
```

Source: `src/memory_pool/mod.rs:208`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Infallibly shrink the provided `reservation` by `shrink` bytes

<a id="op-6a4b695aac89e13396396a9f"></a>
## try_grow

`function` · `datafusion_execution::memory_pool::MemoryPool::try_grow` · datafusion-execution 55.1.0

```rust
fn try_grow(&self, reservation: &MemoryReservation, additional: usize) -> Result<()>
```

Source: `src/memory_pool/mod.rs:213`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Attempt to grow the provided `reservation` by `additional` bytes

On error the `allocation` will not be increased in size

<a id="op-d32df2abd6a19de7f780ba32"></a>
## unregister

`function` · `datafusion_execution::memory_pool::MemoryPool::unregister` · datafusion-execution 55.1.0

```rust
fn unregister(&self, _consumer: &MemoryConsumer)
```

Source: `src/memory_pool/mod.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Records the destruction of a [`MemoryReservation`](../operations/datafusion_execution.memory_pool.MemoryReservation.md#op-a0611bd048c178da7e0d05df) with [`MemoryConsumer`](../operations/datafusion_execution.memory_pool.MemoryConsumer.md#op-a6a1cc9789177d5167596e41)

Note: Prior calls to [`Self::shrink`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-2f7bc0a4f5fb725a09daa959) must be made to free any reserved memory
