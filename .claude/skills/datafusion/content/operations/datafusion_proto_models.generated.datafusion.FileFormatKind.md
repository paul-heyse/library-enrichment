# `datafusion_proto_models::generated::datafusion::FileFormatKind`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.FileFormatKind.json).

<a id="op-8e58c1ac788582ba442782b0"></a>
## FileFormatKind

`enum` · `datafusion_proto_models::generated::datafusion::FileFormatKind` · datafusion-proto-models 55.1.0

```rust
enum FileFormatKind
```

Source: `src/generated/prost.rs:2589`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Identifies a built-in file format supported by DataFusion.
Used by DefaultLogicalExtensionCodec to serialize/deserialize
FileFormatFactory instances (e.g. in CopyTo plans).

<a id="op-c510d2e4ee3b04277718738f"></a>
## Arrow

`variant` · `datafusion_proto_models::generated::datafusion::FileFormatKind::Arrow` · datafusion-proto-models 55.1.0

```rust
Arrow
```

Source: `src/generated/prost.rs:2594`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac4587cfd3403a2507b613a1"></a>
## Avro

`variant` · `datafusion_proto_models::generated::datafusion::FileFormatKind::Avro` · datafusion-proto-models 55.1.0

```rust
Avro
```

Source: `src/generated/prost.rs:2595`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-588b35f31570b69b093dd1a2"></a>
## Csv

`variant` · `datafusion_proto_models::generated::datafusion::FileFormatKind::Csv` · datafusion-proto-models 55.1.0

```rust
Csv
```

Source: `src/generated/prost.rs:2591`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0dadd1939cee261382890909"></a>
## Error

`assoc_type` · `datafusion_proto_models::generated::datafusion::FileFormatKind::Error` · datafusion-proto-models 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatKind", "path": "FileFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2587, 68], "end": [2587, 88], "filename": "src/generated/prost.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/prost.rs:2587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29db583f0481f66d059f40ce"></a>
## Json

`variant` · `datafusion_proto_models::generated::datafusion::FileFormatKind::Json` · datafusion-proto-models 55.1.0

```rust
Json
```

Source: `src/generated/prost.rs:2592`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86365ce0ee2bbd9410c562bd"></a>
## Parquet

`variant` · `datafusion_proto_models::generated::datafusion::FileFormatKind::Parquet` · datafusion-proto-models 55.1.0

```rust
Parquet
```

Source: `src/generated/prost.rs:2593`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3d2806260427a40755947ae"></a>
## Unspecified

`variant` · `datafusion_proto_models::generated::datafusion::FileFormatKind::Unspecified` · datafusion-proto-models 55.1.0

```rust
Unspecified
```

Source: `src/generated/prost.rs:2590`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9651f92170c7c8220a12435a"></a>
## as_str_name

`function` · `datafusion_proto_models::generated::datafusion::FileFormatKind::as_str_name` · datafusion-proto-models 55.1.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatKind", "path": "FileFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2597, 1], "end": [2624, 2], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2602`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-16211f2b67e3fad7feae53b2"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::FileFormatKind::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> FileFormatKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatKind", "path": "FileFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2587, 10], "end": [2587, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:2587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcfe3a8802d93ed402686295"></a>
## cmp

`function` · `datafusion_proto_models::generated::datafusion::FileFormatKind::cmp` · datafusion-proto-models 55.1.0

```rust
fn cmp(&self, other: &FileFormatKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatKind", "path": "FileFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2587, 63], "end": [2587, 66], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/generated/prost.rs:2587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1eb7f44b8bb15457fac70b0b"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::FileFormatKind::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> FileFormatKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatKind", "path": "FileFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2587, 68], "end": [2587, 88], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:2587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aede73dd07e9130dcd2c3881"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::FileFormatKind::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatKind", "path": "FileFormatKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6516, 1], "end": [6581, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:6518`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-edc62aef54b4ffaf48819283"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::FileFormatKind::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &FileFormatKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatKind", "path": "FileFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2587, 30], "end": [2587, 39], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:2587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e03232a71148b9928d240e8a"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::FileFormatKind::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatKind", "path": "FileFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2587, 23], "end": [2587, 28], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:2587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62c467112e11867903c29c56"></a>
## from_i32

`function` · `datafusion_proto_models::generated::datafusion::FileFormatKind::from_i32` · datafusion-proto-models 55.1.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<FileFormatKind>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatKind", "path": "FileFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2587, 68], "end": [2587, 88], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Converts an `i32` to a `FileFormatKind`, or `None` if `value` is not a valid variant.

<a id="op-4c68d0fe12b8889363908b68"></a>
## from_str_name

`function` · `datafusion_proto_models::generated::datafusion::FileFormatKind::from_str_name` · datafusion-proto-models 55.1.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatKind", "path": "FileFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2597, 1], "end": [2624, 2], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2613`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-d9bf1c262a4dbed6932d080a"></a>
## hash

`function` · `datafusion_proto_models::generated::datafusion::FileFormatKind::hash` · datafusion-proto-models 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatKind", "path": "FileFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2587, 45], "end": [2587, 49], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/prost.rs:2587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64a1e450610bf30915a74012"></a>
## is_valid

`function` · `datafusion_proto_models::generated::datafusion::FileFormatKind::is_valid` · datafusion-proto-models 55.1.0

```rust
const fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatKind", "path": "FileFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2587, 68], "end": [2587, 88], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns `true` if `value` is a variant of `FileFormatKind`.

<a id="op-d73ffe40c80e2040adb0a169"></a>
## partial_cmp

`function` · `datafusion_proto_models::generated::datafusion::FileFormatKind::partial_cmp` · datafusion-proto-models 55.1.0

```rust
fn partial_cmp(&self, other: &FileFormatKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatKind", "path": "FileFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2587, 51], "end": [2587, 61], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/generated/prost.rs:2587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff253c9a6a0ab12174e4a7d7"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::FileFormatKind::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatKind", "path": "FileFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6499, 1], "end": [6515, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:6501`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d61d86c8cabc41530f41a84"></a>
## try_from

`function` · `datafusion_proto_models::generated::datafusion::FileFormatKind::try_from` · datafusion-proto-models 55.1.0

```rust
fn try_from(value: i32) -> ::core::result::Result<FileFormatKind, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileFormatKind", "path": "FileFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2587, 68], "end": [2587, 88], "filename": "src/generated/prost.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/prost.rs:2587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
