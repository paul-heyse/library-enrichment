# `arrow_buffer::builder::offset::OffsetBufferBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.builder.offset.OffsetBufferBuilder.json).

<a id="op-8603c4d09c11158ced86be5e"></a>
## OffsetBufferBuilder

`struct` · `arrow_buffer::builder::offset::OffsetBufferBuilder` · arrow-buffer 59.3.0

```rust
struct OffsetBufferBuilder<O: ArrowNativeType>
```

Source: `src/builder/offset.rs:24`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Builder of [`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc)

<a id="op-0642e70b7a82bbd088d69171"></a>
## Target

`assoc_type` · `arrow_buffer::builder::offset::OffsetBufferBuilder::Target` · arrow-buffer 59.3.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::builder::offset::OffsetBufferBuilder", "path": "OffsetBufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [87, 2], "filename": "src/builder/offset.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/builder/offset.rs:82`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8130d67149e486a8592a8203"></a>
## deref

`function` · `arrow_buffer::builder::offset::OffsetBufferBuilder::deref` · arrow-buffer 59.3.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::builder::offset::OffsetBufferBuilder", "path": "OffsetBufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [87, 2], "filename": "src/builder/offset.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/builder/offset.rs:84`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bcb9eec2a245266b3e6e9fa"></a>
## finish

`function` · `arrow_buffer::builder::offset::OffsetBufferBuilder::finish` · arrow-buffer 59.3.0

```rust
fn finish(self) -> OffsetBuffer<O>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::builder::offset::OffsetBufferBuilder", "path": "OffsetBufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 1], "end": [79, 2], "filename": "src/builder/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/offset.rs:62`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Takes the builder itself and returns an [`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc)

# Panics

Panics if offsets overflow `O`

<a id="op-f61a0f53fdbfadc57d3e0535"></a>
## finish_cloned

`function` · `arrow_buffer::builder::offset::OffsetBufferBuilder::finish_cloned` · arrow-buffer 59.3.0

```rust
fn finish_cloned(&self) -> OffsetBuffer<O>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::builder::offset::OffsetBufferBuilder", "path": "OffsetBufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 1], "end": [79, 2], "filename": "src/builder/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/offset.rs:72`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Builds the [OffsetBuffer](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc) without resetting the builder.

# Panics

Panics if offsets overflow `O`

<a id="op-60348de7f4a3612fb696880c"></a>
## fmt

`function` · `arrow_buffer::builder::offset::OffsetBufferBuilder::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::builder::offset::OffsetBufferBuilder", "path": "OffsetBufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 10], "end": [23, 15], "filename": "src/builder/offset.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/offset.rs:23`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e9f05e64e18c9e516a4614f"></a>
## new

`function` · `arrow_buffer::builder::offset::OffsetBufferBuilder::new` · arrow-buffer 59.3.0

```rust
fn new(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::builder::offset::OffsetBufferBuilder", "path": "OffsetBufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 1], "end": [79, 2], "filename": "src/builder/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/offset.rs:31`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new builder with space for `capacity + 1` offsets

<a id="op-d1a8eeb68ecf72f7f8ce235a"></a>
## push_length

`function` · `arrow_buffer::builder::offset::OffsetBufferBuilder::push_length` · arrow-buffer 59.3.0

```rust
fn push_length(&mut self, length: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::builder::offset::OffsetBufferBuilder", "path": "OffsetBufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 1], "end": [79, 2], "filename": "src/builder/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/offset.rs:46`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Push a slice of `length` bytes

# Panics

Panics if adding `length` would overflow `usize`

<a id="op-f104b157cdc5fd89bbc78246"></a>
## reserve

`function` · `arrow_buffer::builder::offset::OffsetBufferBuilder::reserve` · arrow-buffer 59.3.0

```rust
fn reserve(&mut self, additional: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::builder::offset::OffsetBufferBuilder", "path": "OffsetBufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 1], "end": [79, 2], "filename": "src/builder/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/offset.rs:53`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Reserve space for at least `additional` further offsets
