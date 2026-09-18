# `datafusion_proto_models::generated::datafusion::WindowFrame`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.WindowFrame.json).

<a id="op-69ebeaddb9cf38462c9e5de0"></a>
## WindowFrame

`struct` · `datafusion_proto_models::generated::datafusion::WindowFrame` · datafusion-proto-models 55.1.0

```rust
struct WindowFrame
```

Source: `src/generated/prost.rs:1206`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f5fde5904c606557e013a92"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::WindowFrame::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1205, 28], "end": [1205, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:1205`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3cdff2106a505fc60b2d3522"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::WindowFrame::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> WindowFrame
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1205, 10], "end": [1205, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:1205`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1b68826e0054bdd18e75ba1"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::WindowFrame::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1205, 28], "end": [1205, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:1205`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d16112b8b27083d22995cdcf"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::WindowFrame::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [28616, 1], "end": [28713, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:28618`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61a2cf85e3a3335186be1733"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::WindowFrame::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1205, 28], "end": [1205, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:1205`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6de7773861dff74123344486"></a>
## end_bound

`struct_field` · `datafusion_proto_models::generated::datafusion::WindowFrame::end_bound` · datafusion-proto-models 55.1.0

```rust
end_bound: ::core::option::Option<window_frame::EndBound>
```

Source: `src/generated/prost.rs:1214`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

"optional" keyword is stable in protoc 3.15 but prost is still on 3.14 (see <https://github.com/tokio-rs/prost/issues/430> and <https://github.com/tokio-rs/prost/pull/455>)
this syntax is ugly but is binary compatible with the "optional" keyword (see <https://stackoverflow.com/questions/42622015/how-to-define-an-optional-field-in-protobuf-3>)

<a id="op-5a6b70e539caac5adb2a1b41"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::WindowFrame::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &WindowFrame) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1205, 17], "end": [1205, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:1205`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fabf16215f5f7d6671025a7"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::WindowFrame::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1205, 28], "end": [1205, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:1205`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4d7ece07e60286d870bb222"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::WindowFrame::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [28580, 1], "end": [28615, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:28582`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f07f8ca812cfb199821de4a"></a>
## set_window_frame_units

`function` · `datafusion_proto_models::generated::datafusion::WindowFrame::set_window_frame_units` · datafusion-proto-models 55.1.0

```rust
fn set_window_frame_units(&mut self, value: WindowFrameUnits)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1205, 28], "end": [1205, 44], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:1205`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Sets `window_frame_units` to the provided enum value.

<a id="op-6689f1bbde9c1c09115f0d49"></a>
## start_bound

`struct_field` · `datafusion_proto_models::generated::datafusion::WindowFrame::start_bound` · datafusion-proto-models 55.1.0

```rust
start_bound: ::core::option::Option<WindowFrameBound>
```

Source: `src/generated/prost.rs:1210`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6cd616ee2cc4d9b09c739f8"></a>
## window_frame_units

`struct_field` · `datafusion_proto_models::generated::datafusion::WindowFrame::window_frame_units` · datafusion-proto-models 55.1.0

```rust
window_frame_units: i32
```

Source: `src/generated/prost.rs:1208`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4cb2a7f04d35a1cc9a3ce38"></a>
## window_frame_units

`function` · `datafusion_proto_models::generated::datafusion::WindowFrame::window_frame_units` · datafusion-proto-models 55.1.0

```rust
fn window_frame_units(&self) -> WindowFrameUnits
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1205, 28], "end": [1205, 44], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:1205`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the enum value of `window_frame_units`, or the default if the field is set to an invalid enum value.
