# `datafusion_proto::physical_plan::DefaultPhysicalExtensionCodec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.DefaultPhysicalExtensionCodec.json).

<a id="op-095c39fe6e075c5887e391a2"></a>
## DefaultPhysicalExtensionCodec

`struct` · `datafusion_proto::physical_plan::DefaultPhysicalExtensionCodec` · datafusion-proto 55.1.0

```rust
struct DefaultPhysicalExtensionCodec
```

Source: `src/physical_plan/mod.rs:1639`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8de0df9187ad44a1164e52e"></a>
## fmt

`function` · `datafusion_proto::physical_plan::DefaultPhysicalExtensionCodec::fmt` · datafusion-proto 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::DefaultPhysicalExtensionCodec", "path": "DefaultPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1638, 10], "end": [1638, 15], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/physical_plan/mod.rs:1638`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8142c08bfb8f96c4a2409468"></a>
## try_decode

`function` · `datafusion_proto::physical_plan::DefaultPhysicalExtensionCodec::try_decode` · datafusion-proto 55.1.0

```rust
fn try_decode(&self, _buf: &[u8], _inputs: &[Arc<dyn ExecutionPlan>], _ctx: &TaskContext, _proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::DefaultPhysicalExtensionCodec", "path": "DefaultPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1641, 1], "end": [1660, 2], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalExtensionCodec", "path": "PhysicalExtensionCodec"}, "trait_path": "datafusion_proto::physical_plan::PhysicalExtensionCodec"}`

Source: `src/physical_plan/mod.rs:1642`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-876906776f4ba4016a93fc8a"></a>
## try_encode

`function` · `datafusion_proto::physical_plan::DefaultPhysicalExtensionCodec::try_encode` · datafusion-proto 55.1.0

```rust
fn try_encode(&self, _node: Arc<dyn ExecutionPlan>, _buf: &mut Vec<u8>, _proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::DefaultPhysicalExtensionCodec", "path": "DefaultPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1641, 1], "end": [1660, 2], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalExtensionCodec", "path": "PhysicalExtensionCodec"}, "trait_path": "datafusion_proto::physical_plan::PhysicalExtensionCodec"}`

Source: `src/physical_plan/mod.rs:1652`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
