# `datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.generated.datafusion_proto_common.JsonWriterOptions.json).

<a id="op-305e9046fe3926abe44b9369"></a>
## JsonWriterOptions

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions` · datafusion-proto-common 55.1.0

```rust
struct JsonWriterOptions
```

Source: `src/generated/prost.rs:583`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-696015071f743e7a2615fcd9"></a>
## Error

`assoc_type` · `datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions::Error` · datafusion-proto-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions", "path": "protobuf::JsonWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [882, 1], "end": [893, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::json_writer::JsonWriterOptions", "path": "JsonWriterOptions"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:883`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acdb7727a4754a8651a251f2"></a>
## clear

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions::clear` · datafusion-proto-common 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions", "path": "JsonWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [582, 44], "end": [582, 60], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:582`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-243143008b818aca72ab6869"></a>
## clone

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions::clone` · datafusion-proto-common 55.1.0

```rust
fn clone(&self) -> JsonWriterOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions", "path": "JsonWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [582, 10], "end": [582, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:582`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bdc4fe1d706cec02735587c5"></a>
## compression

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions::compression` · datafusion-proto-common 55.1.0

```rust
fn compression(&self) -> CompressionTypeVariant
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions", "path": "JsonWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [582, 44], "end": [582, 60], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:582`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Returns the enum value of `compression`, or the default if the field is set to an invalid enum value.

<a id="op-e0cfdf42f322f96ac267ab63"></a>
## compression

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions::compression` · datafusion-proto-common 55.1.0

```rust
compression: i32
```

Source: `src/generated/prost.rs:585`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-924aafd49b2ccb6c888855be"></a>
## default

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions::default` · datafusion-proto-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions", "path": "JsonWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [582, 44], "end": [582, 60], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:582`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb639b2fb8bb4a4a71d1fe64"></a>
## deserialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions::deserialize` · datafusion-proto-common 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions", "path": "JsonWriterOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5176, 1], "end": [5248, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:5178`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd54cb036ba30bfb676e8b09"></a>
## encoded_len

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions::encoded_len` · datafusion-proto-common 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions", "path": "JsonWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [582, 44], "end": [582, 60], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:582`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6404d3049c3be698d0058ae"></a>
## eq

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions::eq` · datafusion-proto-common 55.1.0

```rust
fn eq(&self, other: &JsonWriterOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions", "path": "JsonWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [582, 23], "end": [582, 32], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:582`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ab3cc69b586a2011bd70ee0"></a>
## fmt

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions::fmt` · datafusion-proto-common 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions", "path": "JsonWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [582, 44], "end": [582, 60], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:582`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d75286a58e446649eb2e9ba3"></a>
## hash

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions::hash` · datafusion-proto-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions", "path": "JsonWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [582, 38], "end": [582, 42], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/prost.rs:582`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfc250ae1eb7ae37d354bceb"></a>
## serialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions::serialize` · datafusion-proto-common 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions", "path": "JsonWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5156, 1], "end": [5175, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:5158`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f68c84b73010372a5bfa124"></a>
## set_compression

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions::set_compression` · datafusion-proto-common 55.1.0

```rust
fn set_compression(&mut self, value: CompressionTypeVariant)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions", "path": "JsonWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [582, 44], "end": [582, 60], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:582`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Sets `compression` to the provided enum value.

<a id="op-51826da9a71e96d73b14c621"></a>
## try_from

`function` · `datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions::try_from` · datafusion-proto-common 55.1.0

```rust
fn try_from(opts: &JsonWriterOptions) -> datafusion_common::Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions", "path": "protobuf::JsonWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [882, 1], "end": [893, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::json_writer::JsonWriterOptions", "path": "JsonWriterOptions"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:885`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
