# `datafusion_proto::logical_plan::file_formats::parquet::ParquetLogicalExtensionCodec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.logical_plan.file_formats.parquet.ParquetLogicalExtensionCodec.json).

<a id="op-87d9fad301f4be1a4808e0ee"></a>
## ParquetLogicalExtensionCodec

`struct` · `datafusion_proto::logical_plan::file_formats::parquet::ParquetLogicalExtensionCodec` · datafusion-proto 55.1.0

```rust
struct ParquetLogicalExtensionCodec
```

Source: `src/logical_plan/file_formats.rs:197`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-423893a041db5ee55dcb709d"></a>
## fmt

`function` · `datafusion_proto::logical_plan::file_formats::parquet::ParquetLogicalExtensionCodec::fmt` · datafusion-proto 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::parquet::ParquetLogicalExtensionCodec", "path": "ParquetLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 14], "end": [196, 19], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/file_formats.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80f84a54aca779bea416149a"></a>
## try_decode

`function` · `datafusion_proto::logical_plan::file_formats::parquet::ParquetLogicalExtensionCodec::try_decode` · datafusion-proto 55.1.0

```rust
fn try_decode(&self, _buf: &[u8], _inputs: &[datafusion_expr::LogicalPlan], _ctx: &TaskContext) -> datafusion_common::Result<datafusion_expr::Extension>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::parquet::ParquetLogicalExtensionCodec", "path": "ParquetLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 5], "end": [277, 6], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f4419d2381895ba0af26db6"></a>
## try_decode_file_format

`function` · `datafusion_proto::logical_plan::file_formats::parquet::ParquetLogicalExtensionCodec::try_decode_file_format` · datafusion-proto 55.1.0

```rust
fn try_decode_file_format(&self, buf: &[u8], _ctx: &TaskContext) -> datafusion_common::Result<Arc<dyn FileFormatFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::parquet::ParquetLogicalExtensionCodec", "path": "ParquetLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 5], "end": [277, 6], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:238`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-136ad9d1ee74052e87c5e37d"></a>
## try_decode_table_provider

`function` · `datafusion_proto::logical_plan::file_formats::parquet::ParquetLogicalExtensionCodec::try_decode_table_provider` · datafusion-proto 55.1.0

```rust
fn try_decode_table_provider(&self, _buf: &[u8], _table_ref: &TableReference, _schema: arrow::datatypes::SchemaRef, _ctx: &TaskContext) -> datafusion_common::Result<Arc<dyn datafusion_catalog::TableProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::parquet::ParquetLogicalExtensionCodec", "path": "ParquetLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 5], "end": [277, 6], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f6e6675a805aa4ea95d2670"></a>
## try_encode

`function` · `datafusion_proto::logical_plan::file_formats::parquet::ParquetLogicalExtensionCodec::try_encode` · datafusion-proto 55.1.0

```rust
fn try_encode(&self, _node: &datafusion_expr::Extension, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::parquet::ParquetLogicalExtensionCodec", "path": "ParquetLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 5], "end": [277, 6], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca4e927a66b6c166b31933cf"></a>
## try_encode_file_format

`function` · `datafusion_proto::logical_plan::file_formats::parquet::ParquetLogicalExtensionCodec::try_encode_file_format` · datafusion-proto 55.1.0

```rust
fn try_encode_file_format(&self, buf: &mut Vec<u8>, node: Arc<dyn FileFormatFactory>) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::parquet::ParquetLogicalExtensionCodec", "path": "ParquetLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 5], "end": [277, 6], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed235d9b3738e449d0e573a6"></a>
## try_encode_table_provider

`function` · `datafusion_proto::logical_plan::file_formats::parquet::ParquetLogicalExtensionCodec::try_encode_table_provider` · datafusion-proto 55.1.0

```rust
fn try_encode_table_provider(&self, _table_ref: &TableReference, _node: Arc<dyn datafusion_catalog::TableProvider>, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::logical_plan::file_formats::parquet::ParquetLogicalExtensionCodec", "path": "ParquetLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 5], "end": [277, 6], "filename": "src/logical_plan/file_formats.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/logical_plan/file_formats.rs:229`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
