# `datafusion_proto_common::generated::datafusion_proto_common::ScalarValue`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.generated.datafusion_proto_common.ScalarValue.json).

<a id="op-be9896b0e4fedc3cf0f31bbd"></a>
## ScalarValue

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarValue` · datafusion-proto-common 55.1.0

```rust
struct ScalarValue
```

Source: `src/generated/prost.rs:327`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab560e2227fe72b2f2deb971"></a>
## Error

`assoc_type` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarValue::Error` · datafusion-proto-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ScalarValue", "path": "protobuf::ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [314, 1], "end": [708, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:315`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3a9d2da088b5434b8619b7d"></a>
## clear

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarValue::clear` · datafusion-proto-common 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 28], "end": [326, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b498eac7fae58b37a9f98fb0"></a>
## clone

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarValue::clone` · datafusion-proto-common 55.1.0

```rust
fn clone(&self) -> ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 10], "end": [326, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9cd31ff8dd5c077a3f2f21a"></a>
## default

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarValue::default` · datafusion-proto-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 28], "end": [326, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97286703fd17addb88389a79"></a>
## deserialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarValue::deserialize` · datafusion-proto-common 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8724, 1], "end": [9258, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:8726`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6152f1f6d79156edd37dbee8"></a>
## encoded_len

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarValue::encoded_len` · datafusion-proto-common 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 28], "end": [326, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a71c99ee5c84a6e42b0867a"></a>
## eq

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarValue::eq` · datafusion-proto-common 55.1.0

```rust
fn eq(&self, other: &ScalarValue) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 17], "end": [326, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54e03489bb5520e0f0e8fada"></a>
## fmt

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarValue::fmt` · datafusion-proto-common 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 28], "end": [326, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dee36d500c38fc377243f386"></a>
## serialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarValue::serialize` · datafusion-proto-common 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8550, 1], "end": [8723, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:8552`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3cc0375213f77ca5c48c415"></a>
## try_from

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarValue::try_from` · datafusion-proto-common 55.1.0

```rust
fn try_from(val: &ScalarValue) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ScalarValue", "path": "protobuf::ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [314, 1], "end": [708, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:317`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e938de9e0eb9b2633a1b6a70"></a>
## value

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarValue::value` · datafusion-proto-common 55.1.0

```rust
value: ::core::option::Option<scalar_value::Value>
```

Source: `src/generated/prost.rs:332`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
