# `object_store::path::parts::PathParts`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.path.parts.PathParts.json).

<a id="op-46c27d09c4d0d797f583a370"></a>
## PathParts

`struct` · `object_store::path::parts::PathParts` · object_store 0.13.2

```rust
struct PathParts<'a>
```

Source: `src/path/parts.rs:140`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

See [`Path::parts`](super::Path::parts)

<a id="op-9fd8d6eb13672579362f62db"></a>
## Item

`assoc_type` · `object_store::path::parts::PathParts::Item` · object_store 0.13.2

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::path::parts::PathParts", "path": "PathParts"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [158, 2], "filename": "src/path/parts.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/path/parts.rs:153`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe18f2a65e7e31fd25328b99"></a>
## clone

`function` · `object_store::path::parts::PathParts::clone` · object_store 0.13.2

```rust
fn clone(&self) -> PathParts<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::path::parts::PathParts", "path": "PathParts"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 17], "end": [139, 22], "filename": "src/path/parts.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/path/parts.rs:139`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df090de30dcd839508f3d41a"></a>
## fmt

`function` · `object_store::path::parts::PathParts::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::path::parts::PathParts", "path": "PathParts"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 10], "end": [139, 15], "filename": "src/path/parts.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/path/parts.rs:139`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb1744b59ffce0b9c1277373"></a>
## next

`function` · `object_store::path::parts::PathParts::next` · object_store 0.13.2

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::path::parts::PathParts", "path": "PathParts"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [158, 2], "filename": "src/path/parts.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/path/parts.rs:155`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47bc4b124258379b29c9f691"></a>
## next_back

`function` · `object_store::path::parts::PathParts::next_back` · object_store 0.13.2

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::path::parts::PathParts", "path": "PathParts"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [162, 1], "end": [166, 2], "filename": "src/path/parts.rs"}, "trait": {"args": null, "id": "core::iter::traits::double_ended::DoubleEndedIterator", "path": "DoubleEndedIterator"}, "trait_path": "core::iter::traits::double_ended::DoubleEndedIterator"}`

Source: `src/path/parts.rs:163`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
