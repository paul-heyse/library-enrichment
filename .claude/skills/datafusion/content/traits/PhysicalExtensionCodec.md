# PhysicalExtensionCodec

`datafusion_proto::physical_plan::PhysicalExtensionCodec`

```rust
trait PhysicalExtensionCodec: Debug + Send + Sync + Any
```

Prose: [`api/datafusion_proto.physical_plan.md`](../api/datafusion_proto.physical_plan.md#physicalextensioncodec) · records: [`model/datafusion_proto.physical_plan.json`](../model/datafusion_proto.physical_plan.json)

## Required

Every implementation must supply these.

```rust
fn try_decode(&self, buf: &[u8], inputs: &[Arc<dyn ExecutionPlan>], ctx: &TaskContext, proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<Arc<dyn ExecutionPlan>>
fn try_encode(&self, node: Arc<dyn ExecutionPlan>, buf: &mut Vec<u8>, proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<()>
```

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
fn try_decode_expr(&self, _buf: &[u8], _inputs: &[Arc<dyn PhysicalExpr>], _ctx: &PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
fn try_decode_higher_order_function(&self, name: &str, _buf: &[u8]) -> Result<Arc<HigherOrderUDF>>
fn try_decode_udaf(&self, name: &str, _buf: &[u8]) -> Result<Arc<AggregateUDF>>
fn try_decode_udf(&self, name: &str, _buf: &[u8]) -> Result<Arc<ScalarUDF>>
fn try_decode_udwf(&self, name: &str, _buf: &[u8]) -> Result<Arc<WindowUDF>>
fn try_encode_expr(&self, _node: &Arc<dyn PhysicalExpr>, _buf: &mut Vec<u8>, _ctx: &PhysicalExprEncodeCtx<'_>) -> Result<()>
fn try_encode_higher_order_function(&self, _node: &HigherOrderUDF, _buf: &mut Vec<u8>) -> Result<()>
fn try_encode_udaf(&self, _node: &AggregateUDF, _buf: &mut Vec<u8>) -> Result<()>
fn try_encode_udf(&self, _node: &ScalarUDF, _buf: &mut Vec<u8>) -> Result<()>
fn try_encode_udwf(&self, _node: &WindowUDF, _buf: &mut Vec<u8>) -> Result<()>
```

## Implementors (3)

Read one before writing your own.

- `datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec`
- `datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec`
- `datafusion_proto::physical_plan::DefaultPhysicalExtensionCodec`

## Demonstrated by 3 upstream example(s)

- [`corpus/examples/custom_data_source/adapter_serialization.rs`](../corpus/examples/custom_data_source/adapter_serialization.rs)
- [`corpus/examples/proto/composed_extension_codec.rs`](../corpus/examples/proto/composed_extension_codec.rs)
- [`corpus/examples/proto/expression_deduplication.rs`](../corpus/examples/proto/expression_deduplication.rs)
