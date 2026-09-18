# `datafusion_proto::bytes::physical_plan_to_bytes_with_proto_converter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.bytes.physical_plan_to_bytes_with_proto_converter.json).

<a id="op-feecb8fe1f42ae97960e5ce7"></a>
## physical_plan_to_bytes_with_proto_converter

`function` · `datafusion_proto::bytes::physical_plan_to_bytes_with_proto_converter` · datafusion-proto 55.1.0

```rust
fn physical_plan_to_bytes_with_proto_converter(plan: std::sync::Arc<dyn ExecutionPlan>, extension_codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<prost::bytes::Bytes>
```

Source: `src/bytes/mod.rs:221`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Serialize a PhysicalPlan as bytes, using the provided extension codec
and protobuf converter.
