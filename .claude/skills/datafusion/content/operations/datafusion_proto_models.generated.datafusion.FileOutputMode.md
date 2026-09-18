# `datafusion_proto_models::generated::datafusion::FileOutputMode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.FileOutputMode.json).

<a id="op-602ee7431263ef3df3f59016"></a>
## FileOutputMode

`enum` · `datafusion_proto_models::generated::datafusion::FileOutputMode` · datafusion-proto-models 55.1.0

```rust
enum FileOutputMode
```

Source: `src/generated/prost.rs:2738`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Determines how file sink output paths are interpreted.

<a id="op-81e5f9cfdf2923d9b9fc9c3c"></a>
## Automatic

`variant` · `datafusion_proto_models::generated::datafusion::FileOutputMode::Automatic` · datafusion-proto-models 55.1.0

```rust
Automatic
```

Source: `src/generated/prost.rs:2740`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Infer output mode from the URL (extension/trailing `/` heuristic).

<a id="op-cb0cfddd3f39c93a32d5753a"></a>
## Directory

`variant` · `datafusion_proto_models::generated::datafusion::FileOutputMode::Directory` · datafusion-proto-models 55.1.0

```rust
Directory
```

Source: `src/generated/prost.rs:2744`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Write to a directory with generated filenames.

<a id="op-9782a86101681490f00841e3"></a>
## Error

`assoc_type` · `datafusion_proto_models::generated::datafusion::FileOutputMode::Error` · datafusion-proto-models 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2736, 68], "end": [2736, 88], "filename": "src/generated/prost.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/prost.rs:2736`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab4275193b6c3c09b6810691"></a>
## SingleFile

`variant` · `datafusion_proto_models::generated::datafusion::FileOutputMode::SingleFile` · datafusion-proto-models 55.1.0

```rust
SingleFile
```

Source: `src/generated/prost.rs:2742`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Write to a single file at the exact output path.

<a id="op-3fafc80c504c451a5ee0de2a"></a>
## as_str_name

`function` · `datafusion_proto_models::generated::datafusion::FileOutputMode::as_str_name` · datafusion-proto-models 55.1.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2746, 1], "end": [2767, 2], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2751`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-ce071c49414bb9e9648e6fad"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::FileOutputMode::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> FileOutputMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2736, 10], "end": [2736, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:2736`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2357dd2d3fc55052b7a47e23"></a>
## cmp

`function` · `datafusion_proto_models::generated::datafusion::FileOutputMode::cmp` · datafusion-proto-models 55.1.0

```rust
fn cmp(&self, other: &FileOutputMode) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2736, 63], "end": [2736, 66], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/generated/prost.rs:2736`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5108fc3d13c1075c1159bfa"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::FileOutputMode::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> FileOutputMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2736, 68], "end": [2736, 88], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:2736`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-608dd4f98e1d32f5a16d806a"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::FileOutputMode::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6802, 1], "end": [6861, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:6804`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68db43c8de6ea3e831971952"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::FileOutputMode::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &FileOutputMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2736, 30], "end": [2736, 39], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:2736`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f03fe032f3f49b11c319f351"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::FileOutputMode::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2736, 23], "end": [2736, 28], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:2736`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-758b3b731b27c436ddfc2195"></a>
## from_i32

`function` · `datafusion_proto_models::generated::datafusion::FileOutputMode::from_i32` · datafusion-proto-models 55.1.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<FileOutputMode>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2736, 68], "end": [2736, 88], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2736`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Converts an `i32` to a `FileOutputMode`, or `None` if `value` is not a valid variant.

<a id="op-b9da8a1ae562bc4820d6a3c0"></a>
## from_str_name

`function` · `datafusion_proto_models::generated::datafusion::FileOutputMode::from_str_name` · datafusion-proto-models 55.1.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2746, 1], "end": [2767, 2], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2759`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-c15fc618ca68b47496bfcc1a"></a>
## hash

`function` · `datafusion_proto_models::generated::datafusion::FileOutputMode::hash` · datafusion-proto-models 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2736, 45], "end": [2736, 49], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/prost.rs:2736`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d89e63bb44e61562c128231"></a>
## is_valid

`function` · `datafusion_proto_models::generated::datafusion::FileOutputMode::is_valid` · datafusion-proto-models 55.1.0

```rust
const fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2736, 68], "end": [2736, 88], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2736`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns `true` if `value` is a variant of `FileOutputMode`.

<a id="op-51c8462b030e5f7dee9fc839"></a>
## partial_cmp

`function` · `datafusion_proto_models::generated::datafusion::FileOutputMode::partial_cmp` · datafusion-proto-models 55.1.0

```rust
fn partial_cmp(&self, other: &FileOutputMode) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2736, 51], "end": [2736, 61], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/generated/prost.rs:2736`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cebc467ce3da5aaff4a800cf"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::FileOutputMode::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6788, 1], "end": [6801, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:6790`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1947878e24a9184a3ebbf87d"></a>
## try_from

`function` · `datafusion_proto_models::generated::datafusion::FileOutputMode::try_from` · datafusion-proto-models 55.1.0

```rust
fn try_from(value: i32) -> ::core::result::Result<FileOutputMode, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileOutputMode", "path": "FileOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2736, 68], "end": [2736, 88], "filename": "src/generated/prost.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/generated/prost.rs:2736`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
