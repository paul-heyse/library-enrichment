# `object_store::attributes::AttributesIter`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.attributes.AttributesIter.json).

<a id="op-e3425560324726f040a57f3b"></a>
## AttributesIter

`struct` · `object_store::attributes::AttributesIter` · object_store 0.13.2

```rust
struct AttributesIter<'a>
```

Source: `src/attributes.rs:183`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Iterator over [`Attributes`](../operations/object_store.attributes.Attributes.md#op-196267bf7f6d968d4ceb156e)

<a id="op-7ddbec94f4fb206a0609cd8f"></a>
## Item

`assoc_type` · `object_store::attributes::AttributesIter::Item` · object_store 0.13.2

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::attributes::AttributesIter", "path": "AttributesIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [195, 2], "filename": "src/attributes.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/attributes.rs:186`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f9de41be5026e6dbcdbe7f9"></a>
## fmt

`function` · `object_store::attributes::AttributesIter::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::attributes::AttributesIter", "path": "AttributesIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 10], "end": [182, 15], "filename": "src/attributes.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/attributes.rs:182`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-088190af513288612d761a87"></a>
## next

`function` · `object_store::attributes::AttributesIter::next` · object_store 0.13.2

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::attributes::AttributesIter", "path": "AttributesIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [195, 2], "filename": "src/attributes.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/attributes.rs:188`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df3e1202f5134024411d2575"></a>
## size_hint

`function` · `object_store::attributes::AttributesIter::size_hint` · object_store 0.13.2

```rust
fn size_hint(&self) -> (usize, Option<usize>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::attributes::AttributesIter", "path": "AttributesIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [195, 2], "filename": "src/attributes.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/attributes.rs:192`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
