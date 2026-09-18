# `datafusion_execution::memory_pool::pool::GreedyMemoryPool`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.memory_pool.pool.GreedyMemoryPool.json).

<a id="op-2f1ce34e98ac037b2abc0c26"></a>
## GreedyMemoryPool

`struct` · `datafusion_execution::memory_pool::pool::GreedyMemoryPool` · datafusion-execution 55.1.0

```rust
struct GreedyMemoryPool
```

Source: `src/memory_pool/pool.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

A [`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af) that implements a greedy first-come first-serve limit.

This pool works well for queries that do not need to spill or have
a single spillable operator. See [`FairSpillPool`](../operations/datafusion_execution.memory_pool.pool.FairSpillPool.md#op-beb0cc08c58aa417e2866734) if there are
multiple spillable operators that all will spill.

<a id="op-676ff2748fa2a5f97d2df658"></a>
## fmt

`function` · `datafusion_execution::memory_pool::pool::GreedyMemoryPool::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::GreedyMemoryPool", "path": "GreedyMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 10], "end": [76, 15], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/memory_pool/pool.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72d11d70645ec56a78d0038e"></a>
## fmt

`function` · `datafusion_execution::memory_pool::pool::GreedyMemoryPool::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::GreedyMemoryPool", "path": "GreedyMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [143, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/memory_pool/pool.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5da4dfe51774cf2ed4f6c395"></a>
## grow

`function` · `datafusion_execution::memory_pool::pool::GreedyMemoryPool::grow` · datafusion-execution 55.1.0

```rust
fn grow(&self, _reservation: &MemoryReservation, additional: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::GreedyMemoryPool", "path": "GreedyMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [130, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85bb8e4bc04b9fa0c677b9d9"></a>
## memory_limit

`function` · `datafusion_execution::memory_pool::pool::GreedyMemoryPool::memory_limit` · datafusion-execution 55.1.0

```rust
fn memory_limit(&self) -> MemoryLimit
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::GreedyMemoryPool", "path": "GreedyMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [130, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdc8177f87babd0d1b825b94"></a>
## name

`function` · `datafusion_execution::memory_pool::pool::GreedyMemoryPool::name` · datafusion-execution 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::GreedyMemoryPool", "path": "GreedyMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [130, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73948b8124dcc14b4cd42f31"></a>
## new

`function` · `datafusion_execution::memory_pool::pool::GreedyMemoryPool::new` · datafusion-execution 55.1.0

```rust
fn new(pool_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::GreedyMemoryPool", "path": "GreedyMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 1], "end": [91, 2], "filename": "src/memory_pool/pool.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/pool.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Create a new pool that can allocate up to `pool_size` bytes

<a id="op-04b57c4737582b820779891a"></a>
## reserved

`function` · `datafusion_execution::memory_pool::pool::GreedyMemoryPool::reserved` · datafusion-execution 55.1.0

```rust
fn reserved(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::GreedyMemoryPool", "path": "GreedyMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [130, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abdae792df235959d542e798"></a>
## shrink

`function` · `datafusion_execution::memory_pool::pool::GreedyMemoryPool::shrink` · datafusion-execution 55.1.0

```rust
fn shrink(&self, _reservation: &MemoryReservation, shrink: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::GreedyMemoryPool", "path": "GreedyMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [130, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2247862f18d3a77b50bd022a"></a>
## try_grow

`function` · `datafusion_execution::memory_pool::pool::GreedyMemoryPool::try_grow` · datafusion-execution 55.1.0

```rust
fn try_grow(&self, reservation: &MemoryReservation, additional: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::GreedyMemoryPool", "path": "GreedyMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [130, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
