# `datafusion_proto::logical_plan::file_formats::parquet`

Crate `datafusion-proto` · 1 public items · structured records in [`model/datafusion_proto.logical_plan.file_formats.parquet.json`](../model/datafusion_proto.logical_plan.file_formats.parquet.json)

## ParquetLogicalExtensionCodec

`struct` · `datafusion_proto::logical_plan::file_formats::parquet::ParquetLogicalExtensionCodec`

Also reachable as `datafusion_proto::logical_plan::file_formats::ParquetLogicalExtensionCodec`

```rust
struct ParquetLogicalExtensionCodec
```

**Implements**: `datafusion_proto::logical_plan::LogicalExtensionCodec`

**Derives**: Debug

**via `datafusion_proto::logical_plan::LogicalExtensionCodec`**

```rust
fn try_decode(&self, _buf: &[u8], _inputs: &[datafusion_expr::LogicalPlan], _ctx: &TaskContext) -> datafusion_common::Result<datafusion_expr::Extension>
fn try_decode_file_format(&self, buf: &[u8], _ctx: &TaskContext) -> datafusion_common::Result<Arc<dyn FileFormatFactory>>
fn try_decode_table_provider(&self, _buf: &[u8], _table_ref: &TableReference, _schema: arrow::datatypes::SchemaRef, _ctx: &TaskContext) -> datafusion_common::Result<Arc<dyn datafusion_catalog::TableProvider>>
fn try_encode(&self, _node: &datafusion_expr::Extension, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
fn try_encode_file_format(&self, buf: &mut Vec<u8>, node: Arc<dyn FileFormatFactory>) -> datafusion_common::Result<()>
fn try_encode_table_provider(&self, _table_ref: &TableReference, _node: Arc<dyn datafusion_catalog::TableProvider>, _buf: &mut Vec<u8>) -> datafusion_common::Result<()>
```

---
