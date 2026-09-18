# `datafusion_proto_models::generated::datafusion::InsertOp`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.InsertOp.json).

<a id="op-eab508c148b2dc8723c67b34"></a>
## InsertOp

`enum` · `datafusion_proto_models::generated::datafusion::InsertOp` · datafusion-proto-models 55.1.0

```rust
enum InsertOp
```

Source: `src/generated/prost.rs:2770`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c871aa1001b13a87eda6673"></a>
## Append

`variant` · `datafusion_proto_models::generated::datafusion::InsertOp::Append` · datafusion-proto-models 55.1.0

```rust
Append
```

Source: `src/generated/prost.rs:2771`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e2b366d09b4e484c66dbcf9"></a>
## Error

`assoc_type` · `datafusion_proto_models::generated::datafusion::InsertOp::Error` · datafusion-proto-models 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2768, 68], "end": [2768, 88], "filename": "src/generated/prost.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/prost.rs:2768`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d08fc08db22370ee2099b994"></a>
## Overwrite

`variant` · `datafusion_proto_models::generated::datafusion::InsertOp::Overwrite` · datafusion-proto-models 55.1.0

```rust
Overwrite
```

Source: `src/generated/prost.rs:2772`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13c6f3135f50d3b7b5ef8509"></a>
## Replace

`variant` · `datafusion_proto_models::generated::datafusion::InsertOp::Replace` · datafusion-proto-models 55.1.0

```rust
Replace
```

Source: `src/generated/prost.rs:2773`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c936012104fec1479dd1e59"></a>
## as_str_name

`function` · `datafusion_proto_models::generated::datafusion::InsertOp::as_str_name` · datafusion-proto-models 55.1.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2775, 1], "end": [2796, 2], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2780`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-898cb6eb6838151a836428d1"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::InsertOp::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> InsertOp
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2768, 10], "end": [2768, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:2768`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-deedcfa9a2a2fe32bb59678d"></a>
## cmp

`function` · `datafusion_proto_models::generated::datafusion::InsertOp::cmp` · datafusion-proto-models 55.1.0

```rust
fn cmp(&self, other: &InsertOp) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2768, 63], "end": [2768, 66], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/generated/prost.rs:2768`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce2dd58b0ed5f5b6662ae134"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::InsertOp::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> InsertOp
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2768, 68], "end": [2768, 88], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:2768`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-354aa979770e0fb4104743ed"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::InsertOp::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::InsertOp", "path": "InsertOp"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9832, 1], "end": [9891, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:9834`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45c434521f261473131446df"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::InsertOp::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &InsertOp) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2768, 30], "end": [2768, 39], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:2768`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78b3e20f3f7b498585dfe295"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::InsertOp::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2768, 23], "end": [2768, 28], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:2768`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-394cacaed69f6b97c5640739"></a>
## from_i32

`function` · `datafusion_proto_models::generated::datafusion::InsertOp::from_i32` · datafusion-proto-models 55.1.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<InsertOp>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2768, 68], "end": [2768, 88], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2768`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Converts an `i32` to a `InsertOp`, or `None` if `value` is not a valid variant.

<a id="op-d06f25556a7d4a9d300d30de"></a>
## from_str_name

`function` · `datafusion_proto_models::generated::datafusion::InsertOp::from_str_name` · datafusion-proto-models 55.1.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2775, 1], "end": [2796, 2], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2788`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-9e96b32fd4883fc880404ca9"></a>
## hash

`function` · `datafusion_proto_models::generated::datafusion::InsertOp::hash` · datafusion-proto-models 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2768, 45], "end": [2768, 49], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/prost.rs:2768`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c0fc723cb2c3528d1e799b4"></a>
## is_valid

`function` · `datafusion_proto_models::generated::datafusion::InsertOp::is_valid` · datafusion-proto-models 55.1.0

```rust
const fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2768, 68], "end": [2768, 88], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2768`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns `true` if `value` is a variant of `InsertOp`.

<a id="op-69e9c9e38ac2cecec6f8a3b8"></a>
## partial_cmp

`function` · `datafusion_proto_models::generated::datafusion::InsertOp::partial_cmp` · datafusion-proto-models 55.1.0

```rust
fn partial_cmp(&self, other: &InsertOp) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2768, 51], "end": [2768, 61], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/generated/prost.rs:2768`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1434e6de01a37617e361a39c"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::InsertOp::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9818, 1], "end": [9831, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:9820`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05f23790aed7f2a8e8f8c0cc"></a>
## try_from

`function` · `datafusion_proto_models::generated::datafusion::InsertOp::try_from` · datafusion-proto-models 55.1.0

```rust
fn try_from(value: i32) -> ::core::result::Result<InsertOp, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2768, 68], "end": [2768, 88], "filename": "src/generated/prost.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/prost.rs:2768`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
