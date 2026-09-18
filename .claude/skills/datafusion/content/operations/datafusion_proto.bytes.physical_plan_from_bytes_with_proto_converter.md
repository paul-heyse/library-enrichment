# `datafusion_proto::bytes::physical_plan_from_bytes_with_proto_converter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.bytes.physical_plan_from_bytes_with_proto_converter.json).

<a id="op-b05639f37ecd9e32d363606a"></a>
## physical_plan_from_bytes_with_proto_converter

`function` · `datafusion_proto::bytes::physical_plan_from_bytes_with_proto_converter` · datafusion-proto 55.1.0

```rust
fn physical_plan_from_bytes_with_proto_converter(bytes: &[u8], ctx: &datafusion_execution::TaskContext, extension_codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

Source: `src/bytes/mod.rs:279`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Deserialize a PhysicalPlan from bytes
