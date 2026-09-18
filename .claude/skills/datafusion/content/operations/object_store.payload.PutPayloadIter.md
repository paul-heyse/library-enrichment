# `object_store::payload::PutPayloadIter`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.payload.PutPayloadIter.json).

<a id="op-a47d2005b966c6640fa3673a"></a>
## PutPayloadIter

`struct` · `object_store::payload::PutPayloadIter` · object_store 0.13.2

```rust
struct PutPayloadIter<'a>
```

Source: `src/payload.rs:87`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An iterator over [`PutPayload`](../operations/object_store.payload.PutPayload.md#op-b48a2317e81523dd3db33f7c)

<a id="op-2ca0c4e18172a33efe73fb59"></a>
## Item

`assoc_type` · `object_store::payload::PutPayloadIter::Item` · object_store 0.13.2

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::payload::PutPayloadIter", "path": "PutPayloadIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [99, 2], "filename": "src/payload.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/payload.rs:90`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8a11eb00704292c01b22a2e"></a>
## fmt

`function` · `object_store::payload::PutPayloadIter::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::payload::PutPayloadIter", "path": "PutPayloadIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 10], "end": [86, 15], "filename": "src/payload.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/payload.rs:86`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd3df513ba2a02a6facfa1a4"></a>
## next

`function` · `object_store::payload::PutPayloadIter::next` · object_store 0.13.2

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::payload::PutPayloadIter", "path": "PutPayloadIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [99, 2], "filename": "src/payload.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/payload.rs:92`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5114fa73a5273dd3ee27d2b5"></a>
## size_hint

`function` · `object_store::payload::PutPayloadIter::size_hint` · object_store 0.13.2

```rust
fn size_hint(&self) -> (usize, Option<usize>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::payload::PutPayloadIter", "path": "PutPayloadIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [99, 2], "filename": "src/payload.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/payload.rs:96`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
