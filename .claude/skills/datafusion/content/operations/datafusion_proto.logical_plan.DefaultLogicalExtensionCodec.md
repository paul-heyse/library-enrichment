# `datafusion_proto::logical_plan::DefaultLogicalExtensionCodec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.logical_plan.DefaultLogicalExtensionCodec.json).

<a id="op-a8dbf54815adaea27993667f"></a>
## DefaultLogicalExtensionCodec

`struct` · `datafusion_proto::logical_plan::DefaultLogicalExtensionCodec` · datafusion-proto 55.1.0

```rust
struct DefaultLogicalExtensionCodec
```

Source: `src/logical_plan/mod.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd5c4063875ee164cf2e03a5"></a>
## clone

`function` · `datafusion_proto::logical_plan::DefaultLogicalExtensionCodec::clone` · datafusion-proto 55.1.0

```rust
fn clone(&self) -> DefaultLogicalExtensionCodec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::DefaultLogicalExtensionCodec", "path": "DefaultLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [226, 17], "end": [226, 22], "filename": "src/logical_plan/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/mod.rs:226`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd724752767d2d4eda2a6637"></a>
## fmt

`function` · `datafusion_proto::logical_plan::DefaultLogicalExtensionCodec::fmt` · datafusion-proto 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::DefaultLogicalExtensionCodec", "path": "DefaultLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [226, 10], "end": [226, 15], "filename": "src/logical_plan/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/mod.rs:226`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94b34223fc3a376987b6a92e"></a>
## try_decode

`function` · `datafusion_proto::logical_plan::DefaultLogicalExtensionCodec::try_decode` · datafusion-proto 55.1.0

```rust
fn try_decode(&self, _buf: &[u8], _inputs: &[LogicalPlan], _ctx: &TaskContext) -> Result<Extension>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::DefaultLogicalExtensionCodec", "path": "DefaultLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [350, 2], "filename": "src/logical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/mod.rs:230`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68413f64ba5f73e9f8543a72"></a>
## try_decode_file_format

`function` · `datafusion_proto::logical_plan::DefaultLogicalExtensionCodec::try_decode_file_format` · datafusion-proto 55.1.0

```rust
fn try_decode_file_format(&self, buf: &[u8], ctx: &TaskContext) -> Result<Arc<dyn FileFormatFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::DefaultLogicalExtensionCodec", "path": "DefaultLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [350, 2], "filename": "src/logical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/mod.rs:262`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bcf2ad32919c58a0158cb832"></a>
## try_decode_table_provider

`function` · `datafusion_proto::logical_plan::DefaultLogicalExtensionCodec::try_decode_table_provider` · datafusion-proto 55.1.0

```rust
fn try_decode_table_provider(&self, _buf: &[u8], _table_ref: &TableReference, _schema: SchemaRef, _ctx: &TaskContext) -> Result<Arc<dyn TableProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::DefaultLogicalExtensionCodec", "path": "DefaultLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [350, 2], "filename": "src/logical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/mod.rs:243`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff685310268a547ef81b8251"></a>
## try_encode

`function` · `datafusion_proto::logical_plan::DefaultLogicalExtensionCodec::try_encode` · datafusion-proto 55.1.0

```rust
fn try_encode(&self, _node: &Extension, _buf: &mut Vec<u8>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::DefaultLogicalExtensionCodec", "path": "DefaultLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [350, 2], "filename": "src/logical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/mod.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5ebf224280da2a178a8f3a1"></a>
## try_encode_file_format

`function` · `datafusion_proto::logical_plan::DefaultLogicalExtensionCodec::try_encode_file_format` · datafusion-proto 55.1.0

```rust
fn try_encode_file_format(&self, buf: &mut Vec<u8>, node: Arc<dyn FileFormatFactory>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::DefaultLogicalExtensionCodec", "path": "DefaultLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [350, 2], "filename": "src/logical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/mod.rs:299`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5c1aea07ab51171747eb226"></a>
## try_encode_table_provider

`function` · `datafusion_proto::logical_plan::DefaultLogicalExtensionCodec::try_encode_table_provider` · datafusion-proto 55.1.0

```rust
fn try_encode_table_provider(&self, _table_ref: &TableReference, _node: Arc<dyn TableProvider>, _buf: &mut Vec<u8>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::DefaultLogicalExtensionCodec", "path": "DefaultLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [350, 2], "filename": "src/logical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/mod.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
