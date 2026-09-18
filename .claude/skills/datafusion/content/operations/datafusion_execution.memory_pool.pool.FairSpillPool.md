# `datafusion_execution::memory_pool::pool::FairSpillPool`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.memory_pool.pool.FairSpillPool.json).

<a id="op-beb0cc08c58aa417e2866734"></a>
## FairSpillPool

`struct` · `datafusion_execution::memory_pool::pool::FairSpillPool` · datafusion-execution 55.1.0

```rust
struct FairSpillPool
```

Source: `src/memory_pool/pool.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

A [`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af) that prevents spillable reservations from using more than
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

<a id="op-74f0771d87560257b4cb9d03"></a>
## fmt

`function` · `datafusion_execution::memory_pool::pool::FairSpillPool::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::FairSpillPool", "path": "FairSpillPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [288, 1], "end": [297, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/memory_pool/pool.rs:289`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b4ac2178bc5f93acfcf54c7"></a>
## fmt

`function` · `datafusion_execution::memory_pool::pool::FairSpillPool::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::FairSpillPool", "path": "FairSpillPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 10], "end": [167, 15], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/memory_pool/pool.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e22b8e277f966cb508a2bfe"></a>
## grow

`function` · `datafusion_execution::memory_pool::pool::FairSpillPool::grow` · datafusion-execution 55.1.0

```rust
fn grow(&self, reservation: &MemoryReservation, additional: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::FairSpillPool", "path": "FairSpillPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [286, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:220`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24d7643d9f3033bf0aafc28f"></a>
## memory_limit

`function` · `datafusion_execution::memory_pool::pool::FairSpillPool::memory_limit` · datafusion-execution 55.1.0

```rust
fn memory_limit(&self) -> MemoryLimit
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::FairSpillPool", "path": "FairSpillPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [286, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:283`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ebd348e7793aa7894c581c0"></a>
## name

`function` · `datafusion_execution::memory_pool::pool::FairSpillPool::name` · datafusion-execution 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::FairSpillPool", "path": "FairSpillPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [286, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78dbec9f09ea2defd3505ce6"></a>
## new

`function` · `datafusion_execution::memory_pool::pool::FairSpillPool::new` · datafusion-execution 55.1.0

```rust
fn new(pool_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::FairSpillPool", "path": "FairSpillPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [200, 2], "filename": "src/memory_pool/pool.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/pool.rs:189`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Allocate up to `limit` bytes

<a id="op-8d72d3163ec36fb39379d5de"></a>
## register

`function` · `datafusion_execution::memory_pool::pool::FairSpillPool::register` · datafusion-execution 55.1.0

```rust
fn register(&self, consumer: &MemoryConsumer)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::FairSpillPool", "path": "FairSpillPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [286, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:207`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df0bae276281e0fc26a20e4b"></a>
## reserved

`function` · `datafusion_execution::memory_pool::pool::FairSpillPool::reserved` · datafusion-execution 55.1.0

```rust
fn reserved(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::FairSpillPool", "path": "FairSpillPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [286, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:278`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a3cb24c202f5c9598f61fbe"></a>
## shrink

`function` · `datafusion_execution::memory_pool::pool::FairSpillPool::shrink` · datafusion-execution 55.1.0

```rust
fn shrink(&self, reservation: &MemoryReservation, shrink: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::FairSpillPool", "path": "FairSpillPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [286, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7dc984c5e0582250860f9a2d"></a>
## try_grow

`function` · `datafusion_execution::memory_pool::pool::FairSpillPool::try_grow` · datafusion-execution 55.1.0

```rust
fn try_grow(&self, reservation: &MemoryReservation, additional: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::FairSpillPool", "path": "FairSpillPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [286, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:236`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7838d40d2612c62584c5973"></a>
## unregister

`function` · `datafusion_execution::memory_pool::pool::FairSpillPool::unregister` · datafusion-execution 55.1.0

```rust
fn unregister(&self, consumer: &MemoryConsumer)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::pool::FairSpillPool", "path": "FairSpillPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [286, 2], "filename": "src/memory_pool/pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::memory_pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "datafusion_execution::memory_pool::MemoryPool"}`

Source: `src/memory_pool/pool.rs:213`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
