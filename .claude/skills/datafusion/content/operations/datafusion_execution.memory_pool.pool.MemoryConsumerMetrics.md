# `datafusion_execution::memory_pool::pool::MemoryConsumerMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.memory_pool.pool.MemoryConsumerMetrics.json).

<a id="op-503ff4c34d8e39bca188fadb"></a>
## MemoryConsumerMetrics

`struct` · `datafusion_execution::memory_pool::pool::MemoryConsumerMetrics` · datafusion-execution 55.1.0

```rust
struct MemoryConsumerMetrics
```

Source: `src/memory_pool/pool.rs:358`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

A point-in-time snapshot of a tracked memory consumer's state.

Returned by [`TrackConsumersPool::metrics()`](../operations/datafusion_execution.memory_pool.pool.TrackConsumersPool.md#op-7735355716c2034d59a21a05).

<a id="op-9abcea535cbb629ab6768f2f"></a>
## can_spill

`struct_field` · `datafusion_execution::memory_pool::pool::MemoryConsumerMetrics::can_spill` · datafusion-execution 55.1.0

```rust
can_spill: bool
```

Source: `src/memory_pool/pool.rs:362`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Whether this consumer can spill to disk

<a id="op-fc467ebb249b1f66b6251e27"></a>
## clone

`function` · `datafusion_execution::memory_pool::pool::MemoryConsumerMetrics::clone` · datafusion-execution 55.1.0

```rust
fn clone(&self) -> MemoryConsumerMetrics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::MemoryConsumerMetrics", "path": "MemoryConsumerMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 17], "end": [357, 22], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/memory_pool/pool.rs:357`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fb02357a1001474e01d76ab"></a>
## fmt

`function` · `datafusion_execution::memory_pool::pool::MemoryConsumerMetrics::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::MemoryConsumerMetrics", "path": "MemoryConsumerMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 10], "end": [357, 15], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/memory_pool/pool.rs:357`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8d4d5ef4083272b92f6d2b0"></a>
## name

`struct_field` · `datafusion_execution::memory_pool::pool::MemoryConsumerMetrics::name` · datafusion-execution 55.1.0

```rust
name: String
```

Source: `src/memory_pool/pool.rs:360`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

The name of the memory consumer

<a id="op-6865ba1253c2bd2ceae4097f"></a>
## peak

`struct_field` · `datafusion_execution::memory_pool::pool::MemoryConsumerMetrics::peak` · datafusion-execution 55.1.0

```rust
peak: usize
```

Source: `src/memory_pool/pool.rs:366`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

The peak number of bytes reserved by this consumer

<a id="op-d1833988c1b3f6342e8c3aa8"></a>
## reserved

`struct_field` · `datafusion_execution::memory_pool::pool::MemoryConsumerMetrics::reserved` · datafusion-execution 55.1.0

```rust
reserved: usize
```

Source: `src/memory_pool/pool.rs:364`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

The number of bytes currently reserved by this consumer
