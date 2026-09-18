# `arrow_schema::datatype::UnionMode`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.datatype.UnionMode.json).

<a id="op-8c1b80abb1f97f0c579d6b67"></a>
## UnionMode

`enum` · `arrow_schema::datatype::UnionMode` · arrow-schema 59.3.0

```rust
enum UnionMode
```

Source: `src/datatype.rs:480`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Sparse or Dense union layouts

<a id="op-b848f94c0cc0f60c401c916c"></a>
## Dense

`variant` · `arrow_schema::datatype::UnionMode::Dense` · arrow-schema 59.3.0

```rust
Dense
```

Source: `src/datatype.rs:484`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Dense union layout

<a id="op-09af2f8773db722262951a02"></a>
## Sparse

`variant` · `arrow_schema::datatype::UnionMode::Sparse` · arrow-schema 59.3.0

```rust
Sparse
```

Source: `src/datatype.rs:482`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Sparse union layout

<a id="op-1502ba98b9cb68d3f6fcb076"></a>
## clone

`function` · `arrow_schema::datatype::UnionMode::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> UnionMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::UnionMode", "path": "UnionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [478, 17], "end": [478, 22], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/datatype.rs:478`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d77e2b06dd0059c64d5be1f5"></a>
## cmp

`function` · `arrow_schema::datatype::UnionMode::cmp` · arrow-schema 59.3.0

```rust
fn cmp(&self, other: &UnionMode) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::UnionMode", "path": "UnionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [478, 57], "end": [478, 60], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/datatype.rs:478`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6eada83d4b80d9dbb82dc478"></a>
## deserialize

`function` · `arrow_schema::datatype::UnionMode::deserialize` · arrow-schema 59.3.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::UnionMode", "path": "UnionMode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [479, 56], "end": [479, 74], "filename": "src/datatype.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/datatype.rs:479`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb97f75eb1e2c17376ff0f0e"></a>
## eq

`function` · `arrow_schema::datatype::UnionMode::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &UnionMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::UnionMode", "path": "UnionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [478, 24], "end": [478, 33], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/datatype.rs:478`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1b4d998e64a4352aa98a80e"></a>
## fmt

`function` · `arrow_schema::datatype::UnionMode::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::UnionMode", "path": "UnionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [478, 10], "end": [478, 15], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/datatype.rs:478`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adce0d2cc7ef534d98dfa3a3"></a>
## hash

`function` · `arrow_schema::datatype::UnionMode::hash` · arrow-schema 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::UnionMode", "path": "UnionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [478, 39], "end": [478, 43], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/datatype.rs:478`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f77ce7a777720f9501826604"></a>
## partial_cmp

`function` · `arrow_schema::datatype::UnionMode::partial_cmp` · arrow-schema 59.3.0

```rust
fn partial_cmp(&self, other: &UnionMode) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::UnionMode", "path": "UnionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [478, 45], "end": [478, 55], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/datatype.rs:478`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-770518b49c436ee4f3384637"></a>
## serialize

`function` · `arrow_schema::datatype::UnionMode::serialize` · arrow-schema 59.3.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::UnionMode", "path": "UnionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [479, 38], "end": [479, 54], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/datatype.rs:479`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
