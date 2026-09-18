# `datafusion_execution::memory_pool::MemoryConsumer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.memory_pool.MemoryConsumer.json).

<a id="op-a6a1cc9789177d5167596e41"></a>
## MemoryConsumer

`struct` · `datafusion_execution::memory_pool::MemoryConsumer` · datafusion-execution 55.1.0

```rust
struct MemoryConsumer
```

Source: `src/memory_pool/mod.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

A memory consumer is a named allocation traced by a particular
[`MemoryReservation`](../operations/datafusion_execution.memory_pool.MemoryReservation.md#op-a0611bd048c178da7e0d05df) in a [`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af). All allocations are registered to
a particular `MemoryConsumer`;

Each `MemoryConsumer` is identifiable by a process-unique id, and is therefore not cloneable,
If you want a clone of a `MemoryConsumer`, you should look into [`MemoryConsumer::clone_with_new_id`](../operations/datafusion_execution.memory_pool.MemoryConsumer.md#op-879a7683e97e018547a86a75),
but note that this `MemoryConsumer` may be treated as a separate entity based on the used pool,
and is only guaranteed to share the name and inner properties.

For help with allocation accounting, see the [`proxy`](../modules/datafusion_execution.memory_pool.proxy.md#op-9435ca29020d6a81558b21a4) module.

[proxy]: datafusion_common::utils::proxy

<a id="op-cb2cebd5fa47ecae3261980f"></a>
## can_spill

`function` · `datafusion_execution::memory_pool::MemoryConsumer::can_spill` · datafusion-execution 55.1.0

```rust
fn can_spill(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryConsumer", "path": "MemoryConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [351, 2], "filename": "src/memory_pool/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/mod.rs:330`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns true if this allocation can spill to disk

<a id="op-879a7683e97e018547a86a75"></a>
## clone_with_new_id

`function` · `datafusion_execution::memory_pool::MemoryConsumer::clone_with_new_id` · datafusion-execution 55.1.0

```rust
fn clone_with_new_id(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryConsumer", "path": "MemoryConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [351, 2], "filename": "src/memory_pool/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/mod.rs:311`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns a clone of this [`MemoryConsumer`](../operations/datafusion_execution.memory_pool.MemoryConsumer.md#op-a6a1cc9789177d5167596e41) with a new unique id,
which can be registered with a [`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af),
This new consumer is separate from the original.

<a id="op-ce2776567a98034a50b1de9b"></a>
## eq

`function` · `datafusion_execution::memory_pool::MemoryConsumer::eq` · datafusion-execution 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryConsumer", "path": "MemoryConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [281, 2], "filename": "src/memory_pool/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/memory_pool/mod.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bef96a9427b307b1e344e50a"></a>
## fmt

`function` · `datafusion_execution::memory_pool::MemoryConsumer::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryConsumer", "path": "MemoryConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 10], "end": [262, 15], "filename": "src/memory_pool/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/memory_pool/mod.rs:262`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e72f492e869b98b8755a02b"></a>
## hash

`function` · `datafusion_execution::memory_pool::MemoryConsumer::hash` · datafusion-execution 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryConsumer", "path": "MemoryConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [291, 2], "filename": "src/memory_pool/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/memory_pool/mod.rs:286`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94bd89d6f34765c489d5abcb"></a>
## id

`function` · `datafusion_execution::memory_pool::MemoryConsumer::id` · datafusion-execution 55.1.0

```rust
fn id(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryConsumer", "path": "MemoryConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [351, 2], "filename": "src/memory_pool/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/mod.rs:320`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Return the unique id of this [`MemoryConsumer`](../operations/datafusion_execution.memory_pool.MemoryConsumer.md#op-a6a1cc9789177d5167596e41)

<a id="op-d067406353d62b488db63381"></a>
## name

`function` · `datafusion_execution::memory_pool::MemoryConsumer::name` · datafusion-execution 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryConsumer", "path": "MemoryConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [351, 2], "filename": "src/memory_pool/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/mod.rs:335`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns the name associated with this allocation

<a id="op-4ad98cb3930c9009989c6496"></a>
## new

`function` · `datafusion_execution::memory_pool::MemoryConsumer::new` · datafusion-execution 55.1.0

```rust
fn new(name: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryConsumer", "path": "MemoryConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [351, 2], "filename": "src/memory_pool/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/mod.rs:300`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Create a new empty [`MemoryConsumer`](../operations/datafusion_execution.memory_pool.MemoryConsumer.md#op-a6a1cc9789177d5167596e41) that can be grown using [`MemoryReservation`](../operations/datafusion_execution.memory_pool.MemoryReservation.md#op-a0611bd048c178da7e0d05df)

<a id="op-591b66978ce774ec96818bc8"></a>
## register

`function` · `datafusion_execution::memory_pool::MemoryConsumer::register` · datafusion-execution 55.1.0

```rust
fn register(self, pool: &Arc<dyn MemoryPool>) -> MemoryReservation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryConsumer", "path": "MemoryConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [351, 2], "filename": "src/memory_pool/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/mod.rs:341`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Registers this [`MemoryConsumer`](../operations/datafusion_execution.memory_pool.MemoryConsumer.md#op-a6a1cc9789177d5167596e41) with the provided [`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af) returning
a [`MemoryReservation`](../operations/datafusion_execution.memory_pool.MemoryReservation.md#op-a0611bd048c178da7e0d05df) that can be used to grow or shrink the memory reservation

<a id="op-51daac0b2b2750b247c01152"></a>
## with_can_spill

`function` · `datafusion_execution::memory_pool::MemoryConsumer::with_can_spill` · datafusion-execution 55.1.0

```rust
fn with_can_spill(self, can_spill: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryConsumer", "path": "MemoryConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [351, 2], "filename": "src/memory_pool/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/mod.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Set whether this allocation can be spilled to disk
