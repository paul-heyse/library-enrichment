# `datafusion_proto_models::generated::datafusion::PlaceholderNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.PlaceholderNode.json).

<a id="op-938e594703d92d6d69479a6c"></a>
## PlaceholderNode

`struct` · `datafusion_proto_models::generated::datafusion::PlaceholderNode` · datafusion-proto-models 55.1.0

```rust
struct PlaceholderNode
```

Source: `src/generated/prost.rs:880`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-834f187041933c8000185283"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::PlaceholderNode::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PlaceholderNode", "path": "PlaceholderNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [879, 28], "end": [879, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:879`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6456e36bf559fb893bc03ac0"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::PlaceholderNode::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> PlaceholderNode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PlaceholderNode", "path": "PlaceholderNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [879, 10], "end": [879, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:879`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-554ad13fa83a22d54fffe287"></a>
## data_type

`struct_field` · `datafusion_proto_models::generated::datafusion::PlaceholderNode::data_type` · datafusion-proto-models 55.1.0

```rust
data_type: ::core::option::Option<super::datafusion_common::ArrowType>
```

Source: `src/generated/prost.rs:886`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

We serialize the data type, metadata, and nullability separately to maintain
compatibility with older versions

<a id="op-59ea7ab078f1bdc5448b0568"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::PlaceholderNode::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PlaceholderNode", "path": "PlaceholderNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [879, 28], "end": [879, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:879`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1ca6e23dcb1df36663b9673"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::PlaceholderNode::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PlaceholderNode", "path": "PlaceholderNode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [22242, 1], "end": [22350, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:22244`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d14f1f67757875a8d6fae0ed"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::PlaceholderNode::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PlaceholderNode", "path": "PlaceholderNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [879, 28], "end": [879, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:879`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb97309246c9ee10b53c446b"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::PlaceholderNode::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &PlaceholderNode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PlaceholderNode", "path": "PlaceholderNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [879, 17], "end": [879, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:879`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2dea1f4b451dac5fef8ecaf9"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::PlaceholderNode::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PlaceholderNode", "path": "PlaceholderNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [879, 28], "end": [879, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:879`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fb38b115fa881e021df81d4"></a>
## id

`struct_field` · `datafusion_proto_models::generated::datafusion::PlaceholderNode::id` · datafusion-proto-models 55.1.0

```rust
id: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:882`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87b117371494dd9fc981e33a"></a>
## metadata

`struct_field` · `datafusion_proto_models::generated::datafusion::PlaceholderNode::metadata` · datafusion-proto-models 55.1.0

```rust
metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>
```

Source: `src/generated/prost.rs:890`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a4b2065c6328624ae859015"></a>
## nullable

`struct_field` · `datafusion_proto_models::generated::datafusion::PlaceholderNode::nullable` · datafusion-proto-models 55.1.0

```rust
nullable: ::core::option::Option<bool>
```

Source: `src/generated/prost.rs:888`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec4f4de5ad3e6ec61ebc5f39"></a>
## nullable

`function` · `datafusion_proto_models::generated::datafusion::PlaceholderNode::nullable` · datafusion-proto-models 55.1.0

```rust
fn nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PlaceholderNode", "path": "PlaceholderNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [879, 28], "end": [879, 44], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:879`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the value of `nullable`, or the default value if `nullable` is unset.

<a id="op-92259f1f7de385486f423038"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::PlaceholderNode::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PlaceholderNode", "path": "PlaceholderNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22206, 1], "end": [22241, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:22208`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
