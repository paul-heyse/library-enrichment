# `arrow_buffer::pool::TrackingMemoryPool`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.pool.TrackingMemoryPool.json).

<a id="op-61b862f236f11b17839235bc"></a>
## TrackingMemoryPool

`struct` · `arrow_buffer::pool::TrackingMemoryPool` · arrow-buffer 59.3.0

```rust
struct TrackingMemoryPool
```

Source: `src/pool.rs:93`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

A simple [`MemoryPool`](../operations/arrow_buffer.pool.MemoryPool.md#op-ed45519ea8493e8aed943b0a) that reports the total memory usage

<a id="op-780b719e4bd257b486f2923a"></a>
## allocated

`function` · `arrow_buffer::pool::TrackingMemoryPool::allocated` · arrow-buffer 59.3.0

```rust
fn allocated(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::pool::TrackingMemoryPool", "path": "TrackingMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [100, 2], "filename": "src/pool.rs"}, "trait": null, "trait_path": null}`

Source: `src/pool.rs:97`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the total allocated size

<a id="op-d8efebdd8b9e487a0c2ab1d7"></a>
## available

`function` · `arrow_buffer::pool::TrackingMemoryPool::available` · arrow-buffer 59.3.0

```rust
fn available(&self) -> isize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::pool::TrackingMemoryPool", "path": "TrackingMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [122, 2], "filename": "src/pool.rs"}, "trait": {"args": null, "id": "arrow_buffer::pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "arrow_buffer::pool::MemoryPool"}`

Source: `src/pool.rs:111`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a81744c998d0fe514e09b24"></a>
## capacity

`function` · `arrow_buffer::pool::TrackingMemoryPool::capacity` · arrow-buffer 59.3.0

```rust
fn capacity(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::pool::TrackingMemoryPool", "path": "TrackingMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [122, 2], "filename": "src/pool.rs"}, "trait": {"args": null, "id": "arrow_buffer::pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "arrow_buffer::pool::MemoryPool"}`

Source: `src/pool.rs:119`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87cce68c68822a557cecef98"></a>
## default

`function` · `arrow_buffer::pool::TrackingMemoryPool::default` · arrow-buffer 59.3.0

```rust
fn default() -> TrackingMemoryPool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::pool::TrackingMemoryPool", "path": "TrackingMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 17], "end": [92, 24], "filename": "src/pool.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/pool.rs:92`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de1761b1a5ff431f4fe265ab"></a>
## fmt

`function` · `arrow_buffer::pool::TrackingMemoryPool::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::pool::TrackingMemoryPool", "path": "TrackingMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 10], "end": [92, 15], "filename": "src/pool.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/pool.rs:92`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d58770cbb0492a7fce40cd7"></a>
## reserve

`function` · `arrow_buffer::pool::TrackingMemoryPool::reserve` · arrow-buffer 59.3.0

```rust
fn reserve(&self, size: usize) -> Box<dyn MemoryReservation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::pool::TrackingMemoryPool", "path": "TrackingMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [122, 2], "filename": "src/pool.rs"}, "trait": {"args": null, "id": "arrow_buffer::pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "arrow_buffer::pool::MemoryPool"}`

Source: `src/pool.rs:103`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53076e1628ba821d61d71e90"></a>
## used

`function` · `arrow_buffer::pool::TrackingMemoryPool::used` · arrow-buffer 59.3.0

```rust
fn used(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::pool::TrackingMemoryPool", "path": "TrackingMemoryPool"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [122, 2], "filename": "src/pool.rs"}, "trait": {"args": null, "id": "arrow_buffer::pool::MemoryPool", "path": "MemoryPool"}, "trait_path": "arrow_buffer::pool::MemoryPool"}`

Source: `src/pool.rs:115`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
