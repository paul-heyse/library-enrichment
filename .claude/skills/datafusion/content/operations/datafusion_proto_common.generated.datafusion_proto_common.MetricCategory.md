# `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.generated.datafusion_proto_common.MetricCategory.json).

<a id="op-d0ec86b86ff3529207ae70cf"></a>
## MetricCategory

`enum` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory` · datafusion-proto-common 55.1.0

```rust
enum MetricCategory
```

Source: `src/generated/prost.rs:1421`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Category of an `EXPLAIN ANALYZE` metric. Mirrors
`datafusion_common::format::MetricCategory`.

<a id="op-c73939aff6c9a1e8138469cd"></a>
## Bytes

`variant` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory::Bytes` · datafusion-proto-common 55.1.0

```rust
Bytes
```

Source: `src/generated/prost.rs:1423`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8cad92c2114b8d6e4cf58c4a"></a>
## Error

`assoc_type` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory::Error` · datafusion-proto-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 68], "end": [1419, 88], "filename": "src/generated/prost.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/prost.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46f7b8f13feb194c104809b0"></a>
## Rows

`variant` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory::Rows` · datafusion-proto-common 55.1.0

```rust
Rows
```

Source: `src/generated/prost.rs:1422`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5753c599237bd8ef04d82bb7"></a>
## Timing

`variant` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory::Timing` · datafusion-proto-common 55.1.0

```rust
Timing
```

Source: `src/generated/prost.rs:1424`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46dbda911ca98a7e9eb02329"></a>
## Uncategorized

`variant` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory::Uncategorized` · datafusion-proto-common 55.1.0

```rust
Uncategorized
```

Source: `src/generated/prost.rs:1425`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d3057831f68322c7056fc74"></a>
## as_str_name

`function` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory::as_str_name` · datafusion-proto-common 55.1.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1427, 1], "end": [1450, 2], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:1432`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-7aee96b4f1296afc1e702eec"></a>
## clone

`function` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory::clone` · datafusion-proto-common 55.1.0

```rust
fn clone(&self) -> MetricCategory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 10], "end": [1419, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09051f49485c78f7c45c696f"></a>
## cmp

`function` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory::cmp` · datafusion-proto-common 55.1.0

```rust
fn cmp(&self, other: &MetricCategory) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 63], "end": [1419, 66], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/generated/prost.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cca025cbbf3e1ac46b7a6216"></a>
## default

`function` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory::default` · datafusion-proto-common 55.1.0

```rust
fn default() -> MetricCategory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 68], "end": [1419, 88], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32fd5e461cf4235385f613e3"></a>
## deserialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory::deserialize` · datafusion-proto-common 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5466, 1], "end": [5527, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:5468`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eddf25b95ac11a3aea03eb88"></a>
## eq

`function` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory::eq` · datafusion-proto-common 55.1.0

```rust
fn eq(&self, other: &MetricCategory) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 30], "end": [1419, 39], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd0f9e58fb219b629fcba216"></a>
## fmt

`function` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory::fmt` · datafusion-proto-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 23], "end": [1419, 28], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-662831adb71fb6c5c751a1bb"></a>
## from_i32

`function` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory::from_i32` · datafusion-proto-common 55.1.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<MetricCategory>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 68], "end": [1419, 88], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Converts an `i32` to a `MetricCategory`, or `None` if `value` is not a valid variant.

<a id="op-cc9d12d3731d7d21aba7453e"></a>
## from_str_name

`function` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory::from_str_name` · datafusion-proto-common 55.1.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1427, 1], "end": [1450, 2], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:1441`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-e6a709e1d861e33a3276d50b"></a>
## hash

`function` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory::hash` · datafusion-proto-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 45], "end": [1419, 49], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/prost.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c91523491b2c5bf223ae7498"></a>
## is_valid

`function` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory::is_valid` · datafusion-proto-common 55.1.0

```rust
const fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 68], "end": [1419, 88], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Returns `true` if `value` is a variant of `MetricCategory`.

<a id="op-dc373106aaf883e4495681ea"></a>
## partial_cmp

`function` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory::partial_cmp` · datafusion-proto-common 55.1.0

```rust
fn partial_cmp(&self, other: &MetricCategory) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 51], "end": [1419, 61], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/generated/prost.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-176152702e70c05f8dc6cc52"></a>
## serialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory::serialize` · datafusion-proto-common 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5451, 1], "end": [5465, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:5453`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21cda7d215f7edbaa270747e"></a>
## try_from

`function` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory::try_from` · datafusion-proto-common 55.1.0

```rust
fn try_from(value: i32) -> ::core::result::Result<MetricCategory, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 68], "end": [1419, 88], "filename": "src/generated/prost.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/prost.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
