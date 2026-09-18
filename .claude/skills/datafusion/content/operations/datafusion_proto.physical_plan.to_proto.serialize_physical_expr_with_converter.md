# `datafusion_proto::physical_plan::to_proto::serialize_physical_expr_with_converter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.to_proto.serialize_physical_expr_with_converter.json).

<a id="op-6e0f7af93d31de799d772936"></a>
## serialize_physical_expr_with_converter

`function` · `datafusion_proto::physical_plan::to_proto::serialize_physical_expr_with_converter` · datafusion-proto 55.1.0

```rust
fn serialize_physical_expr_with_converter(value: &std::sync::Arc<dyn PhysicalExpr>, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<protobuf::PhysicalExprNode>
```

Source: `src/physical_plan/to_proto.rs:269`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Serialize a `PhysicalExpr` to default protobuf representation.

If required, a [`PhysicalExtensionCodec`](../operations/datafusion_proto.physical_plan.PhysicalExtensionCodec.md#op-850a12f382d8c2d858dedae2) can be provided which can handle
serialization of udfs requiring specialized serialization (see [`PhysicalExtensionCodec::try_encode_udf`](../operations/datafusion_proto.physical_plan.PhysicalExtensionCodec.md#op-a736e58f5dfc7fe25d5e9e61)).
A [`PhysicalProtoConverterExtension`](../operations/datafusion_proto.physical_plan.PhysicalProtoConverterExtension.md#op-30fa0e8177aa13ed42ddfcac) can be provided to handle the
conversion process (see [`PhysicalProtoConverterExtension::physical_expr_to_proto`](../operations/datafusion_proto.physical_plan.PhysicalProtoConverterExtension.md#op-ca4fdd61baa82205bbf47418)).
