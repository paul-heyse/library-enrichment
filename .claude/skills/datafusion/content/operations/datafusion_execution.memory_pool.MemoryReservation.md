# `datafusion_execution::memory_pool::MemoryReservation`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.memory_pool.MemoryReservation.json).

<a id="op-a0611bd048c178da7e0d05df"></a>
## MemoryReservation

`struct` · `datafusion_execution::memory_pool::MemoryReservation` · datafusion-execution 55.1.0

```rust
struct MemoryReservation
```

Source: `src/memory_pool/mod.rs:375`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

A [`MemoryReservation`](../operations/datafusion_execution.memory_pool.MemoryReservation.md#op-a0611bd048c178da7e0d05df) tracks an individual reservation of a
number of bytes of memory in a [`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af) that is freed back
to the pool on drop.

The reservation can be grown or shrunk over time.

<a id="op-618ee1be3888fe5ba5736db0"></a>
## consumer

`function` · `datafusion_execution::memory_pool::MemoryReservation::consumer` · datafusion-execution 55.1.0

```rust
fn consumer(&self) -> &MemoryConsumer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryReservation", "path": "MemoryReservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [380, 1], "end": [516, 2], "filename": "src/memory_pool/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/mod.rs:387`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns [MemoryConsumer](../operations/datafusion_execution.memory_pool.MemoryConsumer.md#op-a6a1cc9789177d5167596e41) for this [MemoryReservation](../operations/datafusion_execution.memory_pool.MemoryReservation.md#op-a0611bd048c178da7e0d05df)

<a id="op-fd878aeee0d424ab1c01742f"></a>
## drop

`function` · `datafusion_execution::memory_pool::MemoryReservation::drop` · datafusion-execution 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryReservation", "path": "MemoryReservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [518, 1], "end": [522, 2], "filename": "src/memory_pool/mod.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/memory_pool/mod.rs:519`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cf165b93469405eddb0a5be"></a>
## fmt

`function` · `datafusion_execution::memory_pool::MemoryReservation::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryReservation", "path": "MemoryReservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [374, 10], "end": [374, 15], "filename": "src/memory_pool/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/memory_pool/mod.rs:374`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e33a875d68ea6d48506583d"></a>
## free

`function` · `datafusion_execution::memory_pool::MemoryReservation::free` · datafusion-execution 55.1.0

```rust
fn free(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryReservation", "path": "MemoryReservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [380, 1], "end": [516, 2], "filename": "src/memory_pool/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/mod.rs:393`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Frees all bytes from this reservation back to the underlying
pool, returning the number of bytes freed.

<a id="op-bafbd1f2fd01dc667329ed55"></a>
## grow

`function` · `datafusion_execution::memory_pool::MemoryReservation::grow` · datafusion-execution 55.1.0

```rust
fn grow(&self, capacity: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryReservation", "path": "MemoryReservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [380, 1], "end": [516, 2], "filename": "src/memory_pool/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/mod.rs:465`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Increase the size of this reservation by `capacity` bytes

<a id="op-9218840ae43a07a7ef1f167f"></a>
## new_empty

`function` · `datafusion_execution::memory_pool::MemoryReservation::new_empty` · datafusion-execution 55.1.0

```rust
fn new_empty(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryReservation", "path": "MemoryReservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [380, 1], "end": [516, 2], "filename": "src/memory_pool/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/mod.rs:504`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns a new empty [`MemoryReservation`](../operations/datafusion_execution.memory_pool.MemoryReservation.md#op-a0611bd048c178da7e0d05df) with the same [`MemoryConsumer`](../operations/datafusion_execution.memory_pool.MemoryConsumer.md#op-a6a1cc9789177d5167596e41)

<a id="op-77abc519cdd38a0acdf29aaf"></a>
## resize

`function` · `datafusion_execution::memory_pool::MemoryReservation::resize` · datafusion-execution 55.1.0

```rust
fn resize(&mut self, new_size: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryReservation", "path": "crate::memory_pool::MemoryReservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [57, 2], "filename": "src/memory_pool/arrow.rs"}, "trait": {"args": null, "id": "arrow_buffer::pool::MemoryReservation", "path": "MemoryReservation"}, "trait_path": "arrow_buffer::pool::MemoryReservation"}`

Source: `src/memory_pool/arrow.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee49ff1b6159fbda56ac5979"></a>
## resize

`function` · `datafusion_execution::memory_pool::MemoryReservation::resize` · datafusion-execution 55.1.0

```rust
fn resize(&self, capacity: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryReservation", "path": "MemoryReservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [380, 1], "end": [516, 2], "filename": "src/memory_pool/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/mod.rs:442`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Sets the size of this reservation to `capacity`

<a id="op-bc30263a1284be4ebf16f66b"></a>
## shrink

`function` · `datafusion_execution::memory_pool::MemoryReservation::shrink` · datafusion-execution 55.1.0

```rust
fn shrink(&self, capacity: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryReservation", "path": "MemoryReservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [380, 1], "end": [516, 2], "filename": "src/memory_pool/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/mod.rs:406`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Frees `capacity` bytes from this reservation

# Panics

Panics if `capacity` exceeds [`Self::size`](../operations/datafusion_execution.memory_pool.MemoryReservation.md#op-97a3723bc63723c6ab3952f5)

<a id="op-12160406e4f41fb7747e9c7b"></a>
## size

`function` · `datafusion_execution::memory_pool::MemoryReservation::size` · datafusion-execution 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryReservation", "path": "crate::memory_pool::MemoryReservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [57, 2], "filename": "src/memory_pool/arrow.rs"}, "trait": {"args": null, "id": "arrow_buffer::pool::MemoryReservation", "path": "MemoryReservation"}, "trait_path": "arrow_buffer::pool::MemoryReservation"}`

Source: `src/memory_pool/arrow.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97a3723bc63723c6ab3952f5"></a>
## size

`function` · `datafusion_execution::memory_pool::MemoryReservation::size` · datafusion-execution 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryReservation", "path": "MemoryReservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [380, 1], "end": [516, 2], "filename": "src/memory_pool/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/mod.rs:382`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns the size of this reservation in bytes

<a id="op-57e8409e1fcf37d027128da8"></a>
## split

`function` · `datafusion_execution::memory_pool::MemoryReservation::split` · datafusion-execution 55.1.0

```rust
fn split(&self, capacity: usize) -> MemoryReservation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryReservation", "path": "MemoryReservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [380, 1], "end": [516, 2], "filename": "src/memory_pool/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/mod.rs:489`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Splits off `capacity` bytes from this [`MemoryReservation`](../operations/datafusion_execution.memory_pool.MemoryReservation.md#op-a0611bd048c178da7e0d05df)
into a new [`MemoryReservation`](../operations/datafusion_execution.memory_pool.MemoryReservation.md#op-a0611bd048c178da7e0d05df) with the same
[`MemoryConsumer`](../operations/datafusion_execution.memory_pool.MemoryConsumer.md#op-a6a1cc9789177d5167596e41).

This can be useful to free part of this reservation with RAAI
style dropping

# Panics

Panics if `capacity` exceeds [`Self::size`](../operations/datafusion_execution.memory_pool.MemoryReservation.md#op-97a3723bc63723c6ab3952f5)

<a id="op-702717eb9d4467419c0555a6"></a>
## take

`function` · `datafusion_execution::memory_pool::MemoryReservation::take` · datafusion-execution 55.1.0

```rust
fn take(&mut self) -> MemoryReservation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryReservation", "path": "MemoryReservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [380, 1], "end": [516, 2], "filename": "src/memory_pool/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/mod.rs:513`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Splits off all the bytes from this [`MemoryReservation`](../operations/datafusion_execution.memory_pool.MemoryReservation.md#op-a0611bd048c178da7e0d05df) into
a new [`MemoryReservation`](../operations/datafusion_execution.memory_pool.MemoryReservation.md#op-a0611bd048c178da7e0d05df) with the same [`MemoryConsumer`](../operations/datafusion_execution.memory_pool.MemoryConsumer.md#op-a6a1cc9789177d5167596e41)

<a id="op-bfed990e4985ef0e8d506eeb"></a>
## try_grow

`function` · `datafusion_execution::memory_pool::MemoryReservation::try_grow` · datafusion-execution 55.1.0

```rust
fn try_grow(&self, capacity: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryReservation", "path": "MemoryReservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [380, 1], "end": [516, 2], "filename": "src/memory_pool/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/mod.rs:473`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Try to increase the size of this reservation by `capacity`
bytes, returning error if there is insufficient capacity left
in the pool.

<a id="op-c633522eee68d642435c8548"></a>
## try_resize

`function` · `datafusion_execution::memory_pool::MemoryReservation::try_resize` · datafusion-execution 55.1.0

```rust
fn try_resize(&self, capacity: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryReservation", "path": "MemoryReservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [380, 1], "end": [516, 2], "filename": "src/memory_pool/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/mod.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Try to set the size of this reservation to `capacity`

<a id="op-6000a690d3b6d7ad3ca93bb3"></a>
## try_shrink

`function` · `datafusion_execution::memory_pool::MemoryReservation::try_shrink` · datafusion-execution 55.1.0

```rust
fn try_shrink(&self, capacity: usize) -> Result<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::memory_pool::MemoryReservation", "path": "MemoryReservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [380, 1], "end": [516, 2], "filename": "src/memory_pool/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory_pool/mod.rs:423`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Tries to free `capacity` bytes from this reservation
if `capacity` does not exceed [`Self::size`](../operations/datafusion_execution.memory_pool.MemoryReservation.md#op-97a3723bc63723c6ab3952f5).
Returns new reservation size,
or error if shrinking capacity is more than allocated size.
