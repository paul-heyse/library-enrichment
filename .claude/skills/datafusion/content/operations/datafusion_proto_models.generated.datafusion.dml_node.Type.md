# `datafusion_proto_models::generated::datafusion::dml_node::Type`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.dml_node.Type.json).

<a id="op-af4c10ea3a86ea78805830b4"></a>
## Type

`enum` · `datafusion_proto_models::generated::datafusion::dml_node::Type` · datafusion-proto-models 55.1.0

```rust
enum Type
```

Source: `src/generated/prost.rs:487`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f8584cc0ab5144b2ea5dac3"></a>
## Ctas

`variant` · `datafusion_proto_models::generated::datafusion::dml_node::Type::Ctas` · datafusion-proto-models 55.1.0

```rust
Ctas
```

Source: `src/generated/prost.rs:490`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63399bc4c92ad476b0cd5a27"></a>
## Delete

`variant` · `datafusion_proto_models::generated::datafusion::dml_node::Type::Delete` · datafusion-proto-models 55.1.0

```rust
Delete
```

Source: `src/generated/prost.rs:489`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72d72c2c371629f45afcc6c4"></a>
## Error

`assoc_type` · `datafusion_proto_models::generated::datafusion::dml_node::Type::Error` · datafusion-proto-models 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::dml_node::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [484, 9], "end": [484, 29], "filename": "src/generated/prost.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/prost.rs:484`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c101c956efd073fd94a62cc2"></a>
## InsertAppend

`variant` · `datafusion_proto_models::generated::datafusion::dml_node::Type::InsertAppend` · datafusion-proto-models 55.1.0

```rust
InsertAppend
```

Source: `src/generated/prost.rs:491`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-273e6e6291e5d7f69921b64f"></a>
## InsertOverwrite

`variant` · `datafusion_proto_models::generated::datafusion::dml_node::Type::InsertOverwrite` · datafusion-proto-models 55.1.0

```rust
InsertOverwrite
```

Source: `src/generated/prost.rs:492`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54695361d7116ee03c4ee3fd"></a>
## InsertReplace

`variant` · `datafusion_proto_models::generated::datafusion::dml_node::Type::InsertReplace` · datafusion-proto-models 55.1.0

```rust
InsertReplace
```

Source: `src/generated/prost.rs:493`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1df18f9962a3476305af2902"></a>
## MergeInto

`variant` · `datafusion_proto_models::generated::datafusion::dml_node::Type::MergeInto` · datafusion-proto-models 55.1.0

```rust
MergeInto
```

Source: `src/generated/prost.rs:495`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c014c930b41273ca8675380"></a>
## Truncate

`variant` · `datafusion_proto_models::generated::datafusion::dml_node::Type::Truncate` · datafusion-proto-models 55.1.0

```rust
Truncate
```

Source: `src/generated/prost.rs:494`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21e7436bc5feed014426652c"></a>
## Update

`variant` · `datafusion_proto_models::generated::datafusion::dml_node::Type::Update` · datafusion-proto-models 55.1.0

```rust
Update
```

Source: `src/generated/prost.rs:488`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f955478b8449999977d49e7c"></a>
## as_str_name

`function` · `datafusion_proto_models::generated::datafusion::dml_node::Type::as_str_name` · datafusion-proto-models 55.1.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::dml_node::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [497, 5], "end": [528, 6], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:502`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-22a0a0fb2e2071853ba0a115"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::dml_node::Type::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> Type
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::dml_node::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [476, 9], "end": [476, 14], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:476`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53179efa5c48bcfd9bb05f3a"></a>
## cmp

`function` · `datafusion_proto_models::generated::datafusion::dml_node::Type::cmp` · datafusion-proto-models 55.1.0

```rust
fn cmp(&self, other: &Type) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::dml_node::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 9], "end": [483, 12], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/generated/prost.rs:483`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-833ea27dafd29b40a642d232"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::dml_node::Type::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Type
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::dml_node::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [484, 9], "end": [484, 29], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:484`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5b6fc06d62312ada66a7a02"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::dml_node::Type::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::dml_node::Type", "path": "dml_node::Type"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5687, 1], "end": [5756, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:5689`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-747eb10f52d2762d82b8446d"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::dml_node::Type::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &Type) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::dml_node::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [479, 9], "end": [479, 18], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:479`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eecc800be1881e2993e8c9d0"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::dml_node::Type::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::dml_node::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [478, 9], "end": [478, 14], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:478`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-633655b611fb48a301893f90"></a>
## from_i32

`function` · `datafusion_proto_models::generated::datafusion::dml_node::Type::from_i32` · datafusion-proto-models 55.1.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<Type>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::dml_node::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [484, 9], "end": [484, 29], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:484`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Converts an `i32` to a `Type`, or `None` if `value` is not a valid variant.

<a id="op-258d2fa43d469b59e51be41e"></a>
## from_str_name

`function` · `datafusion_proto_models::generated::datafusion::dml_node::Type::from_str_name` · datafusion-proto-models 55.1.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::dml_node::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [497, 5], "end": [528, 6], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:515`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-8878f116e83b18a52f13f6a8"></a>
## hash

`function` · `datafusion_proto_models::generated::datafusion::dml_node::Type::hash` · datafusion-proto-models 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::dml_node::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [481, 9], "end": [481, 13], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/prost.rs:481`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5c6248f97193f1ebca850dc"></a>
## is_valid

`function` · `datafusion_proto_models::generated::datafusion::dml_node::Type::is_valid` · datafusion-proto-models 55.1.0

```rust
const fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::dml_node::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [484, 9], "end": [484, 29], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:484`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns `true` if `value` is a variant of `Type`.

<a id="op-259dfc7cafd402488fe4d265"></a>
## partial_cmp

`function` · `datafusion_proto_models::generated::datafusion::dml_node::Type::partial_cmp` · datafusion-proto-models 55.1.0

```rust
fn partial_cmp(&self, other: &Type) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::dml_node::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [482, 9], "end": [482, 19], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/generated/prost.rs:482`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7afd3717a380a7eda5475024"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::dml_node::Type::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::dml_node::Type", "path": "dml_node::Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5668, 1], "end": [5686, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:5670`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69806c6e831cafc387e86661"></a>
## try_from

`function` · `datafusion_proto_models::generated::datafusion::dml_node::Type::try_from` · datafusion-proto-models 55.1.0

```rust
fn try_from(value: i32) -> ::core::result::Result<Type, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::dml_node::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [484, 9], "end": [484, 29], "filename": "src/generated/prost.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/prost.rs:484`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
