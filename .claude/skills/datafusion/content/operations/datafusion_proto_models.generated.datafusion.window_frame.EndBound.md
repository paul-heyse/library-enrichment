# `datafusion_proto_models::generated::datafusion::window_frame::EndBound`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.window_frame.EndBound.json).

<a id="op-90deb73bdbbf20843d8edf0e"></a>
## EndBound

`enum` · `datafusion_proto_models::generated::datafusion::window_frame::EndBound` · datafusion-proto-models 55.1.0

```rust
enum EndBound
```

Source: `src/generated/prost.rs:1221`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

"optional" keyword is stable in protoc 3.15 but prost is still on 3.14 (see <https://github.com/tokio-rs/prost/issues/430> and <https://github.com/tokio-rs/prost/pull/455>)
this syntax is ugly but is binary compatible with the "optional" keyword (see <https://stackoverflow.com/questions/42622015/how-to-define-an-optional-field-in-protobuf-3>)

<a id="op-76b29a8f6a252f9e369d2f92"></a>
## Bound

`variant` · `datafusion_proto_models::generated::datafusion::window_frame::EndBound::Bound` · datafusion-proto-models 55.1.0

```rust
Bound
```

Source: `src/generated/prost.rs:1223`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc3cfd3284964dcfeeb61b28"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::window_frame::EndBound::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> EndBound
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_frame::EndBound", "path": "EndBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 14], "end": [1220, 19], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:1220`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41680dae38b2141bb046ee66"></a>
## encode

`function` · `datafusion_proto_models::generated::datafusion::window_frame::EndBound::encode` · datafusion-proto-models 55.1.0

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_frame::EndBound", "path": "EndBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 32], "end": [1220, 46], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:1220`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Encodes the message to a buffer.

<a id="op-f18ec092dc3973eaa2ee8827"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::window_frame::EndBound::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_frame::EndBound", "path": "EndBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 32], "end": [1220, 46], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:1220`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the encoded length of the message without a length delimiter.

<a id="op-8789adae986fa719821e9798"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::window_frame::EndBound::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &EndBound) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_frame::EndBound", "path": "EndBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 21], "end": [1220, 30], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:1220`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c054e1caf15fc73877bf5f9"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::window_frame::EndBound::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_frame::EndBound", "path": "EndBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 32], "end": [1220, 46], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:1220`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f048f4858218378c6f8e3f3f"></a>
## merge

`function` · `datafusion_proto_models::generated::datafusion::window_frame::EndBound::merge` · datafusion-proto-models 55.1.0

```rust
fn merge(field: &mut ::core::option::Option<EndBound>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::window_frame::EndBound", "path": "EndBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 32], "end": [1220, 46], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:1220`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Decodes an instance of the message from a buffer, and merges it into self.
