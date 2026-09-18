# `datafusion_execution::memory_pool::peak_recording::PeakRecordingPool`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.memory_pool.peak_recording.PeakRecordingPool.json).

<a id="op-62aae50647c3bfb123164a55"></a>
## PeakRecordingPool

`struct` · `datafusion_execution::memory_pool::peak_recording::PeakRecordingPool` · datafusion-execution 55.1.0

```rust
struct PeakRecordingPool
```

Source: `src/memory_pool/peak_recording.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Wraps a [`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af), recording the high-water mark of
[`MemoryPool::reserved`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-b2c68b53e8e4ef3cae11576a) as reservations come and go.

Every method delegates to the wrapped pool, so wrapping does not change how
memory is granted, limited, or reported. The one thing it does change is
downcasting: `rt.memory_pool.downcast_ref::<FairSpillPool>()` now finds this
wrapper instead of the pool it wraps. Nothing in the benchmarks relies on
that, and [`Self::from_pool`](../operations/datafusion_execution.memory_pool.peak_recording.PeakRecordingPool.md#op-e22ea613f41970af8032246d) uses the same mechanism to find the recorder.

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

<a id="op-d263331eea5862728c4c549f"></a>
## fmt

`function` · `datafusion_execution::memory_pool::peak_recording::PeakRecordingPool::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::peak_recording::PeakRecordingPool", "path": "PeakRecordingPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 1], "end": [171, 2], "filename": "src/memory_pool/peak_recording.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/memory_pool/peak_recording.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc9c4c6a45f965cc391b4758"></a>
## fmt

`function` · `datafusion_execution::memory_pool::peak_recording::PeakRecordingPool::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::peak_recording::PeakRecordingPool", "path": "PeakRecordingPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 1], "end": [179, 2], "filename": "src/memory_pool/peak_recording.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/memory_pool/peak_recording.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e22ea613f41970af8032246d"></a>
## from_pool

`function` · `datafusion_execution::memory_pool::peak_recording::PeakRecordingPool::from_pool` · datafusion-execution 55.1.0

