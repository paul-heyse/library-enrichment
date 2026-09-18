# `datafusion_execution::memory_pool::arrow::ArrowMemoryPool`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.memory_pool.arrow.ArrowMemoryPool.json).

<a id="op-7de1086832e13fcd2b3f803d"></a>
## ArrowMemoryPool

`struct` · `datafusion_execution::memory_pool::arrow::ArrowMemoryPool` · datafusion-execution 55.1.0

```rust
struct ArrowMemoryPool
```

Source: `src/memory_pool/arrow.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

An adapter that implements Arrow's [`arrow_buffer::MemoryPool`](../operations/arrow_buffer.pool.MemoryPool.md#op-ed45519ea8493e8aed943b0a) trait
by wrapping a DataFusion [`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af).

This allows DataFusion's memory management system to be used with Arrow's
memory allocation APIs. Each reservation made through this pool will be
tracked using the provided [`MemoryConsumer`](../operations/datafusion_execution.memory_pool.MemoryConsumer.md#op-a6a1cc9789177d5167596e41), enabling DataFusion to
monitor and limit memory usage across Arrow operations.

This is useful when you want Arrow operations (such as array builders
or compute kernels) to participate in DataFusion's memory management
and respect the same memory limits as DataFusion operators.

<a id="op-ffb1d47b3f4d3fec6482c91c"></a>
## available

`function` · `datafusion_execution::memory_pool::arrow::ArrowMemoryPool::available` · datafusion-execution 55.1.0

```rust
fn available(&self) -> isize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::arrow::ArrowMemoryPool", "path": "ArrowMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [85, 2], "filename": "src/memory_pool/arrow.rs"}, "trait": {"args": null, "id": "arrow_buffer::pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "arrow_buffer::pool::MemoryPool"}`

Source: `src/memory_pool/arrow.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b48e285f4866f96a58cddeca"></a>
## capacity

`function` · `datafusion_execution::memory_pool::arrow::ArrowMemoryPool::capacity` · datafusion-execution 55.1.0

```rust
fn capacity(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::arrow::ArrowMemoryPool", "path": "ArrowMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [85, 2], "filename": "src/memory_pool/arrow.rs"}, "trait": {"args": null, "id": "arrow_buffer::pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "arrow_buffer::pool::MemoryPool"}`

Source: `src/memory_pool/arrow.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddbda9e6d5fe08a06c5c43cb"></a>
## fmt

`function` · `datafusion_execution::memory_pool::arrow::ArrowMemoryPool::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::arrow::ArrowMemoryPool", "path": "ArrowMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 10], "end": [35, 15], "filename": "src/memory_pool/arrow.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/memory_pool/arrow.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-802f776c4f422aa0e1d14b88"></a>
## new

`function` · `datafusion_execution::memory_pool::arrow::ArrowMemoryPool::new` · datafusion-execution 55.1.0

```rust
fn new(inner: Arc<dyn MemoryPool>, consumer: MemoryConsumer) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::arrow::ArrowMemoryPool", "path": "ArrowMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [47, 2], "filename": "src/memory_pool/arrow.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/arrow.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Creates a new [`ArrowMemoryPool`](../operations/datafusion_execution.memory_pool.arrow.ArrowMemoryPool.md#op-7de1086832e13fcd2b3f803d) that wraps the given DataFusion [`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af)
and tracks allocations under the specified [`MemoryConsumer`](../operations/datafusion_execution.memory_pool.MemoryConsumer.md#op-a6a1cc9789177d5167596e41).

<a id="op-2cb3490406f0f7a95cc8083e"></a>
## reserve

`function` · `datafusion_execution::memory_pool::arrow::ArrowMemoryPool::reserve` · datafusion-execution 55.1.0

```rust
fn reserve(&self, size: usize) -> Box<dyn arrow_buffer::MemoryReservation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::arrow::ArrowMemoryPool", "path": "ArrowMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [85, 2], "filename": "src/memory_pool/arrow.rs"}, "trait": {"args": null, "id": "arrow_buffer::pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "arrow_buffer::pool::MemoryPool"}`

Source: `src/memory_pool/arrow.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c95bed881b0aee497c224c35"></a>
## used

`function` · `datafusion_execution::memory_pool::arrow::ArrowMemoryPool::used` · datafusion-execution 55.1.0

```rust
fn used(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::arrow::ArrowMemoryPool", "path": "ArrowMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [85, 2], "filename": "src/memory_pool/arrow.rs"}, "trait": {"args": null, "id": "arrow_buffer::pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "arrow_buffer::pool::MemoryPool"}`

Source: `src/memory_pool/arrow.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
