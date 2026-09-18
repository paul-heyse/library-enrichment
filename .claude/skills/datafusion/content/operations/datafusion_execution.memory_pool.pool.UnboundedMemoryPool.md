# `datafusion_execution::memory_pool::pool::UnboundedMemoryPool`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.memory_pool.pool.UnboundedMemoryPool.json).

<a id="op-bc210b8fe79074b67e0fb694"></a>
## UnboundedMemoryPool

`struct` · `datafusion_execution::memory_pool::pool::UnboundedMemoryPool` · datafusion-execution 55.1.0

```rust
struct UnboundedMemoryPool
```

Source: `src/memory_pool/pool.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

A [`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af) that enforces no limit

<a id="op-601350ad1de1135b7bd65dcb"></a>
## default

`function` · `datafusion_execution::memory_pool::pool::UnboundedMemoryPool::default` · datafusion-execution 55.1.0

```rust
fn default() -> UnboundedMemoryPool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::UnboundedMemoryPool", "path": "UnboundedMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 17], "end": [32, 24], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/memory_pool/pool.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1aad7e04e708f66162bca14b"></a>
## fmt

`function` · `datafusion_execution::memory_pool::pool::UnboundedMemoryPool::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::UnboundedMemoryPool", "path": "UnboundedMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [69, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/memory_pool/pool.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d3529b9338de50d927cdc1e"></a>
## fmt

`function` · `datafusion_execution::memory_pool::pool::UnboundedMemoryPool::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::UnboundedMemoryPool", "path": "UnboundedMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/memory_pool/pool.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52902cc14e5760561617bf2b"></a>
## grow

`function` · `datafusion_execution::memory_pool::pool::UnboundedMemoryPool::grow` · datafusion-execution 55.1.0

```rust
fn grow(&self, _reservation: &MemoryReservation, additional: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::UnboundedMemoryPool", "path": "UnboundedMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [62, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b158eafc417add55c696287e"></a>
## memory_limit

`function` · `datafusion_execution::memory_pool::pool::UnboundedMemoryPool::memory_limit` · datafusion-execution 55.1.0

```rust
fn memory_limit(&self) -> MemoryLimit
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::UnboundedMemoryPool", "path": "UnboundedMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [62, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c34b5d5946a47649489d6b0"></a>
## name

`function` · `datafusion_execution::memory_pool::pool::UnboundedMemoryPool::name` · datafusion-execution 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::UnboundedMemoryPool", "path": "UnboundedMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [62, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bdadf1054df799d2384f7e04"></a>
## reserved

`function` · `datafusion_execution::memory_pool::pool::UnboundedMemoryPool::reserved` · datafusion-execution 55.1.0

```rust
fn reserved(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::UnboundedMemoryPool", "path": "UnboundedMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [62, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c9fd2799a0d4609d97bbc12"></a>
## shrink

`function` · `datafusion_execution::memory_pool::pool::UnboundedMemoryPool::shrink` · datafusion-execution 55.1.0

```rust
fn shrink(&self, _reservation: &MemoryReservation, shrink: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::UnboundedMemoryPool", "path": "UnboundedMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [62, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-028328c8ba73720151e80769"></a>
## try_grow

`function` · `datafusion_execution::memory_pool::pool::UnboundedMemoryPool::try_grow` · datafusion-execution 55.1.0

```rust
fn try_grow(&self, reservation: &MemoryReservation, additional: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::UnboundedMemoryPool", "path": "UnboundedMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [62, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
