# `deltalake_core::delta_datafusion::DeltaLogicalCodec`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.DeltaLogicalCodec.json).

<a id="op-98417ce1449c047560039619"></a>
## DeltaLogicalCodec

`struct` · `deltalake_core::delta_datafusion::DeltaLogicalCodec` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaLogicalCodec
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L494).

Source: `crates/core/src/delta_datafusion/mod.rs:494`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Does serde on DeltaTables

<a id="op-a58651270e55d9738ae6e17a"></a>
## fmt

`function` · `deltalake_core::delta_datafusion::DeltaLogicalCodec::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L493).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::DeltaLogicalCodec", "path": "DeltaLogicalCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [493, 10], "end": [493, 15], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/delta_datafusion/mod.rs:493`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c17d8b8895d5455a4c1b9d7"></a>
## try_decode

`function` · `deltalake_core::delta_datafusion::DeltaLogicalCodec::try_decode` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_decode(&self, _buf: &[u8], _inputs: &[LogicalPlan], _ctx: &TaskContext) -> Result<Extension, DataFusionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L497).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::DeltaLogicalCodec", "path": "DeltaLogicalCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 1], "end": [534, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `crates/core/src/delta_datafusion/mod.rs:497`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a03145cb09ab3257b82965a"></a>
## try_decode_table_provider

`function` · `deltalake_core::delta_datafusion::DeltaLogicalCodec::try_decode_table_provider` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_decode_table_provider(&self, buf: &[u8], _table_ref: &TableReference, _schema: SchemaRef, _ctx: &TaskContext) -> Result<Arc<dyn TableProvider>, DataFusionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L510).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::DeltaLogicalCodec", "path": "DeltaLogicalCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 1], "end": [534, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `crates/core/src/delta_datafusion/mod.rs:510`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc2da3f6841b5625597f1d3e"></a>
## try_encode

`function` · `deltalake_core::delta_datafusion::DeltaLogicalCodec::try_encode` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_encode(&self, _node: &Extension, _buf: &mut Vec<u8>) -> Result<(), DataFusionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L506).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::DeltaLogicalCodec", "path": "DeltaLogicalCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 1], "end": [534, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `crates/core/src/delta_datafusion/mod.rs:506`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6588467e3027b64421eccf62"></a>
## try_encode_table_provider

`function` · `deltalake_core::delta_datafusion::DeltaLogicalCodec::try_encode_table_provider` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_encode_table_provider(&self, _table_ref: &TableReference, node: Arc<dyn TableProvider>, buf: &mut Vec<u8>) -> Result<(), DataFusionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L522).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::DeltaLogicalCodec", "path": "DeltaLogicalCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 1], "end": [534, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `crates/core/src/delta_datafusion/mod.rs:522`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
