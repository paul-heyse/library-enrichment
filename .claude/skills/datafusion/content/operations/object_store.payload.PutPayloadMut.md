# `object_store::payload::PutPayloadMut`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.payload.PutPayloadMut.json).

<a id="op-f074bb6138ffd7d314168a79"></a>
## PutPayloadMut

`struct` · `object_store::payload::PutPayloadMut` · object_store 0.13.2

```rust
struct PutPayloadMut
```

Source: `src/payload.rs:186`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A builder for [`PutPayload`](../operations/object_store.payload.PutPayload.md#op-b48a2317e81523dd3db33f7c) that avoids reallocating memory

Data is allocated in fixed blocks, which are flushed to [`Bytes`] once full.
Unlike [`Vec`] this avoids needing to repeatedly reallocate blocks of memory,
which typically involves copying all the previously written data to a new
contiguous memory region.

Unresolved upstream links (retained, not inferred): ``Bytes``, ``Vec``.

<a id="op-fa13a9f6f3b61b8a369fe91b"></a>
## content_length

`function` · `object_store::payload::PutPayloadMut::content_length` · object_store 0.13.2

```rust
fn content_length(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayloadMut", "path": "PutPayloadMut"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [272, 2], "filename": "src/payload.rs"}, "trait": null, "trait_path": null}`

Source: `src/payload.rs:260`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns the total length of the [`Bytes`] in this payload

Unresolved upstream links (retained, not inferred): ``Bytes``.

<a id="op-d03c3961bfb71d608fdbdd24"></a>
## default

`function` · `object_store::payload::PutPayloadMut::default` · object_store 0.13.2

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayloadMut", "path": "PutPayloadMut"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [193, 1], "end": [203, 2], "filename": "src/payload.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/payload.rs:194`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a4b231ec02eddc2d2c5a96e"></a>
## extend_from_slice

`function` · `object_store::payload::PutPayloadMut::extend_from_slice` · object_store 0.13.2

```rust
fn extend_from_slice(&mut self, slice: &[u8])
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayloadMut", "path": "PutPayloadMut"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [272, 2], "filename": "src/payload.rs"}, "trait": null, "trait_path": null}`

Source: `src/payload.rs:223`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Write bytes into this [`PutPayloadMut`](../operations/object_store.payload.PutPayloadMut.md#op-f074bb6138ffd7d314168a79)

If there is an in-progress block, data will be first written to it, flushing
it to [`Bytes`] once full. If data remains to be written, a new block of memory
of at least the configured block size will be allocated, to hold the remaining data.

Unresolved upstream links (retained, not inferred): ``Bytes``.

<a id="op-521ccd03f2af7946125c886d"></a>
## fmt

`function` · `object_store::payload::PutPayloadMut::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayloadMut", "path": "PutPayloadMut"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 10], "end": [185, 15], "filename": "src/payload.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/payload.rs:185`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3acd3bc6a6aa155b5fa83f59"></a>
## freeze

`function` · `object_store::payload::PutPayloadMut::freeze` · object_store 0.13.2

```rust
fn freeze(self) -> PutPayload
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayloadMut", "path": "PutPayloadMut"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [272, 2], "filename": "src/payload.rs"}, "trait": null, "trait_path": null}`

Source: `src/payload.rs:265`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Convert into [`PutPayload`](../operations/object_store.payload.PutPayload.md#op-b48a2317e81523dd3db33f7c)

<a id="op-364eede4bd2e18d4cbc57a61"></a>
## is_empty

`function` · `object_store::payload::PutPayloadMut::is_empty` · object_store 0.13.2

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayloadMut", "path": "PutPayloadMut"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [272, 2], "filename": "src/payload.rs"}, "trait": null, "trait_path": null}`

Source: `src/payload.rs:254`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns `true` if this [`PutPayloadMut`](../operations/object_store.payload.PutPayloadMut.md#op-f074bb6138ffd7d314168a79) contains no bytes

<a id="op-2ca2d111d4f1bada6cf5379c"></a>
## new

`function` · `object_store::payload::PutPayloadMut::new` · object_store 0.13.2

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayloadMut", "path": "PutPayloadMut"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [272, 2], "filename": "src/payload.rs"}, "trait": null, "trait_path": null}`

Source: `src/payload.rs:207`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`PutPayloadMut`](../operations/object_store.payload.PutPayloadMut.md#op-f074bb6138ffd7d314168a79)

<a id="op-0e62aad6b58e4ce810d0fb80"></a>
## push

`function` · `object_store::payload::PutPayloadMut::push` · object_store 0.13.2

```rust
fn push(&mut self, bytes: Bytes)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayloadMut", "path": "PutPayloadMut"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [272, 2], "filename": "src/payload.rs"}, "trait": null, "trait_path": null}`

Source: `src/payload.rs:243`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Append a [`Bytes`] to this [`PutPayloadMut`](../operations/object_store.payload.PutPayloadMut.md#op-f074bb6138ffd7d314168a79) without copying

This will close any currently buffered block populated by [`Self::extend_from_slice`](../operations/object_store.payload.PutPayloadMut.md#op-6a4b231ec02eddc2d2c5a96e),
and append `bytes` to this payload without copying.

Unresolved upstream links (retained, not inferred): ``Bytes``.

<a id="op-d0256973ed689ff9faab66a8"></a>
## with_block_size

`function` · `object_store::payload::PutPayloadMut::with_block_size` · object_store 0.13.2

```rust
fn with_block_size(self, block_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayloadMut", "path": "PutPayloadMut"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [272, 2], "filename": "src/payload.rs"}, "trait": null, "trait_path": null}`

Source: `src/payload.rs:214`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configures the minimum allocation size

Defaults to 8KB
