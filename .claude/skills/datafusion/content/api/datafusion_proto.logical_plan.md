# `datafusion_proto::logical_plan`

Crate `datafusion-proto` · 3 public items · structured records in [`model/datafusion_proto.logical_plan.json`](../model/datafusion_proto.logical_plan.json)

## DefaultLogicalExtensionCodec

`struct` · `datafusion_proto::logical_plan::DefaultLogicalExtensionCodec`

```rust
struct DefaultLogicalExtensionCodec
```

**Implements**: `datafusion_proto::logical_plan::LogicalExtensionCodec`

**Derives**: Clone, Debug

**via `datafusion_proto::logical_plan::LogicalExtensionCodec`**

```rust
fn try_decode(&self, _buf: &[u8], _inputs: &[LogicalPlan], _ctx: &TaskContext) -> Result<Extension>
fn try_decode_file_format(&self, buf: &[u8], ctx: &TaskContext) -> Result<Arc<dyn FileFormatFactory>>
fn try_decode_table_provider(&self, _buf: &[u8], _table_ref: &TableReference, _schema: SchemaRef, _ctx: &TaskContext) -> Result<Arc<dyn TableProvider>>
fn try_encode(&self, _node: &Extension, _buf: &mut Vec<u8>) -> Result<()>
fn try_encode_file_format(&self, buf: &mut Vec<u8>, node: Arc<dyn FileFormatFactory>) -> Result<()>
fn try_encode_table_provider(&self, _table_ref: &TableReference, _node: Arc<dyn TableProvider>, _buf: &mut Vec<u8>) -> Result<()>
```

---

## AsLogicalPlan

`trait` · `datafusion_proto::logical_plan::AsLogicalPlan`

```rust
trait AsLogicalPlan: Debug + Send + Sync + Clone
```

**Implementors** (1)

- `datafusion_proto_models::generated::datafusion::LogicalPlanNode`

**Methods** (4)

```rust
fn try_decode(buf: &[u8]) -> Result<Self> where Self: Sized
fn try_encode<B>(&self, buf: &mut B) -> Result<()> where B: BufMut, Self: Sized
fn try_from_logical_plan(plan: &LogicalPlan, extension_codec: &dyn LogicalExtensionCodec) -> Result<Self> where Self: Sized
fn try_into_logical_plan(&self, ctx: &TaskContext, extension_codec: &dyn LogicalExtensionCodec) -> Result<LogicalPlan>
```

---

## LogicalExtensionCodec

`trait` · `datafusion_proto::logical_plan::LogicalExtensionCodec`

```rust
trait LogicalExtensionCodec: Debug + Send + Sync + std::any::Any
```

**Implementors** (7)

- `datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec`
- `datafusion_proto::logical_plan::DefaultLogicalExtensionCodec`
- `datafusion_proto::logical_plan::file_formats::ArrowLogicalExtensionCodec`
- `datafusion_proto::logical_plan::file_formats::AvroLogicalExtensionCodec`
- `datafusion_proto::logical_plan::file_formats::CsvLogicalExtensionCodec`
- `datafusion_proto::logical_plan::file_formats::JsonLogicalExtensionCodec`
- `datafusion_proto::logical_plan::file_formats::parquet::ParquetLogicalExtensionCodec`

**Methods** (14)

```rust
fn try_decode(&self, buf: &[u8], inputs: &[LogicalPlan], ctx: &TaskContext) -> Result<Extension>
fn try_decode_file_format(&self, _buf: &[u8], _ctx: &TaskContext) -> Result<Arc<dyn FileFormatFactory>>
fn try_decode_higher_order_function(&self, name: &str, _buf: &[u8]) -> Result<Arc<HigherOrderUDF>>
fn try_decode_table_provider(&self, buf: &[u8], table_ref: &TableReference, schema: SchemaRef, ctx: &TaskContext) -> Result<Arc<dyn TableProvider>>
fn try_decode_udaf(&self, name: &str, _buf: &[u8]) -> Result<Arc<AggregateUDF>>
fn try_decode_udf(&self, name: &str, _buf: &[u8]) -> Result<Arc<ScalarUDF>>
fn try_decode_udwf(&self, name: &str, _buf: &[u8]) -> Result<Arc<WindowUDF>>
fn try_encode(&self, node: &Extension, buf: &mut Vec<u8>) -> Result<()>
fn try_encode_file_format(&self, _buf: &mut Vec<u8>, _node: Arc<dyn FileFormatFactory>) -> Result<()>
fn try_encode_higher_order_function(&self, _node: &HigherOrderUDF, _buf: &mut Vec<u8>) -> Result<()>
fn try_encode_table_provider(&self, table_ref: &TableReference, node: Arc<dyn TableProvider>, buf: &mut Vec<u8>) -> Result<()>
fn try_encode_udaf(&self, _node: &AggregateUDF, _buf: &mut Vec<u8>) -> Result<()>
fn try_encode_udf(&self, _node: &ScalarUDF, _buf: &mut Vec<u8>) -> Result<()>
fn try_encode_udwf(&self, _node: &WindowUDF, _buf: &mut Vec<u8>) -> Result<()>
```

---
