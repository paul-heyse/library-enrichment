# `datafusion_proto_models::generated::datafusion::FileGroup`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.FileGroup.json).

<a id="op-e715addc6d9fc997dfde681c"></a>
## FileGroup

`struct` · `datafusion_proto_models::generated::datafusion::FileGroup` · datafusion-proto-models 55.1.0

```rust
struct FileGroup
```

Source: `src/generated/prost.rs:1888`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a34f78243696fbc01c4360a"></a>
## Error

`assoc_type` · `datafusion_proto_models::generated::datafusion::FileGroup::Error` · datafusion-proto-models 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileGroup", "path": "protobuf::FileGroup"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PartitionedFile", "path": "protobuf::PartitionedFile"}}}], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "datafusion_common::DataFusionError"}}}}, "name": "Error"}]}}, "id": "core::convert::TryInto", "path": "TryInto"}}}], "generic_params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "T"}}}}}]}, "is_negative": false, "span": {"begin": [209, 1], "end": [223, 2], "filename": "src/to_proto.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"slice": {"generic": "T"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto.rs:213`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93d82c6751add3cdefde1031"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::FileGroup::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1887, 28], "end": [1887, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:1887`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b6f0a667768b7d30c2a015a"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::FileGroup::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> FileGroup
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1887, 10], "end": [1887, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:1887`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52a3cc99574bffd85f2b965c"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::FileGroup::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1887, 28], "end": [1887, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:1887`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bafed3acec483c432d0ef122"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::FileGroup::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileGroup", "path": "FileGroup"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6715, 1], "end": [6787, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:6717`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3e1cb07910505e055d6ca54"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::FileGroup::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1887, 28], "end": [1887, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:1887`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8a2b935b8a899177bdf4e0c"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::FileGroup::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &FileGroup) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1887, 17], "end": [1887, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:1887`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2fe367e069d880f6c6a1e7f"></a>
## files

`struct_field` · `datafusion_proto_models::generated::datafusion::FileGroup::files` · datafusion-proto-models 55.1.0

```rust
files: ::prost::alloc::vec::Vec<PartitionedFile>
```

Source: `src/generated/prost.rs:1890`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90f4094b5385bf40873dce1f"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::FileGroup::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1887, 28], "end": [1887, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:1887`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce8c27cafb0753d3711fceb1"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::FileGroup::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6697, 1], "end": [6714, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:6699`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bbf53746fe803a0172bebd0"></a>
## try_from

`function` · `datafusion_proto_models::generated::datafusion::FileGroup::try_from` · datafusion-proto-models 55.1.0

```rust
fn try_from(files: &[T]) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileGroup", "path": "protobuf::FileGroup"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PartitionedFile", "path": "protobuf::PartitionedFile"}}}], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "datafusion_common::DataFusionError"}}}}, "name": "Error"}]}}, "id": "core::convert::TryInto", "path": "TryInto"}}}], "generic_params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "T"}}}}}]}, "is_negative": false, "span": {"begin": [209, 1], "end": [223, 2], "filename": "src/to_proto.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"slice": {"generic": "T"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
