# `datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.proto.physical_extension_codec.ForeignPhysicalExtensionCodec.json).

<a id="op-41dc6692094fd91ec282e4a6"></a>
## ForeignPhysicalExtensionCodec

`struct` · `datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec` · datafusion-ffi 55.1.0

```rust
struct ForeignPhysicalExtensionCodec
```

Source: `src/proto/physical_extension_codec.rs:333`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must only use the stable functions provided in
FFI_PhysicalExtensionCodec to interact with the foreign table provider.

<a id="op-6af334934d37b88671cb77ea"></a>
## 0

`struct_field` · `datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec::0` · datafusion-ffi 55.1.0

```rust
0: FFI_PhysicalExtensionCodec
```

Source: `src/proto/physical_extension_codec.rs:333`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3868974a245536a0ea68eee"></a>
## fmt

`function` · `datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec", "path": "ForeignPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [332, 10], "end": [332, 15], "filename": "src/proto/physical_extension_codec.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/proto/physical_extension_codec.rs:332`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08546103cc78b9558d07671f"></a>
## try_decode

`function` · `datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec::try_decode` · datafusion-ffi 55.1.0

```rust
fn try_decode(&self, buf: &[u8], inputs: &[Arc<dyn ExecutionPlan>], _ctx: &TaskContext, _proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec", "path": "ForeignPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [442, 2], "filename": "src/proto/physical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalExtensionCodec", "path": "PhysicalExtensionCodec"}, "trait_path": "datafusion_proto::physical_plan::PhysicalExtensionCodec"}`

Source: `src/proto/physical_extension_codec.rs:355`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f73245ce15c1934756d8ac66"></a>
## try_decode_udaf

`function` · `datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec::try_decode_udaf` · datafusion-ffi 55.1.0

```rust
fn try_decode_udaf(&self, name: &str, buf: &[u8]) -> Result<Arc<AggregateUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec", "path": "ForeignPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [442, 2], "filename": "src/proto/physical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalExtensionCodec", "path": "PhysicalExtensionCodec"}, "trait_path": "datafusion_proto::physical_plan::PhysicalExtensionCodec"}`

Source: `src/proto/physical_extension_codec.rs:405`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f10f3296b494de62bb443e5"></a>
## try_decode_udf

`function` · `datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec::try_decode_udf` · datafusion-ffi 55.1.0

```rust
fn try_decode_udf(&self, name: &str, buf: &[u8]) -> Result<Arc<ScalarUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec", "path": "ForeignPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [442, 2], "filename": "src/proto/physical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalExtensionCodec", "path": "PhysicalExtensionCodec"}, "trait_path": "datafusion_proto::physical_plan::PhysicalExtensionCodec"}`

Source: `src/proto/physical_extension_codec.rs:387`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c80ccc0840c58d0d54905b5"></a>
## try_decode_udwf

`function` · `datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec::try_decode_udwf` · datafusion-ffi 55.1.0

```rust
fn try_decode_udwf(&self, name: &str, buf: &[u8]) -> Result<Arc<WindowUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec", "path": "ForeignPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [442, 2], "filename": "src/proto/physical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalExtensionCodec", "path": "PhysicalExtensionCodec"}, "trait_path": "datafusion_proto::physical_plan::PhysicalExtensionCodec"}`

Source: `src/proto/physical_extension_codec.rs:424`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4876fe77d6e2ecacc74dd179"></a>
## try_encode

`function` · `datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec::try_encode` · datafusion-ffi 55.1.0

```rust
fn try_encode(&self, node: Arc<dyn ExecutionPlan>, buf: &mut Vec<u8>, _proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec", "path": "ForeignPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [442, 2], "filename": "src/proto/physical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalExtensionCodec", "path": "PhysicalExtensionCodec"}, "trait_path": "datafusion_proto::physical_plan::PhysicalExtensionCodec"}`

Source: `src/proto/physical_extension_codec.rs:374`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff778cb21c0d7354ab3b998d"></a>
## try_encode_udaf

`function` · `datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec::try_encode_udaf` · datafusion-ffi 55.1.0

```rust
fn try_encode_udaf(&self, node: &AggregateUDF, buf: &mut Vec<u8>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec", "path": "ForeignPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [442, 2], "filename": "src/proto/physical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalExtensionCodec", "path": "PhysicalExtensionCodec"}, "trait_path": "datafusion_proto::physical_plan::PhysicalExtensionCodec"}`

Source: `src/proto/physical_extension_codec.rs:414`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a4710bdff87d58541a96703"></a>
## try_encode_udf

`function` · `datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec::try_encode_udf` · datafusion-ffi 55.1.0

```rust
fn try_encode_udf(&self, node: &ScalarUDF, buf: &mut Vec<u8>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec", "path": "ForeignPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [442, 2], "filename": "src/proto/physical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalExtensionCodec", "path": "PhysicalExtensionCodec"}, "trait_path": "datafusion_proto::physical_plan::PhysicalExtensionCodec"}`

Source: `src/proto/physical_extension_codec.rs:396`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db5744821c9aa8bb7aba8958"></a>
## try_encode_udwf

`function` · `datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec::try_encode_udwf` · datafusion-ffi 55.1.0

```rust
fn try_encode_udwf(&self, node: &WindowUDF, buf: &mut Vec<u8>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec", "path": "ForeignPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [442, 2], "filename": "src/proto/physical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalExtensionCodec", "path": "PhysicalExtensionCodec"}, "trait_path": "datafusion_proto::physical_plan::PhysicalExtensionCodec"}`

Source: `src/proto/physical_extension_codec.rs:433`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
