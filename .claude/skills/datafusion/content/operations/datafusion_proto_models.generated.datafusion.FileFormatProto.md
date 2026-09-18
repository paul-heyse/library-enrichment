# `datafusion_proto_models::generated::datafusion::FileFormatProto`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.FileFormatProto.json).

<a id="op-35f836cbf0208c772feee07c"></a>
## FileFormatProto

`struct` · `datafusion_proto_models::generated::datafusion::FileFormatProto` · datafusion-proto-models 55.1.0

```rust
struct FileFormatProto
```

Source: `src/generated/prost.rs:453`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Wraps a serialized FileFormatFactory with its format kind tag,
so the decoder can dispatch to the correct format-specific codec.

<a id="op-a161da06dd4cf39f675e6ee4"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::FileFormatProto::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatProto", "path": "FileFormatProto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [452, 38], "end": [452, 54], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56524f02f1bdb1112a815483"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::FileFormatProto::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> FileFormatProto
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatProto", "path": "FileFormatProto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [452, 10], "end": [452, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c362377bfe06423734d282a5"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::FileFormatProto::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatProto", "path": "FileFormatProto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [452, 38], "end": [452, 54], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b45d7e8b0fb23ed18b63f6a5"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::FileFormatProto::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatProto", "path": "FileFormatProto"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6610, 1], "end": [6696, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:6612`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-741c9ea1f63bd222a64028e3"></a>
## encoded_file_format

`struct_field` · `datafusion_proto_models::generated::datafusion::FileFormatProto::encoded_file_format` · datafusion-proto-models 55.1.0

```rust
encoded_file_format: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/prost.rs:457`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b21925a14d640c6819fc963d"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::FileFormatProto::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatProto", "path": "FileFormatProto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [452, 38], "end": [452, 54], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6a76cd6c9c08b300290939d"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::FileFormatProto::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &FileFormatProto) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatProto", "path": "FileFormatProto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [452, 17], "end": [452, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ac6eac1e624ff54a7cb1ee7"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::FileFormatProto::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatProto", "path": "FileFormatProto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [452, 38], "end": [452, 54], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a2c150f1f09c5ff916896a0"></a>
## hash

`function` · `datafusion_proto_models::generated::datafusion::FileFormatProto::hash` · datafusion-proto-models 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatProto", "path": "FileFormatProto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [452, 32], "end": [452, 36], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/prost.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f8ee4e6975e1a67ac8c189f"></a>
## kind

`function` · `datafusion_proto_models::generated::datafusion::FileFormatProto::kind` · datafusion-proto-models 55.1.0

```rust
fn kind(&self) -> FileFormatKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatProto", "path": "FileFormatProto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [452, 38], "end": [452, 54], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the enum value of `kind`, or the default if the field is set to an invalid enum value.

<a id="op-8174b4cb2fb8290f17eacf71"></a>
## kind

`struct_field` · `datafusion_proto_models::generated::datafusion::FileFormatProto::kind` · datafusion-proto-models 55.1.0

```rust
kind: i32
```

Source: `src/generated/prost.rs:455`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-013f11629f468f4b4c822583"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::FileFormatProto::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatProto", "path": "FileFormatProto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6582, 1], "end": [6609, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:6584`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73eb84a882c527f2dd97e3b3"></a>
## set_kind

`function` · `datafusion_proto_models::generated::datafusion::FileFormatProto::set_kind` · datafusion-proto-models 55.1.0

```rust
fn set_kind(&mut self, value: FileFormatKind)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatProto", "path": "FileFormatProto"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [452, 38], "end": [452, 54], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Sets `kind` to the provided enum value.
