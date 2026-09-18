# `datafusion_execution::memory_pool::pool::TrackConsumersPool`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.memory_pool.pool.TrackConsumersPool.json).

<a id="op-68ccf7cd2608b07636f02ae9"></a>
## TrackConsumersPool

`struct` · `datafusion_execution::memory_pool::pool::TrackConsumersPool` · datafusion-execution 55.1.0

```rust
struct TrackConsumersPool<I>
```

Source: `src/memory_pool/pool.rs:405`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

A [`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af) that tracks the consumers that have
reserved memory within the inner memory pool.

By tracking memory reservations more carefully this pool
can provide better error messages on the largest memory users
when memory allocation fails.

Tracking is per hashed [`MemoryConsumer`](../operations/datafusion_execution.memory_pool.MemoryConsumer.md#op-a6a1cc9789177d5167596e41), not per [`MemoryReservation`](../operations/datafusion_execution.memory_pool.MemoryReservation.md#op-a0611bd048c178da7e0d05df).
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

<a id="op-347dc9cacfd5c8ee0e35365f"></a>
## fmt

`function` · `datafusion_execution::memory_pool::pool::TrackConsumersPool::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "datafusion_execution::memory_pool::pool::TrackConsumersPool", "path": "TrackConsumersPool"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [404, 10], "end": [404, 15], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/memory_pool/pool.rs:404`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b98916755f3ebff2528790a3"></a>
## fmt

`function` · `datafusion_execution::memory_pool::pool::TrackConsumersPool::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "datafusion_execution::memory_pool::pool::TrackConsumersPool", "path": "TrackConsumersPool"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}}}], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [424, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/memory_pool/pool.rs:415`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb3eaf7baa27e0ddd08eb167"></a>
## grow

`function` · `datafusion_execution::memory_pool::pool::TrackConsumersPool::grow` · datafusion-execution 55.1.0

```rust
fn grow(&self, reservation: &MemoryReservation, additional: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "datafusion_execution::memory_pool::pool::TrackConsumersPool", "path": "TrackConsumersPool"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}}}], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [603, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:550`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3cf374fa7fb5da0e1395ce51"></a>
## inner

`function` · `datafusion_execution::memory_pool::pool::TrackConsumersPool::inner` · datafusion-execution 55.1.0

```rust
fn inner(&self) -> &I
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "datafusion_execution::memory_pool::pool::TrackConsumersPool", "path": "TrackConsumersPool"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}}}], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [426, 1], "end": [518, 2], "filename": "src/memory_pool/pool.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/pool.rs:472`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns a reference to the wrapped inner [`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af).

<a id="op-3432914db59cd779b1c5c807"></a>
## memory_limit

`function` · `datafusion_execution::memory_pool::pool::TrackConsumersPool::memory_limit` · datafusion-execution 55.1.0

```rust
fn memory_limit(&self) -> MemoryLimit
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "datafusion_execution::memory_pool::pool::TrackConsumersPool", "path": "TrackConsumersPool"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}}}], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [603, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:600`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7735355716c2034d59a21a05"></a>
## metrics

`function` · `datafusion_execution::memory_pool::pool::TrackConsumersPool::metrics` · datafusion-execution 55.1.0

```rust
fn metrics(&self) -> Vec<MemoryConsumerMetrics>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "datafusion_execution::memory_pool::pool::TrackConsumersPool", "path": "TrackConsumersPool"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}}}], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [426, 1], "end": [518, 2], "filename": "src/memory_pool/pool.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/pool.rs:477`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns a snapshot of all currently tracked consumers.

<a id="op-fd5e9edd9abc7a9a7d6e1369"></a>
## name

`function` · `datafusion_execution::memory_pool::pool::TrackConsumersPool::name` · datafusion-execution 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "datafusion_execution::memory_pool::pool::TrackConsumersPool", "path": "TrackConsumersPool"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}}}], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [603, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:521`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab717c80b405573bb768e6d0"></a>
## new

`function` · `datafusion_execution::memory_pool::pool::TrackConsumersPool::new` · datafusion-execution 55.1.0

```rust
fn new(inner: I, top: NonZeroUsize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "datafusion_execution::memory_pool::pool::TrackConsumersPool", "path": "TrackConsumersPool"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}}}], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [426, 1], "end": [518, 2], "filename": "src/memory_pool/pool.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/pool.rs:463`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Creates a new [`TrackConsumersPool`](../operations/datafusion_execution.memory_pool.pool.TrackConsumersPool.md#op-68ccf7cd2608b07636f02ae9).

