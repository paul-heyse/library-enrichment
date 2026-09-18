# `datafusion_physical_plan::buffer::MemoryBufferedStream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.buffer.MemoryBufferedStream.json).

<a id="op-537c4e6eddb23dd703d29f99"></a>
## MemoryBufferedStream

`struct` · `datafusion_physical_plan::buffer::MemoryBufferedStream` · datafusion-physical-plan 55.1.0

```rust
struct MemoryBufferedStream<T: SizedMessage>
```

Source: `src/buffer.rs:383`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Decouples production and consumption of messages in a stream with an internal queue, eagerly
filling it up to the specified maximum capacity even before any message is requested.

Allows each message to have a different size, which is taken into account for determining if
the queue is full or not.

<a id="op-6e7c18b70591633f5eca983a"></a>
## Item

`assoc_type` · `datafusion_physical_plan::buffer::MemoryBufferedStream::Item` · datafusion-physical-plan 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::buffer::MemoryBufferedStream", "path": "MemoryBufferedStream"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_physical_plan::buffer::SizedMessage", "path": "SizedMessage"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [507, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/buffer.rs:484`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73ad69f17a805be4647b3afc"></a>
## messages_queued

`function` · `datafusion_physical_plan::buffer::MemoryBufferedStream::messages_queued` · datafusion-physical-plan 55.1.0

```rust
fn messages_queued(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::buffer::MemoryBufferedStream", "path": "MemoryBufferedStream"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_physical_plan::buffer::SizedMessage", "path": "SizedMessage"}}}, {"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [395, 1], "end": [481, 2], "filename": "src/buffer.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer.rs:478`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the number of queued messages.

<a id="op-9e26e5a2056452d9c086331f"></a>
## new

`function` · `datafusion_physical_plan::buffer::MemoryBufferedStream::new` · datafusion-physical-plan 55.1.0

```rust
fn new(input: impl Stream<Item = Result<T>> + Unpin + Send + 'static, capacity: usize, memory_reservation: MemoryReservation) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::buffer::MemoryBufferedStream", "path": "MemoryBufferedStream"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_physical_plan::buffer::SizedMessage", "path": "SizedMessage"}}}, {"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [395, 1], "end": [481, 2], "filename": "src/buffer.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer.rs:399`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Builds a new [MemoryBufferedStream](../operations/datafusion_physical_plan.buffer.MemoryBufferedStream.md#op-537c4e6eddb23dd703d29f99) with the provided capacity and event handler.

This immediately spawns a Tokio task that will start consumption of the input stream.

<a id="op-e32d5cdff69814c15cb69834"></a>
## poll_next

`function` · `datafusion_physical_plan::buffer::MemoryBufferedStream::poll_next` · datafusion-physical-plan 55.1.0

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::buffer::MemoryBufferedStream", "path": "MemoryBufferedStream"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_physical_plan::buffer::SizedMessage", "path": "SizedMessage"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [507, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/buffer.rs:486`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f2e860e9d39ca7aaeaac8f5"></a>
## size_hint

`function` · `datafusion_physical_plan::buffer::MemoryBufferedStream::size_hint` · datafusion-physical-plan 55.1.0

```rust
fn size_hint(&self) -> (usize, Option<usize>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::buffer::MemoryBufferedStream", "path": "MemoryBufferedStream"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_physical_plan::buffer::SizedMessage", "path": "SizedMessage"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [507, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/buffer.rs:499`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
