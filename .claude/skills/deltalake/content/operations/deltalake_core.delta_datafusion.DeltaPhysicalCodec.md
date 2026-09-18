# `deltalake_core::delta_datafusion::DeltaPhysicalCodec`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.DeltaPhysicalCodec.json).

<a id="op-8c64c247aa064bff7024d965"></a>
## DeltaPhysicalCodec

`struct` · `deltalake_core::delta_datafusion::DeltaPhysicalCodec` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaPhysicalCodec
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L457).

Source: `crates/core/src/delta_datafusion/mod.rs:457`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Legacy codec for serialized plans that still contain the retired physical
[`DeltaScan`] wrapper.

Unresolved upstream links (retained, not inferred): ``DeltaScan``.

<a id="op-220beab11946d14618d3c0b5"></a>
## fmt

`function` · `deltalake_core::delta_datafusion::DeltaPhysicalCodec::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L456).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::DeltaPhysicalCodec", "path": "DeltaPhysicalCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 10], "end": [456, 15], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/delta_datafusion/mod.rs:456`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b98db15699f600f4afbba68"></a>
## try_decode

`function` · `deltalake_core::delta_datafusion::DeltaPhysicalCodec::try_decode` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_decode(&self, buf: &[u8], inputs: &[Arc<dyn ExecutionPlan>], _registry: &TaskContext, _converter: &dyn PhysicalProtoConverterExtension) -> Result<Arc<dyn ExecutionPlan>, DataFusionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L461).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::DeltaPhysicalCodec", "path": "DeltaPhysicalCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [460, 1], "end": [490, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalExtensionCodec", "path": "PhysicalExtensionCodec"}, "trait_path": "datafusion_proto::physical_plan::PhysicalExtensionCodec"}`

Source: `crates/core/src/delta_datafusion/mod.rs:461`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b26fcbe1955eb28c9879464"></a>
## try_encode

`function` · `deltalake_core::delta_datafusion::DeltaPhysicalCodec::try_encode` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_encode(&self, node: Arc<dyn ExecutionPlan>, buf: &mut Vec<u8>, _converter: &dyn PhysicalProtoConverterExtension) -> Result<(), DataFusionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L474).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::DeltaPhysicalCodec", "path": "DeltaPhysicalCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [460, 1], "end": [490, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalExtensionCodec", "path": "PhysicalExtensionCodec"}, "trait_path": "datafusion_proto::physical_plan::PhysicalExtensionCodec"}`

Source: `crates/core/src/delta_datafusion/mod.rs:474`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
