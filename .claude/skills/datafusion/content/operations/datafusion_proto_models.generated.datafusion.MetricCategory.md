# `datafusion_proto_models::generated::datafusion::MetricCategory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.MetricCategory.json).

<a id="op-76755915bc35706bd5f39345"></a>
## MetricCategory

`enum` · `datafusion_proto_models::generated::datafusion::MetricCategory` · datafusion-proto-models 55.1.0

```rust
enum MetricCategory
```

Source: `src/generated/datafusion_proto_common.rs:1421`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Category of an `EXPLAIN ANALYZE` metric. Mirrors
`datafusion_common::format::MetricCategory`.

<a id="op-b63f0fad155da2be339f1054"></a>
## Bytes

`variant` · `datafusion_proto_models::generated::datafusion::MetricCategory::Bytes` · datafusion-proto-models 55.1.0

```rust
Bytes
```

Source: `src/generated/datafusion_proto_common.rs:1423`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-520425c49fde85e0883228b6"></a>
## Error

`assoc_type` · `datafusion_proto_models::generated::datafusion::MetricCategory::Error` · datafusion-proto-models 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 68], "end": [1419, 88], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/datafusion_proto_common.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99e115813196eaea817dcfe9"></a>
## Rows

`variant` · `datafusion_proto_models::generated::datafusion::MetricCategory::Rows` · datafusion-proto-models 55.1.0

```rust
Rows
```

Source: `src/generated/datafusion_proto_common.rs:1422`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2ff54df6b465c5900211829"></a>
## Timing

`variant` · `datafusion_proto_models::generated::datafusion::MetricCategory::Timing` · datafusion-proto-models 55.1.0

```rust
Timing
```

Source: `src/generated/datafusion_proto_common.rs:1424`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-886184bb9398cfdfe14a7379"></a>
## Uncategorized

`variant` · `datafusion_proto_models::generated::datafusion::MetricCategory::Uncategorized` · datafusion-proto-models 55.1.0

```rust
Uncategorized
```

Source: `src/generated/datafusion_proto_common.rs:1425`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af6fa795e2a707531d284366"></a>
## as_str_name

`function` · `datafusion_proto_models::generated::datafusion::MetricCategory::as_str_name` · datafusion-proto-models 55.1.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1427, 1], "end": [1450, 2], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:1432`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-ec222760daad4240b7515a90"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::MetricCategory::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> MetricCategory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 10], "end": [1419, 15], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/datafusion_proto_common.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4db1a36d23afb543f2ae4440"></a>
## cmp

`function` · `datafusion_proto_models::generated::datafusion::MetricCategory::cmp` · datafusion-proto-models 55.1.0

```rust
fn cmp(&self, other: &MetricCategory) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 63], "end": [1419, 66], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/generated/datafusion_proto_common.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e9e20ffe1f3bc7037c74de3"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::MetricCategory::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> MetricCategory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 68], "end": [1419, 88], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/datafusion_proto_common.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c032671da2b1fac39dea4772"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::MetricCategory::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &MetricCategory) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 30], "end": [1419, 39], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/datafusion_proto_common.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f81215b0346c840b2ee6576"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::MetricCategory::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 23], "end": [1419, 28], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/datafusion_proto_common.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f1724b1b60b9fef57594f25"></a>
## from_i32

`function` · `datafusion_proto_models::generated::datafusion::MetricCategory::from_i32` · datafusion-proto-models 55.1.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<MetricCategory>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 68], "end": [1419, 88], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Converts an `i32` to a `MetricCategory`, or `None` if `value` is not a valid variant.

<a id="op-054f063abc0003edad620338"></a>
## from_str_name

`function` · `datafusion_proto_models::generated::datafusion::MetricCategory::from_str_name` · datafusion-proto-models 55.1.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1427, 1], "end": [1450, 2], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:1441`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-803b45239d4eb0126690b549"></a>
## hash

`function` · `datafusion_proto_models::generated::datafusion::MetricCategory::hash` · datafusion-proto-models 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 45], "end": [1419, 49], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/datafusion_proto_common.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd8a674be6364703ee6ed295"></a>
## is_valid

`function` · `datafusion_proto_models::generated::datafusion::MetricCategory::is_valid` · datafusion-proto-models 55.1.0

```rust
const fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 68], "end": [1419, 88], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns `true` if `value` is a variant of `MetricCategory`.

<a id="op-ba5fb176f26bde3a03df569f"></a>
## partial_cmp

`function` · `datafusion_proto_models::generated::datafusion::MetricCategory::partial_cmp` · datafusion-proto-models 55.1.0

```rust
fn partial_cmp(&self, other: &MetricCategory) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 51], "end": [1419, 61], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/generated/datafusion_proto_common.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44f844667556ef14c74c65c3"></a>
## try_from

`function` · `datafusion_proto_models::generated::datafusion::MetricCategory::try_from` · datafusion-proto-models 55.1.0

```rust
fn try_from(value: i32) -> ::core::result::Result<MetricCategory, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 68], "end": [1419, 88], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/datafusion_proto_common.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
