# `datafusion_proto_models::generated::datafusion::DateUnit`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.DateUnit.json).

<a id="op-ce5fea512f0146a8ab49e154"></a>
## DateUnit

`enum` · `datafusion_proto_models::generated::datafusion::DateUnit` · datafusion-proto-models 55.1.0

```rust
enum DateUnit
```

Source: `src/generated/prost.rs:2711`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a76477188b2fd236f7da5219"></a>
## DateMillisecond

`variant` · `datafusion_proto_models::generated::datafusion::DateUnit::DateMillisecond` · datafusion-proto-models 55.1.0

```rust
DateMillisecond
```

Source: `src/generated/prost.rs:2713`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5251b0089ff09d79351d97ba"></a>
## Day

`variant` · `datafusion_proto_models::generated::datafusion::DateUnit::Day` · datafusion-proto-models 55.1.0

```rust
Day
```

Source: `src/generated/prost.rs:2712`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0d2203a93b2fef43ee5794a"></a>
## Error

`assoc_type` · `datafusion_proto_models::generated::datafusion::DateUnit::Error` · datafusion-proto-models 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::DateUnit", "path": "DateUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2709, 68], "end": [2709, 88], "filename": "src/generated/prost.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/prost.rs:2709`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ce750f4ef93762b6e8cbde3"></a>
## as_str_name

`function` · `datafusion_proto_models::generated::datafusion::DateUnit::as_str_name` · datafusion-proto-models 55.1.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::DateUnit", "path": "DateUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2715, 1], "end": [2734, 2], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2720`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-cb86c18fc8c508ea339c3f2c"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::DateUnit::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> DateUnit
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::DateUnit", "path": "DateUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2709, 10], "end": [2709, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:2709`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42dac23a282c12782d4d407d"></a>
## cmp

`function` · `datafusion_proto_models::generated::datafusion::DateUnit::cmp` · datafusion-proto-models 55.1.0

```rust
fn cmp(&self, other: &DateUnit) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::DateUnit", "path": "DateUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2709, 63], "end": [2709, 66], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/generated/prost.rs:2709`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d215bed02ff614893fccdfe4"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::DateUnit::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> DateUnit
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::DateUnit", "path": "DateUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2709, 68], "end": [2709, 88], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:2709`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae8890804d83100558d51f1f"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::DateUnit::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::DateUnit", "path": "DateUnit"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5210, 1], "end": [5267, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:5212`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2906776581846f6d0346933f"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::DateUnit::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &DateUnit) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::DateUnit", "path": "DateUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2709, 30], "end": [2709, 39], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:2709`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6259b4a36fb61ea6f414748"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::DateUnit::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::DateUnit", "path": "DateUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2709, 23], "end": [2709, 28], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:2709`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba98855288431ebf3e76f21e"></a>
## from_i32

`function` · `datafusion_proto_models::generated::datafusion::DateUnit::from_i32` · datafusion-proto-models 55.1.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<DateUnit>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::DateUnit", "path": "DateUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2709, 68], "end": [2709, 88], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2709`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Converts an `i32` to a `DateUnit`, or `None` if `value` is not a valid variant.

<a id="op-cb0556dc074529966488e3a8"></a>
## from_str_name

`function` · `datafusion_proto_models::generated::datafusion::DateUnit::from_str_name` · datafusion-proto-models 55.1.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::DateUnit", "path": "DateUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2715, 1], "end": [2734, 2], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2727`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-d56d0aa8b9efad0034e929b2"></a>
## hash

`function` · `datafusion_proto_models::generated::datafusion::DateUnit::hash` · datafusion-proto-models 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::DateUnit", "path": "DateUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2709, 45], "end": [2709, 49], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/prost.rs:2709`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0224bdb1d57b1bb81dfe20ef"></a>
## is_valid

`function` · `datafusion_proto_models::generated::datafusion::DateUnit::is_valid` · datafusion-proto-models 55.1.0

```rust
const fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::DateUnit", "path": "DateUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2709, 68], "end": [2709, 88], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2709`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns `true` if `value` is a variant of `DateUnit`.

<a id="op-bc560852735f6bc49c54021b"></a>
## partial_cmp

`function` · `datafusion_proto_models::generated::datafusion::DateUnit::partial_cmp` · datafusion-proto-models 55.1.0

```rust
fn partial_cmp(&self, other: &DateUnit) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::DateUnit", "path": "DateUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2709, 51], "end": [2709, 61], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/generated/prost.rs:2709`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03c7b2a403bedacf47ab6476"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::DateUnit::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::DateUnit", "path": "DateUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5197, 1], "end": [5209, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:5199`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c3febc81175dfc138931c2d"></a>
## try_from

`function` · `datafusion_proto_models::generated::datafusion::DateUnit::try_from` · datafusion-proto-models 55.1.0

```rust
fn try_from(value: i32) -> ::core::result::Result<DateUnit, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::DateUnit", "path": "DateUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2709, 68], "end": [2709, 88], "filename": "src/generated/prost.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/prost.rs:2709`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
