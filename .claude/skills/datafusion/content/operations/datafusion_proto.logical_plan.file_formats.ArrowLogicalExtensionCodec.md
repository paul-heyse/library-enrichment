# `datafusion_proto::logical_plan::file_formats::ArrowLogicalExtensionCodec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.logical_plan.file_formats.ArrowLogicalExtensionCodec.json).

<a id="op-082fe448e9144f45f9299ba4"></a>
## ArrowLogicalExtensionCodec

`struct` · `datafusion_proto::logical_plan::file_formats::ArrowLogicalExtensionCodec` · datafusion-proto 55.1.0

```rust
struct ArrowLogicalExtensionCodec
```

Source: `src/logical_plan/file_formats.rs:343`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00f9c1da61fce0176d489a4c"></a>
## fmt

`function` · `datafusion_proto::logical_plan::file_formats::ArrowLogicalExtensionCodec::fmt` · datafusion-proto 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::ArrowLogicalExtensionCodec", "path": "ArrowLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [342, 10], "end": [342, 15], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/file_formats.rs:342`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d15d8f60ed2e9ddf865402c2"></a>
## try_decode

`function` · `datafusion_proto::logical_plan::file_formats::ArrowLogicalExtensionCodec::try_decode` · datafusion-proto 55.1.0

```rust
fn try_decode(&self, _buf: &[u8], _inputs: &[datafusion_expr::LogicalPlan], _ctx: &TaskContext) -> datafusion_common::Result<datafusion_expr::Extension>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::ArrowLogicalExtensionCodec", "path": "ArrowLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [398, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:347`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ae66cf984115fee9bd42438"></a>
## try_decode_file_format

`function` · `datafusion_proto::logical_plan::file_formats::ArrowLogicalExtensionCodec::try_decode_file_format` · datafusion-proto 55.1.0

```rust
fn try_decode_file_format(&self, __buf: &[u8], __ctx: &TaskContext) -> datafusion_common::Result<Arc<dyn FileFormatFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::ArrowLogicalExtensionCodec", "path": "ArrowLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [398, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:383`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-997983ac0e5a870b15647d51"></a>
## try_decode_table_provider

`function` · `datafusion_proto::logical_plan::file_formats::ArrowLogicalExtensionCodec::try_decode_table_provider` · datafusion-proto 55.1.0

```rust
fn try_decode_table_provider(&self, _buf: &[u8], _table_ref: &TableReference, _schema: arrow::datatypes::SchemaRef, _ctx: &TaskContext) -> datafusion_common::Result<Arc<dyn datafusion_catalog::TableProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::ArrowLogicalExtensionCodec", "path": "ArrowLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [398, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:364`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be5eb24c817778f51a08362b"></a>
## try_encode

`function` · `datafusion_proto::logical_plan::file_formats::ArrowLogicalExtensionCodec::try_encode` · datafusion-proto 55.1.0

```rust
fn try_encode(&self, _node: &datafusion_expr::Extension, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::ArrowLogicalExtensionCodec", "path": "ArrowLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [398, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:356`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ce06044f68ab01f75f06fa4"></a>
## try_encode_file_format

`function` · `datafusion_proto::logical_plan::file_formats::ArrowLogicalExtensionCodec::try_encode_file_format` · datafusion-proto 55.1.0

```rust
fn try_encode_file_format(&self, __buf: &mut Vec<u8>, __node: Arc<dyn FileFormatFactory>) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::ArrowLogicalExtensionCodec", "path": "ArrowLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [398, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:391`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9423f5866d5b44bdc53a0737"></a>
## try_encode_table_provider

`function` · `datafusion_proto::logical_plan::file_formats::ArrowLogicalExtensionCodec::try_encode_table_provider` · datafusion-proto 55.1.0

```rust
fn try_encode_table_provider(&self, _table_ref: &TableReference, _node: Arc<dyn datafusion_catalog::TableProvider>, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::ArrowLogicalExtensionCodec", "path": "ArrowLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [398, 2], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:374`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