# Arguments
* `inner` - The underlying memory pool that handles actual memory allocation
* `top` - The number of top memory consumers to include in error messages

# Note
In most cases, you should use [`RuntimeEnvBuilder::with_memory_limit()`](crate::runtime_env::RuntimeEnvBuilder::with_memory_limit)
instead of creating this pool manually, as it automatically sets up tracking with
sensible defaults (top 5 consumers).

# Example

```rust
use datafusion_execution::memory_pool::{
    FairSpillPool, GreedyMemoryPool, TrackConsumersPool,
};
use std::num::NonZeroUsize;

// Create with a greedy pool backend, reporting top 3 consumers in error messages
let tracked_greedy = TrackConsumersPool::new(
    GreedyMemoryPool::new(1024 * 1024), // 1MB limit
    NonZeroUsize::new(3).unwrap(),
);

// Create with a fair spill pool backend, reporting top 5 consumers in error messages
let tracked_fair = TrackConsumersPool::new(
    FairSpillPool::new(2 * 1024 * 1024), // 2MB limit
    NonZeroUsize::new(5).unwrap(),
);
```

# Impact on Error Messages

The `top` determines how many Top K [`MemoryConsumer`](../operations/datafusion_execution.memory_pool.MemoryConsumer.md#op-a6a1cc9789177d5167596e41)s to include
in the reported [`DataFusionError::ResourcesExhausted`](../operations/datafusion_common.error.DataFusionError.md#op-34eaf91f0bb9b35d94bf98e7).

<a id="op-4d0c51a08ae925799c4794cf"></a>
## register

`function` · `datafusion_execution::memory_pool::pool::TrackConsumersPool::register` · datafusion-execution 55.1.0

```rust
fn register(&self, consumer: &MemoryConsumer)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "datafusion_execution::memory_pool::pool::TrackConsumersPool", "path": "TrackConsumersPool"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}}}], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [603, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:525`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-893ccb0184a29d34fff5645b"></a>
## report_top

`function` · `datafusion_execution::memory_pool::pool::TrackConsumersPool::report_top` · datafusion-execution 55.1.0

```rust
fn report_top(&self, top: usize) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "datafusion_execution::memory_pool::pool::TrackConsumersPool", "path": "TrackConsumersPool"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}}}], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [426, 1], "end": [518, 2], "filename": "src/memory_pool/pool.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/pool.rs:486`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns a formatted string with the top memory consumers.

<a id="op-67fb2572214b30e87e7c9346"></a>
## reserved

`function` · `datafusion_execution::memory_pool::pool::TrackConsumersPool::reserved` · datafusion-execution 55.1.0

```rust
fn reserved(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "datafusion_execution::memory_pool::pool::TrackConsumersPool", "path": "TrackConsumersPool"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}}}], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [603, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:596`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5eedd51145c6dc64b53f59d9"></a>
## shrink

`function` · `datafusion_execution::memory_pool::pool::TrackConsumersPool::shrink` · datafusion-execution 55.1.0

```rust
fn shrink(&self, reservation: &MemoryReservation, shrink: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "datafusion_execution::memory_pool::pool::TrackConsumersPool", "path": "TrackConsumersPool"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}}}], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [603, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:560`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c798ca1d7e6e07b83d011e0f"></a>
## try_grow

`function` · `datafusion_execution::memory_pool::pool::TrackConsumersPool::try_grow` · datafusion-execution 55.1.0

```rust
fn try_grow(&self, reservation: &MemoryReservation, additional: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "datafusion_execution::memory_pool::pool::TrackConsumersPool", "path": "TrackConsumersPool"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}}}], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [603, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:570`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43a1b42e30f2c6955bbeb2e8"></a>
## unregister

`function` · `datafusion_execution::memory_pool::pool::TrackConsumersPool::unregister` · datafusion-execution 55.1.0

```rust
fn unregister(&self, consumer: &MemoryConsumer)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "datafusion_execution::memory_pool::pool::TrackConsumersPool", "path": "TrackConsumersPool"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}}}], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [603, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:545`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
