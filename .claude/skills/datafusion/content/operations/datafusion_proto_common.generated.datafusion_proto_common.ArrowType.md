# `datafusion_proto_common::generated::datafusion_proto_common::ArrowType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.generated.datafusion_proto_common.ArrowType.json).

<a id="op-1be66b8ea9e3348e4a782baf"></a>
## ArrowType

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ArrowType` · datafusion-proto-common 55.1.0

```rust
struct ArrowType
```

Source: `src/generated/prost.rs:471`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Serialized data type

<a id="op-c6dc381d907e29c2781af1c0"></a>
## Error

`assoc_type` · `datafusion_proto_common::generated::datafusion_proto_common::ArrowType::Error` · datafusion-proto-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ArrowType", "path": "protobuf::ArrowType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [116, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-781df2aa2a1259ec89dd0ee7"></a>
## arrow_type_enum

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ArrowType::arrow_type_enum` · datafusion-proto-common 55.1.0

```rust
arrow_type_enum: ::core::option::Option<arrow_type::ArrowTypeEnum>
```

Source: `src/generated/prost.rs:476`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b879e1d58db2a1f0892dffeb"></a>
## clear

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ArrowType::clear` · datafusion-proto-common 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ArrowType", "path": "ArrowType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [470, 28], "end": [470, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:470`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f075fc324aef3b89bb8943f"></a>
## clone

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ArrowType::clone` · datafusion-proto-common 55.1.0

```rust
fn clone(&self) -> ArrowType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ArrowType", "path": "ArrowType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [470, 10], "end": [470, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:470`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31609fbe1c5f1aeb5e37f6a6"></a>
## default

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ArrowType::default` · datafusion-proto-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ArrowType", "path": "ArrowType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [470, 28], "end": [470, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:470`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af07b2e14f1d475ef4551608"></a>
## deserialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ArrowType::deserialize` · datafusion-proto-common 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ArrowType", "path": "ArrowType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [771, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:295`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddab4e574b9920e996049502"></a>
## encoded_len

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ArrowType::encoded_len` · datafusion-proto-common 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ArrowType", "path": "ArrowType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [470, 28], "end": [470, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:470`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe9103a5af1fc349af1a1c85"></a>
## eq

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ArrowType::eq` · datafusion-proto-common 55.1.0

```rust
fn eq(&self, other: &ArrowType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ArrowType", "path": "ArrowType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [470, 17], "end": [470, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:470`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75df478a13ea56af36bc7246"></a>
## fmt

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ArrowType::fmt` · datafusion-proto-common 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ArrowType", "path": "ArrowType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [470, 28], "end": [470, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:470`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea05b0297b752b7fb8e8fe9c"></a>
## serialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ArrowType::serialize` · datafusion-proto-common 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ArrowType", "path": "ArrowType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 1], "end": [292, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5ca94d259191ff5b8b2ba4c"></a>
## try_from

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ArrowType::try_from` · datafusion-proto-common 55.1.0

```rust
fn try_from(val: &DataType) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ArrowType", "path": "protobuf::ArrowType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [116, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
