# `datafusion_proto_common::from_proto::Error`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.from_proto.Error.json).

<a id="op-90d80df533903b4dae58be63"></a>
## Error

`enum` · `datafusion_proto_common::from_proto::Error` · datafusion-proto-common 55.1.0

```rust
enum Error
```

Source: `src/from_proto/mod.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a29aa8642909ac6c1499c3d"></a>
## AtLeastOneValue

`variant` · `datafusion_proto_common::from_proto::Error::AtLeastOneValue` · datafusion-proto-common 55.1.0

```rust
AtLeastOneValue
```

Source: `src/from_proto/mod.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7085119639ff0c44f36e7bef"></a>
## DataFusionError

`variant` · `datafusion_proto_common::from_proto::Error::DataFusionError` · datafusion-proto-common 55.1.0

```rust
DataFusionError
```

Source: `src/from_proto/mod.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d562eb3ad546cdabf0e2ca0"></a>
## General

`variant` · `datafusion_proto_common::from_proto::Error::General` · datafusion-proto-common 55.1.0

```rust
General
```

Source: `src/from_proto/mod.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-703c1ac65ab45a6131d5bf4b"></a>
## MissingRequiredField

`variant` · `datafusion_proto_common::from_proto::Error::MissingRequiredField` · datafusion-proto-common 55.1.0

```rust
MissingRequiredField
```

Source: `src/from_proto/mod.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2a3089b48673d35710b56d4"></a>
## UnknownEnumVariant

`variant` · `datafusion_proto_common::from_proto::Error::UnknownEnumVariant` · datafusion-proto-common 55.1.0

```rust
UnknownEnumVariant
```

Source: `src/from_proto/mod.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a86748227aeb770cb98b811"></a>
## fmt

`function` · `datafusion_proto_common::from_proto::Error::fmt` · datafusion-proto-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::from_proto::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 10], "end": [51, 15], "filename": "src/from_proto/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/from_proto/mod.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b46624be5fb1d782ed9fa103"></a>
## fmt

`function` · `datafusion_proto_common::from_proto::Error::fmt` · datafusion-proto-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::from_proto::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [84, 2], "filename": "src/from_proto/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/from_proto/mod.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa7b67dd857d1f4a95d6559f"></a>
## from

`function` · `datafusion_proto_common::from_proto::Error::from` · datafusion-proto-common 55.1.0

```rust
fn from(e: DataFusionError) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::from_proto::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [92, 2], "filename": "src/from_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/from_proto/mod.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3dcd6a9893d2cb2ac0f5681"></a>
## required

`function` · `datafusion_proto_common::from_proto::Error::required` · datafusion-proto-common 55.1.0

```rust
fn required(field: impl Into<String>) -> Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::from_proto::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [105, 2], "filename": "src/from_proto/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/from_proto/mod.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0b0c215b21615f6bed6d7a4"></a>
## unknown

`function` · `datafusion_proto_common::from_proto::Error::unknown` · datafusion-proto-common 55.1.0

```rust
fn unknown(name: impl Into<String>, value: i32) -> Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::from_proto::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [105, 2], "filename": "src/from_proto/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/from_proto/mod.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
