# `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.unnest_options.NullHandling.json).

<a id="op-56fae4fa5bfc83fe11b68801"></a>
## NullHandling

`enum` · `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling` · datafusion-proto-models 55.1.0

```rust
enum NullHandling
```

Source: `src/generated/prost.rs:692`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a6ee4d5d50ccb641bda9f09"></a>
## Drop

`variant` · `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling::Drop` · datafusion-proto-models 55.1.0

```rust
Drop
```

Source: `src/generated/prost.rs:696`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Drop both null and empty lists from the output.

<a id="op-9d094e0aee7cda815a8eeed1"></a>
## Error

`assoc_type` · `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling::Error` · datafusion-proto-models 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::unnest_options::NullHandling", "path": "NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [689, 9], "end": [689, 29], "filename": "src/generated/prost.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/prost.rs:689`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-510fba7e1b5ba555c2c915a5"></a>
## Preserve

`variant` · `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling::Preserve` · datafusion-proto-models 55.1.0

```rust
Preserve
```

Source: `src/generated/prost.rs:694`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Preserve nulls; empty lists produce no rows. The historical default.

<a id="op-9e50435017bb1d606a12cbd9"></a>
## PreserveAndExpandEmpty

`variant` · `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling::PreserveAndExpandEmpty` · datafusion-proto-models 55.1.0

```rust
PreserveAndExpandEmpty
```

Source: `src/generated/prost.rs:699`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Preserve nulls, and additionally expand empty lists into a single
NULL output row (outer-unnest semantics).

<a id="op-8f8a25a940195bd1893ee6da"></a>
## as_str_name

`function` · `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling::as_str_name` · datafusion-proto-models 55.1.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::unnest_options::NullHandling", "path": "NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [701, 5], "end": [722, 6], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:706`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-4e0d3b83ebdeb32adc917237"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> NullHandling
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::unnest_options::NullHandling", "path": "NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [681, 9], "end": [681, 14], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:681`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2bf656f55c29aacaa39708c"></a>
## cmp

`function` · `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling::cmp` · datafusion-proto-models 55.1.0

```rust
fn cmp(&self, other: &NullHandling) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::unnest_options::NullHandling", "path": "NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [688, 9], "end": [688, 12], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/generated/prost.rs:688`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb0821f44004cbe9155bfa6a"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> NullHandling
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::unnest_options::NullHandling", "path": "NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [689, 9], "end": [689, 29], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:689`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8b4288b3d78c2605023fd45"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::unnest_options::NullHandling", "path": "unnest_options::NullHandling"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [27615, 1], "end": [27674, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:27617`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4b6b74443ea883fe1cc8cbb"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &NullHandling) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::unnest_options::NullHandling", "path": "NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [684, 9], "end": [684, 18], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:684`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40b838f8a8a7fa2e3963fd89"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::unnest_options::NullHandling", "path": "NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [683, 9], "end": [683, 14], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:683`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b15b01b860b25e06e2a144c"></a>
## from_i32

`function` · `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling::from_i32` · datafusion-proto-models 55.1.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<NullHandling>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::unnest_options::NullHandling", "path": "NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [689, 9], "end": [689, 29], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:689`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Converts an `i32` to a `NullHandling`, or `None` if `value` is not a valid variant.

<a id="op-71e8009593a6a98cb8c8bfbc"></a>
## from_str_name

`function` · `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling::from_str_name` · datafusion-proto-models 55.1.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::unnest_options::NullHandling", "path": "NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [701, 5], "end": [722, 6], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:714`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-24ebc26b7728fdb606d8eb41"></a>
## hash

`function` · `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling::hash` · datafusion-proto-models 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::unnest_options::NullHandling", "path": "NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [686, 9], "end": [686, 13], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/prost.rs:686`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6596d0c1a80e256cb73e19b1"></a>
## is_valid

`function` · `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling::is_valid` · datafusion-proto-models 55.1.0

```rust
const fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::unnest_options::NullHandling", "path": "NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [689, 9], "end": [689, 29], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:689`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns `true` if `value` is a variant of `NullHandling`.

<a id="op-73ec98a143c035d082c9aece"></a>
## partial_cmp

`function` · `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling::partial_cmp` · datafusion-proto-models 55.1.0

```rust
fn partial_cmp(&self, other: &NullHandling) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::unnest_options::NullHandling", "path": "NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [687, 9], "end": [687, 19], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/generated/prost.rs:687`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e84980dba1b626af2411523e"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::unnest_options::NullHandling", "path": "unnest_options::NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27601, 1], "end": [27614, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:27603`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4960f011b10f3e7cfb85ca97"></a>
## try_from

`function` · `datafusion_proto_models::generated::datafusion::unnest_options::NullHandling::try_from` · datafusion-proto-models 55.1.0

```rust
fn try_from(value: i32) -> ::core::result::Result<NullHandling, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::unnest_options::NullHandling", "path": "NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [689, 9], "end": [689, 29], "filename": "src/generated/prost.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/prost.rs:689`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