```rust
fn from_pool(pool: &dyn MemoryPool) -> Option<&Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::peak_recording::PeakRecordingPool", "path": "PeakRecordingPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [161, 2], "filename": "src/memory_pool/peak_recording.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/peak_recording.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

The recorder installed as `pool`, if there is one.

Returns `None` whenever a benchmark runs without a memory limit, since
`CommonOpt::runtime_env_builder` only installs the wrapper alongside a
pool it has a limit for.

<a id="op-3ae26d1bf13df9b50911b48f"></a>
## grow

`function` · `datafusion_execution::memory_pool::peak_recording::PeakRecordingPool::grow` · datafusion-execution 55.1.0

```rust
fn grow(&self, reservation: &MemoryReservation, additional: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::peak_recording::PeakRecordingPool", "path": "PeakRecordingPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [217, 2], "filename": "src/memory_pool/peak_recording.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/peak_recording.rs:194`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7bb6a8748430659960d8cf4"></a>
## max_reserved

`function` · `datafusion_execution::memory_pool::peak_recording::PeakRecordingPool::max_reserved` · datafusion-execution 55.1.0

```rust
fn max_reserved(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::peak_recording::PeakRecordingPool", "path": "PeakRecordingPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [161, 2], "filename": "src/memory_pool/peak_recording.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/peak_recording.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Peak reservation, in bytes, since this pool was created.

Unlike [`Self::peak_reserved`](../operations/datafusion_execution.memory_pool.peak_recording.PeakRecordingPool.md#op-217a2cf6b86d48282593086c) this is never reset, so it reports the
peak across every query that shared this pool.

<a id="op-b5967864c6b384db03fee85e"></a>
## memory_limit

`function` · `datafusion_execution::memory_pool::peak_recording::PeakRecordingPool::memory_limit` · datafusion-execution 55.1.0

```rust
fn memory_limit(&self) -> MemoryLimit
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::peak_recording::PeakRecordingPool", "path": "PeakRecordingPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [217, 2], "filename": "src/memory_pool/peak_recording.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/peak_recording.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e20494398d2db3e2e9d6cd7b"></a>
## name

`function` · `datafusion_execution::memory_pool::peak_recording::PeakRecordingPool::name` · datafusion-execution 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::peak_recording::PeakRecordingPool", "path": "PeakRecordingPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [217, 2], "filename": "src/memory_pool/peak_recording.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/peak_recording.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b1d1f51ce5d2e8fd67cf563"></a>
## new

`function` · `datafusion_execution::memory_pool::peak_recording::PeakRecordingPool::new` · datafusion-execution 55.1.0

```rust
fn new(inner: Arc<dyn MemoryPool>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::peak_recording::PeakRecordingPool", "path": "PeakRecordingPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [161, 2], "filename": "src/memory_pool/peak_recording.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/peak_recording.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Wrap `inner`, recording its peak reservation from here on.

`inner` is expected to be empty: the running total starts at zero, so
anything reserved before wrapping is not counted.

<a id="op-217a2cf6b86d48282593086c"></a>
## peak_reserved

`function` · `datafusion_execution::memory_pool::peak_recording::PeakRecordingPool::peak_reserved` · datafusion-execution 55.1.0

```rust
fn peak_reserved(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::peak_recording::PeakRecordingPool", "path": "PeakRecordingPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [161, 2], "filename": "src/memory_pool/peak_recording.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/peak_recording.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Peak reservation, in bytes, since the last [`Self::reset_peak`](../operations/datafusion_execution.memory_pool.peak_recording.PeakRecordingPool.md#op-2ac562e53ba5465662667b11).

<a id="op-fdcc26a45aff00cbf6a7df5e"></a>
## register

`function` · `datafusion_execution::memory_pool::peak_recording::PeakRecordingPool::register` · datafusion-execution 55.1.0

```rust
fn register(&self, consumer: &MemoryConsumer)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::peak_recording::PeakRecordingPool", "path": "PeakRecordingPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [217, 2], "filename": "src/memory_pool/peak_recording.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/peak_recording.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2aa170df35d11528b3b30043"></a>
## reserved

`function` · `datafusion_execution::memory_pool::peak_recording::PeakRecordingPool::reserved` · datafusion-execution 55.1.0

```rust
fn reserved(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::peak_recording::PeakRecordingPool", "path": "PeakRecordingPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [217, 2], "filename": "src/memory_pool/peak_recording.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/peak_recording.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ac562e53ba5465662667b11"></a>
## reset_peak

`function` · `datafusion_execution::memory_pool::peak_recording::PeakRecordingPool::reset_peak` · datafusion-execution 55.1.0

```rust
fn reset_peak(&self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::peak_recording::PeakRecordingPool", "path": "PeakRecordingPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [161, 2], "filename": "src/memory_pool/peak_recording.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/peak_recording.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Reset the value returned by [`Self::peak_reserved`](../operations/datafusion_execution.memory_pool.peak_recording.PeakRecordingPool.md#op-217a2cf6b86d48282593086c) to what is reserved
right now, so the next reading covers only what follows.

`BenchmarkRun::start_new_case` calls this, giving each benchmark query
its own reading. Anything still held when a query starts — data the
benchmark loaded up front, say — stays in the reading, since the query
runs with those bytes reserved.

<a id="op-7cb6ce2937be5e30952d9221"></a>
## shrink

`function` · `datafusion_execution::memory_pool::peak_recording::PeakRecordingPool::shrink` · datafusion-execution 55.1.0

```rust
fn shrink(&self, reservation: &MemoryReservation, shrink: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::peak_recording::PeakRecordingPool", "path": "PeakRecordingPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [217, 2], "filename": "src/memory_pool/peak_recording.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/peak_recording.rs:199`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c4e1ca574890e9b83c0c21f"></a>
## try_grow

`function` · `datafusion_execution::memory_pool::peak_recording::PeakRecordingPool::try_grow` · datafusion-execution 55.1.0

```rust
fn try_grow(&self, reservation: &MemoryReservation, additional: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::peak_recording::PeakRecordingPool", "path": "PeakRecordingPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [217, 2], "filename": "src/memory_pool/peak_recording.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/peak_recording.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91c0583545f45f09cebaeaf6"></a>
## unregister

`function` · `datafusion_execution::memory_pool::peak_recording::PeakRecordingPool::unregister` · datafusion-execution 55.1.0

```rust
fn unregister(&self, consumer: &MemoryConsumer)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::peak_recording::PeakRecordingPool", "path": "PeakRecordingPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [217, 2], "filename": "src/memory_pool/peak_recording.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/peak_recording.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
