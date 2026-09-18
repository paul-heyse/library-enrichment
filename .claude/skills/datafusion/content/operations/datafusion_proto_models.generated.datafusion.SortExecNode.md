# `datafusion_proto_models::generated::datafusion::SortExecNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.SortExecNode.json).

<a id="op-5c4baef4d29262f6c37a2333"></a>
## SortExecNode

`struct` · `datafusion_proto_models::generated::datafusion::SortExecNode` · datafusion-proto-models 55.1.0

```rust
struct SortExecNode
```

Source: `src/generated/prost.rs:2273`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3fe8a6ae51325482aee9da4"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::SortExecNode::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::SortExecNode", "path": "SortExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2272, 28], "end": [2272, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:2272`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58d1e4bfe7bf71741c1dcdc7"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::SortExecNode::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> SortExecNode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::SortExecNode", "path": "SortExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2272, 10], "end": [2272, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:2272`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca7cac63e9f2e9b8950f8cc1"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::SortExecNode::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::SortExecNode", "path": "SortExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2272, 28], "end": [2272, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:2272`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80770037a4b6cf9c72f7c4a9"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::SortExecNode::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::SortExecNode", "path": "SortExecNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [25043, 1], "end": [25163, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:25045`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f34ee89e9321bef0a44dc62"></a>
## dynamic_filter

`struct_field` · `datafusion_proto_models::generated::datafusion::SortExecNode::dynamic_filter` · datafusion-proto-models 55.1.0

```rust
dynamic_filter: ::core::option::Option<PhysicalExprNode>
```

Source: `src/generated/prost.rs:2285`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional dynamic filter expression for TopK pushdown.

<a id="op-776b30163c27302fbce09544"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::SortExecNode::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::SortExecNode", "path": "SortExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2272, 28], "end": [2272, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:2272`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff2567e291386fb30e7c7837"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::SortExecNode::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &SortExecNode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::SortExecNode", "path": "SortExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2272, 17], "end": [2272, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:2272`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1da943292ed4f27d85346bf"></a>
## expr

`struct_field` · `datafusion_proto_models::generated::datafusion::SortExecNode::expr` · datafusion-proto-models 55.1.0

```rust
expr: ::prost::alloc::vec::Vec<PhysicalExprNode>
```

Source: `src/generated/prost.rs:2277`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3135f859f1ad4c9e1f05bd46"></a>
## fetch

`struct_field` · `datafusion_proto_models::generated::datafusion::SortExecNode::fetch` · datafusion-proto-models 55.1.0

```rust
fetch: i64
```

Source: `src/generated/prost.rs:2280`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Maximum number of highest/lowest rows to fetch; negative means no limit

<a id="op-0565c10fdaf498d25519eee9"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::SortExecNode::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::SortExecNode", "path": "SortExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2272, 28], "end": [2272, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:2272`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-992fdcdab9ffd16503310b94"></a>
## input

`struct_field` · `datafusion_proto_models::generated::datafusion::SortExecNode::input` · datafusion-proto-models 55.1.0

```rust
input: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>
```

Source: `src/generated/prost.rs:2275`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d5b6f7693ca19945ee92cba"></a>
## preserve_partitioning

`struct_field` · `datafusion_proto_models::generated::datafusion::SortExecNode::preserve_partitioning` · datafusion-proto-models 55.1.0

```rust
preserve_partitioning: bool
```

Source: `src/generated/prost.rs:2282`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf7a0086f4c41f8b2591bfe3"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::SortExecNode::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::SortExecNode", "path": "SortExecNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24999, 1], "end": [25042, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:25001`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
