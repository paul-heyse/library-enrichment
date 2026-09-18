# `datafusion_proto_models::generated::datafusion::Not`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.Not.json).

<a id="op-b70534a17fffd0b73eb830f8"></a>
## Not

`struct` · `datafusion_proto_models::generated::datafusion::Not` · datafusion-proto-models 55.1.0

```rust
struct Not
```

Source: `src/generated/prost.rs:975`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebdb5edf1a14315e4666b8be"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::Not::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::Not", "path": "Not"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [974, 28], "end": [974, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:974`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-134b7f0bb767ea3b9da3e602"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::Not::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> Not
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::Not", "path": "Not"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [974, 10], "end": [974, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:974`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26507ebc34ba36c2f6f10d87"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::Not::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::Not", "path": "Not"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [974, 28], "end": [974, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:974`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-761ffe27f6dbca8ce4d0bb67"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::Not::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::Not", "path": "Not"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [15694, 1], "end": [15766, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:15696`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d49797194328d614e2f6bbda"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::Not::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::Not", "path": "Not"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [974, 28], "end": [974, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:974`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95056b40191d5634545bb43d"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::Not::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &Not) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::Not", "path": "Not"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [974, 17], "end": [974, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:974`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01a5177b9bba8bd178bcb19e"></a>
## expr

`struct_field` · `datafusion_proto_models::generated::datafusion::Not::expr` · datafusion-proto-models 55.1.0

```rust
expr: ::core::option::Option<::prost::alloc::boxed::Box<LogicalExprNode>>
```

Source: `src/generated/prost.rs:977`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-662364f1ad6adbd0cd357878"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::Not::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::Not", "path": "Not"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [974, 28], "end": [974, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:974`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe2e063dbed95941c395a69b"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::Not::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::Not", "path": "Not"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [15676, 1], "end": [15693, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:15678`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
