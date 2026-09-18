# `object_store::path::parts::PathPart`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.path.parts.PathPart.json).

<a id="op-c427f5625ecac2ba0151d814"></a>
## PathPart

`struct` · `object_store::path::parts::PathPart` · object_store 0.13.2

```rust
struct PathPart<'a>
```

Source: `src/path/parts.rs:48`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The PathPart type exists to validate the directory/file names that form part
of a path.

A [`PathPart`](../operations/object_store.path.parts.PathPart.md#op-c427f5625ecac2ba0151d814) is guaranteed to:

* Contain no ASCII control characters or `/`
* Not be a relative path segment, i.e. `.` or `..`

<a id="op-29af5133cd40a5f5b6decbbf"></a>
## as_ref

`function` · `object_store::path::parts::PathPart::as_ref` · object_store 0.13.2

```rust
fn as_ref(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "object_store::path::parts::PathPart", "path": "PathPart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [136, 2], "filename": "src/path/parts.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/path/parts.rs:133`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cfe00f49b75848cce586436"></a>
## clone

`function` · `object_store::path::parts::PathPart::clone` · object_store 0.13.2

```rust
fn clone(&self) -> PathPart<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::path::parts::PathPart", "path": "PathPart"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 10], "end": [47, 15], "filename": "src/path/parts.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/path/parts.rs:47`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f047fc6886956753b9435669"></a>
## cmp

`function` · `object_store::path::parts::PathPart::cmp` · object_store 0.13.2

```rust
fn cmp(&self, other: &PathPart<'a>) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::path::parts::PathPart", "path": "PathPart"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 44], "end": [47, 47], "filename": "src/path/parts.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/path/parts.rs:47`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c70e2c2659acda549b195a14"></a>
## default

`function` · `object_store::path::parts::PathPart::default` · object_store 0.13.2

```rust
fn default() -> PathPart<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::path::parts::PathPart", "path": "PathPart"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 56], "end": [47, 63], "filename": "src/path/parts.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/path/parts.rs:47`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd731ba296ae744b82fd88e1"></a>
## eq

`function` · `object_store::path::parts::PathPart::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &PathPart<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::path::parts::PathPart", "path": "PathPart"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 17], "end": [47, 26], "filename": "src/path/parts.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/path/parts.rs:47`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02f18c8db71fbca118f478d7"></a>
## fmt

`function` · `object_store::path::parts::PathPart::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::path::parts::PathPart", "path": "PathPart"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 49], "end": [47, 54], "filename": "src/path/parts.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/path/parts.rs:47`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-105d1c636b7876ea39237a0b"></a>
## from

`function` · `object_store::path::parts::PathPart::from` · object_store 0.13.2

```rust
fn from(s: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'static"}], "constraints": []}}, "id": "object_store::path::parts::PathPart", "path": "PathPart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [130, 2], "filename": "src/path/parts.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/path/parts.rs:125`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5f5b5882cd77efb7dd67187"></a>
## from

`function` · `object_store::path::parts::PathPart::from` · object_store 0.13.2

```rust
fn from(v: &'a str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::path::parts::PathPart", "path": "PathPart"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [122, 2], "filename": "src/path/parts.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/path/parts.rs:119`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e46022ab7dc696c5f2a4fbcf"></a>
## from

`function` · `object_store::path::parts::PathPart::from` · object_store 0.13.2

```rust
fn from(v: &'a [u8]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::path::parts::PathPart", "path": "PathPart"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [116, 2], "filename": "src/path/parts.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"slice": {"primitive": "u8"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/path/parts.rs:106`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-167123ca4425d565ee13ef9e"></a>
## hash

`function` · `object_store::path::parts::PathPart::hash` · object_store 0.13.2

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::path::parts::PathPart", "path": "PathPart"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 65], "end": [47, 69], "filename": "src/path/parts.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/path/parts.rs:47`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea6bfbca1447906a5e445d6d"></a>
## parse

`function` · `object_store::path::parts::PathPart::parse` · object_store 0.13.2

```rust
fn parse(segment: &'a str) -> Result<Self, InvalidPart>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::path::parts::PathPart", "path": "PathPart"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [76, 2], "filename": "src/path/parts.rs"}, "trait": null, "trait_path": null}`

Source: `src/path/parts.rs:54`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Parse the provided path segment as a [`PathPart`](../operations/object_store.path.parts.PathPart.md#op-c427f5625ecac2ba0151d814) returning an error if invalid

<a id="op-2ad5929bb906911db600df79"></a>
## partial_cmp

`function` · `object_store::path::parts::PathPart::partial_cmp` · object_store 0.13.2

```rust
fn partial_cmp(&self, other: &PathPart<'a>) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::path::parts::PathPart", "path": "PathPart"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 32], "end": [47, 42], "filename": "src/path/parts.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/path/parts.rs:47`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
