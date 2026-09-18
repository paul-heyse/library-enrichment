# `object_store::payload::PutPayload`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.payload.PutPayload.json).

<a id="op-b48a2317e81523dd3db33f7c"></a>
## PutPayload

`struct` · `object_store::payload::PutPayload` · object_store 0.13.2

```rust
struct PutPayload
```

Source: `src/payload.rs:23`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A cheaply cloneable, ordered collection of [`Bytes`]

Unresolved upstream links (retained, not inferred): ``Bytes``.

<a id="op-7f940dd9f893a2038f3d8de9"></a>
## IntoIter

`assoc_type` · `object_store::payload::PutPayload::IntoIter` · object_store 0.13.2

```rust
IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [83, 2], "filename": "src/payload.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/payload.rs:75`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3b32bffdefd5f7d5c7d1cc2"></a>
## Item

`assoc_type` · `object_store::payload::PutPayload::Item` · object_store 0.13.2

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [83, 2], "filename": "src/payload.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/payload.rs:74`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c7dbd4e609605acfbc9a3af"></a>
## as_ref

`function` · `object_store::payload::PutPayload::as_ref` · object_store 0.13.2

```rust
fn as_ref(&self) -> &[Bytes]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [62, 2], "filename": "src/payload.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"slice": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/payload.rs:59`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-deed01bcd61030f562ae8dc2"></a>
## clone

`function` · `object_store::payload::PutPayload::clone` · object_store 0.13.2

```rust
fn clone(&self) -> PutPayload
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 17], "end": [22, 22], "filename": "src/payload.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/payload.rs:22`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bede255f7df0ec141a91184f"></a>
## content_length

`function` · `object_store::payload::PutPayload::content_length` · object_store 0.13.2

```rust
fn content_length(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 1], "end": [56, 2], "filename": "src/payload.rs"}, "trait": null, "trait_path": null}`

Source: `src/payload.rs:48`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns the total length of the [`Bytes`] in this payload

Unresolved upstream links (retained, not inferred): ``Bytes``.

<a id="op-1aa13e78656676d5b8b62d80"></a>
## default

`function` · `object_store::payload::PutPayload::default` · object_store 0.13.2

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [29, 2], "filename": "src/payload.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/payload.rs:26`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b4a3380d7350b5204f81194"></a>
## fmt

`function` · `object_store::payload::PutPayload::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 10], "end": [22, 15], "filename": "src/payload.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/payload.rs:22`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e082e89bec3e8674d2edef3"></a>
## from

`function` · `object_store::payload::PutPayload::from` · object_store 0.13.2

```rust
fn from(value: PutPayloadMut) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [278, 2], "filename": "src/payload.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayloadMut", "path": "PutPayloadMut"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/payload.rs:275`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32a3930942a0b2805c0f4f5e"></a>
## from

`function` · `object_store::payload::PutPayload::from` · object_store 0.13.2

```rust
fn from(value: &'static [u8]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [145, 2], "filename": "src/payload.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'static", "type": {"slice": {"primitive": "u8"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/payload.rs:142`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-366deb6c6d1e1ce8d737d0c9"></a>
## from

`function` · `object_store::payload::PutPayload::from` · object_store 0.13.2

```rust
fn from(value: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [151, 2], "filename": "src/payload.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/payload.rs:148`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89464729cac297b47dfb3e72"></a>
## from

`function` · `object_store::payload::PutPayload::from` · object_store 0.13.2

```rust
fn from(value: Bytes) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [127, 2], "filename": "src/payload.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/payload.rs:124`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9200293b21406098b521812d"></a>
## from

`function` · `object_store::payload::PutPayload::from` · object_store 0.13.2

```rust
fn from(value: &'static str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [139, 2], "filename": "src/payload.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'static", "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/payload.rs:136`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96eeda8de993338724136665"></a>
## from

`function` · `object_store::payload::PutPayload::from` · object_store 0.13.2

```rust
fn from(value: Vec<u8>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [133, 2], "filename": "src/payload.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u8"}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/payload.rs:130`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56028c8c7b1a740edcd0732a"></a>
## from_bytes

`function` · `object_store::payload::PutPayload::from_bytes` · object_store 0.13.2

```rust
fn from_bytes(s: Bytes) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 1], "end": [56, 2], "filename": "src/payload.rs"}, "trait": null, "trait_path": null}`

Source: `src/payload.rs:43`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Creates a [`PutPayload`](../operations/object_store.payload.PutPayload.md#op-b48a2317e81523dd3db33f7c) from a [`Bytes`]

Unresolved upstream links (retained, not inferred): ``Bytes``.

<a id="op-49172ac00e376077e40a98eb"></a>
## from_iter

`function` · `object_store::payload::PutPayload::from_iter` · object_store 0.13.2

```rust
fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [157, 2], "filename": "src/payload.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u8"}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/payload.rs:154`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74e3aade74eb97277c5c9a00"></a>
## from_iter

`function` · `object_store::payload::PutPayload::from_iter` · object_store 0.13.2

```rust
fn from_iter<T: IntoIterator<Item = Bytes>>(iter: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [159, 1], "end": [163, 2], "filename": "src/payload.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/payload.rs:160`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b46cf0d5e1957cfc88c9e0ed"></a>
## from_static

`function` · `object_store::payload::PutPayload::from_static` · object_store 0.13.2

```rust
fn from_static(s: &'static [u8]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 1], "end": [56, 2], "filename": "src/payload.rs"}, "trait": null, "trait_path": null}`

Source: `src/payload.rs:38`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Creates a [`PutPayload`](../operations/object_store.payload.PutPayload.md#op-b48a2317e81523dd3db33f7c) from a static slice

<a id="op-40a7f51596185b6dcf96f1bb"></a>
## into_iter

`function` · `object_store::payload::PutPayload::into_iter` · object_store 0.13.2

```rust
fn into_iter(self) -> Self::IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [83, 2], "filename": "src/payload.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/payload.rs:77`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7017db2c4538431d9dbbd68"></a>
## iter

`function` · `object_store::payload::PutPayload::iter` · object_store 0.13.2

```rust
fn iter(&self) -> PutPayloadIter<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 1], "end": [56, 2], "filename": "src/payload.rs"}, "trait": null, "trait_path": null}`

Source: `src/payload.rs:53`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns an iterator over the [`Bytes`] in this payload

Unresolved upstream links (retained, not inferred): ``Bytes``.

<a id="op-43c316c8c7924bd257006543"></a>
## new

`function` · `object_store::payload::PutPayload::new` · object_store 0.13.2

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::payload::PutPayload", "path": "PutPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 1], "end": [56, 2], "filename": "src/payload.rs"}, "trait": null, "trait_path": null}`

Source: `src/payload.rs:33`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new empty [`PutPayload`](../operations/object_store.payload.PutPayload.md#op-b48a2317e81523dd3db33f7c)
